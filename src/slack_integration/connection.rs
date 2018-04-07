use slack::{self, Event, RtmClient};
use slack_api::rtm::StartResponse;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub enum SlackMessage
{
    Start(StartResponse),
    Event(Event),
    End(String),
}

pub struct SlackConnection
{
    pub incoming: Sender<SlackMessage>
}

impl SlackConnection
{
    pub fn start_in_background(token: String, incoming: Sender<SlackMessage>) {
        thread::spawn(move|| {
            let mut handler = SlackConnection { incoming: incoming.clone() };
            while let Err(e) = RtmClient::login_and_run(&token, &mut handler) {
                let error_message = format!("Error connecting to slack. {}", e);
                SlackConnection::send_to_channel(&incoming, SlackMessage::End(error_message));
                thread::sleep(Duration::from_secs(10));
            }
        });
    }

    fn send_to_channel(channel: &Sender<SlackMessage>, message: SlackMessage) {
        if let Err(e) = channel.send(message) {
            println!("Could not send to UI channel. {}", e)
        }
    }
}

impl slack::EventHandler for SlackConnection {
    fn on_event(&mut self, _cli: &RtmClient, event: Event) {
        println!("on_event(event: {:?})", event);
        SlackConnection::send_to_channel(&self.incoming, SlackMessage::Event(event));
    }

    fn on_close(&mut self, _cli: &RtmClient) {
        println!("on_close");
        let end = "Connection closed".to_owned();
        SlackConnection::send_to_channel(&self.incoming, SlackMessage::End(end));
    }

    fn on_connect(&mut self, cli: &RtmClient) {
        println!("on_connect");
        let start = cli.start_response().clone();
        SlackConnection::send_to_channel(&self.incoming, SlackMessage::Start(start));
        // let _ = cli.sender().send_message(&general_channel_id, "Hello world! (rtm)");
    }
}
