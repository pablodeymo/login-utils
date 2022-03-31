use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct LoginReq {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginResp {
    pub token: String,
}

impl Into<HttpResponse> for LoginResp {
    fn into(self) -> HttpResponse {
        HttpResponse::Ok()
            .content_type("application/json")
            .body(serde_json::to_string(&self).unwrap())
    }
}
