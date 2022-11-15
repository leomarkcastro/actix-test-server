use std::sync::Mutex;

use actix_web::{
    delete, get, guard, post, put,
    web::{block, scope, Data, Json, Path},
    Error, HttpResponse, Scope,
};

use crate::types::db::DbPool;
use crate::{repository::models::post::NewPost, services::post};

// This struct represents state
pub struct AppState {
    calls: Mutex<i32>,
}

#[get("/list")]
pub async fn get_post(pool: Data<DbPool>, state: Data<AppState>) -> Result<HttpResponse, Error> {
    // use web::block to offload blocking Diesel code without blocking server thread
    let posts = block(move || {
        let mut _calls = state.calls.lock().unwrap();
        *_calls += 1;

        let mut conn = pool.get()?;
        post::get(&mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(posts) = posts {
        Ok(HttpResponse::Ok().json(posts))
    } else {
        let res = HttpResponse::NotFound().body("No Posts Yet");
        Ok(res)
    }
}

#[get("/get/{post_id}")]
pub async fn get_post_by_id(pool: Data<DbPool>, post_id: Path<i32>) -> Result<HttpResponse, Error> {
    let post_id_parsed = post_id.into_inner();

    // use web::block to offload blocking Diesel code without blocking server thread
    let posts = block(move || {
        let mut conn = pool.get()?;
        post::get_one(&mut conn, post_id_parsed)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(posts) = posts {
        Ok(HttpResponse::Ok().json(posts))
    } else {
        let res = HttpResponse::NotFound().body(format!("No post found with id: {post_id_parsed}"));
        Ok(res)
    }
}

#[post("/new")]
pub async fn new_post(pool: Data<DbPool>, form: Json<NewPost>) -> Result<HttpResponse, Error> {
    // use web::block to offload blocking Diesel code without blocking server thread
    let post = block(move || {
        let mut conn = pool.get()?;
        post::post(&mut conn, form.title.clone(), form.body.clone())
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(post))
}

#[put("/publish/{post_id}")]
pub async fn publish_post(pool: Data<DbPool>, post_id: Path<i32>) -> Result<HttpResponse, Error> {
    let post_id_parsed = post_id.into_inner();

    // use web::block to offload blocking Diesel code without blocking server thread
    let posts = block(move || {
        let mut conn = pool.get()?;
        post::publish(&mut conn, post_id_parsed)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(posts) = posts {
        Ok(HttpResponse::Ok().json(posts))
    } else {
        let res = HttpResponse::NotFound().body(format!("No post found with id: {post_id_parsed}"));
        Ok(res)
    }
}

#[delete("/delete/{post_id}")]
pub async fn delete_post(pool: Data<DbPool>, post_id: Path<i32>) -> Result<HttpResponse, Error> {
    let post_id_parsed = post_id.into_inner();

    // use web::block to offload blocking Diesel code without blocking server thread
    let posts = block(move || {
        let mut conn = pool.get()?;
        post::delete(&mut conn, post_id_parsed)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(posts) = posts {
        Ok(HttpResponse::Ok().json(posts))
    } else {
        let res = HttpResponse::NotFound().body(format!("No post found with id: {post_id_parsed}"));
        Ok(res)
    }
}

#[get("/ctx")]
pub async fn post_counter(state: Data<AppState>) -> Result<HttpResponse, Error> {
    let _calls = state.calls.lock().unwrap();
    Ok(HttpResponse::Ok().json(*_calls))
}

pub fn routes_posts() -> Scope {
    let app_state = Data::new(AppState {
        calls: Mutex::new(0),
    });
    scope("/posts")
        // .guard(guard::Header("Host", "www.rust-lang.org"))
        .app_data(app_state)
        .service(get_post)
        .service(get_post_by_id)
        .service(new_post)
        .service(publish_post)
        .service(delete_post)
        .service(post_counter)
}
