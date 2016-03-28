const N: usize = 10_000;

/// Entities with velocity and position component.
pub const N_VEL: usize = N / 10;

/// Entities with position component only.
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
