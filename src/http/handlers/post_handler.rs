use crate::controllers::Controller;

pub struct PostHandler;

impl PostHandler {
    pub fn handle<C: Controller>(&self, request: &str, controller: C, path: &str) -> String {
        controller.handle_post(request, path)
    }
}