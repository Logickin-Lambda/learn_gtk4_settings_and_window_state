mod custom_window_imp;

use gtk::{glib, gio, Application, subclass::prelude::ObjectSubclassIsExt, traits::GtkWindowExt, prelude::*};
use glib::{*};
use gio::*;

use crate::APP_ID_SETTINGS;

glib::wrapper! {
    pub struct Window(ObjectSubclass<custom_window_imp::Window>)
        // since this "object" is of course a widget and it is related to window setting,
        // so we need to "inherit" those modules
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,

        // we need to implement some of the traites, to ensure the "object" to have certain behavior:
        // TODO: study the following structs and see the purpose of them:
        // Action Group    : 
        // ActionMap       : 
        // Accessible      : 
        // Buildable       : 
        // ConstraintTarget: 
        // Native          : 
        // Root            : 
        // ShortcutManager : 

        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &Application) -> Self {
        // Construct a new window
        Object::builder().property("application", app).build()
    }

    // Used for load the settings in the gschema
    fn setup_settings(&self){
        let settings = Settings::new(APP_ID_SETTINGS);
        self.imp()
            .settings
            .set(settings)
            .expect("`settings` should not be set before calling `setup_settings`.");
    }

    // obtain the settings for changing the properties
    fn settings(&self) -> &Settings {
        self.imp()
            .settings
            .get()
            .expect("`settings` should be set in `setup_settings`.")
    }

    pub fn save_window_size(&self) -> Result<(), glib::BoolError> {
        // get the size of the window
        let (width, height) = self.default_size();

        // Set the window state in the "settings"
        self.settings().set_int("window-width", width)?;
        self.settings().set_int("window-height", height)?;
        self.settings().set_boolean("is-maximized", self.is_maximized())?;

        Ok(())
    }

    fn load_window_size(&self) {
        // Get the window state from "Settings"
        let width = self.settings().int("window-width");
        let height = self.settings().int("window-height");
        let is_maximized = self.settings().boolean("is-maximized");

        // Set the size of the window
        self.set_default_size(width, height);

        // Set the window to maximize if it is not
        if is_maximized{
            self.maximize();
        }
    }
}