//! TODO: doc
use actix::prelude::*;
use serde::Deserialize;

use crate::actors::App;
use crate::error;

/// TODO: doc
#[derive(Debug, Deserialize)]
pub struct GetTransactions(pub ());

impl GetTransactions {}

impl Message for GetTransactions {
    type Result = Result<(), error::Error>;
}

impl Handler<GetTransactions> for App {
    type Result = Result<(), error::Error>;

    fn handle(&mut self, _msg: GetTransactions, _ctx: &mut Self::Context) -> Self::Result {
        unimplemented!()
    }
}
