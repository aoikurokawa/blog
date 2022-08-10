use crate::routes::{
    get_blog_by_id, get_blogs, get_categories, health_check, post_blog, post_category,
};
use actix_cors::Cors;
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = Data::new(db_pool);
    let server = HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .route("/health_check", web::get().to(health_check))
            .route("/blogs", web::get().to(get_blogs))
            .route("/blog/{id}", web::get().to(get_blog_by_id))
            .route("/blog", web::post().to(post_blog))
            .route("/categories", web::get().to(get_categories))
            .route("/category", web::post().to(post_category))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
