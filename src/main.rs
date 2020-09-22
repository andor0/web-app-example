use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    envs: Vec<Env>
}

#[derive(Debug, Clone)]
#[derive(Template)]
#[template(source = "{{ key }} = {{ value }}", ext = "txt")]
struct Env {
    key: String,
    value: String,
}

async fn index(state: web::Data<Vec<Env>>) -> impl Responder {
    let body = IndexTemplate{ envs: state.to_vec() }
        .render()
        .expect("could not render template");
    HttpResponse::Ok().body(body)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let app_envs = std::env::vars()
        .filter(|env| env.0.starts_with("APP_"))
        .map(|env| Env { key: env.0, value: env.1 })
        .collect::<Vec<_>>();
    HttpServer::new(move || {
        App::new()
            .data(app_envs.clone())
            .route("/", web::get().to(index))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
