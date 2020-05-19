#![allow(dead_code)] // FIXME remove
#![allow(unused_variables)] // FIXME remove
// FIXME: see what the clock members represents
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct Clock {
    pub m: u8,
    pub t: u8,
}

impl Clock {
    pub fn tick(&mut self, cycle: u8) {
        self.m = cycle;
        self.t = 4 * cycle;
    }
}

