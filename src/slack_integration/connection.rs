use slack::{self, Event, RtmClient};
use slack_api::rtm::StartResponse;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;
use std::time::Duration;
use std::sync::mpsc;

#[derive(Debug)]
pub enum SlackMessage
{
    Start(StartResponse),
    Event(Event),
    End(String),
}

// passed around by client code to receive and send messages from/to slack
pub struct SlackConnection
{
    pub incoming_receiver: Receiver<SlackMessage>,
    pub outgoing_sender: Sender<SlackMessage>,
}

// used in the background thread to send messages to the SlackConnection
struct SlackHandler
{
    incoming_sender: Sender<SlackMessage>,
    outgoing_receiver: Receiver<SlackMessage>,
}

impl SlackConnection
{
    pub fn start_in_background(token: String) -> SlackConnection {

        let (incoming_sender, incoming_receiver) = mpsc::channel();
        let (outgoing_sender, outgoing_receiver) = mpsc::channel();

        let connection = SlackConnection {
            incoming_receiver: incoming_receiver,
            outgoing_sender: outgoing_sender,
        };

        thread::spawn(move|| {
            let mut handler = SlackHandler {
                incoming_sender: incoming_sender.clone(),
                outgoing_receiver: outgoing_receiver,
            };
            while let Err(e) = RtmClient::login_and_run(&token, &mut handler) {
                let error_message = format!("Error connecting to slack. {}", e);
                SlackConnection::send_to_channel(&incoming_sender, SlackMessage::End(error_message));
                thread::sleep(Duration::from_secs(10));
            }
        });

        connection
    }

    fn send_to_channel(channel: &Sender<SlackMessage>, message: SlackMessage) {
        if let Err(e) = channel.send(message) {
            println!("Could not send to UI channel. {}", e)
        }
    }
}

impl slack::EventHandler for SlackHandler {
    fn on_event(&mut self, _cli: &RtmClient, event: Event) {
        println!("on_event(event: {:?})", event);
        SlackConnection::send_to_channel(&self.incoming_sender, SlackMessage::Event(event));
    }

    fn on_close(&mut self, _cli: &RtmClient) {
        println!("on_close");
        let end = "Connection closed".to_owned();
        SlackConnection::send_to_channel(&self.incoming_sender, SlackMessage::End(end));
    }

    fn on_connect(&mut self, cli: &RtmClient) {
        println!("on_connect");
        let start = cli.start_response().clone();
        SlackConnection::send_to_channel(&self.incoming_sender, SlackMessage::Start(start));
        // let _ = cli.sender().send_message(&general_channel_id, "Hello world! (rtm)");
    }
}
