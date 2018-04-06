use slack::{self, Event, RtmClient};
use slack_api::rtm::StartResponse;
use std::sync::mpsc::Sender;

#[derive(Debug)]
pub enum SlackMessage
{
    Start(StartResponse),
    Event(Event)
}

pub struct SlackApi
{
    pub incoming: Sender<SlackMessage>
}

impl slack::EventHandler for SlackApi {
    fn on_event(&mut self, _cli: &RtmClient, event: Event) {
        println!("on_event(event: {:?})", event);
        self.incoming.send(SlackMessage::Event(event)).unwrap();
    }

    fn on_close(&mut self, _cli: &RtmClient) {
        println!("on_close");
    }

    fn on_connect(&mut self, cli: &RtmClient) {
        println!("on_connect");
        let start = cli.start_response().clone();
        self.incoming.send(SlackMessage::Start(start)).unwrap();
        // let _ = cli.sender().send_message(&general_channel_id, "Hello world! (rtm)");
    }
}
