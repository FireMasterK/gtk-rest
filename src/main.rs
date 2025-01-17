mod application;
mod client;
mod window;
use application::GtkRestApplication;
use gtk::gio;
use gtk::prelude::*;
use window::GtkRestWindow;

const APP_ID: &str = "com.bnyro.rest";

fn main() {
    gio::resources_register_include!("resources.gresource").expect("Failed to register resources.");

    // Create a new GtkApplication. The application manages our main loop,
    // application windows, integration with the window manager/compositor, and
    // desktop features such as file opening and single-instance applications.
    let app = GtkRestApplication::new(APP_ID, &gio::ApplicationFlags::empty());

    // Run the application. This function will block until the application
    // exits. Upon return, we have our exit code to return to the shell. (This
    // is the code you see when you do `echo $?` after running a command in a
    // terminal.
    std::process::exit(app.run());
}
