use dotenv::dotenv;
use lib_gui;

fn main() {
    dotenv().ok();

    let app = lib_gui::init_app();
    lib_gui::run_app(&app);
}
