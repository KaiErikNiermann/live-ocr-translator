use ::glib::{Receiver, Sender};
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button, Menu, MenuBar, MenuItem};
use lib_ocr::win_sc::*;
use lib_translator;
use std::thread;
use tokio::runtime;
use tokio::runtime::Runtime;

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

fn get_runtime() -> Runtime {
    return runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap();
}

#[cfg(target_os = "linux")]
fn take_sc() {
    println!("No support on linux yet");
}

#[cfg(target_os = "windows")]
fn take_sc() {
    let image_handler = thread::spawn(|| {
        monitor::monitor_sc(Some(&window::get_window_rect(window::window_handle(
            "translator",
        ))));
    });

    match image_handler.join() {
        Ok(res) => println!("{:?}", res),
        Err(_) => println!("Error"),
    }
}

#[derive(Clone)]
enum UpdateMessage {
    UpdateLabel(String),
}

fn get_lang_dropdown(
    onclick: impl Fn(&MenuItem) + Clone + 'static
) -> Menu {
    let language_choices: Vec<MenuItem> = vec![
        MenuItem::with_label("jp"),
        MenuItem::with_label("eng"),
        MenuItem::with_label("de"),
    ];

    let lang_menu = Menu::new();
    for lang in language_choices {
        lang_menu.append(&lang);
        lang.connect_activate(onclick.clone());
    }
    lang_menu
}

pub fn build_ui(
    application: &Application,
    mainwindow: &ApplicationWindow,
    textwindow: &ApplicationWindow,
) {
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 10);
    let tbox = gtk::Box::new(gtk::Orientation::Vertical, 10);

    tbox.set_opacity(0.5);
    mainwindow.set_app_paintable(true);

    let label = gtk::Label::new(Some("<translated text>"));
    let button = Button::with_label("Translate");
    let menu = MenuBar::new();

    let source = MenuItem::with_label("Source");
    let target = MenuItem::with_label("Target");

    source.set_submenu(Some(&get_lang_dropdown(
        move |_: &MenuItem| println!("clicked in source"),
    )));

    target.set_submenu(Some(&get_lang_dropdown(
        move |_: &MenuItem| println!("clicked in target"),
    )));

    menu.append(&source);
    menu.append(&target);

    vbox.pack_start(&menu, false, false, 10);
    vbox.pack_start(&button, false, false, 10);
    vbox.pack_start(&label, false, false, 10);

    // set padding around vbox
    vbox.set_margin(25);

    let rt = get_runtime();

    button.connect_clicked(glib::clone!(@weak label, @weak tbox => move |_| {
        // Set translation window opacity to 0
        tbox.set_opacity(0.0);
        
        take_sc();
        
        let (sender, receiver): (
            gtk::glib::Sender<UpdateMessage>,
            gtk::glib::Receiver<UpdateMessage>,
        ) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
        
        let tokio_handle = rt.handle();
        
        /*
         * Because reqwuest is async this is the only somewhat sane approach I found for 
         * being able to get the translated text and then update the label. Credit to slomo and their 
         * blog here : https://coaxion.net/blog/2019/02/mpsc-channel-api-for-painless-usage-of-threads-with-gtk-in-rust/
         */
        tokio_handle.spawn(async move {
            let text = lib_ocr::run_ocr("./screenshot.png", "eng");

            let translated_text = match lib_translator::translate_text(&text, "f191652f-38ee-caed-ab30-f20a9a0cc21e:fx").await {
                Ok(text) => text,
                Err(error_text) => panic!("{:?}", error_text)
            };

            let _ = sender.send(UpdateMessage::UpdateLabel(translated_text));
        });

        let label_clone = label.clone();
        receiver.attach(None, move |msg| {
            match msg {
                UpdateMessage::UpdateLabel(text) => label_clone.set_text(text.as_str()),
            }

            glib::Continue(true)
        });

        label.set_line_wrap(true);
        label.set_size_request(500, -1);
    }));

    textwindow.set_child(Some(&vbox));
    mainwindow.set_child(Some(&tbox));

    mainwindow.show_all();
    textwindow.show_all();
}
