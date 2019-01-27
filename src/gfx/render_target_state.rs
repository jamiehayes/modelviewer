// external refs

// local refs
use crate::gfx::*;
use crate::numerics::*;

///
/// Handles binding render targets to the graphics pipeline
///
pub struct RenderTargetState {
    rt: RenderTarget,
    ds: Option<DepthStencilTarget>,
    clear_color: ColorF,
    clear_depth: f32,
    flags: u32
}

impl RenderTargetState {
    pub const CLEAR_COLOR: u32  = 1 << 0;
    pub const CLEAR_Z: u32  = 1 << 0;

    ///
    /// Creates a new render target state with default settings
    ///
    pub fn new(rt: RenderTarget, ds: Option<DepthStencilTarget>) -> Self {
        Self {
            rt,
            ds,
            clear_color: ColorF::transparent_black(),
            clear_depth: 0.0f32,
            flags: 0
        }
    }

    ///
    /// Sets the color to clear the render target to
    ///
    pub fn enable_clear_color(&mut self, color: ColorF) {
        self.flags |= Self::CLEAR_COLOR;
        self.clear_color = color;
    }

    ///
    /// Disables clearing of the render target state
    ///
    pub fn disable_clear_color(&mut self) {
        self.flags &= !Self::CLEAR_COLOR;
    }

    ///
    /// Enables clearing of the depth buffer
    ///
    pub fn enable_clear_depth(&mut self, depth: f32) {
        self.flags |= Self::CLEAR_Z;
        self.clear_depth = depth;
    }

    ///
    /// Disables clearing of the depth buffer
    ///
    pub fn disable_clear_depth(&mut self) {
        self.flags &= !Self::CLEAR_Z;
    }

    ///
    /// Begins rendering to the render target state
    ///
    pub fn begin(&self, gfx: &Graphics) {
        if (self.flags & Self::CLEAR_COLOR) != 0  {
            gfx.clear_render_target(&self.rt, &self.clear_color);
        }
        if let Some(ds) = &self.ds {
            if (self.flags & Self::CLEAR_Z) != 0 {
                gfx.clear_depth_stencil_target(&ds, true, self.clear_depth, false, 0);
            }
            gfx.set_render_target_and_depth(&self.rt, &ds);
        }
        else {
            gfx.set_render_target(&self.rt);
        }
    }

    ///
    /// Finishes rendering to the render target state
    ///
    pub fn end(&self, gfx: &Graphics) {
        gfx.unbind_render_targets();
    }
}