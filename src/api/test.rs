use actix_web::{
    delete, get, guard, post, put,
    web::{block, scope, Data, Json, Path},
    Error, HttpResponse, Scope,
};

#[get("/")]
pub async fn post_counter() -> Result<HttpResponse, Error> {
    HttpResponse.
}

pub fn routes_tests() -> Scope {
    scope("/tests").service(post_counter)
}
