use crate::http::response::http_response;

use crate::http::handlers::get_handler::GetHandler;
use crate::controllers::pi::PiController;

pub fn handle_request(request: &str) -> String {
    let first_line = request.lines().next().unwrap_or("");
    let method = first_line.split_whitespace().next().unwrap_or("");
    let path = first_line.split_whitespace().nth(1).unwrap_or("");
    let base_path = extract_base_path(path);

    match method {
        "GET" => match base_path.as_str() {
            "/pi" => {
                let controller = PiController;
                GetHandler.handle(request, controller, path)
            }
            _ => http_response(404, "text/plain", "Not Found"),
        },
        "POST" => match base_path.as_str() {
            _ => http_response(404, "text/plain", "Not Found"),
        },
        _ => http_response(405, "text/plain", "Method Not Allowed"),
    }
}

fn extract_base_path(path: &str) -> String {
    let segments: Vec<&str> = path.split('/').collect();
    format!("/{}", segments.get(1).unwrap_or(&""))
}