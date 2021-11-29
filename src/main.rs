mod api;
mod error;
mod server;
mod static_site;

use api::api_filter;
use error::Result;
use server::Server;
use static_site::static_site_filter;

use warp::Filter;

const HOST: [u8; 4] = [127, 0, 0, 1];
const PORT: u16 = 3000;
pub(crate) const WWW: &str = "www";

#[tokio::main]
async fn main() -> Result<()> {
    let content = warp::fs::dir(WWW);
    // let server = Server::new(HOST, PORT, Some(WWW));
    let api = api_filter();
    let static_site = static_site_filter(content);
    let router = api.or(static_site);

    // Start Server
    // server.start(router).await?;
    format_listening(HOST, PORT);
    warp::serve(router).run((HOST, PORT)).await;

    Ok(())
}

fn format_listening(host: [u8; 4], port: u16) -> String {
    let host = HOST.iter().enumerate().fold("".to_owned(), |acc, (i, n)| {
        acc + format!("{n}").as_str()
            + match i {
                3 => "",
                _ => ".",
            }
    });

    format!("Listeneing on {host}:{PORT}")
}
