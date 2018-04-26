#![feature(proc_macro)]

#[macro_use] extern crate relm;
#[macro_use] extern crate relm_derive;
extern crate relm_attributes;
extern crate gtk;
extern crate gio;
extern crate gdk;
extern crate slack;  // real-time messaging (rtm) client
extern crate slack_api; // web api and request / response models
extern crate config;
extern crate failure;

pub mod slack_integration;
pub mod ui;

use std::collections::HashMap;
use failure::Error;
use relm::Widget;
use slack_integration::connection::SlackConnection;
use ui::main_window::MainWindow;

fn main() {
    let settings = read_settings("Settings.toml")
        .expect("Could not find Settings.toml configuration file.");

    // set up a real-time connection to slack in a background thread.
    // it will use a channel to communicate with the UI.
    let token = settings["token"].to_owned();
    let connection = SlackConnection::start_in_background(token);

    // start main window
    MainWindow::run(connection).unwrap();
}

fn read_settings(filename: &str) -> Result<HashMap<String, String>, Error> {
    let mut settings_file = config::Config::default();
    settings_file.merge(config::File::with_name(filename))?;
    let settings = settings_file.try_into::<HashMap<String, String>>()?;
    Ok(settings)
}
