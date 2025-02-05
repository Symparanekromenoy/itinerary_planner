use gtk::prelude::*;
use gtk::Fixed;
use crate::util;
use crate::universals;

pub fn view(window: &gtk::ApplicationWindow) {
    let fixed_container = Fixed::new();
    util::add_text(
        "DEBUGGING SCREEN",
        &[40, 20, 50, 20],
        &[0.1, 0.1],
        &fixed_container,
        Some("big"),
    );

    let navigation = universals::nav(&universals::Screens::Debug);
    fixed_container.add(&navigation);

    let console = universals::debug_console(&[1000, 300], &[150, 5, 50, 50], Some("console"));
    fixed_container.add(&console);

    window.add(&fixed_container);
    window.show_all();
}