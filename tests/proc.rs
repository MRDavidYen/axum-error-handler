#[cfg(test)]
mod tests {
    use axum::{body::to_bytes, http::StatusCode, response::IntoResponse};
    use axum_error_handler::AxumErrorResponse;
    use thiserror::Error;

    #[derive(Debug, Error, AxumErrorResponse)]
    pub enum TestError {
        #[error("Bad request: {0}")]
        #[status_code("400")]
        #[code("BAD_REQUEST")]
        BadRequest(String),
        #[error("Internal server error {0}")]
        #[status_code("500")]
        #[code("INTERNAL_SERVER_ERROR")]
        AnotherNoStringError(#[from] InnerError),
    }

    #[derive(Debug, Error)]
    pub enum InnerError {
        #[error("Bad request: {0}")]
        BadRequest(String),
        #[error("Internal server error {0}")]
        AnotherNoStringError(axum::Error),
    }

    #[tokio::test]
    async fn is_correct_body() {
        let err = TestError::AnotherNoStringError(InnerError::BadRequest("foo".to_string()));

        let resp = err.into_response();
        let body = resp.into_body();

        let bytes = to_bytes(body, 10485760).await.unwrap();
        let body_str = String::from_utf8_lossy(&bytes).to_string();

        println!("{:?}", body_str);
    }

    #[test]
    fn is_correct_status_code() {
        let err = TestError::BadRequest("foo".to_string());

        let resp = err.into_response();
        let status_code = resp.status();

        assert_eq!(status_code, 400);
    }

    #[test]
    fn parse_http_code() {
        let status_code = StatusCode::from_u16(400).unwrap();

        assert_eq!(status_code, StatusCode::BAD_REQUEST);
    }
}
