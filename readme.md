# Axum Error Handler

A simple parser that implemented Axum `IntoResponse` trait.

> Please notice that this is a experimental project.
> This proc-macros depends on `axum`, `thiserror` and `serde_json` crates. 

## Basic Usage

```rust
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
    InternalError(String),
}
```

### Basic Output

```json
{
  "result": null,
  "error": {
    "code": "BAD_REQUEST",
    "message": "Bad request: invalid input"
  }
}
```

## Nested Response Support

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
    
    // Use nested response to delegate to inner error's response
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
    
    #[error("Database connection failed: {0}")]
    #[status_code("503")]
    #[code("DATABASE_ERROR")]
    DatabaseError(String),
    
    // Supports multiple levels of nesting
    #[error("{0}")]
    #[response(nested)]
    ValidationError(#[from] ValidationError),
}

#[derive(Debug, Error, AxumErrorResponse)]
pub enum ValidationError {
    #[error("Validation failed: {0}")]
    #[status_code("422")]
    #[code("VALIDATION_ERROR")]
    FieldValidation(String),
    
    #[error("Permission denied: {0}")]
    #[status_code("403")]
    #[code("PERMISSION_DENIED")]
    PermissionDenied(String),
}
```

### Usage

```rust
// Direct error
let err = AppError::BadRequest("missing field".to_string());

// Nested error - preserves inner error's status code and details
let service_err = ServiceError::AuthenticationError("invalid token".to_string());
let app_err = AppError::ServiceError(service_err);

// Multi-level nesting
let validation_err = ValidationError::FieldValidation("email format invalid".to_string());
let service_err = ServiceError::ValidationError(validation_err);
let app_err = AppError::ServiceError(service_err);
```

### Nested Response Output

When using nested responses, the inner error's status code, error code, and message are preserved:

```json
{
  "result": null,
  "error": {
    "code": "AUTHENTICATION_ERROR",
    "message": "Authentication error: invalid token"
  }
}
```

### Key Features

- **Status Code Preservation**: Nested errors maintain their original HTTP status codes
- **Error Code Forwarding**: Custom error codes from inner errors are preserved
- **Multiple Nesting Levels**: Supports arbitrarily deep error nesting
- **Automatic Conversion**: Use `#[from]` attribute for automatic error conversion
- **Consistent JSON Format**: All responses follow the same `{"result": null, "error": {...}}` structure
