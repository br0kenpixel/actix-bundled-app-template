use actix_web::{get, web::Data, HttpResponse};
use tera::{Context, Tera};

#[get("/")]
async fn index(tera: Data<Tera>) -> HttpResponse {
    let body = tera.render("index.html.tera", &Context::new()).unwrap();

    HttpResponse::Ok().body(body)
}
