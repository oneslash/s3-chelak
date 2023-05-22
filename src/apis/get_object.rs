use actix_web::{error, head, web, Error, HttpRequest, HttpResponse};
use std::fs::File;
use std::io::Read;

pub async fn get_object(
    bucket_name: String,
    object_name: String,
    data: web::Data<crate::AppState>,
) -> Result<HttpResponse, Error> {
    let working_folder = &data.working_folder;
    let filepath = format!("{}/{}/{}", working_folder, bucket_name, object_name);

    let mut file = File::open(filepath)?;
    if file.metadata()?.is_file() {
        let mut content = Vec::new();
        file.read_to_end(&mut content)?;
        return Ok(HttpResponse::Ok().body(content));
    }

    Err(error::ErrorNotFound("Not Found"))
}

#[head("/{bucket_name}/{object_name}")]
pub async fn get_object_head(
    req: HttpRequest,
    data: web::Data<crate::AppState>,
) -> Result<HttpResponse, Error> {
    let working_folder = &data.working_folder;
    let bucket_name = req.match_info().get("bucket_name").unwrap();
    let object_name = req.match_info().get("object_name").unwrap();

    let filepath = format!("{}/{}/{}", working_folder, bucket_name, object_name);
    let file = File::open(&filepath)?;

    if file.metadata()?.is_file() {
        let guess = mime_guess::from_path(&filepath).first_or_octet_stream();
        let mut resp = HttpResponse::Ok();
        resp.append_header(("Content-Length", file.metadata()?.len().to_string()));
        resp.append_header(("Content-Type", guess.to_string()));
        resp.append_header((
            "Last-Modified",
            crate::utils::get_timestamp(file.metadata()?.modified()?),
        ));

        return Ok(resp.finish());
    }

    Ok(HttpResponse::Ok().finish())
}
