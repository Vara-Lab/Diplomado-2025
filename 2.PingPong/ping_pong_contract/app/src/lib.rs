#![no_std]
use sails_rs::{
    prelude::*,
    gstd::{
        program,
        route,
        msg
    },
    cell::RefCell
};

pub mod service;
pub mod states;

use service::{
    ping_pong_service::PingService,
    query_service::QueryService
};
use states::ping_pong_state::{
    PingState,
    PingEnum
};

pub struct PingProgram {
    pub ping_service: RefCell<PingState>
}


#[program]  
impl PingProgram {
    pub fn new() -> Self {
        let ping_service = RefCell::new(PingState {
            last_who_call: (msg::source(), PingEnum::Ping),
            all_calls: Vec::new()
        });

        Self {
            ping_service
        }
    }

    #[route("Ping")]
    pub fn ping_svc(&self) -> PingService<'_> {
        PingService::new(
            self.ping_service.borrow_mut()
        )
    }

    #[route("Query")]
    pub fn query_svc(&self) -> QueryService<'_> {
        QueryService::new(
            self.ping_service.borrow()
        )
    }
}