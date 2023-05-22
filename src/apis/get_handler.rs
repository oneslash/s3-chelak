use crate::apis::get_object::get_object;
use crate::apis::list_buckets;
use actix_router::{Path, ResourceDef, Url};
use actix_web::{get, web, Error, HttpRequest, HttpResponse};

#[get("/{tail}*")]
pub async fn handle_get(
    _: web::Path<String>,
    req: HttpRequest,
    data: web::Data<crate::AppState>,
) -> Result<HttpResponse, Error> {
    let mut path = Path::new(Url::new(req.uri().clone()));

    let routes = [
        ("/{bucket}/{object_name}", "bucket and object"),
        ("/", "get bucket"),
    ];

    for (pattern, description) in &routes {
        if ResourceDef::new(*pattern).capture_match_info(&mut path) {
            return match (path.get("bucket"), path.get("object_name")) {
                (Some(bucket), Some(object_name)) if *description == "bucket and object" => {
                    get_object(bucket.to_string(), object_name.to_string(), data).await
                }
                (None, None) if *description == "get bucket" => {
                    list_buckets::list_buckets(data).await
                }
                _ => return Ok(HttpResponse::InternalServerError().finish()),
            };
        }
    }

    Err(actix_web::error::ErrorNotFound("Not Found"))
}
