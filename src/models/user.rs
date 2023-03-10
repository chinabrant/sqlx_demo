use serde::{Deserialize, Serialize};
use actix_web::{Responder, HttpResponse, http::header::ContentType, body::BoxBody };

#[derive(Debug, Serialize)]
pub struct User {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct NewUser {
    pub name: String,
}

impl Responder for User {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok().content_type(ContentType::json()).body(body)
    }
}