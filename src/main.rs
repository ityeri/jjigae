use jjigae::{
    application_runner::JjigaeApplicationRunner, viewport2_app::Viewport2App,
    viewport3_app::Viewport3App,
};

fn main() {
    JjigaeApplicationRunner::new(Viewport3App::default()).run();
}
