use crate::services::pi;
use crate::http::response::http_response;
use crate::controllers::Controller;
pub struct PiController;

impl Controller for PiController {
    fn handle_get(&self, request: &str, path: &str) -> String {
        match path {
            "/" => self.calculate_pi_handler(request),
            _ => http_response(404, "text/plain", "Not Found"),
        }
    }

    fn handle_post(&self, _request: &str, _path: &str) -> String {
        http_response(404, "text/plain", "Not Found")
    }
}

impl PiController {
    fn calculate_pi_handler(&self, num_str: &str) -> String {
        if let Ok(num) = num_str.parse::<u128>() {
            let pi = pi::leibniz_pi(num);
            let result = format!("Pi result is: {}", pi);
            return http_response(200, "text/plain", &result);
        }

        http_response(400, "text/plain", "Invalid number")
    }
}