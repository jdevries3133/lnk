use axum::{
    response::IntoResponse,
    http::{header, HeaderMap}
};

/// Take wrap some string response content with a text/html content-type header
pub fn html_response(content: String) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/html".parse().unwrap());
    (headers, content)
}
