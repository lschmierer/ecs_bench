#[macro_use]
extern crate serde_derive;
extern crate serde;

pub mod pos_vel {

    /// Entities with velocity and position component.
    pub const N_POS_VEL: usize = 1000;

    /// Entities with position component only.
    pub const N_POS: usize = 9000;

    #[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Position {
        pub x: f32,
        pub y: f32,
    }

    #[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Velocity {
        pub dx: f32,
        pub dy: f32,
    }

}

pub mod parallel {

    pub const N: usize = 10000;

    #[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct R {
        pub x: f32,
    }

    #[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct W1 {
        pub x: f32,
    }

    #[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct W2 {
        pub x: f32,
    }
}
