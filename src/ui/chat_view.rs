use relm_attributes::widget;
use relm::Widget;
use gdk::{
    self,
    EventKey
};
use gtk::{
    self,
    OrientableExt,
    TextBufferExt,
    TextViewExt,
    WidgetExt,
    Inhibit,
};
use slack_api;

pub struct Channel
{
    pub name: String
}

#[derive(Msg)]
pub enum ChatMsg {
    ChatUpdate(slack_api::MessageStandard),
    UserTyped(String)
}

impl ChatView {
    fn fire_text_event(text_view: &gtk::TextView, _key: &gdk::EventKey) -> ChatMsg {
        let buffer = text_view.get_buffer().unwrap();
        let (start, end) = buffer.get_bounds();
        let text = buffer.get_text(&start, &end, true).unwrap();
        ChatMsg::UserTyped(text)
    }
}

#[widget]
impl Widget for ChatView {
    fn model(channel: Channel) -> Channel {
        channel
    }

    fn init_view(&mut self) {
    }

    fn update(&mut self, event: ChatMsg) {
        match event {
            ChatMsg::ChatUpdate(message) => {
                self.chat_log.get_buffer().unwrap().set_text(&message.text.unwrap());
            },
            ChatMsg::UserTyped(message) => {
                println!("{}", message);
            }
        }
    }

    view! {
        gtk::Box {
            orientation: gtk::Orientation::Vertical,

            gtk::ScrolledWindow {
                #[name="chat_log"]
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
                key_press_event(text_view, key) => (ChatView::fire_text_event(text_view, key), Inhibit(false))
            },
        }
    }
}

