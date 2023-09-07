use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button};
use lib_ocr::win_sc::*;
use std::thread;

pub struct WindowLayout {
    pub width: i32,
    pub height: i32,
    pub title: String,
    pub opacity: f32,
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

pub fn build_ui(
    application: &Application,
    mainwindow: &ApplicationWindow,
    textwindow: &ApplicationWindow,
) {
    // Create a vertical box to hold the label and button
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 10);
    let tbox = gtk::Box::new(gtk::Orientation::Vertical, 10);

    tbox.set_opacity(0.5);
    mainwindow.set_app_paintable(true);

    let label = gtk::Label::new(Some("<translated text>"));
    let button = Button::with_label("Translate");

    vbox.pack_start(&button, false, false, 10);
    vbox.pack_start(&label, false, false, 10);

    // set padding around vbox
    vbox.set_margin(25);

    button.connect_clicked(glib::clone!(@weak label, @weak tbox => move |_| {
        let text = lib_ocr::run_ocr("./assets/english1.png", "eng");
        // Set translation window opacity to 0
        tbox.set_opacity(0.0);

        // Take a screenshot of the window
        thread::spawn(|| {
            monitor::monitor_sc(
                Some(&window::get_window_rect(
                    window::window_handle("translator"))
                )
            );
        });

        // Set the translation window opacity to 1
        tbox.set_opacity(0.5);

        // Get the text from the screenshot

        label.set_text(&text);
        label.set_line_wrap(true);
        label.set_size_request(500, -1);
    }));

    textwindow.set_child(Some(&vbox));
    mainwindow.set_child(Some(&tbox));

    mainwindow.show_all();
    textwindow.show_all();
}
