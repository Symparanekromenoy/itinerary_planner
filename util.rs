use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use glib::clone;
use gtk::prelude::*;
use gtk::{CssProvider, Entry, Label};

// Connects a function to an entry's "changed" event (triggered when the text changes)
pub fn on_change<F>(entry: gtk::Entry, function: F)
where
    F: Fn() + 'static,
{
    entry.connect_changed(clone!(@weak entry => move |_| {
        function();
    }));
}

// Helper function to set margins for a widget (top, bottom, start, end)
fn set_margins(widget: &impl gtk::prelude::WidgetExt, margin: &[i32]) {
    widget.set_margin_top(margin[0]);
    widget.set_margin_bottom(margin[1]);
    widget.set_margin_start(margin[2]);
    widget.set_margin_end(margin[3]);
}

// Adds a Label with text to a Fixed container, applies margins, and optional class
pub fn add_text(
    text: &str,
    margin: &[i32],
    pos: &[f32],
    window: &gtk::Fixed,
    class: Option<&str>,
) {
    let label = Label::new(Some(text));
    set_margins(&label, margin);
    label.set_xalign(pos[0]);
    label.set_yalign(pos[1]);

    // Apply class if provided
    if let Some(value) = class {
        label.style_context().add_class(value);
    }

    window.add(&label);
}

// Returns a Label with text, applying margins and optional class (without adding to window)
pub fn add_text_manual(
    text: &str,
    margin: &[i32],
    class: Option<&str>,
) -> gtk::Label {
    let label = Label::new(Some(text));
    set_margins(&label, margin);

    // Apply class if provided
    if let Some(value) = class {
        label.style_context().add_class(value);
    }

    label
}

// Formats a HashMap into a string with "key: value" pairs separated by newlines
#[allow(dead_code)]
pub fn format_hashmap(map: &HashMap<String, String>) -> String {
    map.iter()
        .map(|(key, value)| format!("{}: {}", key, value))
        .collect::<Vec<_>>()
        .join("\n")
}

// Creates a Box containing a Button with text, applies margins, size, and optional class
#[allow(dead_code)]
pub fn add_text_button_manual(
    text: &str,
    margin: &[i32],
    size: &[i32],
    class: Option<&str>,
) -> gtk::Box {
    let container = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    let button = gtk::Button::with_label(text);

    set_margins(&button, margin);
    button.set_size_request(size[0], size[1]);

    // Apply class if provided
    if let Some(value) = class {
        button.style_context().add_class(value);
    }

    container.pack_start(&button, false, false, 0);

    container
}

// Creates a Box containing a Label and Entry, applies margins, sizes, and optional classes
pub fn create_text_entry_manual(
    label_text: &str,
    size: &[i32],
    margin: &[i32],
    class_label: Option<&str>,
    class_entry: Option<&str>,
) -> gtk::Box {
    let text_box = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    let text_entry = gtk::Entry::new();
    let label = gtk::Label::new(Some(label_text));

    label.set_xalign(0.0);
    label.set_width_chars(20);

    // Apply class to label if provided
    if let Some(value) = class_label {
        label.style_context().add_class(value);
    }

    // Apply class to entry if provided
    if let Some(value) = class_entry {
        text_entry.style_context().add_class(value);
    }

    set_margins(&text_box, margin);
    text_entry.set_size_request(size[0], size[1]);

    text_box.pack_start(&label, false, false, 0);
    text_box.pack_start(&text_entry, false, false, 0);

    text_box
}

// Searches for and returns the first Entry widget within a Box container
pub fn find_entry_in_box(box_container: &gtk::Box) -> Entry {
    for child in box_container.children() {
        if let Ok(entry) = child.downcast::<Entry>() {
            return entry;
        }
    }
    panic!("No Entry found in the box");
}

// Loads a CSS file from the specified path and returns a CssProvider
// If the file fails to load, an error message is printed
pub fn load_css(path: &str) -> gtk::CssProvider {
    let provider = CssProvider::new();
    if provider.load_from_path(path).is_err() {
        eprintln!("Failed to load CSS file: {}", path);
    }
    provider
}

// Removes all child widgets from the given window and redraws it
pub fn clear_window(window: &gtk::ApplicationWindow) {
    for child in window.children() {
        window.remove(&child);
    }
    window.queue_draw();
}

// Updates the current screen variable to a new screen identifier
pub fn set_current_screen(
    screen_var: &Rc<RefCell<String>>,
    new_screen_var: &str,
) {
    *screen_var.borrow_mut() = new_screen_var.to_string();
}