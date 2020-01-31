use std::path::PathBuf;

#[rustfmt::skip]
use warp::{
    filters::BoxedFilter,
    fs::File,
    path::Peek,
    path,
    Filter, Reply,
};

fn main() {
    let localhost = [0, 0, 0, 0];
    let port = 8000;
    let addr = (localhost, port);

    // You will need to change this if you use this as a template for your application.
    const ASSETS_DIR: &str = "../static";
    let assets_dir: PathBuf = PathBuf::from(ASSETS_DIR);

    let routes = static_files_handler(assets_dir);

    warp::serve(routes).run(addr);
}


/// Expose filters that work with static files
pub fn static_files_handler(assets_dir: PathBuf) -> BoxedFilter<(impl Reply,)> {
    const INDEX_HTML: &str = "index.html";

    let files =
        assets(assets_dir.clone())
        .or(index_static_file_redirect(assets_dir.join(INDEX_HTML)));

    warp::any().and(files).boxed()
}

/// If the path does not start with /api, return the index.html, so the app will bootstrap itself
/// regardless of whatever the frontend-specific path is.
fn index_static_file_redirect(index_file_path: PathBuf) -> BoxedFilter<(impl Reply,)> {
    warp::get2()
        .and(warp::fs::file(index_file_path))
        .boxed()
}

/// Gets the file within the specified dir.
fn assets(dir_path: PathBuf) -> BoxedFilter<(impl Reply,)> {
    warp::get2()
        .and(warp::fs::dir(dir_path))
        .and(warp::path::end())
        .boxed()
}
