use gtk::prelude::*;
use gtk::Application;

pub mod window;

pub fn init_app() -> gtk::Application {
    return Application::builder()
        .application_id("org.liveTranslator.gui")
        .build();
}

pub fn run_app(app: &gtk::Application) {
    window::make_windows(app);
    app.run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }

    #[test]
    fn test_gui_add() {
    }
}