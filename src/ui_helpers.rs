use gtk4::prelude::*;
use gtk4::{Box, Label, Orientation};

pub fn create_page() -> Box {
    let page = Box::new(Orientation::Vertical, 12);
    page.set_margin_top(18);
    page.set_margin_bottom(18);
    page.set_margin_start(18);
    page.set_margin_end(18);
    page
}

pub fn create_row() -> Box {
    let row = Box::new(Orientation::Horizontal, 12);
    row.set_margin_top(6);
    row.set_margin_bottom(6);
    row
}

pub fn add_label(container: &Box, text: &str, width: i32) {
    let label = Label::new(Some(text));
    label.set_width_chars(width / 10);
    label.set_xalign(0.0);
    if width > 0 {
        label.set_width_request(width);
    } else {
        label.set_hexpand(true);
    }
    container.append(&label);
}

pub fn add_header(container: &Box, text: &str) {
    let header = Label::new(Some(text));
    header.add_css_class("title-1");
    header.set_halign(gtk4::Align::Start);
    header.set_margin_bottom(12);
    container.append(&header);
}
