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

        // we need to implement some of the interfaces, to ensure the "object" to have certain behavior:
        // TODO: study the following structs and see the purpose of them:
        // The purpose might be wrong, but they will be corrected once I have more understanding to the interfaces

        // Action Group    : Grouping actions for an object, supporting adding, changing, removing, changing state of actions using signal.
        //                   It generally used for showing list of action for user using menu, suggested by the document.
        // ActionMap       : Providing naming mapping so that makes sure that the action group name are unique.
        // Accessible      : Describes UI elements with the concept of "role" and "attribute"
        // Buildable       : Used for extending the obejct by editing the setting names and properties while in the deserialized form.
        // ConstraintTarget: Used for targeting constraint? Documentations don't seem clear about this interface.
        // Native          : Providing a surface (e.g. a window) for the ui widgets
        // Root            : Makes the widget as a top level widget so that can be used for managing the layout of the application, and
        //                 : it can also used for setting the keyboard focus.
        // ShortcutManager : Used for define behavior when there is a control short-cut input 

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