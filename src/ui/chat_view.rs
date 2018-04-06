use relm_attributes::widget;
use relm::Widget;
use gtk::{
    self,
    OrientableExt,
    TextBufferExt,
    TextViewExt,
    WidgetExt,
};
use slack_api;

pub struct Channel
{
    pub name: String,
}

#[derive(Msg)]
pub enum ChatMsg {
    ChatUpdate(slack_api::MessageStandard)
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
            },
        }
    }
}

