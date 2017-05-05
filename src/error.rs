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

use std::fmt;
use std::error;
use std::ptr;
use packetgraph_sys::{pg_error, pg_error_is_set, pg_error_free};

#[derive(Debug)]
pub struct Error {
    pub ptr: *mut pg_error,
    comment: String,
}

impl Error {
    pub fn new() -> Error {
        Error {
            ptr: ptr::null_mut(),
            comment: String::new(),
        }
    }

    pub fn set<S: Into<String>>(&mut self, comment: S) {
        self.comment = comment.into();
    }

    pub fn is_set(&mut self) -> bool {
        unsafe {
            return pg_error_is_set(&mut self.ptr) || self.comment.len() > 0;
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Packetgraph error: [TODO]")
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "todo"
    }
}

impl Drop for Error {
    fn drop(&mut self) {
        unsafe {
            pg_error_free(self.ptr);
        }
    }
}
