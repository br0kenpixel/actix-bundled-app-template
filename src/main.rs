use actix_files::Files;
use actix_web::{web::Data, App, HttpServer};

mod routes;
mod startup;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Extracting assets...");
    let asset_dir = startup::AssetExtractor::extract_assets()?;
    let static_dir = asset_dir.path().join("static");

    println!("Loading templates...");
    let tera = Data::new(startup::setup_templates(asset_dir.path())?);

    println!("Starting server...");
    HttpServer::new(move || {
        App::new()
            .app_data(tera.clone())
            .service(Files::new("/img", static_dir.join("img")))
            .service(Files::new("/js", static_dir.join("js")))
            .service(Files::new("/styles", static_dir.join("css")))
            .service(routes::index::index)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
