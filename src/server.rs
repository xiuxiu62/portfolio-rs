use crate::error::Result;

use warp::{Filter, Reply};

use std::fmt::{self, Display};

pub struct Server<'a> {
    host: [u8; 4],
    port: u16,
    _content_directory: Option<&'a str>,
}

impl<'a> Server<'a> {
    pub fn new(host: [u8; 4], port: u16, _content_directory: Option<&'a str>) -> Self {
        Self {
            host,
            port,
            _content_directory,
        }
    }

    pub async fn start<F>(&self, filter: F) -> Result<()>
    where
        F: Filter + Clone + Send + Sync + 'static,
        F::Extract: Reply,
    {
        println!("Listeneing on {self}");
        warp::serve(filter).run((self.host, self.port)).await;
        Ok(())
    }
}

impl<'a> Display for Server<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let port = self.port;
        let host = self
            .host
            .iter()
            .enumerate()
            .fold("".to_owned(), |acc, (i, n)| {
                acc + format!("{n}").as_str()
                    + match i {
                        3 => "",
                        _ => ".",
                    }
            });

        write!(f, "{host}:{port}")
    }
}
