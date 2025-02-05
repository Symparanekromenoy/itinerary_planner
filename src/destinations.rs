use glib::clone;
use gtk::Label;
use crate::globals;
use gtk::Orientation;
use gtk::prelude::*;
use gtk::Fixed;
use crate::util;
use crate::universals;

// Enum to differentiate between label and box widgets
enum Widget {
    Label(Label),
    Box(gtk::Box),
}

pub fn view(window: &gtk::ApplicationWindow) {
    let database = globals::get_database();

    // Create main containers
    let fixed_container = Fixed::new();
    let vertical_container = gtk::Box::new(Orientation::Vertical, 5);

    // Create UI elements
    let title_label = util::add_text_manual("ITINERARY PLANNER", &[0, 50, 0, 0], Some("big"));
    let subtitle_label = util::add_text_manual("Add Destination: ", &[0, 10, 0, 0], Some("medium"));
    let country_title_entry = util::create_text_entry_manual("Country Name: ", &[500, 20], &[0, 0, 30, 0], Some("text_entry"), Some("text_entry_label"));
    let country_start_date_entry = util::create_text_entry_manual("Start Date: ", &[500, 20], &[0, 0, 30, 0], Some("text_entry"), Some("text_entry_label"));
    let country_end_date_entry = util::create_text_entry_manual("End Date: ", &[500, 20], &[0, 0, 30, 0], Some("text_entry"), Some("text_entry_label"));

    // Extract entry fields for further use
    let ext_country_entry = util::find_entry_in_box(&country_title_entry);
    let ext_country_start_date_entry = util::find_entry_in_box(&country_start_date_entry);
    let ext_country_end_date_entry = util::find_entry_in_box(&country_end_date_entry);

    // Load session data into entry fields
    let db_instance = database.lock().unwrap();
    if let Some(current_country_title) = db_instance.get_session_storage().get("current_country_title") {
        ext_country_entry.set_text(current_country_title);
    }
    if let Some(current_country_start_date) = db_instance.get_session_storage().get("current_country_start_date") {
        ext_country_start_date_entry.set_text(current_country_start_date);
    }
    if let Some(current_country_end_date) = db_instance.get_session_storage().get("current_country_end_date") {
        ext_country_end_date_entry.set_text(current_country_end_date);
    }

    // Set up event listeners to store input changes
    util::on_change(ext_country_entry.clone(), clone!(@weak ext_country_entry, @strong database => move || {
        let new_text = ext_country_entry.text().to_string();
        let mut db_instance = database.lock().unwrap();
        db_instance.set_kv_session_storage("current_country_title", &new_text);
    }));

    util::on_change(ext_country_start_date_entry.clone(), clone!(@weak ext_country_start_date_entry, @strong database => move || {
        let new_text = ext_country_start_date_entry.text().to_string();
        let mut db_instance = database.lock().unwrap();
        db_instance.set_kv_session_storage("current_country_start_date", &new_text);
    }));

    util::on_change(ext_country_end_date_entry.clone(), clone!(@weak ext_country_end_date_entry, @strong database => move || {
        let new_text = ext_country_end_date_entry.text().to_string();
        let mut db_instance = database.lock().unwrap();
        db_instance.set_kv_session_storage("current_country_end_date", &new_text);
    }));
    

    // Group widgets for easier layout management
    let widgets = vec![
        Widget::Label(title_label),
        Widget::Label(subtitle_label),
        Widget::Box(country_title_entry),
        Widget::Box(country_start_date_entry),
        Widget::Box(country_end_date_entry),
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
    let nav = universals::nav(&universals::Screens::Destinations);
    fixed_container.add(&nav);

    // Attach UI to the window and display it
    window.add(&fixed_container);
    window.show_all();
}