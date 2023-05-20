use actix_web::{error, put, web, Error, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateBucketConfiguration {
    #[serde(rename = "LocationConstraint")]
    pub location_constraint: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct CreateBucketError {
    pub error: BucketError,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct BucketError {
    pub code: String,
    pub message: String,
    pub resource: String,
    pub request_id: String,
    pub host_id: String,
}

#[put("/{bucket_name}")]
async fn create_bucket(
    bucket_name: web::Path<String>,
    data: web::Data<crate::AppState>,
) -> Result<HttpResponse, Error> {
    let bucket_name = bucket_name.into_inner();
    let working_folder = &data.working_folder;

    let bucket_path = format!("{}/{}", working_folder, bucket_name);
    let bucket_path = std::path::Path::new(&bucket_path);

    match std::fs::create_dir(bucket_path) {
        Ok(_) => {
            Ok(HttpResponse::Ok().body(format!("Bucket {} created successfully.", bucket_name)))
        }
        Err(e) => {
            tracing::error!("Cannot create bucket: {:?}", e);
            match e.kind() {
                std::io::ErrorKind::AlreadyExists => {
                    let err = CreateBucketError {
                        error: BucketError {
                            code: "BucketAlreadyExists".to_string(),
                            message: "The requested bucket name is not available.".to_string(),
                            resource: bucket_name,
                            request_id: "00".to_string(),
                            host_id: "00".to_string(),
                        },
                    };

                    let err = quick_xml::se::to_string(&err).unwrap();
                    Err(error::ErrorConflict(err))
                }
                _ => Err(error::ErrorInternalServerError(e)),
            }
        }
    }
}
