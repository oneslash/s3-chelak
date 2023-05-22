use actix_router::{Path, ResourceDef, Url};

use crate::apis::create_bucket::create_bucket;
use crate::apis::put_object::put_object;
use actix_web::{put, web, Error, HttpRequest, HttpResponse};

#[put("/{tail}*")]
pub async fn handle_put(
    _: web::Path<String>,
    req: HttpRequest,
    data: web::Data<crate::AppState>,
    payload: Option<web::Payload>,
) -> Result<HttpResponse, Error> {
    let mut path = Path::new(Url::new(req.uri().clone()));

    let routes = [
        ("/{bucket}/{object_name}", "bucket and object"),
        ("/{bucket}", "bucket"),
    ];

    for (pattern, description) in &routes {
        if ResourceDef::new(*pattern).capture_match_info(&mut path) {
            return match (path.get("bucket"), path.get("object_name")) {
                (Some(bucket), Some(object_name)) if *description == "bucket and object" => {
                    put_object(
                        bucket.to_string(),
                        object_name.to_string(),
                        data,
                        payload.unwrap(),
                    )
                    .await
                }
                (Some(bucket), None) if *description == "bucket" => {
                    create_bucket(bucket.to_string(), data).await
                }
                _ => return Ok(HttpResponse::InternalServerError().finish()),
            };
        }
    }

    Err(actix_web::error::ErrorNotFound("Not Found"))
}
