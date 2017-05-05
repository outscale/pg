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
use std::ptr;
use packetgraph_sys::{pg_brick, pg_brick_destroy, pg_tap_new};

pub struct Tap {
    pub brick: *mut pg_brick,
    pub name: String,
}

impl Tap {
    pub fn new<S: Into<String>>(name: S) -> Tap {
        let name = name.into();
        let cname = CString::new(name.as_str()).unwrap();
        let mut error = Error::new();
        unsafe {
            Tap {
                brick: pg_tap_new(cname.as_ptr(), ptr::null(), &mut error.ptr),
                name: name,
            }
        }
    }

    pub fn pollable(&self) -> bool {
        true
    }
}

impl Drop for Tap {
    fn drop(&mut self) {
        unsafe {
            pg_brick_destroy(self.brick);
        }
    }
}
