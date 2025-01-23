use sails_rs::{
    prelude::*,
    gstd::{
        service,
        msg
    },
    cell::RefMut
};

use crate::states::ping_pong_state::{
    PingState,
    PingEnum
};

pub struct PingService<'a> {
    pub state: RefMut<'a, PingState>
}

#[service]
impl<'a> PingService<'a> {
    pub fn new(
        state: RefMut<'a, PingState>
    ) -> Self {
        Self {
            state
        }
    }

    pub fn ping(&mut self) -> PingEnum {
        let caller: ActorId = msg::source();

        self
            .state
            .last_who_call = (caller.clone(), PingEnum::Ping);

        self
            .state
            .all_calls
            .push((caller, PingEnum::Ping));

            PingEnum::Pong
    }

    pub fn pong(&mut self) -> PingEnum {
        let caller: ActorId = msg::source();

        self
            .state
            .last_who_call = (caller.clone(), PingEnum::Pong);

        self
            .state
            .all_calls
            .push((caller, PingEnum::Pong));

            PingEnum::Ping
    }
}
