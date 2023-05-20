use crate::utils::get_timestamp;
use actix_web::http::header::ContentType;
use actix_web::{get, web, Error, HttpResponse};
use quick_xml::se::to_string;
use serde::{Deserialize, Serialize};
use tracing::error;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ListAllMyBucketsResult {
    pub owner: Owner,
    pub buckets: Vec<Bucket>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Owner {
    pub id: String,
    pub display_name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Bucket {
    pub name: String,
    pub creation_date: String,
}

/// List Buckets (GET Bucket) (https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListBuckets.html)
#[get("/")]
pub async fn list_buckets(data: web::Data<crate::AppState>) -> Result<HttpResponse, Error> {
    let working_folder = &data.working_folder;
    let entities = std::fs::read_dir(working_folder)?;

    let buckets: Vec<Bucket> = entities
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let metadata = entry.metadata().ok()?;
            if metadata.is_dir() {
                let bucket = Bucket {
                    name: entry.file_name().into_string().ok()?,
                    creation_date: get_timestamp(entry.metadata().ok()?.created().ok()?),
                };
                Some(bucket) // We directly add the bucket to the list
            } else {
                None
            }
        })
        .collect();

    let list_result = to_string(&ListAllMyBucketsResult {
        owner: Owner {
            id: "OWNER_TEST".to_string(),
            display_name: "TEST OWNER".to_string(),
        },
        buckets,
    });

    let list_result = match list_result {
        Ok(result) => result,
        Err(err) => {
            error!("Error: {:?}", err);
            return Ok(HttpResponse::InternalServerError().body("Error"));
        }
    };

    println!("Buckets: {:?}", list_result);

    Ok(HttpResponse::Ok()
        .content_type(ContentType::xml())
        .body(list_result))
}
