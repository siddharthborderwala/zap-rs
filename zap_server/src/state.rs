use actix::prelude::*;
use zap_core::Zap;

#[derive(Message)]
#[rtype(result = "String")]
pub struct Get(pub String);

#[derive(Message)]
#[rtype(result = "()")]
pub struct Set(pub String, pub String);

#[derive(Message)]
#[rtype(result = "()")]
pub struct Delete(pub String);

#[derive(Message)]
#[rtype(result = "bool")]
pub struct Has(pub String);

#[derive(Message)]
#[rtype(result = "String")]
pub struct List;

pub struct ZapActor {
    state: Zap,
}

impl ZapActor {
    pub fn new() -> Self {
        Self { state: Zap::new() }
    }
}

impl Actor for ZapActor {
    type Context = Context<Self>;
}

impl Handler<Get> for ZapActor {
    type Result = String;

    fn handle(&mut self, msg: Get, _ctx: &mut Context<Self>) -> Self::Result {
        self.state.get(msg.0.as_str()).unwrap_or("nil".to_string())
    }
}

impl Handler<Set> for ZapActor {
    type Result = ();

    fn handle(&mut self, msg: Set, _ctx: &mut Context<Self>) -> Self::Result {
        self.state.set(msg.0, msg.1);
    }
}

impl Handler<Delete> for ZapActor {
    type Result = ();

    fn handle(&mut self, msg: Delete, _ctx: &mut Context<Self>) -> Self::Result {
        self.state.delete(msg.0);
    }
}

impl Handler<Has> for ZapActor {
    type Result = bool;

    fn handle(&mut self, msg: Has, _ctx: &mut Context<Self>) -> Self::Result {
        self.state.has(msg.0)
    }
}

impl Handler<List> for ZapActor {
    type Result = String;

    fn handle(&mut self, _: List, _ctx: &mut Context<Self>) -> Self::Result {
        self.state
            .list()
            .map(|(k, v)| format!("'{}': '{}'\r\n", k, v))
            .collect()
    }
}
