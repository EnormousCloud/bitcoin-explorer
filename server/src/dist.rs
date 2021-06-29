use crate::State;
use std::path::{Path, PathBuf};
use std::{ffi::OsStr, io};
use tide::{Body, Next, Request, Response, StatusCode};

// This is an example of middleware that keeps its own state and could
// be provided as a third party crate
#[derive(Default)]
pub struct Middleware {}

impl Middleware {
    pub fn new() -> Self {
        Self {}
    }
}

#[tide::utils::async_trait]
impl tide::Middleware<State> for Middleware {
    async fn handle(&self, req: Request<State>, next: Next<'_, State>) -> tide::Result {
        let path = req.url().path().to_owned();
        let method = req.method().to_string();

        if method == "GET" && path != "/" {
            let dir = PathBuf::from(req.state().static_dir.clone());
            let path = path.trim_start_matches('/');
            let mut file_path = dir.clone();
            for p in Path::new(path) {
                if p == OsStr::new(".") {
                    continue;
                } else if p == OsStr::new("..") {
                    file_path.pop();
                } else {
                    file_path.push(&p);
                }
            }
            let file_path = async_std::path::PathBuf::from(file_path);
            if !file_path.starts_with(&dir) {
                tracing::warn!("Unauthorized attempt to read: {:?}", file_path);
                return Ok(Response::new(StatusCode::Forbidden));
            } else {
                return match Body::from_file(&file_path).await {
                    Ok(body) => Ok(Response::builder(StatusCode::Ok)
                        .header("Cache-Control", "public, max-age=604800, immutable")
                        .body(body)
                        .build()),
                    Err(e) if e.kind() == io::ErrorKind::NotFound => {
                        tracing::warn!("File not found: {:?}", &file_path);
                        Ok(Response::new(StatusCode::NotFound))
                    }
                    Err(e) => Err(e.into()),
                };
            }
        }

        Ok(next.run(req).await)
    }
}
