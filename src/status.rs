use actix_web::{get, HttpResponse, Responder};
use serde_json::json;

const TRAVIS_BRANCH: Option<&str> = option_env!("TRAVIS_BRANCH");
const TRAVIS_BUILD_NUMBER: Option<&str> = option_env!("TRAVIS_BUILD_NUMBER");
const TRAVIS_COMMIT: Option<&str> = option_env!("TRAVIS_COMMIT");

#[get("/status")]
pub async fn status() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "system": "{{project-name}}",
        "branch": TRAVIS_BRANCH.unwrap_or("no branch ID"),
        "build-id": TRAVIS_BUILD_NUMBER.unwrap_or("no build ID"),
        "commit": TRAVIS_COMMIT.unwrap_or("no commit SHA1"),
    }))
}
