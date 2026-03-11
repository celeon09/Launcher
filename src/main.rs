mod app_index;
mod executor;
mod search;
mod ui;

use adw::prelude::*;
use gtk::glib;

fn main() -> glib::ExitCode {

    let app = adw::Application::builder()
        .application_id("com.example.launcher")
        .flags(gio::ApplicationFlags::HANDLES_COMMAND_LINE)
        .build();

    app.connect_startup(|app| {
        ui::load_css();
        std::mem::forget(app.hold()); // Keep the application running forever in the background
    });

    // We handle the command line signal to intercept secondary launches.
    app.connect_command_line(|app, _cmdline| {
        // Build (or retrieve) the launcher window
        let is_first_launch = app.windows().is_empty();

        let window = if is_first_launch {
            let win = ui::build_ui(app);
            // On first launch, show the window
            win.present();
            win
        } else {
            app.windows().into_iter().next()
                .unwrap()
                .downcast::<adw::ApplicationWindow>()
                .expect("Expected ApplicationWindow")
        };

        // If it's NOT the first launch, toggle visibility
        if !is_first_launch {
            if window.is_visible() {
                window.hide();
            } else {
                window.present();
            }
        }

        glib::ExitCode::SUCCESS // Return exit code for the secondary instance
    });

    // We must return glib::ExitCode
    app.run()
}
