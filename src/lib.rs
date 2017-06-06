/* Copyright 2017 Outscale SAS
 *
 * This file is part of Pg, a Rust Wrapper for packetgraph C library.
 *
 * Pg is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 3 as published
 * by the Free Software Foundation.
 *
 * Packetgraph is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Packetgraph.  If not, see <http://www.gnu.org/licenses/>.
 */

extern crate packetgraph_sys;
#[macro_use]
extern crate lazy_static;
extern crate libc;

pub mod error;
pub mod brick;
pub mod graph;
pub mod nop;
pub mod firewall;
pub mod tap;
pub mod switch;
pub mod nic;
pub mod hub;
pub mod vhost;

pub use error::Error;
pub use brick::Brick;
pub use graph::Graph;
pub use nop::Nop;
pub use firewall::Firewall;
pub use tap::Tap;
pub use switch::Switch;
pub use nic::Nic;
pub use hub::Hub;
pub use vhost::Vhost;

use std::ffi::CString;
use std::sync::Mutex;
use packetgraph_sys::{pg_start_str, pg_side};

lazy_static! {
    static ref DPDK_OPTS: Mutex<String> = Mutex::new(
        String::from("-c1 -n1 --no-huge --no-shconf --lcores 0,1 -l 0,1"));
    static ref DPDK_OK: Mutex<bool> = Mutex::new(false);
}

pub fn set_dpdk_params<S: Into<String>>(params: S) {
    let mut s = DPDK_OPTS.lock().unwrap();
    *s = params.into();
}

pub fn init() {
    let mut ok = DPDK_OK.lock().unwrap();
    if !*ok {
        let dpdk_opt = DPDK_OPTS.lock().unwrap();
        let params = CString::new(dpdk_opt.as_str()).unwrap();
        if unsafe { pg_start_str(params.as_ptr()) } < 0 {
            panic!("Cannot init packetgraph with dpdk parameters {}, adjust with set_dpdk_params()",
                   params.into_string().unwrap());
        } else {
            *ok = true;
        }
    }
}

pub enum Side {
    West,
    East,
}

impl Into<pg_side> for Side {
    fn into(self) -> pg_side {
        match self {
            Side::West => pg_side::PG_WEST_SIDE,
            Side::East => pg_side::PG_EAST_SIDE,
        }
    }
}
