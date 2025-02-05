use std::cell::RefCell;
use std::rc::Rc;
use gtk::ApplicationWindow;

use crate::create;
use crate::debug;
use crate::destinations;
use crate::flights;
use crate::globals;
use crate::home;
use crate::places;
use crate::util;
use crate::open;

/// Routes to a specified screen by clearing the window and updating the current screen state.
fn route(window: &ApplicationWindow, current_screen: &Rc<RefCell<String>>, new_screen: &str) {
    util::clear_window(window);
    util::set_current_screen(current_screen, new_screen);
    
    match new_screen {
        "HOME" => home::view(window),
        "FLIGHTS" => flights::view(window),
        "DESTINATIONS" => destinations::view(window),
        "PLACES" => places::view(window),
        "DEBUG" => debug::view(window),
        "CREATE" => create::view(window),
        "OPEN" => open::view(window),
        _ => globals::add_to_debug_log("ERROR in 'routes.rs': 'new_screen' matched no specified route."),
    }
}

/// Determines which screen to return to based on the current screen state.
/// Ensures logical navigation backward within the application.
pub fn route_back(window: &ApplicationWindow, current_screen: &Rc<RefCell<String>>) {
    let screen = current_screen.borrow().clone();
    match screen.as_str() {
        "HOME" => route(window, current_screen, "HOME"), // No back navigation from HOME
        "FLIGHTS" => route(window, current_screen, "CREATE"),
        "DESTINATIONS" => route(window, current_screen, "CREATE"),
        "PLACES" => route(window, current_screen, "CREATE"),
        "CREATE" => route(window, current_screen, "HOME"),
        "DEBUG" => route(window, current_screen, "HOME"),
        "OPEN" => route(window, current_screen, "HOME"),
        _ => globals::add_to_debug_log("ERROR in 'routes.rs' ln 61: 'current_screen' in route_back matched no specified route."),
    }
}

/// Determines the next screen to navigate to based on the current state.
/// If no valid forward navigation is available, it safely does nothing.
pub fn route_forward(window: &ApplicationWindow, current_screen: &Rc<RefCell<String>>, new_screen: &str) {
    let screen = current_screen.borrow().clone();
    match screen.as_str() {
        "HOME" => match new_screen {
            "CREATE" => route(window, current_screen, "CREATE"),
            "DEBUG" => route(window, current_screen, "DEBUG"),
            "OPEN" => route(window, current_screen, "OPEN"),
            _ => {}
        },
        "FLIGHTS" => {
            // No forward navigation available from FLIGHTS currently.
            // Placeholder for future functionality.
        }
        "DESTINATIONS" => {
            // No forward navigation available from DESTINATIONS currently.
            // Placeholder for future functionality.
        }
        "PLACES" => {
            // No forward navigation available from PLACES currently.
            // Placeholder for future functionality.
        }
        "CREATE" => match new_screen {
            "FLIGHTS" => route(window, current_screen, "FLIGHTS"),
            "DESTINATIONS" => route(window, current_screen, "DESTINATIONS"),
            "PLACES" => route(window, current_screen, "PLACES"),
            "OPEN" => route(window, current_screen, "OPEN"),
            _ => {}
        },
        "DEBUG" => {
            // No forward navigation available from DEBUG currently.
            // Placeholder for future functionality.
        }
        _ => globals::add_to_debug_log("ERROR in 'routes.rs': 'current_screen' in route_forward matched no specified route."),
    }
}