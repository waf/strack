#![feature(proc_macro)]

#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;
extern crate relm_attributes;
extern crate gtk;
extern crate gio;
extern crate gdk;
extern crate futures_glib;

use gtk::{
    CssProvider,
    CssProviderExt,
    GtkWindowExt,
    Inhibit,
    OrientableExt,
    StackExt,
    StackSidebarExt,
    TextViewExt,
    WidgetExt,

};
use gio::File;
use gdk::Screen;
use relm::Widget;
use relm_attributes::widget;

// model
pub struct Workspace
{
    pub channels: Vec<Channel>
}
pub struct Channel
{
    pub name: String
}

#[derive(Msg)]
pub enum Msg {
    Quit,
}

#[widget]
impl Widget for ChatView {
    fn model() -> () {
    }

    fn init_view(&mut self) {
    }

    fn update(&mut self, _event: ()) {
    }

    view! {
        gtk::Box {
            orientation: gtk::Orientation::Vertical,

            gtk::ScrolledWindow {
                gtk::TextView {
                    name: "chat_log",
                    hexpand: true,
                    vexpand: true,
                    editable: false,
                    cursor_visible: false,
                    wrap_mode: gtk::WrapMode::Word
                },
            },

            gtk::TextView {
                name: "chat_input",
            },
        }
    }
}

#[widget]
impl Widget for Win {
    fn model() -> Workspace {
        Workspace  {
            channels: Vec::new()
        }
    }

    fn init_view(&mut self) {
        // load style sheet
        let css = CssProvider::new();
        css.load_from_file(&File::new_for_commandline_arg("./style.css")).unwrap();
        gtk::StyleContext::add_provider_for_screen(&Screen::get_default().unwrap(), &css, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

        // add some fake slack channels
        let general = relm::init::<ChatView>(()).unwrap();
        self.stack.add_titled(general.widget(), "general", "# general");
        let random = relm::init::<ChatView>(()).unwrap();
        self.stack.add_titled(random.widget(), "random", "# random");

        self.sidebar.set_stack(&self.stack);
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Quit => gtk::main_quit()
        }
    }

    /*
    fn subscriptions(&mut self, relm: &Relm<Self>) {
        let stream = Interval::new(Duration::from_secs(1));
        relm.connect_exec_ignore_err(stream, Msg::Update);
    }
    */

    view! {
        #[name="window"]
        gtk::Window {
            title: "Strack",
            gtk::Box {
                orientation: gtk::Orientation::Horizontal,

                #[name="sidebar"]
                gtk::StackSidebar {
                    property_width_request: 170,
                    property_height_request: 600
                },

                #[name="stack"]
                gtk::Stack {
                    hexpand: true,
                    vexpand: true,
                    property_width_request: 800,
                    property_height_request: 600
                },
            },
            delete_event(_, _) => (Msg::Quit, Inhibit(false)),
        }
    }
}

fn main() {
    Win::run(()).unwrap();
}
