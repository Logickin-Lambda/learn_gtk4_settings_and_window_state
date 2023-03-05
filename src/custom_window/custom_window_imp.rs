use gio::Settings;
use glib::signal::Inhibit;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, ApplicationWindow};
use glib::once_cell::sync::OnceCell;

#[derive(Default)]
pub struct Window{
    pub settings: OnceCell<Settings>,
}

#[glib::object_subclass]
impl ObjectSubclass for Window{
    const NAME: &'static str = "WindowWithMemorizableSize";
    type Type = super::Window;

    // this might cause error due to lack of implementation, so we must implement ApplicationWindowImpl for Window
    type ParentType = ApplicationWindow; 
}

impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();
        // Load latest window state
        let obj = self.obj();
        obj.setup_settings();
        obj.load_window_size();
    }
}

impl WindowImpl for Window {
    // Save window state right before the window will be closed
    fn close_request(&self) -> Inhibit {
        // Save window size
        self.obj()
            .save_window_size()
            .expect("Failed to save window state");

        // Don't inhibit the default handler
        Inhibit(false)
    }
}

impl WidgetImpl for Window {}

impl ApplicationWindowImpl for Window{}