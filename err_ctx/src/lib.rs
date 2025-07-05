//! # Error Response Context
//! 
//! This module provides types and traits for creating standardized error response contexts
//! that can be used with custom error response functions in the Axum Error Handler crate.
//! 
//! The main purpose is to allow custom error response functions to receive structured
//! error information (status code, error code, message) that can be used to generate
//! custom response formats while maintaining consistency with the error handling system.
//! 
//! ## Example Usage
//! 
//! ```rust
//! use axum_error_handler::ErrorResponseContext;
//! use axum::response::Response;
//! 
//! fn custom_error_handler(ctx: ErrorResponseContext) -> Response {
//!     // Access error information
//!     let status = ctx.status_code().unwrap_or(500);
//!     let code = ctx.code().unwrap_or(&"UNKNOWN".to_string());
//!     let message = ctx.message().unwrap_or(&"Error occurred".to_string());
//!     
//!     // Create custom response format
//!     Response::builder()
//!         .status(status)
//!         .body(format!("Error {}: {}", code, message).into())
//!         .unwrap()
//! }
//! ```

use axum::response::IntoResponse;

/// A trait for converting error types into structured error response contexts.
/// 
/// This trait allows error types to be converted into a standardized context
/// that contains status code, error code, and message information. This context
/// can then be used by custom error response functions to generate appropriate
/// HTTP responses.
/// 
/// # Example Implementation
/// 
/// ```rust
/// use axum_error_handler::{IntoErrorResponseContext, ErrorResponseContext};
/// 
/// struct MyError {
///     message: String,
/// }
/// 
/// impl IntoErrorResponseContext for MyError {
///     fn into_response_context(self) -> ErrorResponseContext {
///         ErrorResponseContext::builder()
///             .status_code(400)
///             .code("MY_ERROR".to_string())
///             .message(self.message)
///             .build()
///     }
/// }
/// ```
pub trait IntoErrorResponseContext {
    /// Converts the error into an `ErrorResponseContext`.
    fn into_response_context(self) -> ErrorResponseContext;
}

/// A structured context containing error information for generating HTTP responses.
/// 
/// This struct holds the essential components of an error response:
/// - HTTP status code
/// - Error code (for API consumers)
/// - Human-readable error message
/// 
/// It can be used by custom error response functions to access error details
/// and generate appropriate responses in any desired format.
/// 
/// # Example
/// 
/// ```rust
/// use axum_error_handler::ErrorResponseContext;
/// 
/// let context = ErrorResponseContext::builder()
///     .status_code(404)
///     .code("NOT_FOUND".to_string())
///     .message("Resource not found".to_string())
///     .build();
/// 
/// assert_eq!(context.status_code(), Some(404));
/// assert_eq!(context.code(), Some(&"NOT_FOUND".to_string()));
/// ```
#[derive(Clone)]
pub struct ErrorResponseContext {
    status_code: Option<u16>,
    code: Option<String>,
    message: Option<String>,
}

impl ErrorResponseContext {
    /// Creates a new builder for constructing an `ErrorResponseContext`.
    /// 
    /// This is the recommended way to create a new context with specific values.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use axum_error_handler::ErrorResponseContext;
    /// 
    /// let context = ErrorResponseContext::builder()
    ///     .status_code(400)
    ///     .code("VALIDATION_ERROR".to_string())
    ///     .message("Invalid input provided".to_string())
    ///     .build();
    /// ```
    pub fn builder() -> ErrorResponseBuilder {
        ErrorResponseBuilder::new()
    }

    /// Creates a new empty `ErrorResponseContext`.
    /// 
    /// All fields will be `None` initially. Use the builder pattern or
    /// the setter methods to populate the context.
    pub fn new() -> Self {
        Self {
            status_code: None,
            code: None,
            message: None,
        }
    }

    /// Sets the HTTP status code for this error context.
    /// 
    /// # Arguments
    /// 
    /// * `status_code` - The HTTP status code (e.g., 400, 404, 500)
    pub(crate) fn set_status_code(&mut self, status_code: u16) {
        self.status_code = Some(status_code);
    }

    /// Sets the error code for this error context.
    /// 
    /// The error code is typically a machine-readable identifier
    /// that API consumers can use to handle specific error types.
    /// 
    /// # Arguments
    /// 
    /// * `code` - A string identifier for the error type (e.g., "VALIDATION_ERROR")
    pub(crate) fn set_code(&mut self, code: String) {
        self.code = Some(code);
    }

    /// Sets the error message for this error context.
    /// 
    /// The message should be human-readable and provide details
    /// about what went wrong.
    /// 
    /// # Arguments
    /// 
    /// * `message` - A descriptive error message
    pub(crate) fn set_message(&mut self, message: String) {
        self.message = Some(message);
    }

    /// Returns the HTTP status code if set.
    /// 
    /// # Returns
    /// 
    /// `Some(status_code)` if a status code was set, `None` otherwise.
    pub fn status_code(&self) -> Option<u16> {
        self.status_code
    }

    /// Returns a reference to the error code if set.
    /// 
    /// # Returns
    /// 
    /// `Some(&code)` if an error code was set, `None` otherwise.
    pub fn code(&self) -> Option<&String> {
        self.code.as_ref()
    }

    /// Returns a reference to the error message if set.
    /// 
    /// # Returns
    /// 
    /// `Some(&message)` if a message was set, `None` otherwise.
    pub fn message(&self) -> Option<&String> {
        self.message.as_ref()
    }
}

/// A builder for constructing `ErrorResponseContext` instances.
/// 
/// This builder provides a fluent interface for setting error context properties
/// and ensures that contexts are created in a consistent manner.
/// 
/// # Example
/// 
/// ```rust
/// use axum_error_handler::ErrorResponseContext;
/// 
/// let context = ErrorResponseContext::builder()
///     .status_code(422)
///     .code("VALIDATION_FAILED".to_string())
///     .message("The provided data failed validation".to_string())
///     .build();
/// ```
pub struct ErrorResponseBuilder {
    code: Option<String>,
    message: Option<String>,
    status_code: Option<u16>,
}

impl ErrorResponseBuilder {
    /// Creates a new empty builder.
    pub fn new() -> Self {
        Self {
            code: None,
            message: None,
            status_code: None,
        }
    }

    /// Sets the error code for the context being built.
    /// 
    /// # Arguments
    /// 
    /// * `code` - A machine-readable error identifier
    /// 
    /// # Returns
    /// 
    /// The builder instance for method chaining.
    pub fn code(mut self, code: String) -> Self {
        self.code = Some(code);
        self
    }

    /// Sets the error message for the context being built.
    /// 
    /// # Arguments
    /// 
    /// * `message` - A human-readable error description
    /// 
    /// # Returns
    /// 
    /// The builder instance for method chaining.
    pub fn message(mut self, message: String) -> Self {
        self.message = Some(message);
        self
    }

    /// Sets the HTTP status code for the context being built.
    /// 
    /// # Arguments
    /// 
    /// * `status_code` - The HTTP status code (e.g., 400, 404, 500)
    /// 
    /// # Returns
    /// 
    /// The builder instance for method chaining.
    pub fn status_code(mut self, status_code: u16) -> Self {
        self.status_code = Some(status_code);
        self
    }

    /// Builds the final `ErrorResponseContext` with the configured values.
    /// 
    /// # Returns
    /// 
    /// A new `ErrorResponseContext` instance with the values set on this builder.
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

/// Default implementation for converting an `ErrorResponseContext` into an Axum HTTP response.
/// 
/// This implementation creates a standardized JSON response with the following format:
/// 
/// ```json
/// {
///   "result": null,
///   "error": {
///     "code": "ERROR_CODE",
///     "message": "Error description"
///   }
/// }
/// ```
/// 
/// # Defaults
/// 
/// - Status code: 500 (Internal Server Error) if not specified
/// - Error code: "UNKNOWN_ERROR" if not specified  
/// - Message: "An error occurred" if not specified
/// 
/// # Example
/// 
/// ```rust
/// use axum::response::IntoResponse;
/// use axum_error_handler::ErrorResponseContext;
/// 
/// let context = ErrorResponseContext::builder()
///     .status_code(404)
///     .code("NOT_FOUND".to_string())
///     .message("The requested resource was not found".to_string())
///     .build();
/// 
/// let response = context.into_response();
/// // Creates a 404 response with JSON body
/// ```
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
