mod color;

pub use self::color::*;

// use na to generate our typical numeric types
use nalgebra as na;
pub use self::na::*;

pub type Vector2F = na::Vector2<f32>;
pub type Vector3F = na::Vector3<f32>;
pub type Point2F = na::Point3<f32>;
pub type Point3F = na::Point3<f32>;
pub type Translation2F = na::Translation3<f32>;
pub type Translation3F = na::Translation3<f32>;
pub type Matrix3F = na::Matrix3<f32>;
pub type Matrix4F = na::Matrix4<f32>;
pub type QuaternionF = na::UnitQuaternion<f32>;
pub type TransformF = na::Similarity3<f32>;
pub type Perspective3F = na::Perspective3<f32>;
pub type Orthographic3F = na::Orthographic3<f32>;

// Constants
pub const DEG_TO_RAD: f32 = std::f32::consts::PI / 180.0f32;
pub const RAD_TO_DEG: f32 = 180.0f32 / std::f32::consts::PI;

// Helper funcs
pub fn deg_to_rad(f: f32) -> f32 { f * DEG_TO_RAD }
pub fn rad_to_deg(f: f32) -> f32 { f * RAD_TO_DEG }