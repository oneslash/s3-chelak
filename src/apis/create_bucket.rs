use actix_web::{put, web, Error, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateBucketConfiguration {
    #[serde(rename = "LocationConstraint")]
    pub location_constraint: String,
}

#[put("/{bucket_name}")]
async fn create_bucket(bucket_name: web::Path<String>) -> Result<HttpResponse, Error> {
    todo!()
}
