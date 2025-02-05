use gtk::prelude::*;
use gtk::Fixed;
use crate::util;
use crate::universals;
use crate::globals;

pub fn view(window: &gtk::ApplicationWindow) {
    let fixed = Fixed::new();
    let database = globals::get_database();
    let database_instance = database.lock().unwrap();

    util::add_text("ITINERARY PLANNER", &[40, 100, 50, 20], &[0.1, 0.1], &fixed, Some("big"));
    util::add_text("Saved Itineraries:",                      &[140, 20, 50, 20], &[0.5, 0.5], &fixed, Some("medium"));

    let mut i = 0;
    for itinerary in database_instance.list_all_itineraries() {
        util::add_text(&format!("{}. {}", i + 2, itinerary), &[(180 + (i * 25)), 20, 100, 20], &[0.1, 0.1], &fixed, None);
        i += 1;
    }

    if database_instance.list_all_itineraries().len() == 0 {
        util::add_text("No itineraries found.", &[180, 20, 100, 20], &[0.1, 0.1], &fixed, None);
    }

    let nav = universals::nav(&universals::Screens::Open);
    fixed.add(&nav);

    window.add(&fixed);
    window.show_all();
}