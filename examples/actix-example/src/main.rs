use std::sync::Arc;

use actix_web::{web, App, HttpResponse, HttpServer};
use models::{post::Post, Session};
use serde::Deserialize;

#[macro_use]
extern crate actix_web;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    let conn = models::init_db().await;

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(conn.clone()))
            .service(web::scope("/api").configure(get_routes))
    })
    .bind(("127.0.0.1", 7000))?
    .run();

    println!("Starting server at http://127.0.0.1:7000");

    server.await
}

#[derive(Clone)]
pub struct AppState {
    pub conn: Arc<Session>,
}

fn get_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_posts)
        .service(get_post)
        .service(create_post);
}

#[get("/posts")]
async fn get_posts(conn: web::Data<Session>) -> HttpResponse {
    let posts = Post::find_all(&conn).await;

    HttpResponse::Ok().json(posts)
}

#[get("/posts/{id}")]
async fn get_post(conn: web::Data<Session>, paths: web::Path<(String,)>) -> HttpResponse {
    let post = Post::find_one(&conn, &paths.0).await;

    HttpResponse::Ok().json(post)
}

#[post("/posts")]
async fn create_post(conn: web::Data<Session>, post: web::Json<PostCreateRequest>) -> HttpResponse {
    let post = Post::create_post(&conn, &post.title, &post.text).await;

    HttpResponse::Created().json(post)
}

#[derive(Debug, Deserialize)]
struct PostCreateRequest {
    title: String,
    text: String,
}
