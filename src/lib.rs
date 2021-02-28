//! Core logic of the application
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::default_trait_access)]
#![allow(clippy::missing_errors_doc)]
// TODO: Temporary, remove later
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(missing_docs)]
#![allow(clippy::too_many_lines)]

pub mod domain;
pub mod server;
pub mod setup;

use futures::{FutureExt, StreamExt};
use log::{debug, error, info, trace, warn};
use stable_eyre::eyre::{eyre, Result, WrapErr};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
