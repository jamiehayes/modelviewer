mod app;
mod gfx;
mod components;
mod numerics;

use crate::gfx::*;
use crate::numerics::*;

///
/// Data used by this sample
///
struct ModelViewer {
    rt_state: RenderTargetState,
    model: Model,
    cbuff: ShaderBuffer,
    sampler: Sampler,
    time: f32
}

#[derive(Copy, Clone)]
struct BuffData {
    world: Matrix4F,
    view_proj: Matrix4F,
    camera_pos: Vector3F,
    specular_amount : f32,
    material_color: Color4F,
    ambient_color: Color4F,
    specular_color: Color3F,
    specular_power: f32,
    sun_color: Color4F,
    sun_dir: Vector3F
}

impl ModelViewer {
    ///
    /// Creates and initializes a new ModelViewer object
    ///
    pub fn new(app: &mut app::Application) -> Result<ModelViewer, ()> {
        let (width, height) = app.window.get_window_size();

        // initialize render target state
        let rt = app.graphics.create_render_target(&app.display)?;
        let ds = app.graphics.create_depth_stencil_target(DepthStencilFormat::D24UNormS8,
            width, height)?;
        let mut rt_state = RenderTargetState::new(rt, Some(ds));
        rt_state.enable_clear_color(Color4F::black());
        rt_state.enable_clear_depth(1.0f32);

        let objpath = std::path::Path::new("data\\objects\\test2.obj");
        let model = ModelBuilder::load_from_obj(objpath)?.build(&app.graphics)?;

        let cbuff = app.graphics.create_constant_buffer(std::mem::size_of::<BuffData>())?;

        let samp_data = SamplerData {
            mode: SampleMode::Linear,
            address_u: AddressMode::Wrap,
            address_v: AddressMode::Wrap,
            address_w: AddressMode::Wrap,
            comparison: ComparisonFunc::Never,
            lod_bias: 0.0f32,
            max_anisotropy: 1,
            border_color: BorderColor::OpaqueWhite,
            min_lod: -std::f32::MAX,
            max_lod: std::f32::MAX
        };
        let sampler = app.graphics.create_sampler(samp_data)?;

        Ok(Self { rt_state, model, cbuff, sampler, time: 0.0f32 })
    }
}

impl app::AppInterface for ModelViewer {
    ///
    /// Does nothing. Just needed to implement AppInterface
    ///
    fn update(&mut self, _app: &mut app::Application) {
        self.time += 1.0f32 / 60.0f32; // yeah, gross and hard-coded
    }

    ///
    /// Clears the render target for the frame
    ///
    fn render(&mut self, app: &mut app::Application) {
        // set up model and camera matrices
        let rot_speed = deg_to_rad(45.0f32);
        let world = Matrix4F::from_axis_angle(&Vector3F::y_axis(), rot_speed * self.time);
        let cam_trans = Vector3F::new(1.0f32, 1.5f32, 5.0f32);
        let cam_dir = -cam_trans.normalize();
        let cam_rot = QuaternionF::rotation_between(&-Vector3::z_axis(), &cam_dir).unwrap();
        let cam_xform = TransformF::from_parts(Translation3F::from(cam_trans), cam_rot, 1.0f32);
        let view = Matrix4F::try_inverse(Matrix4F::from(cam_xform)).unwrap();
        let proj = Matrix4F::new_perspective(16.0f32 / 9.0f32, deg_to_rad(45.0f32), 0.1f32, 5000.0f32);
        let view_proj = proj * view;
        let material_color = Color4F::from_rgba(0.3f32, 0.3f32, 0.3f32, 1.0f32);
        let ambient_color = Color4F::from_rgba(0.0f32, 0.1f32, 0.2f32, 1.0f32);
        let specular_color = Color3F::from_rgb(0.4f32, 0.4f32, 0.4f32);
        let specular_power = 1.0f32;
        let specular_amount = 0.5f32;
        let sun_dir = Vector3F::new(1.0f32, 1.0f32, 1.0f32);
        let sun_color = Color4F::from_rgba(1.0f32, 1.0f32, 0.75f32, 1.0f32);
        let buffdata = BuffData {
            world: world,
            view_proj,
            camera_pos: cam_trans,
            specular_amount,
            material_color,
            ambient_color,
            specular_color,
            specular_power,
            sun_color,
            sun_dir
        };

        app.graphics.map_and_set_buffer_data(&self.cbuff, &buffdata);
        app.graphics.set_vs_constant_buffer(0, &self.cbuff);
        app.graphics.set_ps_constant_buffer(0, &self.cbuff);
        app.graphics.set_ps_sampler(0, &self.sampler);

        self.rt_state.begin(&app.graphics);
        self.model.draw(&app.graphics);
        self.rt_state.end(&app.graphics);
    }
}

///
/// Program entry point for ModelViewer
///
fn main() -> Result<(), i32> {
    let init_err = |_| {
        println!("Failed to initialize application");
        1
    };
    let mut app = app::Application::create("Model Viewer", 1280, 720).map_err(init_err)?;
    let mut sample = ModelViewer::new(&mut app).map_err(init_err)?;
    app.run(&mut sample)
}
