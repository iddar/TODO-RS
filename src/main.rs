extern crate gtk;

use gtk::*;

mod components;
use components::App;

mod db;
use self::db::DataConect;

fn main() {
    let conn = DataConect::new("TODO.db");
    // conn.add("Milk");
    let app = App::new(conn);
    app.window.show_all();
    gtk::main();
}