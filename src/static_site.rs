use warp::Filter;

use crate::WWW;

pub fn static_site_filter(
    content: impl Filter<Extract = impl warp::Reply, Error = warp::Rejection>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> {
    let index = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file(format!("{WWW}/index.html")));
    let not_found = warp::get()
        .and(warp::any())
        .and(warp::fs::file(format!("{WWW}/404.html")));

    content.or(index).or(not_found)
}
