pub trait ColorScalar {
    const WHITE: Self;
    const BLACK: Self;
}

impl ColorScalar for f32 {
    const WHITE: f32 = 1.0f32;
    const BLACK: f32 = 0.0f32;
}

impl ColorScalar for u8 {
    const WHITE: u8 = 255;
    const BLACK: u8 = 0;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color3T<T> {
    pub r: T,
    pub g: T,
    pub b: T
}

impl<T: ColorScalar> Color3T<T> {
    ///
    /// Creates a new color with the given components
    ///
    #[inline] pub fn from_rgb(r: T, g: T, b: T) -> Self {
        Self { r, g, b }
    }

    ///
    /// Constant colors
    ///
    #[inline] pub fn white() -> Self {
        Self::from_rgb(T::WHITE, T::WHITE, T::WHITE)
    }
    #[inline] pub fn black() -> Self {
        Self::from_rgb(T::BLACK, T::BLACK, T::BLACK)
    }
    #[inline] pub fn red() -> Self {
        Self::from_rgb(T::WHITE, T::BLACK, T::BLACK)
    }
    #[inline] pub fn green() -> Self {
        Self::from_rgb(T::BLACK, T::WHITE, T::BLACK)
    }
    #[inline] pub fn blue() -> Self {
        Self::from_rgb(T::BLACK, T::BLACK, T::WHITE)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color4T<T> {
    pub r: T,
    pub g: T,
    pub b: T,
    pub a: T
}

impl<T: ColorScalar + Copy> Color4T<T> {
    ///
    /// Creates a new color with the given components
    ///
    #[inline] pub fn from_rgba(r: T, g: T, b: T, a: T) -> Self {
        Self { r, g, b, a }
    }

    ///
    /// Creates a new color with the given components
    ///
    #[inline] pub fn from_color3_alpha(rgb: Color3T<T>, a: T) -> Self {
        Self {
            r: rgb.r,
            g: rgb.g,
            b: rgb.g,
            a
        }
    }

    ///
    /// Creates a new color with the given components
    ///
    #[inline] pub fn from_rgb(r: T, g: T, b: T) -> Self {
        Self { r, g, b, a: T::WHITE }
    }

    ///
    /// Creates a new color with the given components
    ///
    #[inline] pub fn from_color3(rgb: Color3T<T>) -> Self {
        Self::from_color3_alpha(rgb, T::WHITE)
    }

    ///
    /// Constant colors
    ///
    #[inline] pub fn white() -> Self {
        Self::from_rgb(T::WHITE, T::WHITE, T::WHITE)
    }
    #[inline] pub fn black() -> Self {
        Self::from_rgb(T::BLACK, T::BLACK, T::BLACK)
    }
    #[inline] pub fn transparent_black() -> Self {
        Self::from_rgba(T::BLACK, T::BLACK, T::BLACK, T::BLACK)
    }
    #[inline] pub fn red() -> Self {
        Self::from_rgb(T::WHITE, T::BLACK, T::BLACK)
    }
    #[inline] pub fn green() -> Self {
        Self::from_rgb(T::BLACK, T::WHITE, T::BLACK)
    }
    #[inline] pub fn blue() -> Self {
        Self::from_rgb(T::BLACK, T::BLACK, T::WHITE)
    }
}

pub type Color3 = Color3T<u8>;
pub type Color4 = Color4T<u8>;

pub type Color3F = Color3T<f32>;
pub type Color4F = Color4T<f32>;