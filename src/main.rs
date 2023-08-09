use lib_translator;
use lib_ocr;
use lib_gui;

fn main() {
    // entry point
    lib_gui::gui();
    lib_ocr::ocr();
    lib_translator::translate();
}
