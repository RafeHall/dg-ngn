#![feature(generic_const_exprs)]

pub mod color;
pub mod interp;
pub mod line;
pub mod matrix;
pub mod plane;
pub mod rect;
pub mod rotor;
pub mod vector;
pub mod transform;
pub mod aabb;
pub mod frustrum;
pub mod projection;
pub mod algorithms;

#[cfg(not(feature = "high-precision"))]
pub type Scalar = f32;

#[cfg(feature = "high-precision")]
pub type Scalar = f64;

pub mod scalar {
    #[cfg(not(feature = "high-precision"))]
    pub mod consts {
        use crate::Scalar;

        /// Archimedes' constant (π) or 180 degrees
        pub const PI: Scalar = std::f32::consts::PI;

        /// The full circle constant (τ)
        ///
        /// Equal to 2π or 360 degrees
        pub const TAU: Scalar = std::f32::consts::TAU;

        /// π/2 or 90 degrees
        pub const FRAC_PI_2: Scalar = std::f32::consts::FRAC_PI_2;

        /// π/3 or 60 degrees
        pub const FRAC_PI_3: Scalar = std::f32::consts::FRAC_PI_3;

        /// π/4 or 45 degrees
        pub const FRAC_PI_4: Scalar = std::f32::consts::FRAC_PI_4;

        /// π/6 or 30 degrees
        pub const FRAC_PI_6: Scalar = std::f32::consts::FRAC_PI_6;

        /// π/8 22.5 degrees
        pub const FRAC_PI_8: Scalar = std::f32::consts::FRAC_PI_8;

        /// 1/π 
        pub const FRAC_1_PI: Scalar = std::f32::consts::FRAC_1_PI;

        /// 2/π
        pub const FRAC_2_PI: Scalar = std::f32::consts::FRAC_2_PI;

        /// 2/sqrt(π)
        pub const FRAC_2_SQRT_PI: Scalar = std::f32::consts::FRAC_2_SQRT_PI;

        /// sqrt(2)
        pub const SQRT_2: Scalar = std::f32::consts::SQRT_2;

        /// 1/sqrt(2)
        pub const FRAC_1_SQRT_2: Scalar = std::f32::consts::FRAC_1_SQRT_2;

        /// Euler's number (e)
        pub const E: Scalar = std::f32::consts::E;

        /// log<sub>2</sub>(e)
        pub const LOG2_E: Scalar = std::f32::consts::LOG2_E;

        /// log<sub>2</sub>(10)
        pub const LOG2_10: Scalar = std::f32::consts::LOG2_10;

        /// log<sub>10</sub>(e)
        pub const LOG10_E: Scalar = std::f32::consts::LOG10_E;

        /// log<sub>10</sub>(2)
        pub const LOG10_2: Scalar = std::f32::consts::LOG10_2;

        /// ln(2)
        pub const LN_2: Scalar = std::f32::consts::LN_2;

        /// ln(10)
        pub const LN_10: Scalar = std::f32::consts::LN_10;
    }

    #[cfg(feature = "high-precision")]
    pub mod consts {
        use crate::Scalar;

        /// Archimedes' constant (π)
        pub const PI: Scalar = std::f64::consts::PI;

        /// The full circle constant (τ)
        ///
        /// Equal to 2π or 180 degrees
        pub const TAU: Scalar = std::f64::consts::TAU;

        /// π/2 or 90 degrees
        pub const FRAC_PI_2: Scalar = std::f64::consts::FRAC_PI_2;

        /// π/3 or 60 degrees
        pub const FRAC_PI_3: Scalar = std::f64::consts::FRAC_PI_3;

        /// π/4 or 45 degrees
        pub const FRAC_PI_4: Scalar = std::f64::consts::FRAC_PI_4;

        /// π/6 or 30 degrees
        pub const FRAC_PI_6: Scalar = std::f64::consts::FRAC_PI_6;

        /// π/8 or 22.5 degrees
        pub const FRAC_PI_8: Scalar = std::f64::consts::FRAC_PI_8;

        /// 1/π
        pub const FRAC_1_PI: Scalar = std::f64::consts::FRAC_1_PI;

        /// 2/π
        pub const FRAC_2_PI: Scalar = std::f64::consts::FRAC_2_PI;

        /// 2/sqrt(π)
        pub const FRAC_2_SQRT_PI: Scalar = std::f64::consts::FRAC_2_SQRT_PI;

        /// sqrt(2)
        pub const SQRT_2: Scalar = std::f64::consts::SQRT_2;

        /// 1/sqrt(2)
        pub const FRAC_1_SQRT_2: Scalar = std::f64::consts::FRAC_1_SQRT_2;

        /// Euler's number (e)
        pub const E: Scalar = std::f64::consts::E;

        /// log<sub>2</sub>(10)
        pub const LOG2_10: Scalar = std::f64::consts::LOG2_10;

        /// log<sub>2</sub>(e)
        pub const LOG2_E: Scalar = std::f64::consts::LOG2_E;

        /// log<sub>10</sub>(2)
        pub const LOG10_2: Scalar = std::f64::consts::LOG10_2;

        /// log<sub>10</sub>(e)
        pub const LOG10_E: Scalar = std::f64::consts::LOG10_E;

        /// ln(2)
        pub const LN_2: Scalar = std::f64::consts::LN_2;

        /// ln(10)
        pub const LN_10: Scalar = std::f64::consts::LN_10;
    }
}

#[cfg(not(feature = "large-integers"))]
pub type Integer = i32;

#[cfg(feature = "large-integers")]
pub type Integer = i64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Side {
    Inside,
    Outside,
    On,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction2D {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction3D {
    Up,
    Down,
    North,
    East,
    South,
    West,
}