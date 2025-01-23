use sails_rs::{
    prelude::*,
    gstd::service,
    cell::Ref
};

use crate::states::ping_pong_state::{
    PingState,
    UserData
};

pub struct QueryService<'a> {
    ping_state: Ref<'a, PingState>
}

#[service]
impl<'a> QueryService<'a> {
    pub fn new(
        ping_state: Ref<'a, PingState>
    ) -> Self {
        Self {
            ping_state
        }
    }
    
    pub fn last_who_call(&self) -> UserData {
        self
            .ping_state
            .last_who_call
            .to_owned()
    }

    pub fn all_calls(&self) -> Vec<UserData> {
        self
            .ping_state
            .all_calls
            .to_owned()
    }
}