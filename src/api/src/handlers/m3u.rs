use std::{convert::Infallible, ffi::OsStr, path::PathBuf};

use tokio::fs::{read_dir, DirEntry, File};
use tokio_util::io::ReaderStream;

use warp::{hyper::Body, reply::Response};

pub async fn get_latest_m3u_file() -> Result<Response, Infallible> {
    let path = get_latest_m3u_path().await;

    let file = File::open(path)
        .await
        .expect("Could not open m3u file from disc");

    let stream = ReaderStream::new(file);

    let body = Body::wrap_stream(stream);

    let response = warp::hyper::Response::builder()
        .status(200)
        .header(
            "Content-Disposition",
            "attachement; filename = \"playlist.m3u\"",
        )
        .body(body)
        .unwrap_or_default();

    Ok(response)
}

async fn get_latest_m3u_path() -> PathBuf {
    let mut dir = read_dir(".").await.unwrap();

    let mut files: Vec<DirEntry> = vec![];

    while let Some(file) = dir.next_entry().await.unwrap_or_default() {
        let extension = file
            .path()
            .extension()
            .and_then(OsStr::to_str)
            .unwrap_or_default()
            .to_owned();

        if extension == "m3u" {
            files.push(file)
        }
    }

    let mut paths: Vec<PathBuf> = files.iter().map(|file| file.path()).collect();
    paths.sort();

    let freshesh_file = paths.last().unwrap().to_path_buf();

    freshesh_file
}