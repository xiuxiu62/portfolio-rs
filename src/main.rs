mod error;

use error::Result;

use warp::{Filter, Reply};

use std::fmt::{self, Display};

const HOST: [u8; 4] = [127, 0, 0, 1];
const PORT: u16 = 3000;
const WWW: &str = "www";

pub struct Server<'a> {
    host: [u8; 4],
    port: u16,
    content: Option<&'a str>,
}

impl<'a> Server<'a> {
    pub fn new(host: [u8; 4], port: u16, content: Option<&'a str>) -> Self {
        Self {
            host,
            port,
            content,
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

impl Display for Server<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
        let port = self.port;

        write!(f, "{host}:{port}")
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Api
    let ping = warp::get().and(warp::path("ping")).map(|| "pong");
    let api_prefix = warp::path!("api" / "v1");
    let api = api_prefix.and(ping);

    // Static Site
    let content = warp::fs::dir(WWW);

    let index = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file(format!("{WWW}/index.html")));
    let not_found = warp::get()
        .and(warp::any())
        .and(warp::fs::file(format!("{WWW}/404.html")));
    let static_site = content.or(index).or(not_found);

    let routes = api.or(static_site);

    // Start Server
    let server = Server::new(HOST, PORT, Some(WWW));
    server.start(routes).await?;
    // warp::serve(routes).run((HOST, PORT)).await;

    Ok(())
}
