mod header;
use self::header::Header;

use gtk;
use gtk::*;
use std::process;

const CSS: &str = include_str!("styles/app.css");

use crate::db;

pub struct App {
    pub window:  Window,
    pub header:  Header,    
}

impl App {
    pub fn new(conn: db::DataConect) -> App {
        if gtk::init().is_err() {
            println!("failed to init GTK");
            process::exit(1);
        }
    
        let window = Window::new(WindowType::Toplevel);
        let header = Header::new();
        
        //Add custom CSS
        let screen = window.get_screen().unwrap();
        let style = CssProvider::new();
        let _ = CssProviderExt::load_from_data(&style, CSS.as_bytes());
        
        StyleContext::add_provider_for_screen(&screen, &style, STYLE_PROVIDER_PRIORITY_USER);
       
        window.set_default_size(600, 350);
        window.set_titlebar(&header.container);
        window.connect_delete_event(move |_, _| {
            main_quit();
            Inhibit(false)
        });


        let padding_between_children = 0;

        let listbox = ListBox::new();
        listbox.set_selection_mode(SelectionMode::None);
        
        let items = conn.list();
        for el in items.iter() {
            let name_item = format!("item {}", el.title);
            let row = ListBoxRow::new();
            let name = Label::new(&name_item[..]);
            name.set_halign(Align::Start);
            row.add(&name);
            listbox.add(&row);  
        }

        let vertical_box = Box::new(Orientation::Vertical, padding_between_children);
        vertical_box.pack_start(&listbox, true, true, 0);

        let scroll_view = ScrolledWindow::new(None, None);
        scroll_view.add_with_viewport(&vertical_box);
        
        window.add(&scroll_view);

        //return
        App {
            window,
            header,
        }
    }
}