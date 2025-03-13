pub mod pi;

pub trait Controller {
    fn handle_get(&self, request: &str, path: &str) -> String;
    fn handle_post(&self, request: &str, path: &str) -> String;
}