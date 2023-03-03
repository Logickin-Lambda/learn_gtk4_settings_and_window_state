use gio::Settings;
use gtk::gio::SettingsBindFlags;
use gtk::glib::signal::Inhibit;
use gtk::{prelude::*, Window};
use gtk::{gio, glib, Align, Application, ApplicationWindow, Switch};

const APP_ID: &str = "learn_gtk4_settings";

fn main() -> glib::ExitCode{
    
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui (app: &Application){

    // initialize settings from gschema
    let settings = Settings::new(APP_ID);

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

    let window = Window::builder()
        .application(app)
        .title("Button with State")
        .child(&switch)
        .build();

    window.present();
}