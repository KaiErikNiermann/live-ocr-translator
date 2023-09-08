use gtk::prelude::*;
use gtk::Application;

pub mod window;

pub fn init_app() -> gtk::Application {
    return Application::builder()
        .application_id("org.liveTranslator.gui")
        .build();
}

pub fn run_app(app: &gtk::Application) {
    app.connect_activate(|app| { 
        window::build_ui(app, 
            &window::window(app, &(window::WindowLayout {
                width: 400,
                height: 300,
                title: String::from("translator"),
                opacity: 0.5,
            })), 
            &window::window(app, &(window::WindowLayout {
                width: 400,
                height: 250,
                title: String::from("textwindow"),
                opacity: 1.0,
            })));
    });
    app.run();
}
