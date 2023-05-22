use actix_web::{web, Error, HttpResponse};
use futures::StreamExt;
use std::fs::File;
use std::io::Write;

pub async fn put_object(
    bucket_name: String,
    object_name: String,
    data: web::Data<crate::AppState>,
    mut payload: web::Payload,
) -> Result<HttpResponse, Error> {
    let working_folder = &data.working_folder;

    let filepath = format!("{}/{}/{}", working_folder, bucket_name, object_name);
    let mut file = File::create(&filepath)?;

    while let Some(bytes) = payload.next().await {
        let data = bytes.unwrap();
        file.write_all(&data)?;
    }

    Ok(HttpResponse::Ok().finish())
}
