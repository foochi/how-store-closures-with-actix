use actix::*;
use actix_web::ws::{Client, Message, ProtocolError};
use futures::Future;

struct MyActor {
    handler: Box<Fn(String) -> () + 'static>,
}

impl Actor for MyActor {
    type Context = Context<Self>;
}

impl StreamHandler<Message, ProtocolError> for MyActor {
    fn handle(&mut self, msg: Message, _ctx: &mut Context<Self>) {
        match msg {
            Message::Text(text) => {
                (self.handler)(text)
            },
            _ => panic!(),
        }
    }
}

pub struct Event {
    handler: Box<Fn(String) -> () + 'static>,
}

pub struct EventManager {
    events: Vec<Event>,
}

impl EventManager {

    pub fn new() -> Self {
        Self { events: vec![] }
    }

    pub fn capture<F>(&mut self, function: F)
    where
        F: for<'h> Fn(String) -> () + 'static
    {
        let event = Event { handler: Box::new(function), };
        self.events.push(event);
    }

    pub fn run(&self) {
        let runner = System::new("example");
        let event = &self.events[0];

        Arbiter::spawn(
            Client::new("example")
                .connect()
                .map(|(reader, _writer)| {
                    MyActor::create(|ctx| {
                        MyActor::add_stream(reader, ctx);
                        MyActor { handler: event.handler }
                    });
                })
                .map_err(|err| {})
        );

        runner.run();
    }
}
