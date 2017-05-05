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
use packetgraph_sys::{pg_brick, pg_brick_destroy, pg_vhost_new, pg_vhost_start,
                      PG_VHOST_USER_CLIENT, PG_VHOST_USER_NO_RECONNECT,
                      PG_VHOST_USER_DEQUEUE_ZERO_COPY};

use std::sync::Mutex;

lazy_static! {
    static ref VHOST_INITIALIZED: Mutex<bool> = Mutex::new(false);
}

pub const VHOST_USER_CLIENT: u64 = PG_VHOST_USER_CLIENT as u64;
pub const VHOST_USER_NO_RECONNECT: u64 = PG_VHOST_USER_NO_RECONNECT as u64;
pub const VHOST_USER_DEQUEUE_ZERO_COPY: u64 = PG_VHOST_USER_DEQUEUE_ZERO_COPY as u64;

pub struct Vhost {
    pub brick: *mut pg_brick,
    pub name: String,
}

impl Vhost {
    // flags:
    // - VHOST_USER_CLIENT
    // - VHOST_USER_NO_RECONNECT
    // - VHOST_USER_DEQUEUE_ZERO_COPY
    pub fn new<S: Into<String>>(name: S, flags: u64) -> Result<Vhost, Error> {
        {
            let mut vhi = VHOST_INITIALIZED.lock().unwrap();
            let mut error = Error::new();
            if !*vhi {
                let ret = unsafe {
                    pg_vhost_start(CString::new("/tmp").unwrap().as_ptr(), &mut error.ptr)
                };
                if ret < 0 {
                    assert!(error.is_set());
                    return Err(error);
                } else {
                    *vhi = true;
                }
            }
        }
        let mut error = Error::new();
        let name = name.into();
        let cname = CString::new(name.as_str()).unwrap();
        let b = unsafe {
            Vhost {
                brick: pg_vhost_new(cname.as_ptr(), flags as u64, &mut error.ptr),
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

impl Drop for Vhost {
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
        let vhost1 = Vhost::new("vhost", VHOST_USER_NO_RECONNECT).unwrap();
        let vhost2 = Vhost::new("vhost", VHOST_USER_NO_RECONNECT).unwrap();
        let mut b1 = Brick::Vhost(vhost1);
        let mut b2 = Brick::Vhost(vhost2);
        b1.link(&mut b2).unwrap();
        b1.poll().unwrap();
        b2.poll().unwrap();
    }
}
