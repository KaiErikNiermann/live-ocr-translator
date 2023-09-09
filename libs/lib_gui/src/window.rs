use gtk::glib::{Receiver, Sender};
use gtk::{glib, Application, ApplicationWindow, Button, Entry, Menu, MenuBar, MenuItem};
use gtk::{prelude::*, Label};
use lib_ocr::win_sc::*;
use lib_translator;
use lib_translator::Language;
use std::collections::HashMap;
use std::env;
use std::hash::Hash;
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
    println!("No support on linux yet, coming soon hopefully");
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
enum UpdateText {
    UpdateLabel(String),
}

fn get_lang_choices(deepl: &lib_translator::DeepL) -> HashMap<String, MenuItem> {
    let res: Vec<lib_translator::Language> = match deepl.get_supported() {
        Ok(res) => res,
        Err(_) => panic!("Error getting supported languages"),
    };

    res.iter()
        .map(|lang: &Language| {
            let lang_code = String::from(&lang.language[0..2]).to_lowercase();
            (lang_code.clone(), MenuItem::with_label(&lang_code))
        })
        .collect::<HashMap<String, MenuItem>>()
}

fn get_lang_dropdown(deepl: &lib_translator::DeepL, lang_choice: &Label) -> Menu {
    let lang_choices = get_lang_choices(deepl);
    let lang_menu = Menu::new();
    for (lang_code_str, lang_choice_item) in lang_choices {
        lang_menu.append(&lang_choice_item);
        lang_choice_item.connect_activate(
            glib::clone!(@strong lang_code_str, @weak lang_choice => move |_| {
                lang_choice.set_text(&lang_code_str);
                println!("You chose {}", lang_code_str);
            }),
        );
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
    let source_lang_choice = gtk::Label::new(Some("eng"));
    let target_lang_choice = gtk::Label::new(Some("eng"));
    let api_key_entry = gtk::Entry::new();
    let set_api_key_button = Button::with_label("Set API key");
    let deepl = &mut lib_translator::DeepL::new(String::from(""));

    let menu = MenuBar::new();
    let source = MenuItem::with_label("Source");
    let target = MenuItem::with_label("Target");

    add_actions(
        deepl,
        &set_api_key_button,
        &api_key_entry,
        &source_lang_choice,
        &target_lang_choice,
        &label,
        &tbox,
        &mainwindow,
        &button,
    );

    api_key_entry.set_placeholder_text(Some("DeepL API key"));
    source.set_submenu(Some(&get_lang_dropdown(&deepl, &source_lang_choice)));
    target.set_submenu(Some(&get_lang_dropdown(&deepl, &target_lang_choice)));

    menu.append(&source);
    menu.append(&target);

    let api_key_set_box = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    api_key_set_box.pack_start(&api_key_entry, true, true, 10);
    api_key_set_box.pack_start(&set_api_key_button, false, true, 10);

    let button_lang_box = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    button_lang_box.pack_start(&button, true, true, 10);
    button_lang_box.pack_start(&source_lang_choice, false, true, 10);
    button_lang_box.pack_start(&target_lang_choice, false, true, 10);

    vbox.pack_start(&menu, false, false, 10);
    vbox.pack_end(&api_key_set_box, false, false, 10);
    vbox.pack_start(&button_lang_box, false, false, 10);
    vbox.pack_start(&label, true, true, 10);

    vbox.set_margin(25);

    textwindow.set_child(Some(&vbox));
    mainwindow.set_child(Some(&tbox));

    mainwindow.show_all();
    textwindow.show_all();
}

fn add_actions(
    deepl: &mut lib_translator::DeepL,
    set_api_key_button: &Button,
    api_key_entry: &Entry,
    source_lang_choice: &Label,
    target_lang_choice: &Label,
    label: &Label,
    tbox: &gtk::Box,
    mainwindow: &gtk::ApplicationWindow,
    button: &Button,
) {
    let rt = get_runtime();
    deepl.set(String::from("e7624521-5fdf-cb4f-eae4-c1d6c024d571:fx"));

    set_api_key_button.connect_clicked(glib::clone!(@weak api_key_entry => move |_| {
        println!("Set api key to {}", api_key_entry.text());
    }));

    button.connect_clicked(
        glib::clone!(@weak label, @weak tbox, @weak mainwindow, @weak source_lang_choice, @strong deepl, @strong target_lang_choice => move |_| {
            // Set translation window opacity to 0
            tbox.set_opacity(0.0);

            take_sc();

            let tokio_handle = rt.handle();

            let (sender, receiver): (
                gtk::glib::Sender<UpdateText>,
                gtk::glib::Receiver<UpdateText>,
            ) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

            /*
             * Because reqwuest is async this is the only somewhat sane approach I found for
             * being able to get the translated text and then update the label. Credit to slomo and their
             * blog here : https://coaxion.net/blog/2019/02/mpsc-channel-api-for-painless-usage-of-threads-with-gtk-in-rust/
             */
            // let from_lang = source_lang_choice.text();
            let from_lang = "eng";
            let to_lang = target_lang_choice.text();

            #[cfg(target_os = "windows")]
            let text = lib_ocr::run_ocr("./screenshot.png", &from_lang);

            #[cfg(target_os = "linux")]
            let text = lib_ocr::run_ocr("./placeholder.png", &from_lang);

            tokio_handle.spawn(glib::clone!(@strong deepl, @strong to_lang => async move {
                let translated_text = match deepl.translate_text(&text, &to_lang.to_uppercase()).await {
                    Ok(text) => text,
                    Err(_) => String::from("Could not translate")
                };

                let _ = sender.send(UpdateText::UpdateLabel(translated_text));
            }));

            let label_clone = label.clone();
            receiver.attach(None, move |msg| {
                match msg {
                    UpdateText::UpdateLabel(text) => label_clone.set_text(text.as_str()),
                }

                glib::Continue(true)
            });

            label.set_line_wrap(true);
            label.set_size_request(500, -1);
        }),
    );
}
