use axum::response::IntoResponse;

/// This trait is used to convert an error type into a response context.
/// It allows for customization of the error response by implementing the `IntoResponseContext` trait.
/// The context can include a status code, error code, and message.
pub trait IntoErrorResponseContext {
    fn into_response_context(self) -> ErrorResponseContext;
}

#[derive(Clone)]
pub struct ErrorResponseContext {
    status_code: Option<u16>,
    code: Option<String>,
    message: Option<String>,
}

impl ErrorResponseContext {
    pub fn builder() -> ErrorResponseBuilder {
        ErrorResponseBuilder::new()
    }

    pub fn new() -> Self {
        Self {
            status_code: None,
            code: None,
            message: None,
        }
    }

    pub(crate) fn set_status_code(&mut self, status_code: u16) {
        self.status_code = Some(status_code);
    }

    pub(crate) fn set_code(&mut self, code: String) {
        self.code = Some(code);
    }

    pub(crate) fn set_message(&mut self, message: String) {
        self.message = Some(message);
    }

    pub fn status_code(&self) -> Option<u16> {
        self.status_code
    }

    pub fn code(&self) -> Option<&String> {
        self.code.as_ref()
    }

    pub fn message(&self) -> Option<&String> {
        self.message.as_ref()
    }
}

pub struct ErrorResponseBuilder {
    code: Option<String>,
    message: Option<String>,
    status_code: Option<u16>,
}

impl ErrorResponseBuilder {
    pub fn new() -> Self {
        Self {
            code: None,
            message: None,
            status_code: None,
        }
    }

    pub fn code(mut self, code: String) -> Self {
        self.code = Some(code);
        self
    }

    pub fn message(mut self, message: String) -> Self {
        self.message = Some(message);
        self
    }

    pub fn status_code(mut self, status_code: u16) -> Self {
        self.status_code = Some(status_code);
        self
    }

    pub fn build(self) -> ErrorResponseContext {
        let mut context = ErrorResponseContext::new();
        if let Some(code) = self.code {
            context.set_code(code);
        }
        if let Some(message) = self.message {
            context.set_message(message);
        }
        if let Some(status_code) = self.status_code {
            context.set_status_code(status_code);
        }
        context
    }
}

/// Implements the Default behavior for converting an error response context into an Axum response.
impl IntoResponse for ErrorResponseContext {
    fn into_response(self) -> axum::response::Response {
        let status_code = self.status_code.unwrap_or(500);
        let code = self.code.unwrap_or_else(|| "UNKNOWN_ERROR".to_string());
        let message = self
            .message
            .unwrap_or_else(|| "An error occurred".to_string());

        let body = axum::Json(serde_json::json!({
            "result": null,
            "error": {
                "code": code,
                "message": message,
            }
        }));

        axum::http::Response::builder()
            .status(status_code)
            .header("content-type", "application/json")
            .body(body.into_response().into_body())
            .unwrap()
    }
}
