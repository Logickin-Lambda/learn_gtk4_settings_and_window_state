mod custom_window;

use gio::Settings;
use gtk::gio::SettingsBindFlags;
use gtk::{prelude::*};
use gtk::{gio, glib, Align, Application, Switch};
use custom_window::Window;

const APP_ID_SETTINGS: &str = "learn_gtk4.settings_basics";

fn main() -> glib::ExitCode{
    
    let app_settings = Application::builder().application_id(APP_ID_SETTINGS).build();

    app_settings.connect_activate(build_ui_settings);

    app_settings.run()
}

fn build_ui_settings (app: &Application){

    // initialize settings from gschema
    let settings = Settings::new(APP_ID_SETTINGS);

    // get the switch state
    let is_switch_enabled = settings.boolean("is-switch-enabled");

    // create switch
    let switch = Switch::builder()
        .margin_top(48)
        .margin_bottom(48)
        .margin_start(48)
        .margin_end(48)
        .valign(Align::Center)
        .halign(Align::Center)
        .state(is_switch_enabled)
        .build();

    // saving the switch state on click
    // switch.connect_state_set( move |_, is_enabled|{

    //     // Save the settings back to the gschema
    //     settings
    //         .set_boolean("is-switch-enabled", is_enabled)
    //         .expect("Error found in setting status of the switch");
    //     // Don't inhibit the default handler
    //     Inhibit(false)  // <- Not sure what it really does, but if that is true, the visual style retains but the state.
    // });

    // or bind the switch state with the button; they both behave the same
    settings
        .bind("is-switch-enabled", &switch, "state")
        .flags(SettingsBindFlags::DEFAULT)
        .build();

    let window = Window::new(app);
    window.set_title(Some("Button with State"));
    window.set_child(Some(&switch));
    window.present();

}