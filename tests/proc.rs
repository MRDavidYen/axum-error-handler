#[cfg(test)]
mod tests {
    use std::error::Error;

    use axum::{body::to_bytes, http::StatusCode, response::Response};
    use axum_error_handler::AxumErrorResponse;
    use serde_json;
    use thiserror::Error;

    pub fn custom_fn(_ctx: axum_error_handler::ErrorResponseContext) -> Response {
        Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(axum::body::Body::from("Custom error response"))
            .unwrap()
    }

    #[derive(Debug, Error, AxumErrorResponse)]
    pub enum TestError {
        #[error("Bad request: {0}")]
        #[status_code("400")]
        #[code("BAD_REQUEST")]
        BadRequest(String),

        #[error("{0}")]
        #[response(nested)]
        AnotherNoStringError(#[from] InnerError),
    }

    #[derive(Debug, Error, AxumErrorResponse)]
    pub enum InnerError {
        #[error("Bad request: {0}")]
        #[status_code("400")]
        #[code("BAD_REQUEST")]
        BadRequest(String),

        #[error("Authentication error: {0}")]
        #[status_code("401")]
        #[code("AUTHENTICATION_ERROR")]
        AuthenticationError(String),

        #[error("Internal server error {0}")]
        #[status_code("500")]
        #[code("INTERNAL_SERVER_ERROR")]
        AnotherNoStringError(axum::Error),

        #[error("{0}")]
        #[response(nested)]
        DeepNested(#[from] DeepError),
    }

    #[derive(Debug, Error, AxumErrorResponse)]
    #[response(custom_fn = "custom_fn")]
    pub enum DeepError {
        #[error("Database connection failed: {0}")]
        #[status_code("503")]
        #[code("DATABASE_ERROR")]
        DatabaseError(String),

        #[error("Validation failed: {0}")]
        #[status_code("422")]
        #[code("VALIDATION_ERROR")]
        ValidationError(String),

        #[error("Permission denied: {0}")]
        #[status_code("403")]
        #[code("PERMISSION_DENIED")]
        PermissionDenied(String),
    }

    #[tokio::test]
    async fn test_general_response_body_format() {
        use axum::response::IntoResponse;

        let err = TestError::BadRequest("invalid input".to_string());
        let resp = err.into_response();
        let body = resp.into_body();

        let bytes = to_bytes(body, 10485760).await.unwrap();
        let body_str = String::from_utf8_lossy(&bytes).to_string();
        let parsed: serde_json::Value = serde_json::from_str(&body_str).unwrap();

        assert_eq!(parsed["result"], serde_json::Value::Null);
        assert_eq!(parsed["error"]["code"], "BAD_REQUEST");
        assert_eq!(parsed["error"]["message"], "Bad request: invalid input");
    }

    #[tokio::test]
    async fn test_general_response_status_code() {
        use axum::response::IntoResponse;

        let err = TestError::BadRequest("test".to_string());
        let resp = err.into_response();

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_nested_response_body_format() {
        use axum::response::IntoResponse;

        let inner_err = InnerError::AuthenticationError("invalid token".to_string());
        let err = TestError::AnotherNoStringError(inner_err);

        let resp = err.into_response();
        let body = resp.into_body();

        let bytes = to_bytes(body, 10485760).await.unwrap();
        let body_str = String::from_utf8_lossy(&bytes).to_string();
        let parsed: serde_json::Value = serde_json::from_str(&body_str).unwrap();

        assert_eq!(parsed["result"], serde_json::Value::Null);
        assert_eq!(parsed["error"]["code"], "AUTHENTICATION_ERROR");
        assert_eq!(
            parsed["error"]["message"],
            "Authentication error: invalid token"
        );
    }

    #[tokio::test]
    async fn test_nested_response_status_code() {
        use axum::response::IntoResponse;

        let inner_err = InnerError::AuthenticationError("invalid token".to_string());
        let err = TestError::AnotherNoStringError(inner_err);
        let resp = err.into_response();

        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_nested_response_different_error_types() {
        use axum::response::IntoResponse;

        // Test with BadRequest inner error
        let inner_err = InnerError::BadRequest("missing field".to_string());
        let err = TestError::AnotherNoStringError(inner_err);
        let resp = err.into_response();

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

        let body = resp.into_body();
        let bytes = to_bytes(body, 10485760).await.unwrap();
        let body_str = String::from_utf8_lossy(&bytes).to_string();
        let parsed: serde_json::Value = serde_json::from_str(&body_str).unwrap();

        assert_eq!(parsed["error"]["code"], "BAD_REQUEST");
        assert_eq!(parsed["error"]["message"], "Bad request: missing field");
    }

    #[tokio::test]
    async fn test_three_layer_nested_database_error() {
        use axum::response::IntoResponse;

        let deep_err = DeepError::DatabaseError("connection timeout".to_string());
        let inner_err = InnerError::DeepNested(deep_err);
        let err = TestError::AnotherNoStringError(inner_err);

        let resp = err.into_response();
        assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);

        let body = resp.into_body();
        let bytes = to_bytes(body, 10485760).await.unwrap();
        let body_str = String::from_utf8_lossy(&bytes).to_string();
        let parsed: serde_json::Value = serde_json::from_str(&body_str).unwrap();

        assert_eq!(parsed["result"], serde_json::Value::Null);
        assert_eq!(parsed["error"]["code"], "DATABASE_ERROR");
        assert_eq!(
            parsed["error"]["message"],
            "Database connection failed: connection timeout"
        );
    }

    #[tokio::test]
    async fn test_three_layer_nested_validation_error() {
        use axum::response::IntoResponse;

        let deep_err = DeepError::ValidationError("email format invalid".to_string());
        let inner_err = InnerError::DeepNested(deep_err);
        let err = TestError::AnotherNoStringError(inner_err);

        let resp = err.into_response();
        assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);

        let body = resp.into_body();
        let bytes = to_bytes(body, 10485760).await.unwrap();
        let body_str = String::from_utf8_lossy(&bytes).to_string();
        let parsed: serde_json::Value = serde_json::from_str(&body_str).unwrap();

        assert_eq!(parsed["result"], serde_json::Value::Null);
        assert_eq!(parsed["error"]["code"], "VALIDATION_ERROR");
        assert_eq!(
            parsed["error"]["message"],
            "Validation failed: email format invalid"
        );
    }

    #[tokio::test]
    async fn test_three_layer_nested_permission_error() {
        use axum::response::IntoResponse;

        let deep_err = DeepError::PermissionDenied("admin access required".to_string());
        let inner_err = InnerError::DeepNested(deep_err);
        let err = TestError::AnotherNoStringError(inner_err);

        let resp = err.into_response();
        assert_eq!(resp.status(), StatusCode::FORBIDDEN);

        let body = resp.into_body();
        let bytes = to_bytes(body, 10485760).await.unwrap();
        let body_str = String::from_utf8_lossy(&bytes).to_string();
        let parsed: serde_json::Value = serde_json::from_str(&body_str).unwrap();

        assert_eq!(parsed["result"], serde_json::Value::Null);
        assert_eq!(parsed["error"]["code"], "PERMISSION_DENIED");
        assert_eq!(
            parsed["error"]["message"],
            "Permission denied: admin access required"
        );
    }

    #[test]
    fn test_is_correct_status_code() {
        use axum::response::IntoResponse;

        let err = TestError::BadRequest("foo".to_string());

        let resp = err.into_response();
        let status_code = resp.status();

        assert_eq!(status_code, 400);
    }

    #[test]
    fn test_parse_http_code() {
        let status_code = StatusCode::from_u16(400).unwrap();

        assert_eq!(status_code, StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_inner_error() {
        use axum::response::IntoResponse;

        let err =
            TestError::AnotherNoStringError(InnerError::AuthenticationError("foo".to_string()));

        let resp = err.into_response();
        let status_code = resp.status();

        assert_eq!(status_code, 401);
    }

    #[tokio::test]
    async fn test_custom_error_response() {
        use axum::response::IntoResponse;

        // Test that DeepError uses the custom_fn for response generation
        let deep_err = DeepError::DatabaseError("connection timeout".to_string());
        let resp = deep_err.into_response();

        // Should use custom status code from custom_fn (500)
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);

        // Should use custom body from custom_fn
        let body = resp.into_body();
        let bytes = to_bytes(body, 10485760).await.unwrap();
        let body_str = String::from_utf8_lossy(&bytes).to_string();

        // Custom function returns plain text, not JSON
        assert_eq!(body_str, "Custom error response");
    }

    #[tokio::test]
    async fn test_custom_error_response_all_variants() {
        use axum::response::IntoResponse;

        // Test all variants of DeepError to ensure they all use custom_fn
        let test_cases = vec![
            DeepError::DatabaseError("db error".to_string()),
            DeepError::ValidationError("validation error".to_string()),
            DeepError::PermissionDenied("permission error".to_string()),
        ];

        for error in test_cases {
            let resp = error.into_response();

            // All should use custom function's status code
            assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);

            // All should use custom function's body
            let body = resp.into_body();
            let bytes = to_bytes(body, 10485760).await.unwrap();
            let body_str = String::from_utf8_lossy(&bytes).to_string();
            assert_eq!(body_str, "Custom error response");
        }
    }
}
