use crate::errors::AppError;
use crate::routes::convert;
use crate::{models, Pool};
use actix_web::{web, Error, HttpResponse};
use diesel::prelude::*;
use futures::Future;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct PostInput {
    title: String,
    body: String,
}

async fn add_post(
    user_id: web::Path<i32>,
    post: web::Json<PostInput>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || {
        let conn = &pool.get().unwrap();
        let key = models::UserKey::ID(user_id.into_inner());
        models::find_user(conn, key).and_then(|user| {
            let post = post.into_inner();
            let title = post.title;
            let body = post.body;
            models::create_post(conn, &user, title.as_str(), body.as_str())
        })
    })
    .await
    .map(|post| HttpResponse::Created().json(post))
    .map_err(|_| HttpResponse::InternalServerError())?)
}
