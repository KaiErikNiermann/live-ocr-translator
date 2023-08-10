use std::env::Args;

use gtk::prelude::*;
use gtk::{ApplicationWindow, Button, Application, glib};
use lib_ocr::*;

pub struct WindowLayout {
    pub width: i32,
    pub height: i32,
    pub title: String,
    pub opacity: f32,
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

pub fn build_ui(application: &Application, mainwindow: &ApplicationWindow, textwindow: &ApplicationWindow) {
    // Create a vertical box to hold the label and button
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 10);

    let label = gtk::Label::new(Some("<translated text>"));
    
    let button = Button::with_label("Translate");
    
    vbox.pack_start(&button, false, false, 10);
    
    vbox.pack_start(&label, false, false, 10);

    button.connect_clicked(glib::clone!(@weak label => move |_| {
        // set label text 
        let text = lib_ocr::run_ocr("assets/english1.png", "eng");
        label.set_text(&text);
    }));

    // Set the vbox as the child of mainwindow
    textwindow.set_child(Some(&vbox));

    mainwindow.show_all();
    textwindow.show_all();
}

