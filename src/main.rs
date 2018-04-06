#![feature(proc_macro)]

#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;
extern crate relm_attributes;
extern crate gtk;
extern crate gio;
extern crate gdk;
extern crate slack;  // real-time messaging (rtm) client
extern crate slack_api; // web api and models
extern crate config;
extern crate failure;

pub mod slack_integration;
pub mod ui;

use std::thread;
use std::sync::mpsc;
use std::collections::HashMap;
use relm::Widget;
use slack::{RtmClient};
use slack_integration::connection::SlackApi;
use ui::main_window::MainWindow;
use failure::Error;

fn main() {
    let settings = read_settings("Setting.toml")
        .expect("Could not find Settings.toml configuration file");

    let (sender, receiver) = mpsc::channel();

    // set up a real-time connection to slack in a background thread.
    // it will use a channel to communicate with the UI.
    thread::spawn(move|| {
        let mut handler = SlackApi {
            incoming: sender
        };
        let token = &settings["token"];
        RtmClient::login_and_run(token, &mut handler).unwrap();
    });

    // start main window
    MainWindow::run(receiver).unwrap();
}

fn read_settings(filename: &str) -> Result<HashMap<String, String>, Error> {
    let mut settings_file = config::Config::default();
    settings_file.merge(config::File::with_name(filename))?;
    let settings = settings_file.try_into::<HashMap<String, String>>()?;
    Ok(settings)
}
