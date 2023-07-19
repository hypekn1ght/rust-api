use {
    actix_web::HttpResponse,
    actix_web::http::header,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct NotFoundMessage {
    message: String,
}

impl NotFoundMessage {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

pub enum ResponseType<T> {
    Ok(T),
    NotFound(T),
    Created(T),
}

impl<T: Serialize> ResponseType<T> {
    pub fn get_response(&self) -> HttpResponse {
        match self {
            ResponseType::Ok(payload) => HttpResponse::Ok()
                .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
                .content_type("application/json")
                .json(payload),
            ResponseType::NotFound(message) => HttpResponse::BadRequest()
                .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
                .content_type("application/json")
                .json(message),
            ResponseType::Created(payload) => HttpResponse::Created()
                .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
                .content_type("application/json")
                .json(payload),
        }
    }
}
