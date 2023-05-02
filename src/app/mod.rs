use crate::window::Window;
use gtk::{prelude::*, Application, Label};

const WINDOW_MIN_HEIGHT: i32 = 120;

pub mod utils;

pub fn build_main_window(
    app: &Application,
    full_width_label_bg: bool,
    hide_label_on_empty_text: bool,
    allow_click_through_me: bool,
) -> Window {
    let window = Window::new(app);

    window.set_size_request(500, WINDOW_MIN_HEIGHT);
    window.set_title(Some("Waylyrics"));
    window.set_decorated(false);
    window.present();

    let olabel = Label::builder().label("Waylyrics").build();
    let tlabel = Label::builder()
        .label("")
        .name("translated")
        .visible(false)
        .build();

    if hide_label_on_empty_text {
        olabel.connect_label_notify(utils::hide_on_empty);
        tlabel.connect_label_notify(utils::hide_on_empty);
    }

    olabel.set_vexpand(true);
    tlabel.set_vexpand(true);

    if !full_width_label_bg {
        olabel.set_halign(gtk::Align::Center);
        tlabel.set_halign(gtk::Align::Center);
    }

    let verical_box = gtk::Box::builder()
        .baseline_position(gtk::BaselinePosition::Center)
        .orientation(gtk::Orientation::Vertical)
        .build();
    verical_box.set_vexpand(true);
    verical_box.set_valign(gtk::Align::Center);

    let slibing: Option<&gtk::Box> = None;
    verical_box.insert_child_after(&olabel, slibing);
    verical_box.insert_child_after(&tlabel, Some(&olabel));

    window.set_child(Some(&verical_box));

    if allow_click_through_me {
        utils::allow_click_through(&window);
    }

    window
}

pub fn get_label(window: &gtk::Window, translated: bool) -> Label {
    let vbox: gtk::Box = window.child().unwrap().downcast().unwrap();
    if !translated {
        vbox.first_child().unwrap().downcast().unwrap()
    } else {
        vbox.last_child().unwrap().downcast().unwrap()
    }
}