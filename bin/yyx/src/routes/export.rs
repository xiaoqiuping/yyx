use crate::helpers::*;
use crate::result::*;
use serde_json::{self, Value};
use std::path::Path;
use warp::{path, Filter, Rejection, Reply};
use yyx_data::save_exported_file;

pub fn export_json() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
  warp::post2()
    .and(path!("export" / "json" / String))
    .and(warp::body::json())
    .and_then(|name: String, value: Value| {
      block(move || -> Result<_, Rejection> {
        let pretty_json = serde_json::to_string_pretty(&value as &Value)
          .map_err(YyxError::internal)
          .map_err(warp::reject::custom)?;
        let name = save_exported_file(&name, &pretty_json)
          .map_err(YyxError::internal)
          .map_err(warp::reject::custom)?;
        Ok(warp::reply::json(&name))
      })
    })
}

pub fn files() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
  path("export-files")
    .and(warp::fs::dir("exports"))
    .and(warp::filters::path::full())
    .map(|file, fullpath: warp::filters::path::FullPath| {
      let path = Path::new(fullpath.as_str());
      let name = path
        .file_name()
        .map(|name| name.to_string_lossy().to_string())
        .unwrap_or_else(|| "noname".to_string());
      attachment(file, &name)
    })
}
