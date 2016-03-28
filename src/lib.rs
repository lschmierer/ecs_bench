use std::thread;
use std::time::Duration;


const N: usize = 10_000;
pub const N_VEL: usize = N / 10;
pub const N_POS: usize = N - N_VEL;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Velocity {
    pub dx: f32,
    pub dy: f32,
}
