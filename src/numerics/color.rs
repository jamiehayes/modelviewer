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
pub struct ColorT<T> {
    pub r: T,
    pub g: T,
    pub b: T,
    pub a: T
}

impl<T: ColorScalar> ColorT<T> {
    ///
    /// Creates a new color with the given components
    ///
    #[inline] pub fn from_rgba(r: T, g: T, b: T, a: T) -> Self {
        Self { r, g, b, a }
    }

    ///
    /// Creates a new color with the given components
    ///
    #[inline] pub fn from_rgb(r: T, g: T, b: T) -> Self {
        Self { r, g, b, a: T::WHITE }
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

pub type Color = ColorT<u8>;
pub type ColorF = ColorT<f32>;