use std::io;
use std::io::Read;
use std::io::Write;
use std::process;

use failure::format_err;
use failure::Error;
use failure::ResultExt;

use super::Transport;

pub struct DockerTransport {
    child: process::Child,
}

impl DockerTransport {
    pub fn new(base_image: String) -> Result<DockerTransport, Error> {
        let child = process::Command::new("docker")
            .args(&[
                "run",
                "--rm",
                "--net",
                "host",
                "--interactive",
                &base_image,
                "/bin/sh",
            ])
            .stdin(process::Stdio::piped())
            .stdout(process::Stdio::piped())
            .stderr(process::Stdio::inherit())
            .spawn()
            .with_context(|_| format_err!("exec docker for {:?}", base_image))?;
        let mut transport = DockerTransport { child };
        transport.write_all(b"printf 'pad:%05d tab:\\t space:%4s\\n' '1234' 'ab'\n")?;
        const EXPECTED: [u8; 27] = *b"pad:01234 tab:\t space:  ab\n";
        let mut buf = [0u8; EXPECTED.len()];
        transport.read_exact(&mut buf)?;

        if EXPECTED != buf {
            Err(format_err!(
                "unexpected output: {:?}",
                String::from_utf8_lossy(&buf)
            ))?;
        }
        Ok(transport)
    }
}

impl Transport for DockerTransport {
    fn write_all_from<R: Read>(&mut self, mut from: R) -> Result<(), Error>
    where
        Self: Sized,
    {
        io::copy(&mut from, self.child.stdin.as_mut().expect("requested"))?;
        Ok(())
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), Error> {
        self.child
            .stdout
            .as_mut()
            .expect("requested")
            .read_exact(buf)?;
        Ok(())
    }

    fn shutdown(mut self: Box<Self>) -> Result<(), Error> {
        drop(self.child.stdin.take());
        self.child.wait()?;
        Ok(())
    }
}
