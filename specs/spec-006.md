# Spec 006: JiraClient Core — HTTP Client & Auth

- [ ] Not implemented

## Goal

Build the core HTTP client struct that all API operations use, with Basic Auth, proper headers, timeouts, retry, and rate limiting.

## Requirements

### JiraClient Struct

```rust
pub struct JiraClient {
    base_url: String,          // https://company.atlassian.net
    http: reqwest::Client,     // Reusable connection pool
    auth_header: String,       // "Basic base64(email:token)"
    max_page_size: usize,      // Default 100
    max_retries: u32,          // Default 3
    retry_base_ms: u64,        // Default 1000ms
}
```

### Construction

- `JiraClient::new(config: &Config) -> Result<Self>`
- Builds `reqwest::Client` with:
  - `rustls-tls` (no OpenSSL)
  - Connection timeout: 30s
  - Read timeout: 60s
  - HTTP/2 support
  - Connection pooling
- Computes `auth_header` from `base64(email:token)` per RFC 7617
- Sets `User-Agent`: `jari/{version} (Rust CLI; +https://github.com/...)`

### Generic HTTP Helpers

- `async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T>`
- `async fn post<T: DeserializeOwned, B: Serialize>(&self, path: &str, body: &B) -> Result<T>`
- `async fn put<T: DeserializeOwned, B: Serialize>(&self, path: &str, body: &B) -> Result<T>`
- `async fn delete(&self, path: &str) -> Result<()>`

### Behaviors

- All requests include `Authorization: Basic ...` and `User-Agent` headers
- All requests include `Accept: application/json`
- POST/PUT include `Content-Type: application/json`
- Non-2xx responses parsed into `JariError` variants by status code
- 429 responses include `Retry-After` header parsing for `RateLimit` error
- Request logging at `trace` level (sanitized of auth headers)
- Paths prefixed with `/rest/api/3` automatically
