use std::fs;

use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use clap::Parser;

pub mod apis;
pub mod utils;

async fn not_found(request: HttpRequest) -> impl Responder {
    HttpResponse::NotFound().body(format!("Cannot find {}.", request.path()))
}

#[derive(Debug, Clone, Parser)]
#[command(version, author, about, long_about = None)]
struct Config {
    #[clap(long, default_value = "localhost")]
    server_url: String,
    #[clap(long, default_value = "9090")]
    server_port: String,
    #[clap(long, default_value = "/tmp/s3-server")]
    working_folder: String,
}

#[derive(Debug, Clone)]
pub struct AppState {
    working_folder: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    let config = Config::parse();

    // Test Read
    fs::read_dir(config.working_folder.clone())?;

    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(apis::list_buckets::list_buckets)
            .default_service(web::route().to(not_found))
            .app_data(web::Data::new(AppState {
                working_folder: config.working_folder.clone(),
            }))
    });

    let server_url = format!("{}:{}", config.server_url, config.server_port);
    let server = server.bind(&server_url).unwrap();

    println!("Starting server at {server_url}");
    server.run().await.unwrap();

    Ok(())
}
