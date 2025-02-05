use std::cell::RefCell;
use std::rc::Rc;

use gtk::gdk::EventKey;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

// Project-wide modules
mod globals;
mod database;
mod util;
mod universals;
mod create;
mod debug;
mod destinations;
mod flights;
mod home;
mod places;
mod routes;
mod open;

/// Logs messages to the application's debug log.
fn log(message: &str) {
    globals::add_to_debug_log(message);
}

/// Sets up key press event handling for the window.
fn listen(window: &ApplicationWindow, app: &Application) {
    let current_screen = Rc::new(RefCell::new(String::from("HOME")));

    window.connect_key_press_event({
        let app = app.clone();
        let window = window.clone();
        move |_, event| {
            handle_key_press(event, &app, &window, &current_screen);
            Inhibit(false)
        }
    });
}

/// Processes key press events and updates the screen state accordingly.
fn handle_key_press(
    event: &EventKey,
    app: &Application,
    window: &ApplicationWindow,
    current_screen: &Rc<RefCell<String>>,
) {
    let key = event.keyval();

    match key {
        // Escape
        gtk::gdk::keys::constants::Escape => app.quit(), // Quit application
        // Shift + 1
        gtk::gdk::keys::constants::exclam => {
            // Toggle between HOME and CREATE screen
            if *current_screen.borrow() != "HOME" {
                routes::route_back(window, current_screen);
            } else {
                routes::route_forward(window, current_screen, "CREATE");
            }
        }
        // Shift + 2
        gtk::gdk::keys::constants::at => {
            // Only switch to OPEN if Shift + 2 is pressed in HOME screen.
            if *current_screen.borrow() == "HOME" {
                routes::route_forward(window, current_screen, "OPEN");
            }
        }
        // Shift + 3
        gtk::gdk::keys::constants::numbersign => {
            routes::route_forward(window, current_screen, "DESTINATIONS");
        }
        // Shift + 4
        gtk::gdk::keys::constants::dollar => {
            routes::route_forward(window, current_screen, "FLIGHTS");
        }
        // Shift + 5
        gtk::gdk::keys::constants::percent => {
            routes::route_forward(window, current_screen, "PLACES");
        }
        // Shift + 6
        gtk::gdk::keys::constants::asciicircum => {
            // Test function: Adds a sample itinerary entry
            let database = globals::get_database();
            let database_instance = database.lock().unwrap();

            log("Adding test itinerary with title: Yami");
            match database_instance.add_itinerary("Yami".to_string()) {
                Err(e) => log(&format!(
                    "Error adding itinerary in database: {}",
                    e
                )),
                Ok(s) => log(&format!("Itinerary added: {}", s)),
            }
        }
        // Shift + 9
        gtk::gdk::keys::constants::parenleft => {
            // In CREATE screen.
            let database = globals::get_database();
            let mut database_instance = database.lock().unwrap();

            if *current_screen.borrow() == "CREATE" {
                match database_instance.store_session_storage_in_database() {
                    Ok(s) => {
                        if s == "SUCCESS" {
                            log("Success in storing session storage to database.");
                            routes::route_back(window, current_screen);
                        } 
                        else if s == "ERROR_NO_TITLE" {
                            log("Error in database.rs in store_session_storage_in_database(): Session storage key 'title' had no value..");
                            routes::route_back(window, current_screen);
                            routes::route_forward(window, current_screen, "DEBUG");
                        }
                    }
                    Err(e) => {
                        log(&format!("Error in main.rs: {}", e));
                        routes::route_back(window, current_screen);
                        routes::route_forward(window, current_screen, "DEBUG");
                    }
                }
            }

            // In DESTINATIONS screen.
            if *current_screen.borrow() == "DESTINATIONS" {
                database_instance.add_current_destination_to_session_storage();
                routes::route_back(window, current_screen);
            }
        }
        // Shift + F
        gtk::gdk::keys::constants::F => {
            routes::route_forward(window, current_screen, "DEBUG");
        }
        _ => println!("Unhandled key press: {:?}", key),
    }
}

fn main() {
    let app = Application::new(Some("com.example.ItineraryPlanner"), Default::default());

    app.connect_activate(|app| {
        // Initialize database session and log session ID
        let database = globals::get_database();
        let db_instance = database.lock().unwrap();
        log(&format!("Session ID: {}", db_instance.session_id()));

        // Set up main application window
        let window = ApplicationWindow::new(app);
        window.set_title("Black Window");
        window.set_default_size(1300, 800);
        window.style_context().add_class("black-window");

        // Load and display the home screen
        home::view(&window);
        
        // Enable key press event listening
        listen(&window, app);

        // Apply custom styling from CSS file
        let css_provider = util::load_css("style.css");
        gtk::StyleContext::add_provider_for_screen(
            &gtk::gdk::Screen::default().expect("Failed to initialize GTK screen"),
            &css_provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    });

    app.run();
}