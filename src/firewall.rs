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
use packetgraph_sys::{pg_brick, pg_brick_destroy, pg_firewall_new, pg_firewall_rule_add,
                      pg_firewall_rule_flush, pg_firewall_reload, PG_NONE};
use std::sync::Mutex;

lazy_static! {
    // TODO: get ride of this by making pg_firewall interface thread safe
    static ref NPF_USE: Mutex<bool> = Mutex::new(true);
}

pub struct Firewall {
    pub brick: *mut pg_brick,
    pub name: String,
}

impl Firewall {
    pub fn new<S: Into<String>>(name: S) -> Firewall {
        let mut m = NPF_USE.lock().unwrap();
        let name = name.into();
        let cname = CString::new(name.as_str()).unwrap();
        let mut error = Error::new();
        *m = true;
        unsafe {
            Firewall {
                brick: pg_firewall_new(cname.as_ptr(), PG_NONE as u64, &mut error.ptr),
                name: name,
            }
        }
    }

    pub fn pollable(&self) -> bool {
        false
    }

    pub fn rule_add<S: Into<String>>(&mut self, rule: S, side: Side) -> Result<(), Error> {
        let mut m = NPF_USE.lock().unwrap();
        let mut error = Error::new();
        let filter = CString::new(rule.into().as_str()).unwrap();
        unsafe {
            pg_firewall_rule_add(self.brick, filter.as_ptr(), side.into(), 1, &mut error.ptr);
        }

        *m = true;
        match error.is_set() {
            true => Err(error),
            false => Ok(()),
        }
    }

    pub fn flush(&mut self) {
        let mut m = NPF_USE.lock().unwrap();
        *m = true;
        unsafe {
            pg_firewall_rule_flush(self.brick);
        }
    }

    pub fn reload(&mut self) -> Result<(), Error> {
        let mut m = NPF_USE.lock().unwrap();
        let mut error = Error::new();
        unsafe {
            pg_firewall_reload(self.brick, &mut error.ptr);
        }

        *m = true;
        match error.is_set() {
            true => Err(error),
            false => Ok(()),
        }
    }
}

impl Drop for Firewall {
    fn drop(&mut self) {
        let mut m = NPF_USE.lock().unwrap();
        unsafe {
            pg_brick_destroy(self.brick);
        }
        *m = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::init;
    use super::super::Side;

    #[test]
    fn add_flush_reload() {
        init();
        let mut fw = Firewall::new("fw");
        fw.rule_add("src host 10::1", Side::West).unwrap();
        fw.rule_add("src host 10::1", Side::West).unwrap();
        fw.rule_add("src host 10::2", Side::East).unwrap();
        assert!(fw.rule_add("invalid rule", Side::West).is_err());
        fw.flush();
        fw.flush();
        fw.rule_add("src host 10::1", Side::West).unwrap();
        fw.rule_add("src host 10::2", Side::East).unwrap();
        fw.reload().unwrap();
        fw.reload().unwrap();
        fw.flush();
        fw.reload().unwrap();
    }
}
