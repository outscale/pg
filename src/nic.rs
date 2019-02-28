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
use packetgraph_sys::{pg_brick, pg_brick_destroy, pg_nic_new, pg_nic_new_by_id};

pub struct Nic {
    pub brick: *mut pg_brick,
    pub name: String,
}

impl Nic {
    pub fn new<S: Into<String>>(name: S, vdev: S) -> Result<Nic, Error> {
        let name = name.into();
        let vdev = vdev.into();
        let cname = CString::new(name.as_str()).unwrap();
        let cvdev = CString::new(vdev.as_str()).unwrap();
        let mut error = Error::new();
        let b = unsafe {
            Nic {
                brick: pg_nic_new(cname.as_ptr(), cvdev.as_ptr(), &mut error.ptr),
                name: name,
            }
        };
        match error.is_set() {
            true => Err(error),
            false => Ok(b),
        }
    }

    pub fn new_port<S: Into<String>>(name: S, port: u8) -> Result<Nic, Error> {
        let name = name.into();
        let cname = CString::new(name.as_str()).unwrap();
        let mut error = Error::new();
        let b = unsafe {
            Nic {
                brick: pg_nic_new_by_id(cname.as_ptr(), port as u16, &mut error.ptr),
                name: name,
            }
        };
        match error.is_set() {
            true => Err(error),
            false => Ok(b),
        }
    }

    pub fn pollable(&self) -> bool {
        true
    }
}

impl Drop for Nic {
    fn drop(&mut self) {
        unsafe {
            pg_brick_destroy(self.brick);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::init;
    use super::super::brick::Brick;

    #[test]
    fn rings() {
        init();
        let nic1 = Nic::new("nic", "eth_ring0").unwrap();
        let nic2 = Nic::new("nic", "eth_ring1").unwrap();
        let mut b1 = Brick::Nic(nic1);
        let mut b2 = Brick::Nic(nic2);
        b1.link(&mut b2).unwrap();
        b1.poll().unwrap();
        b2.poll().unwrap();
    }
}
