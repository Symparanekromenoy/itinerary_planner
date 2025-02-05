use gtk::prelude::*;
use gtk::{TextView, Box as GtkBox, Orientation, WrapMode};
use std::cell::RefCell;
use std::rc::Rc;
use gtk::glib::timeout_add_local;
use std::time::Duration;
use crate::util;
use crate::globals;

pub enum Screens {
    Home,
    Create,
    Debug,
    Destinations,
    Flights,
    Places,
    Open,
}

// Adds a text label to the given container, styled as a navbar item
fn add_nav_text(container: &gtk::Box, text: &str, class: Option<&str>) {
    if let Some(value) = class {
    	let label = util::add_text_manual(text, &[0, 0, 0, 0], Some(value));
    	container.pack_start(&label, false, false, 0);
    } else {
    	let label = util::add_text_manual(text, &[0, 0, 0, 0], Some("navbar-item"));
    	container.pack_start(&label, false, false, 0);
    }
}

// Creates a navigation bar with contextual shortcuts based on the current screen
pub fn nav(screen_type: &Screens) -> gtk::Box {
    let container_outer = gtk::Box::new(gtk::Orientation::Vertical, 5);
    let container_inner = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    container_inner.set_margin_top(500);
    container_inner.set_margin_bottom(5);
    container_inner.set_margin_start(50);
    container_inner.set_margin_end(50);


    match screen_type {
        Screens::Home => {
			add_nav_text(&container_inner, "ESC to quit", None);
			container_outer.pack_start(&container_inner, false, false, 0);
        }
        Screens::Create => {
			add_nav_text(&container_inner, "ESC to quit", None);
            add_nav_text(&container_inner, "Shift + 1 to go back", None);
            add_nav_text(&container_inner, "Shift + 9 to create/save itinerary", None);
            container_outer.pack_start(&container_inner, false, false, 0);
        }
        Screens::Open => {
        	add_nav_text(&container_inner, "ESC to quit", None);
			add_nav_text(&container_inner, "Shift + 1 to go back", None);
			add_nav_text(&container_inner, "Shift + [x] to select", None);
            container_outer.pack_start(&container_inner, false, false, 0);

            let pager_container = gtk::Box::new(gtk::Orientation::Horizontal, 5);
            pager_container.set_margin_top(20);
		    pager_container.set_margin_bottom(5);
		    pager_container.set_margin_start(50);
		    pager_container.set_margin_end(50);

            add_nav_text(&pager_container, "Shift + 8 to next page", Some("pager")); 
            add_nav_text(&pager_container, "Shift + 9 to previous page", Some("pager")); 

            container_outer.pack_start(&pager_container, false, false, 0);

        }
        Screens::Destinations => {
        	container_inner.set_margin_top(580);
			add_nav_text(&container_inner, "ESC to quit", None);
            add_nav_text(&container_inner, "Shift + 1 to go back", None);
            container_outer.pack_start(&container_inner, false, false, 0);

            let pager_container = gtk::Box::new(gtk::Orientation::Horizontal, 5);
            pager_container.set_margin_top(20);
		    pager_container.set_margin_bottom(5);
		    pager_container.set_margin_start(50);
		    pager_container.set_margin_end(50);

            add_nav_text(&pager_container, "Shift + 9 to save destination", None);
            add_nav_text(&pager_container, "Shift + Ctrl + [x] to delete destination", None);

            container_outer.pack_start(&pager_container, false, false, 0);
        }

        _ => {
        	add_nav_text(&container_inner, "ESC to quit", None);
            add_nav_text(&container_inner, "Shift + 1 to go back", None);
            container_outer.pack_start(&container_inner, false, false, 0);
        }
    }

    container_outer
}

// Applies margin settings to a given widget
fn set_margins(widget: &impl gtk::prelude::WidgetExt, margins: &[i32]) {
    widget.set_margin_top(margins[0]);
    widget.set_margin_bottom(margins[1]);
    widget.set_margin_start(margins[2]);
    widget.set_margin_end(margins[3]);
}

// Creates a debug console that displays log messages in a non-editable text area
pub fn debug_console(size: &[i32], margins: &[i32], class: Option<&str>) -> GtkBox {
    let container = GtkBox::new(Orientation::Horizontal, 5);
    set_margins(&container, margins);

    let console = TextView::new();
    if let Some(value) = class {
        console.style_context().add_class(value);
    }

    console.set_editable(false); // Prevents user input
    console.set_cursor_visible(false);
    console.set_wrap_mode(WrapMode::Word); // Enables word wrapping
    console.set_size_request(size[0], size[1]);

    // Load initial logs into the text buffer
    let buffer = console.buffer().unwrap();
    let log = globals::get_debug_log().join("\n");
    buffer.set_text(&log);

    // Store the console in Rc<RefCell<>> to allow dynamic updates
    let console_ref = Rc::new(RefCell::new(console.clone()));

    // Periodically update the console with new log data
    let console_clone = console_ref.clone();
    timeout_add_local(Duration::from_millis(500), move || {
        let log = globals::get_debug_log().join("\n");
        let buffer = console_clone.borrow().buffer().unwrap();
        buffer.set_text(&log);
        gtk::glib::Continue(true) // Ensures the update loop continues
    });

    container.pack_start(&*console_ref.borrow(), false, false, 0);
    container
}
