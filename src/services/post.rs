// use crate::repository::schema::posts;
use diesel::prelude::*;

use crate::repository::{
    models::post::{NewPost, Post},
    schema::posts::{dsl::*, table},
};

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn get(conn: &mut PgConnection) -> Result<Option<Vec<Post>>, DbError> {
    let get_posts = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(conn)
        .optional()?;

    Ok(get_posts)
}

pub fn get_one(conn: &mut PgConnection, post_id: i32) -> Result<Option<Post>, DbError> {
    let get_posts = posts
        .filter(id.eq(post_id))
        .first::<Post>(conn)
        .optional()?;

    Ok(get_posts)
}

pub fn post(
    conn: &mut PgConnection,
    post_title: String,
    post_body: String,
) -> Result<NewPost, DbError> {
    let new_post = NewPost {
        title: post_title,
        body: post_body,
    };

    diesel::insert_into(table).values(&new_post).execute(conn)?;

    Ok(new_post)
}

pub fn publish(conn: &mut PgConnection, id_to_publish: i32) -> Result<Option<Post>, DbError> {
    let updated_post = diesel::update(posts.find(id_to_publish))
        .set(published.eq(true))
        .get_result::<Post>(conn)
        .optional()?;

    Ok(updated_post)
}

pub fn delete(conn: &mut PgConnection, id_to_delete: i32) -> Result<Option<Post>, DbError> {
    let deleted_post = diesel::delete(posts.find(id_to_delete))
        .get_result(conn)
        .optional()?;

    Ok(deleted_post)
}
