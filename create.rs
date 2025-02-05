use gtk::prelude::*;
use gtk::{Fixed, Label, Orientation};
use glib::clone;
use crate::{globals, universals, util};

// Enum to differentiate between label and box widgets
enum Widget {
    Label(Label),
    Box(gtk::Box),
}

/// Builds and displays the "Create Itinerary" screen
pub fn view(window: &gtk::ApplicationWindow) {
    let database = globals::get_database();

    // Create main containers
    let fixed_container = Fixed::new();
    let vertical_container = gtk::Box::new(Orientation::Vertical, 5);

    // Create UI elements
    let title_label = util::add_text_manual("ITINERARY PLANNER", &[0, 50, 0, 0], Some("big"));
    let subtitle_label = util::add_text_manual("Options: ", &[0, 10, 0, 0], Some("medium"));
    let title_entry = util::create_text_entry_manual("Title: ", &[500, 20], &[0, 0, 30, 0], Some("text_entry"), Some("text_entry_label"));
    let start_date_entry = util::create_text_entry_manual("Start Date: ", &[500, 20], &[0, 0, 30, 0], Some("text_entry"), Some("text_entry_label"));
    let end_date_entry = util::create_text_entry_manual("End Date: ", &[500, 20], &[0, 0, 30, 0], Some("text_entry"), Some("text_entry_label"));
    let destinations_label = util::add_text_manual("Shift + 3 - Destination/s", &[15, 0, 30, 0], None);
    let flights_label = util::add_text_manual("Shift + 4 - Flights", &[15, 0, 30, 0], None);
    let places_label = util::add_text_manual("Shift + 5 - Places", &[15, 0, 30, 0], None);
    let travel_label = util::add_text_manual("Shift + 6 - Travel", &[15, 0, 30, 0], None);

    // Extract entry fields for further use
    let ext_title_entry = util::find_entry_in_box(&title_entry);
    let ext_start_date_entry = util::find_entry_in_box(&start_date_entry);
    let ext_end_date_entry = util::find_entry_in_box(&end_date_entry);

    // Load session data into entry fields
    let db_instance = database.lock().unwrap();
    if let Some(title) = db_instance.get_session_storage().get("title") {
        ext_title_entry.set_text(title);
    }
    if let Some(start_date) = db_instance.get_session_storage().get("start_date") {
        ext_start_date_entry.set_text(start_date);
    }
    if let Some(end_date) = db_instance.get_session_storage().get("end_date") {
        ext_end_date_entry.set_text(end_date);
    }

    // Set up event listeners to store input changes
    util::on_change(ext_title_entry.clone(), clone!(@weak ext_title_entry, @strong database => move || {
        let new_text = ext_title_entry.text().to_string();
        let mut db_instance = database.lock().unwrap();
        db_instance.set_kv_session_storage("title", &new_text);
    }));

    util::on_change(ext_start_date_entry.clone(), clone!(@weak ext_start_date_entry, @strong database => move || {
        let new_text = ext_start_date_entry.text().to_string();
        let mut db_instance = database.lock().unwrap();
        db_instance.set_kv_session_storage("start_date", &new_text);
    }));

    util::on_change(ext_end_date_entry.clone(), clone!(@weak ext_end_date_entry, @strong database => move || {
        let new_text = ext_end_date_entry.text().to_string();
        let mut db_instance = database.lock().unwrap();
        db_instance.set_kv_session_storage("end_date", &new_text);
    }));

    // Group widgets for easier layout management
    let widgets = vec![
        Widget::Label(title_label),
        Widget::Label(subtitle_label),
        Widget::Box(title_entry),
        Widget::Box(start_date_entry),
        Widget::Box(end_date_entry),
        Widget::Label(destinations_label),
        Widget::Label(flights_label),
        Widget::Label(places_label),
        Widget::Label(travel_label),
    ];

    // Add widgets to vertical container
    for widget in widgets {
        match widget {
            Widget::Label(label) => {
                label.set_xalign(0.0);
                vertical_container.pack_start(&label, false, false, 0);
            }
            Widget::Box(box_widget) => {
                vertical_container.pack_start(&box_widget, false, false, 0);
            }
        }
    }

    // Set margins and add containers to the window
    vertical_container.set_margin_top(50);
    vertical_container.set_margin_start(50);
    fixed_container.add(&vertical_container);

    // Add navigation bar
    let nav = universals::nav(&universals::Screens::Create);
    fixed_container.add(&nav);

    // Attach UI to the window and display it
    window.add(&fixed_container);
    window.show_all();
}
