use failure::bail;
use failure::err_msg;
use failure::format_err;
use failure::Error;
use failure::ResultExt;
use log::info;

use super::Transport;

macro_rules! builtin_worker {
    () => {
        include_bytes!("../../../target/x86_64-unknown-linux-musl/release/stricter-worker")
    };
}

/// Because we're uploading the launch code every time, it doesn't have to deal
/// with protocols or feature detection or etc.
///
/// Feature discovery:
///  * test if `gzip` is working
///  * arch
///  * writeable file systems?
///  * ...
///
/// Reuse:
///  * Ask an existing binary, if any, if it can talk our protocol version.
///
/// If yes:
///  * exec it
///
/// If no:
///  * request a new binary, with encoding options?
///  * write binary
///  * re-test?
///  * exec it
pub fn init(transport: &mut dyn Transport) -> Result<(), Error> {
    let cache_dir = "~/.cache/stricter";
    let worker = format!("{}/worker", cache_dir);
    let version = env!("VERGEN_SHA");

    let binary = &builtin_worker!()[..];

    let script = format!(
        "D={cache_dir}; set -x && {banner} && ({happy_path} || {upload} && {happy_path})\n",
        cache_dir = cache_dir,
        banner = "echo H",
        happy_path = format!(
            "([ -x \"$D/worker\" ] && \"$D/worker\" --accept {version} && exec \"$D/worker\" --start)",
            version = version,
        ),
        upload = format!(
            "(echo N && mkdir -p \"$D\" && dd ibs=1 of=\"$D/worker\" count={size} && chmod a+x \"$D/worker\")",
            size = binary.len(),
        )
    );
    transport.write_all(script.as_bytes())?;
    let h = read_code(transport)
        .with_context(|_| err_msg("initial startup response code (banner?)"))?;

    if b'H' != h {
        bail!("invalid startup code: {:?}", h);
    }

    let u = read_code(transport).with_context(|_| err_msg("upload request response code"))?;

    match u {
        b'N' => {
            info!("needs upload");
            transport.write_all(binary)?;
            // upload required
        }

        b'U' => return Ok(()),

        other => bail!("invalid upload code: {:?}", u),
    }

    let u2 = read_code(transport).with_context(|_| err_msg("start request response code"))?;

    if b'U' != u2 {
        bail!("invalid start request code: {:?}", u2);
    }

    Ok(())
}

fn read_code(transport: &mut dyn Transport) -> Result<u8, Error> {
    let mut buf = [0u8; 2];
    transport.read_exact(&mut buf)?;
    if b'\n' == buf[1] {
        Ok(buf[0])
    } else {
        Err(format_err!("sync error, buf was {:?}", buf))
    }
}
