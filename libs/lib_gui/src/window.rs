use gtk::prelude::*;
use gtk::ApplicationWindow;

pub struct WindowLayout {
    width: i32,
    height: i32,
    title: String,
    opacity: f32,
}

pub fn window_corners(window: &ApplicationWindow) -> Vec<(i32, i32)> {
    let (x, y) = window.position();
    let (width, height) = window.size();
    return vec![(x, y), (x + width, y), (x, y + height), (x + width, y + height)];
}

pub fn window(app: &gtk::Application, layout: &WindowLayout) -> ApplicationWindow {
    return ApplicationWindow::builder()
        .application(app)
        .default_width(layout.width)
        .default_height(layout.height)
        .title((layout.title).clone())
        .opacity(layout.opacity as f64)
        .build();
}

pub fn add_text(window: &ApplicationWindow, text: &str) {
    let label = gtk::Label::new(Some(text));
    window.set_child(Some(&label));
}

pub fn make_windows(application: &gtk::Application) {
    application.connect_activate(|app| {
        // create the main window
        window(app, &(WindowLayout {
            width: 400,
            height: 300,
            title: String::from("translator"),
            opacity: 0.5,
        }))
            .show_all();

        // create text window
        let textwindow = window(app, &(WindowLayout {
            width: 400,
            height: 300,
            title: String::from("textwindow"),
            opacity: 1.0,
        }));

        add_text(&textwindow, "template text");

        textwindow.show_all();
    });
}
