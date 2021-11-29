use warp::Filter;

pub fn api_filter() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let api_prefix = warp::path!("api" / "v1");
    let ping = warp::path("ping")
        .and(warp::get())
        .and(warp::path::end())
        .map(|| "pong");

    api_prefix.and(ping)
}
