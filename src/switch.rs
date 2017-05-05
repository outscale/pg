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

use super::Side;
use error::Error;
use std::ffi::CString;
use packetgraph_sys::{pg_brick, pg_brick_destroy, pg_switch_new};

pub struct Switch {
    pub brick: *mut pg_brick,
    pub name: String,
}

impl Switch {
    pub fn new<S: Into<String>>(name: S, west_max: u32, east_max: u32, output: Side) -> Switch {
        let name = name.into();
        let cname = CString::new(name.as_str()).unwrap();
        let mut error = Error::new();
        let b = unsafe {
            Switch {
                brick: pg_switch_new(cname.as_ptr(),
                                     west_max,
                                     east_max,
                                     output.into(),
                                     &mut error.ptr),
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

impl Drop for Switch {
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
    use super::super::Side;

    #[test]
    fn plug() {
        init();
        let mut sw = Brick::Switch(Switch::new("sw", 2, 2, Side::West));
        let mut nop1 = Brick::Nop(Nop::new("nop1"));
        let mut nop2 = Brick::Nop(Nop::new("nop2"));
        let mut nop3 = Brick::Nop(Nop::new("nop3"));
        let mut nop4 = Brick::Nop(Nop::new("nop4"));
        let mut nop5 = Brick::Nop(Nop::new("nop5"));
        sw.link(&mut nop1).unwrap();
        sw.link(&mut nop2).unwrap();
        assert!(sw.link(&mut nop3).is_err());
        nop3.link(&mut sw).unwrap();
        nop4.link(&mut sw).unwrap();
        assert!(nop5.link(&mut sw).is_err());
    }
}
