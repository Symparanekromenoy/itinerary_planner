use gtk::prelude::*;
use gtk::Fixed;
use crate::util;
use crate::universals;

pub fn view(window: &gtk::ApplicationWindow) {
    let fixed = Fixed::new();

    util::add_text("ITINERARY PLANNER", &[40, 100, 50, 20], &[0.1, 0.1], &fixed, Some("big"));

    util::add_text("Options",                      &[140, 20, 50, 20], &[0.5, 0.5], &fixed, None);
    util::add_text("Add Place: ",                &[180, 20, 100, 20], &[0.1, 0.1], &fixed, None);

    let nav = universals::nav(&universals::Screens::Places);
    fixed.add(&nav);

    window.add(&fixed);
    window.show_all();
}