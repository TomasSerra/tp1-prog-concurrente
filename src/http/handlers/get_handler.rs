use crate::controllers::Controller;

pub struct GetHandler;

impl GetHandler {
    pub fn handle<C: Controller>(&self, request: &str, controller: C, path: &str) -> String {
        controller.handle_get(request, path)
    }
}