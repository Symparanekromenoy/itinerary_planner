use gtk::prelude::*;
use gtk::Fixed;
use crate::util;
use crate::universals;

pub fn view(window: &gtk::ApplicationWindow) {
    let fixed = Fixed::new();
    util::add_text("ITINERARY PLANNER",        &[40, 20, 50, 20], &[0.1, 0.1], &fixed, Some("big"));
    util::add_text("Shift + 1 - Create new itinerary", &[140, 20, 80, 20], &[0.1, 0.1], &fixed, None);
    util::add_text("Shift + 2 - Open itinerary",       &[180, 20, 80, 20], &[0.1, 0.1], &fixed, None);
    util::add_text("Shift + F - Debug",       &[220, 20, 80, 20], &[0.1, 0.1], &fixed, None);

    let nav = universals::nav(&universals::Screens::Home);
    fixed.add(&nav);

    window.add(&fixed);
    window.show_all();
}