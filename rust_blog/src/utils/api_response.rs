// pub struct ApiResponse {
//     pub status: String,
//     pub message: String,
//     pub data: Option<serde_json::Value>,
// }

use actix_web::{HttpResponse, Responder, body::BoxBody, http::StatusCode, web};

pub struct ApiResponse {
    pub status_code: u16,
    pub body: String,
    response_code: StatusCode,
}

impl ApiResponse {
    pub fn new(status_code: u16, body: String) -> Self {
        let response_code =
            StatusCode::from_u16(status_code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        Self {
            status_code,
            body,
            response_code,
        }
    }
}

impl Responder for ApiResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let body: BoxBody = BoxBody::new(web::BytesMut::from(self.body.as_bytes()));
        HttpResponse::new(self.response_code).set_body(body)

        // actix_web::HttpResponse::build(self.response_code).body(self.body)
    }
}
