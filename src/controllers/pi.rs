use crate::services::pi;
use crate::http::response::http_response;
use crate::controllers::Controller;
pub struct PiController;

impl Controller for PiController {
    fn handle_get(&self, _request: &str, path: &str) -> String {
        if path == "/pi" {
            return self.calculate_pi(1000);
        } else if path.starts_with("/pi?") {
            return if path.contains("iterations=") {
                let iterations_str = path
                    .split("iterations=")
                    .nth(1)
                    .unwrap_or("1000")
                    .split('&')
                    .next()
                    .unwrap_or("1000");

                self.calculate_pi_from_string(iterations_str)
            } else {
                // No iterations parameter, use default
                self.calculate_pi(1000)
            }
        }

        http_response(404, "text/plain", "Not Found")
    }

    fn handle_post(&self, _request: &str, _path: &str) -> String {
        http_response(404, "text/plain", "Not Found")
    }
}

impl PiController {
    fn calculate_pi_from_string(&self, iterations_str: &str) -> String {
        if let Ok(iterations) = iterations_str.parse::<u128>() {
            self.calculate_pi(iterations)
        } else {
            http_response(400, "text/plain", "Invalid iterations parameter")
        }
    }

    fn calculate_pi(&self, iterations: u128) -> String {
        let pi = pi::leibniz_pi(iterations);
        let result = format!("Pi result is: {}", pi);
        http_response(200, "text/plain", &result)
    }
}