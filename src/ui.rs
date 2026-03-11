use adw::prelude::*;
use gtk::glib;

use crate::app_index::{load_apps, AppEntry};
use crate::executor;
use crate::search::fuzzy_search;

use std::cell::RefCell;
use std::rc::Rc;

const MAX_RESULTS: usize = 10;

pub fn build_ui(app: &adw::Application) -> adw::ApplicationWindow {
    // ── Window ──────────────────────────────────────────────────────────────
    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("Launcher")
        .default_width(640)
        .resizable(false)
        .decorated(false)
        .hide_on_close(true)
        .build();

    window.add_css_class("launcher-window");

    // ── Root container ───────────────────────────────────────────────────────
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    vbox.add_css_class("launcher-box");

    // ── Search entry ─────────────────────────────────────────────────────────
    let search_entry = gtk::SearchEntry::new();
    search_entry.set_placeholder_text(Some("Search applications…"));
    search_entry.add_css_class("launcher-entry");
    search_entry.set_hexpand(true);
    search_entry.set_margin_top(8);
    search_entry.set_margin_bottom(8);
    search_entry.set_margin_start(12);
    search_entry.set_margin_end(12);

    vbox.append(&search_entry);

    // ── Separator ────────────────────────────────────────────────────────────
    let separator = gtk::Separator::new(gtk::Orientation::Horizontal);
    separator.set_visible(false);
    vbox.append(&separator);

    // ── Results list ─────────────────────────────────────────────────────────
    let results_box = gtk::ListBox::new();
    results_box.set_selection_mode(gtk::SelectionMode::Browse);
    results_box.add_css_class("results-list");

    let scrolled = gtk::ScrolledWindow::builder()
        .child(&results_box)
        .hscrollbar_policy(gtk::PolicyType::Never)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .max_content_height(400)
        .propagate_natural_height(true)
        .build();
    scrolled.set_visible(false);

    vbox.append(&scrolled);

    window.set_content(Some(&vbox));

    // ── App state ────────────────────────────────────────────────────────────
    let apps: Rc<Vec<AppEntry>> = Rc::new(load_apps());
    let current_results: Rc<RefCell<Vec<AppEntry>>> = Rc::new(RefCell::new(Vec::new()));

    // ── Search handler ───────────────────────────────────────────────────────
    {
        let results_box = results_box.clone();
        let scrolled = scrolled.clone();
        let separator = separator.clone();
        let apps = Rc::clone(&apps);
        let current_results = Rc::clone(&current_results);

        search_entry.connect_search_changed(move |entry| {
            let query = entry.text().to_string();

            // Clear previous results
            while let Some(child) = results_box.first_child() {
                results_box.remove(&child);
            }

            let matches = fuzzy_search(&query, &apps, MAX_RESULTS);
            let has_results = !matches.is_empty();

            let mut results = current_results.borrow_mut();
            results.clear();

            for (_, app) in &matches {
                let row = build_result_row(app);
                results_box.append(&row);
                results.push((*app).clone());
            }

            // Select the first row automatically
            if has_results {
                if let Some(first) = results_box.row_at_index(0) {
                    results_box.select_row(Some(&first));
                }
            }

            scrolled.set_visible(has_results);
            separator.set_visible(has_results);
        });
    }

    // ── Keyboard handler ─────────────────────────────────────────────────────
    let key_ctrl = gtk::EventControllerKey::new();
    key_ctrl.set_propagation_phase(gtk::PropagationPhase::Capture);
    {
        let search_entry = search_entry.clone();
        let results_box = results_box.clone();
        let window = window.clone();
        let current_results = Rc::clone(&current_results);

        key_ctrl.connect_key_pressed(move |_, key, _, _| {
            match key {
                gtk::gdk::Key::Escape => {
                    search_entry.set_text("");
                    window.hide();
                    glib::Propagation::Stop
                }
                gtk::gdk::Key::Return | gtk::gdk::Key::KP_Enter => {
                    let app_to_launch = if let Some(row) = results_box.selected_row() {
                        let idx = row.index() as usize;
                        let results = current_results.borrow();
                        results.get(idx).cloned()
                    } else {
                        None
                    };

                    if let Some(app) = app_to_launch {
                        if let Err(e) = executor::launch(&app) {
                            eprintln!("Failed to launch {}: {e}", app.name);
                        } else {
                            // Clear search entry and hide window
                            search_entry.set_text("");
                            window.hide();
                        }
                    }
                    glib::Propagation::Stop
                }
                gtk::gdk::Key::Up => {
                    if let Some(row) = results_box.selected_row() {
                        let idx = row.index();
                        if idx > 0 {
                            if let Some(prev) = results_box.row_at_index(idx - 1) {
                                results_box.select_row(Some(&prev));
                            }
                        }
                    }
                    glib::Propagation::Stop
                }
                gtk::gdk::Key::Down => {
                    if let Some(row) = results_box.selected_row() {
                        let idx = row.index();
                        if let Some(next) = results_box.row_at_index(idx + 1) {
                            results_box.select_row(Some(&next));
                        }
                    }
                    glib::Propagation::Stop
                }
                _ => glib::Propagation::Proceed,
            }
        });
    }
    window.add_controller(key_ctrl);

    // ── Double-click / activate row ──────────────────────────────────────────
    {
        let search_entry = search_entry.clone();
        let window = window.clone();
        let current_results = Rc::clone(&current_results);

        results_box.connect_row_activated(move |_, row| {
            let idx = row.index() as usize;
            let app_to_launch = {
                let results = current_results.borrow();
                results.get(idx).cloned()
            };

            if let Some(app) = app_to_launch {
                if let Err(e) = executor::launch(&app) {
                    eprintln!("Failed to launch {}: {e}", app.name);
                } else {
                    search_entry.set_text("");
                    window.hide();
                }
            }
        });
    }

    // ── Focus loss: hide ─────────────────────────────────────────────────────
    {
        let search_entry_clone = search_entry.clone();
        window.connect_is_active_notify(move |win| {
            if !win.is_active() {
                search_entry_clone.set_text("");
                win.hide();
            }
        });
    }

    window
}

fn build_result_row(app: &AppEntry) -> gtk::ListBoxRow {
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    hbox.set_margin_top(6);
    hbox.set_margin_bottom(6);
    hbox.set_margin_start(12);
    hbox.set_margin_end(12);

    // Icon
    let icon = gtk::Image::from_icon_name(&app.icon_name);
    icon.set_pixel_size(24);
    hbox.append(&icon);

    // Text column
    let text_box = gtk::Box::new(gtk::Orientation::Vertical, 2);

    let name_label = gtk::Label::new(Some(&app.name));
    name_label.set_halign(gtk::Align::Start);
    name_label.add_css_class("app-name");
    text_box.append(&name_label);

    if !app.description.is_empty() {
        let desc_label = gtk::Label::new(Some(&app.description));
        desc_label.set_halign(gtk::Align::Start);
        desc_label.add_css_class("app-description");
        desc_label.set_ellipsize(gtk::pango::EllipsizeMode::End);
        desc_label.set_max_width_chars(60);
        text_box.append(&desc_label);
    }

    hbox.append(&text_box);

    let row = gtk::ListBoxRow::new();
    row.set_child(Some(&hbox));
    row
}

/// Load custom CSS stylesheet.
pub fn load_css() {
    let provider = gtk::CssProvider::new();
    provider.load_from_data(include_str!("style.css"));
    gtk::style_context_add_provider_for_display(
        &gtk::gdk::Display::default().expect("Could not connect to a display"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
