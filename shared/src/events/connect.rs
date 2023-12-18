use bevy::prelude::*;
use serde::{Serialize, Deserialize};

use super::NamedEvent;

#[derive(Event, Serialize, Deserialize, Debug, Clone)]
pub struct Req {
    pub name: Option<String>,
}

#[derive(Event, Serialize, Deserialize, Debug, Clone)]
pub struct Res {
    pub your_name: String,
    pub client_id: u64,
}

//impl super::C2S for ConnectEventClient {}

pub struct ClientConnect;

impl super::NEC2S for ClientConnect {
    type ClientSend = Req;
    type ServerResponse = Res;
}
