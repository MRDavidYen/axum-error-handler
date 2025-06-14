# Axum Error Handler

[![Crates.io](https://img.shields.io/crates/v/axum-error-handler.svg)](https://crates.io/crates/axum-error-handler)
[![Documentation](https://docs.rs/axum-error-handler/badge.svg)](https://docs.rs/axum-error-handler)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A procedural macro for generating standardized error responses in Axum applications. This crate provides a derive macro that automatically implements the `IntoResponse` trait for your error enums, creating consistent JSON error responses with proper HTTP status codes.

## Features

- 🚀 **Easy Integration**: Simple derive macro for error enums
- 📝 **Consistent Format**: Standardized JSON error responses
- 🔧 **Flexible Configuration**: Custom status codes and error codes
- 🏗️ **Nested Error Support**: Forward inner errors with preserved status codes
- 🎨 **Custom Response Functions**: Use custom functions for specialized error handling
- ⚡ **Zero Runtime Cost**: All code generation happens at compile time

> **Note**: This is an experimental project. The API may change in future versions.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
axum-error-handler = "0.2.1"
axum = "0.8"
thiserror = "2.0"
serde_json = "1.0"
``` 

## Usage Examples

#[derive(Debug, Error, AxumErrorResponse)]
pub enum TestError {
    #[error("Bad request: {0}")]
    #[status_code("400")]
    #[code("BAD_REQUEST")]
    BadRequest(String),
    
    #[error("Internal server error {0}")]
    #[status_code("500")]
    #[code("INTERNAL_SERVER_ERROR")]
    InternalError(String),
}
```

This generates the following JSON response format:

```json
{
  "result": null,
  "error": {
    "code": "BAD_REQUEST",
    "message": "Bad request: invalid input"
  }
}
```

## Advanced Usage

### Nested Response Support

The library supports nested error handling through the `#[response(nested)]` attribute. This allows inner errors that also implement `AxumErrorResponse` to be properly forwarded with their own status codes and error details.

### Example

```rust
use axum_error_handler::AxumErrorResponse;
use thiserror::Error;

#[derive(Debug, Error, AxumErrorResponse)]
pub enum AppError {
    #[error("Bad request: {0}")]
    #[status_code("400")]
    #[code("BAD_REQUEST")]
    BadRequest(String),
    
    #[error("Internal server error {0}")]
    #[status_code("500")]
    #[code("INTERNAL_SERVER_ERROR")]
    InternalError(String),
}
```

Generates JSON responses like:
```json
{
  "result": null,
  "error": {
    "code": "BAD_REQUEST",
    "message": "Bad request: invalid input"
  }
}
```

### Nested Error Responses

Use `#[response(nested)]` to delegate to inner error responses:

```rust
#[derive(Debug, Error, AxumErrorResponse)]
pub enum AppError {
    #[error("Bad request: {0}")]
    #[status_code("400")]
    #[code("BAD_REQUEST")]
    BadRequest(String),
    
    #[error("{0}")]
    #[response(nested)]
    ServiceError(#[from] ServiceError),
}

#[derive(Debug, Error, AxumErrorResponse)]
pub enum ServiceError {
    #[error("Authentication error: {0}")]
    #[status_code("401")]
    #[code("AUTHENTICATION_ERROR")]
    AuthenticationError(String),
}
```

### Custom Error Response Functions

For specialized error handling, use custom response functions:

```rust
use axum_error_handler::{AxumErrorResponse, ErrorResponseContext};
use axum::{response::Response, http::StatusCode};

// Custom response function
fn custom_error_response(_ctx: ErrorResponseContext) -> Response {
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(axum::body::Body::from("Custom error format"))
        .unwrap()
}

#[derive(Debug, Error, AxumErrorResponse)]
#[response(custom_fn = "custom_error_response")]
pub enum CustomError {
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
}
```

This allows you to return any response format (plain text, XML, custom JSON, etc.) instead of the standard JSON format.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Repository

```json
{
  "result": null,
  "error": {
    "code": "AUTHENTICATION_ERROR",
    "message": "Authentication error: invalid token"
  }
}
```

### Benefits of Nested Responses

- **Status Code Preservation**: Nested errors maintain their original HTTP status codes
- **Error Code Forwarding**: Custom error codes from inner errors are preserved
- **Multiple Nesting Levels**: Supports arbitrarily deep error nesting
- **Automatic Conversion**: Use `#[from]` attribute for automatic error conversion
- **Consistent JSON Format**: All responses follow the same `{"result": null, "error": {...}}` structure

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Repository

Find this project on GitHub: [axum-error-handler](https://github.com/MRDavidYen/axum-error-handler)
