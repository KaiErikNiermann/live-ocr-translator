use gtk::ffi::gtk_widget_set_visible;
use gtk::glib::{Receiver, Sender};
use gtk::{glib, Application, ApplicationWindow, Button, Entry, Menu, MenuBar, MenuItem};
use gtk::{prelude::*, Label};
use gtk::{StyleProvider, StyleProperties, StyleContext};
use lib_ocr::text::clean_text;
use lib_ocr::run_ocr_img;
#[cfg(target_os = "windows")]
use lib_ocr::win_sc::*;
use lib_translator;
use gdk::Screen;
use lib_translator::Language;
use serde::{Deserialize, Serialize};
use image::DynamicImage;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::hash::Hash;
use std::io::prelude::*;
use std::thread;
use std::time::{Duration, Instant};
use tokio::runtime;
use tokio::runtime::Runtime;

#[cfg(target_os = "windows")]
use lib_ocr::win_sc;

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
fn take_sc(mode: &ImageMode) -> win_sc::error::Result<DynamicImage> {
    let image_handler = thread::spawn(move || { // Use move to transfer ownership of mode_clone
        let window_handle = window::window_handle("translator")?;
        monitor::monitor_sc(Some(&window::get_window_rect(window_handle)))
    });

    let img_resource = image_handler.join().unwrap()?;

    match mode {
        ImageMode::Save => win_sc::save_as_image(&ImageResource {
            bits: img_resource.bits,
            size: img_resource.size,
        }),
        ImageMode::NoSave => win_sc::create_dynamic_image(&ImageResource {
            bits: img_resource.bits,
            size: img_resource.size,
        })
    }
}


fn get_target_langs(deepl: &lib_translator::DeepL) -> HashMap<String, MenuItem> {
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

fn get_lang_dropdown(lang_choice: &Label, lang_choices: HashMap<String, MenuItem>) -> Menu {
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

fn get_src_langs() -> HashMap<String, MenuItem> {
    lib_ocr::get_tesseract_supported()
        .into_iter()
        .map(|lang_code| (lang_code.clone(), MenuItem::with_label(&lang_code)))
        .collect::<HashMap<String, MenuItem>>()
}

#[derive(Debug, Serialize, Deserialize)]
struct ConfigOptions {
    api_key: String,
}

fn create_config(config: &ConfigOptions) {
    let mut file = match File::create("config.txt") {
        Ok(file) => file,
        Err(e) => panic!("{:?}", e),
    };

    let _ = file.write_all(serde_json::to_string(config).unwrap().as_bytes());
}

fn get_config(config_file: &mut File) -> ConfigOptions {
    let mut string_config = String::new();
    config_file.read_to_string(&mut string_config).unwrap();
    let config: ConfigOptions = serde_json::from_str(&string_config).unwrap();
    return config;
}

pub fn build_api_dependent(
    api_key: &str,
    api_key_label: &Label,
    source_lang_choice: &Label,
    target_lang_choice: &Label,
    source: &MenuItem,
    target: &MenuItem,
    mainwindow: &ApplicationWindow,
    textwindow: &ApplicationWindow,
) {
    println!("API key: {}", api_key);
    api_key_label.set_text(&api_key);
    let deepl = &mut lib_translator::DeepL::new(String::from(api_key));

    source.set_submenu(Some(&get_lang_dropdown(
        &source_lang_choice,
        get_src_langs(),
    )));
    target.set_submenu(Some(&get_lang_dropdown(
        &target_lang_choice,
        get_target_langs(deepl),
    )));

    mainwindow.show_all();
    textwindow.show_all();
}

pub fn build_ui(
    application: &Application,
    mainwindow: &ApplicationWindow,
    textwindow: &ApplicationWindow,
    authwindow: &ApplicationWindow,
) {
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 10);
    let tbox = gtk::Box::new(gtk::Orientation::Vertical, 10);

    tbox.set_opacity(0.5);
    mainwindow.set_app_paintable(true);

    let label = gtk::Label::new(Some("<translated text>"));
    let overlay_text = gtk::Label::new(Some("Overlay text"));
    let button = Button::with_label("Translate");
    let source_lang_choice = gtk::Label::new(Some("eng"));
    let target_lang_choice = gtk::Label::new(Some("de"));
    let api_key_entry = Entry::new();
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

    overlay_text.set_widget_name("overlay_text");

    let provider = gtk::CssProvider::new();
    let style = format!(
        "label#overlay_text {{ color: white; font-size: 70px; font-weight: bold; }}",
    );
    provider.load_from_data(style.as_bytes()).unwrap();

    StyleContext::add_provider_for_screen(
        &gdk::Screen::default().expect("Error initializing gtk css provider."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    provider.connect_parsing_error(|_, _, err| {
        println!("Error: {}", err);
    });

    tbox.pack_start(&overlay_text, false, false, 10);

    vbox.pack_start(&menu, false, false, 10);
    vbox.pack_start(&button_lang_box, false, false, 10);
    vbox.pack_start(&label, true, true, 10);

    vbox.set_margin(25);

    textwindow.set_child(Some(&vbox));
    mainwindow.set_child(Some(&tbox));

    match File::open("config.txt") {
        Ok(config_file) => {
            let mut file = config_file;
            let app_configuration = get_config(&mut file);
            build_api_dependent(
                &app_configuration.api_key,
                &api_key_label,
                &source_lang_choice,
                &target_lang_choice,
                &source,
                &target,
                mainwindow,
                textwindow,
            )
        }
        Err(_) => println!("No saved configuration found"),
    };

    set_api_key_button.connect_clicked(
        glib::clone!(
                @weak mainwindow,
                @weak textwindow,
                @strong api_key_label,
                @weak source_lang_choice,
                @weak target_lang_choice,
                @strong source,
                @strong target => move |_| {

            let api_key = api_key_entry.text().to_string();
            build_api_dependent(&api_key, &api_key_label, &source_lang_choice, &target_lang_choice, &source, &target, &mainwindow, &textwindow);
            create_config(&ConfigOptions { api_key: api_key });
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

            let start = Instant::now();
            
            let from_lang = source_lang_choice.text();
            let to_lang = target_lang_choice.text();

            #[cfg(target_os = "windows")]
            let text = match take_sc(&ImageMode::NoSave) {
                Ok(dynamic_image) => match run_ocr_img(&dynamic_image, &from_lang) {
                    Ok(text) => text, 
                    Err(err) => lib_ocr::errors::err_to_string(&err)
                },
                Err(err) => win_sc::error::err_to_string(&err)
            };
            
            #[cfg(target_os = "linux")]
            let text = match lib_ocr::run_ocr("./assets/placeholder_de.png", &from_lang) {
                Ok(text) => text,
                Err(e) => lib_ocr::errors::err_to_string(&e)
            };
            
            let api_key = api_key_label.text().to_string();
            let deepl = lib_translator::DeepL::new(api_key);

            let translated_text = match deepl.translate_text(&text, &to_lang.to_uppercase()) {
                Ok(text) => text,
                Err(_) => String::from("Could not translate")
            };

            let duration = start.elapsed();
            println!("duration : {:?}", duration);

            label.set_text(&translated_text);
            label.set_line_wrap(true);
            label.set_size_request(500, -1);
        }),
    );
}
