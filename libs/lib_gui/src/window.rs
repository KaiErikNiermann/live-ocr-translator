use gtk::ffi::gtk_widget_set_visible;
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
    authwindow: &ApplicationWindow
) {
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 10);
    let tbox = gtk::Box::new(gtk::Orientation::Vertical, 10);

    tbox.set_opacity(0.5);
    mainwindow.set_app_paintable(true);

    let label = gtk::Label::new(Some("<translated text>"));
    let button = Button::with_label("Translate");
    let source_lang_choice = gtk::Label::new(Some("eng"));
    let target_lang_choice = gtk::Label::new(Some("de"));
    let api_key_entry = gtk::Entry::new();
    let api_key_label = gtk::Label::new(Some("Selected API key"));
    let set_api_key_button = Button::with_label("Set API key");

    let menu = MenuBar::new();
    let source = MenuItem::with_label("Source");
    let target = MenuItem::with_label("Target");

    api_key_entry.set_placeholder_text(Some("DeepL API key"));
    
    let api_key_set_box = gtk::Box::new(gtk::Orientation::Vertical, 10);
    api_key_set_box.pack_start(&api_key_label, true, true, 10);
    api_key_set_box.pack_start(&api_key_entry, true, true, 10);
    api_key_set_box.pack_end(&set_api_key_button, false, true, 10);
    authwindow.set_child(Some(&api_key_set_box));

    add_actions(
        &api_key_label,
        &source_lang_choice,
        &target_lang_choice,
        &label,
        &tbox,
        &mainwindow,
        &button,
    );
    
    menu.append(&source);
    menu.append(&target);
    
    let button_lang_box = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    button_lang_box.pack_start(&button, true, true, 10);
    button_lang_box.pack_start(&source_lang_choice, false, true, 10);
    button_lang_box.pack_start(&target_lang_choice, false, true, 10);

    vbox.pack_start(&menu, false, false, 10);
    vbox.pack_start(&button_lang_box, false, false, 10);
    vbox.pack_start(&label, true, true, 10);
    
    vbox.set_margin(25);
    
    textwindow.set_child(Some(&vbox));
    mainwindow.set_child(Some(&tbox));

    set_api_key_button.connect_clicked(
        glib::clone!(
                @weak mainwindow, 
                @weak textwindow, 
                @strong api_key_label, 
                @weak source_lang_choice, 
                @weak target_lang_choice, 
                @strong source, 
                @strong target => move |_| {

            // Use the API to setup API dependent components
            let api_key = api_key_entry.text().to_string();
            api_key_label.set_text(&api_key);
            let deepl = &mut lib_translator::DeepL::new(String::from(api_key));
            
            source.set_submenu(Some(&get_lang_dropdown(&deepl, &source_lang_choice)));
            target.set_submenu(Some(&get_lang_dropdown(&deepl, &target_lang_choice)));

            mainwindow.show_all();
            textwindow.show_all();
        })
    );
    
    authwindow.show_all();
}

fn add_actions(
    api_key_label: &Label,
    source_lang_choice: &Label,
    target_lang_choice: &Label,
    label: &Label,
    tbox: &gtk::Box,
    mainwindow: &gtk::ApplicationWindow,
    button: &Button,
) {
    button.connect_clicked(
        glib::clone!(@weak label, @weak tbox, @weak mainwindow, @weak source_lang_choice, @strong target_lang_choice, @strong api_key_label => move |_| {
            tbox.set_opacity(0.0);

            take_sc();

            // let from_lang = source_lang_choice.text();
            let from_lang = "eng";
            let to_lang = target_lang_choice.text();

            #[cfg(target_os = "windows")]
            let text = lib_ocr::run_ocr("./screenshot.png", &from_lang);

            #[cfg(target_os = "linux")]
            let text = lib_ocr::run_ocr("./placeholder.png", &from_lang);
            
            let api_key = api_key_label.text().to_string();
            let deepl = lib_translator::DeepL::new(api_key);

            let translated_text = match deepl.translate_text(&text, &to_lang.to_uppercase()) {
                Ok(text) => text,
                Err(_) => String::from("Could not translate")
            };

            label.set_text(&translated_text);
            label.set_line_wrap(true);
            label.set_size_request(500, -1);
        }),
    );
}
