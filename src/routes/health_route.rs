use warp::{filters::BoxedFilter, Filter};

fn path_prefix() -> BoxedFilter<()> {
    warp::path("health").boxed()
}

pub fn health() -> BoxedFilter<()> {
    warp::get() // 3.
        .and(path_prefix()) // 4.
        .boxed()
}
