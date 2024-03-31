use actix_web::{
    error::ResponseError,
    get,
    http::{header::ContentType, StatusCode},
    post, put,
    web::Data,
    web::Json,
    web::Path,
    HttpResponse,
};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[get("/task/{task_global_id}")]

pub async fn get_task() -> Json<String> {
    return Json("Hell world".to_string());
}
