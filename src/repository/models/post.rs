// use crate::repository::schema::posts;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

type DbError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

use crate::repository::schema::posts;
#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub title: String,
    pub body: String,
}
