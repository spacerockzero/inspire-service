use actix_web::{web, App, HttpRequest, HttpServer, Responder};
mod content;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn quote() -> impl Responder {
    // let quote = "im a quote";
    let quote = content::get_random_quote();
    quote
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/quote", web::get().to(quote))
            .route("/{name}", web::get().to(greet))
    })
    .workers(12)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
