// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate fnv;
extern crate habitat_builder_protocol as protocol;
extern crate hyper;
extern crate libc;
#[macro_use]
extern crate log;
extern crate protobuf;
extern crate rustc_serialize;
extern crate time;
extern crate zmq;

pub mod config;
pub mod error;
pub mod oauth;
pub mod routing;
pub mod server;

use std::process::Command;

pub use self::error::{Error, Result};
pub use self::server::{Application, ServerReg, Supervisor};

pub fn hostname() -> Result<String> {
    let output = try!(Command::new("sh")
        .arg("-c")
        .arg("hostname | awk '{printf \"%s\", $NF; exit}'")
        .output());
    match output.status.success() {
        true => {
            debug!("Hostname address is {}",
                   String::from_utf8_lossy(&output.stdout));
            let hostname = try!(String::from_utf8(output.stdout).or(Err(Error::Sys)));
            Ok(hostname)
        }
        false => {
            debug!("Hostname address command returned: OUT: {} ERR: {}",
                   String::from_utf8_lossy(&output.stdout),
                   String::from_utf8_lossy(&output.stderr));
            Err(Error::Sys)
        }
    }
}
