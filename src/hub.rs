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

use error::Error;
use std::ffi::CString;
use packetgraph_sys::{pg_brick, pg_brick_destroy, pg_hub_new};

pub struct Hub {
    pub brick: *mut pg_brick,
    pub name: String,
}

impl Hub {
    pub fn new<S: Into<String>>(name: S, west_max: u32, east_max: u32) -> Hub {
        let name = name.into();
        let cname = CString::new(name.as_str()).unwrap();
        let mut error = Error::new();
        let b = unsafe {
            Hub {
                brick: pg_hub_new(cname.as_ptr(), west_max, east_max, &mut error.ptr),
                name: name,
            }
        };
        assert!(!error.is_set());
        return b;
    }

    pub fn pollable(&self) -> bool {
        false
    }
}

impl Drop for Hub {
    fn drop(&mut self) {
        unsafe {
            pg_brick_destroy(self.brick);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::brick::Brick;
    use super::super::nop::Nop;
    use super::super::init;

    #[test]
    fn plug() {
        init();
        let mut hub = Brick::Hub(Hub::new("hub", 2, 2));
        let mut nop1 = Brick::Nop(Nop::new("nop1"));
        let mut nop2 = Brick::Nop(Nop::new("nop2"));
        let mut nop3 = Brick::Nop(Nop::new("nop3"));
        let mut nop4 = Brick::Nop(Nop::new("nop4"));
        let mut nop5 = Brick::Nop(Nop::new("nop5"));
        hub.link(&mut nop1).unwrap();
        hub.link(&mut nop2).unwrap();
        assert!(hub.link(&mut nop3).is_err());
        nop3.link(&mut hub).unwrap();
        nop4.link(&mut hub).unwrap();
        assert!(nop5.link(&mut hub).is_err());
    }
}
