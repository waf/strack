use std::collections::HashMap;
use std::sync::mpsc::{Receiver, TryRecvError};
use relm_attributes::widget;
use relm::{self, Relm, Widget, interval};
use gio::File;
use gdk::Screen;
use gtk::{
    self,
    CssProvider,
    CssProviderExt,
    GtkWindowExt,
    Inhibit,
    OrientableExt,
    StackExt,
    StackSidebarExt,
    WidgetExt,
};
use ui::chat_view::{ChatView, ChatMsg, Channel};
use slack;
use slack_integration::connection::{SlackMessage};

pub struct Workspace
{
    pub receiver: Receiver<SlackMessage>,
    pub channels: HashMap<String, relm::Component<ChatView>>,
}

#[derive(Msg)]
pub enum Msg {
    Quit,
    TryUpdate,
}

#[widget]
impl Widget for MainWindow {
    fn model(receiver: Receiver<SlackMessage>) -> Workspace {
        Workspace  {
            receiver: receiver,
            channels: HashMap::new()
        }
    }

    fn init_view(&mut self) {
        // load style sheet
        let css = CssProvider::new();
        css.load_from_file(&File::new_for_commandline_arg("./style.css")).unwrap();
        gtk::StyleContext::add_provider_for_screen(&Screen::get_default().unwrap(), &css, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

        self.sidebar.set_stack(&self.stack);
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Quit => gtk::main_quit(),
            Msg::TryUpdate => {
                match self.model.receiver.try_recv() {
                    Ok(msg) => self.handle_slack_message(msg),
                    Err(_disconnect @ TryRecvError::Empty) => { /* no updates */ },
                    Err(_err @ TryRecvError::Disconnected) => println!("Disconnected :("),
                }
            },
        }
    }

    fn subscriptions(&mut self, relm: &Relm<Self>) {
        interval(relm.stream(), 1000, || Msg::TryUpdate);
    }

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


impl MainWindow {
    fn new_channel(&mut self, id: &str, name: &str) -> () {
        let chat = relm::init::<ChatView>(Channel {
            name: name.to_string()
        }).unwrap();

        let display_name = format!("# {}", name);
        self.stack.add_titled(chat.widget(), id, &display_name);
        self.model.channels.insert(id.to_string(), chat);
    }

    fn handle_slack_message(&mut self, msg: SlackMessage) {
        match msg {
            SlackMessage::Start(start) => {
                println!("{:#?}", start);
                for channel in start.channels.unwrap() {
                    let id = channel.id.unwrap().to_string();
                    let name = channel.name.unwrap().to_string();
                    self.new_channel(&id, &name);
                }
            },
            SlackMessage::Event(event) => {
                println!("{:#?}", event);
                match event {
                    slack::Event::Message(contents) => match *contents {
                        slack::Message::Standard(standard) => {
                            let channel = &standard.channel.clone().unwrap();
                            self.model.channels[channel].emit(ChatMsg::ChatUpdate(standard))
                        },
                        _ => {}
                    },
                    _ => {}
                }
            },
            SlackMessage::End(msg) => println!("{}", msg)
        }
    }
}
