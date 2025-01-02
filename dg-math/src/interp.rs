use crate::scalar;

use super::Scalar;

pub type Ease = fn(Scalar) -> Scalar;

#[deprecated = "ease_linear is pointless but exists for the sake of completeness"]
pub fn ease_linear(t: Scalar) -> Scalar {
    t
}

pub fn ease_in_sine(t: Scalar) -> Scalar {
    1.0 - (t * scalar::consts::PI / 2.0).cos()
}

pub fn ease_out_sine(t: Scalar) -> Scalar {
    (t * scalar::consts::PI as Scalar / 2.0).sin()
}

pub fn ease_in_out_sine(t: Scalar) -> Scalar {
    -((t * scalar::consts::PI as Scalar).cos() - 1.0) / 2.0
}

pub fn ease_in_quad(t: Scalar) -> Scalar {
    t * t
}

pub fn ease_out_quad(t: Scalar) -> Scalar {
    1.0 - (1.0 - t) * (1.0 - t)
}

pub fn ease_in_out_quad(t: Scalar) -> Scalar {
    if t < 0.5 {
        2.0 * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
    }
}

pub fn ease_in_cubic(t: Scalar) -> Scalar {
    t * t * t
}

pub fn ease_out_cubic(t: Scalar) -> Scalar {
    1.0 - (1.0 - t).powi(3)
}

pub fn ease_in_out_cubic(t: Scalar) -> Scalar {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
    }
}

pub fn ease_in_quart(t: Scalar) -> Scalar {
    t * t * t * t
}

pub fn ease_out_quart(t: Scalar) -> Scalar {
    1.0 - (1.0 - t).powi(4)
}

pub fn ease_in_out_quart(t: Scalar) -> Scalar {
    if t < 0.5 {
        8.0 * t * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(4) / 2.0
    }
}

pub fn ease_in_quint(t: Scalar) -> Scalar {
    t * t * t * t * t
}

pub fn ease_out_quint(t: Scalar) -> Scalar {
    1.0 - (1.0 - t).powi(5)
}

pub fn ease_in_out_quint(t: Scalar) -> Scalar {
    if t < 0.5 {
        16.0 * t * t * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(5) / 2.0
    }
}

pub fn ease_in_expo(t: Scalar) -> Scalar {
    if t == 0.0 {
        0.0
    } else {
        (2.0 as Scalar).powf(10.0 * t - 10.0)
    }
}

pub fn ease_out_expo(t: Scalar) -> Scalar {
    if t == 1.0 {
        1.0
    } else {
        1.0 - (2.0 as Scalar).powf(-10.0 * t)
    }
}

pub fn ease_in_out_expo(t: Scalar) -> Scalar {
    if t == 0.0 {
        0.0
    } else if t == 1.0 {
        1.0
    } else if t < 0.5 {
        (2.0 as Scalar).powf(20.0 * t - 10.0) / 2.0
    } else {
        (2.0 - (2.0 as Scalar).powf(-20.0 * t + 10.0)) / 2.0
    }
}

pub fn ease_in_circ(t: Scalar) -> Scalar {
    1.0 - (1.0 - t.powi(2)).sqrt()
}

pub fn ease_out_circ(t: Scalar) -> Scalar {
    (1.0 - (t - 1.0).powi(2)).sqrt()
}

pub fn ease_in_out_circ(t: Scalar) -> Scalar {
    if t < 0.5 {
        (1.0 - (1.0 - (2.0 * t).powi(2)).sqrt()) / 2.0
    } else {
        ((1.0 - (-2.0 * t + 2.0).powi(2)).sqrt() + 1.0) / 2.0
    }
}

pub fn ease_in_back(t: Scalar) -> Scalar {
    const A: Scalar = 1.70158;
    const B: Scalar = 2.70158;

    B * t * t * t - A * t * t
}

pub fn ease_out_back(t: Scalar) -> Scalar {
    const A: Scalar = 1.70158;
    const B: Scalar = 2.70158;

    1.0 + B * (t - 1.0).powi(3) + A * (t - 1.0).powi(2)
}

pub fn ease_in_out_back(t: Scalar) -> Scalar {
    // const A: Scalar = 1.70158;
    const A: Scalar = 2.5949095; // A * 1.525

    if t < 0.5 {
        ((2.0 * t).powi(2) * ((A + 1.0) * 2.0 * t - A)) / 2.0
    } else {
        ((2.0 * t - 2.0).powi(2) * ((A + 1.0) * (t * 2.0 - 2.0) + A) + 2.0) / 2.0
    }
}

pub fn ease_in_elastic(t: Scalar) -> Scalar {
    const A: Scalar = 2.0943951; // 2.0 * PI / 3.0

    if t == 0.0 {
        0.0
    } else if t == 1.0 {
        1.0
    } else {
        -(2.0 as Scalar).powf(10.0 * t - 10.0) * ((t * 10.0 - 10.75) * A).sin()
    }
}

pub fn ease_out_elastic(t: Scalar) -> Scalar {
    const A: Scalar = 2.0943951; // 2.0 * PI / 3.0

    if t == 0.0 {
        0.0
    } else if t == 1.0 {
        1.0
    } else {
        (2.0 as Scalar).powf(-10.0 * t) * ((t * 10.0 - 0.75) * A).sin() + 1.0
    }
}

pub fn ease_in_out_elastic(t: Scalar) -> Scalar {
    const A: Scalar = 1.3962634; // 2.0 * PI / 4.5

    if t == 0.0 {
        0.0
    } else if t == 1.0 {
        1.0
    } else if t < 0.5 {
        -((2.0 as Scalar).powf(20.0 * t - 10.0) * ((20.0 * t - 11.125) * A).sin()) / 2.0
    } else {
        ((2.0 as Scalar).powf(-20.0 * t + 10.0) * ((20.0 * t - 11.125) * A).sin()) / 2.0 + 1.0
    }
}

pub fn ease_in_bounce(t: Scalar) -> Scalar {
    1.0 - ease_out_bounce(1.0 - t)
}

pub fn ease_out_bounce(t: Scalar) -> Scalar {
    const A: Scalar = 7.5625;
    const B: Scalar = 2.75;

    if t < 1.0 / B {
        A * t * t
    } else if t < 2.0 / B {
        let t = t - 1.5 / B;
        A * t * t + 0.75
    } else if t < 2.5 / B {
        let t = t - 2.25 / B;
        A * t * t + 0.9375
    } else {
        let t = t - 2.625 / B;
        A * t * t + 0.984375
    }
}

pub fn ease_in_out_bounce(t: Scalar) -> Scalar {
    if t < 0.5 {
        (1.0 - ease_out_bounce(1.0 - 2.0 * t)) / 2.0
    } else {
        (1.0 + ease_out_bounce(2.0 * t - 1.0)) / 2.0
    }
}

pub trait SphericalInterp: Sized {
    fn slerp(a: Self, b: Self, t: Scalar) -> Self;

    fn slerp_to(self, other: Self, t: Scalar) -> Self {
        Self::slerp(self, other, t)
    }

    fn slerp_eased(a: Self, b: Self, t: Scalar, ease: Ease) -> Self {
        Self::slerp(a, b, ease(t))
    }

    fn slerp_to_eased(self, other: Self, t: Scalar, ease: Ease) -> Self {
        Self::slerp_eased(self, other, t, ease)
    }
}

pub trait LinearInterp: Sized {
    fn lerp(a: Self, b: Self, t: Scalar) -> Self;

    fn lerp_to(self, other: Self, t: Scalar) -> Self {
        Self::lerp(self, other, t)
    }

    fn lerp_eased(a: Self, b: Self, t: Scalar, ease: Ease) -> Self {
        Self::lerp(a, b, ease(t))
    }

    fn lerp_to_eased(self, other: Self, t: Scalar, ease: Ease) -> Self {
        Self::lerp_eased(self, other, t, ease)
    }
}

impl<T: LinearInterp + Clone> LinearInterp for Vec<T> {
    fn lerp(a: Self, b: Self, t: Scalar) -> Self {
        a.iter()
            .zip(b)
            .map(|(a, b)| a.clone().lerp_to(b, t))
            .collect()
    }
}

macro_rules! impl_interp_float {
    ($t:ty) => {
        impl LinearInterp for $t {
            fn lerp(a: Self, b: Self, t: Scalar) -> Self {
                a + (b - a) * t as Self
            }
        }
    };
    ( $($t:ty),* ) => {
        $(
            impl_interp_float!($t);
        )*
    };
}

impl_interp_float!(f32, f64/*, f128*/);

macro_rules! impl_interp_int {
    ($t:ty) => {
        impl LinearInterp for $t {
            fn lerp(a: Self, b: Self, t: Scalar) -> Self {
                (a as Scalar + (b as Scalar - a as Scalar) * t) as $t
            }
        }
    };
    ( $($t:ty),* ) => {
        $(
            impl_interp_int!($t);
        )*
    };
}

impl_interp_int!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);
