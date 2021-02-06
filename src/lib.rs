//! Core logic of the application
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
// TODO: Temporary, remove later
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(missing_docs)]
#![allow(dead_code)]

pub mod domain;
pub mod server;
pub mod setup;

#[macro_use]
extern crate derive_builder;

use futures::{
    FutureExt,
    StreamExt,
};
use log::{
    debug,
    error,
    info,
    trace,
    warn,
};
use stable_eyre::eyre::{
    eyre,
    Result,
    WrapErr,
};

use serde::{
    Deserialize,
    Serialize,
};
use std::collections::HashMap;
