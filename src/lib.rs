extern crate rustc_serialize;

pub mod pos_vel {

    /// Entities with velocity and position component.
    pub const N_POS_VEL: usize = 1000;

    /// Entities with position component only.
    pub const N_POS: usize = 9000;

    #[derive(Copy, Clone, Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct Position {
        pub x: f32,
        pub y: f32,
    }

    #[derive(Copy, Clone, Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct Velocity {
        pub dx: f32,
        pub dy: f32,
    }

}

pub mod parallel {

    pub const N: usize = 10000;

    #[derive(Copy, Clone, Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct R {
        pub x: f32,
    }

    #[derive(Copy, Clone, Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct W1 {
        pub x: f32,
    }

    #[derive(Copy, Clone, Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct W2 {
        pub x: f32,
    }
}
