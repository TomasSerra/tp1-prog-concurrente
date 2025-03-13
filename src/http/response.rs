pub fn http_response(status_code: u16, content_type: &str, body: &str) -> String {
    let status_text = match status_code {
        200 => "OK",
        201 => "Created",
        204 => "No Content",
        400 => "Bad Request",
        401 => "Unauthorized",
        403 => "Forbidden",
        404 => "Not Found",
        500 => "Internal Server Error",
        _ => "Unknown Status",
    };

    format!(
        "HTTP/1.1 {} {}\r\nContent-Type: {}\r\n\r\n{}",
        status_code, status_text, content_type, body
    )
}