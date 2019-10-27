use std::io;
use std::io::Read;

use failure::Error;

mod docker_transport;

pub enum Connector {
    DockerTest { base_image: String },
}

// This should really deal with the ability for ssh to drop files, instead of us moving them
// around through memory. But, does it even really matter?
pub trait Transport {
    fn write_all_from<R: Read>(&mut self, from: R) -> Result<(), Error>
    where
        Self: Sized;

    fn write_all(&mut self, bytes: &[u8]) -> Result<(), Error>
    where
        Self: Sized,
    {
        self.write_all_from(io::Cursor::new(bytes))
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), Error>;

    // non-move, or return self on error?
    fn shutdown(self: Box<Self>) -> Result<(), Error>;
}

impl Connector {
    pub fn connect(self) -> Result<Box<dyn Transport>, Error> {
        Ok(match self {
            Connector::DockerTest { base_image } => {
                Box::new(docker_transport::DockerTransport::new(base_image)?)
            }
        })
    }
}
