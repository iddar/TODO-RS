extern crate gtk;
use gtk::*;

mod components;
use components::App;

fn main() {
    let app = App::new();
    app.window.show_all();
    gtk::main();
}