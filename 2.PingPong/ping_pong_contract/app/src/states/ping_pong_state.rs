use sails_rs::prelude::*;

pub type UserData = (ActorId, PingEnum);

#[derive(Encode, Decode, TypeInfo)]
pub struct PingState {
    pub last_who_call: UserData,
    pub all_calls: Vec<UserData>
}

#[derive(Encode, Decode, TypeInfo, Clone)]
pub enum PingEnum {
    Ping,
    Pong
}