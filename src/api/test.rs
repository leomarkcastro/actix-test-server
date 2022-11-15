use actix_files as fs;

use actix_web::{
    get,
    http::header::{ContentDisposition, ContentType, DispositionType},
    post,
    web::{scope, Form, Json, Path, Query},
    Error, HttpRequest, HttpResponse, Scope,
};

use serde::{Deserialize, Serialize};

// ==============================================

#[get("/plain")]
pub async fn call_plain() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json("Hello"))
}

// ==============================================

#[derive(Deserialize)]
pub struct PathParams {
    object: String,
    method: String,
    id: i32,
}

// ==============================================

#[get("/path/{object}/{method}/{id}")]
pub async fn call_path(path_params: Path<PathParams>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(format!(
        "Received path params: {} {} {}",
        path_params.object, path_params.method, path_params.id
    )))
}

// ==============================================

#[derive(Deserialize)]
struct QueryQuery {
    val: String,
}

#[get("/query")]
async fn call_query(query: Query<QueryQuery>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(format!("Welcome {}!", query.val)))
}

// ==============================================

#[derive(Deserialize)]
pub struct PathQueryParams {
    param1: String,
}

#[derive(Deserialize)]
struct PathQueryQuery {
    val: String,
}

#[get("/param_query/{param1}")]
async fn call_paramquery(
    path_params: Path<PathQueryParams>,
    query: Query<PathQueryQuery>,
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(format!(
        "Params: {} || Query: {}",
        path_params.param1, query.val
    )))
}

// ==============================================

#[derive(Deserialize)]
struct PostJsonBodyJson {
    username: String,
}

#[post("/post_json")]
async fn call_postjson(body_json: Json<PostJsonBodyJson>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(format!("Body Json: {}", body_json.username)))
}

// ==============================================

#[derive(Deserialize)]
struct PostURLEncodedForm {
    username: String,
}

#[post("/post_form")]
async fn call_posturlencoded(body_form: Form<PostURLEncodedForm>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(format!("Body Form: {}", body_form.username)))
}

// ==============================================

#[derive(Serialize)]
struct SampleJSONRespJSON {
    username: String,
}

#[get("/respjson")]
async fn call_respjson() -> Result<HttpResponse, Error> {
    let body = serde_json::to_string(&SampleJSONRespJSON {
        username: "leo".to_string(),
    })
    .unwrap();

    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(body))
}

// ==============================================

fn call_file(mount_path: &str, serve_from: &str, file_listing: bool) -> actix_files::Files {
    let file = fs::Files::new(mount_path, serve_from);

    if file_listing {
        let file_proc = file.show_files_listing();
        return file_proc;
    }

    file.index_file("index.html")
}

// ==============================================

pub fn routes_tests() -> Scope {
    scope("/tests")
        .service(call_path)
        .service(call_plain)
        .service(call_query)
        .service(call_paramquery)
        .service(call_postjson)
        .service(call_posturlencoded)
        .service(call_respjson)
        .service(call_file("/static", "./static", false))
}
