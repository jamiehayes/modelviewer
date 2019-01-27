use crate::gfx::{Graphics, Display};
use crate::app::Window;
use crate::numerics::*;

///
/// Interface to be implemented by the user to extend application functionality
///
pub trait AppInterface {
    fn update(&mut self, app: &mut Application);
    fn render(&mut self, app: &mut Application);
}

///
/// Basic application data
///
pub struct Application {
    pub window: Window,
    pub graphics: Graphics,
    pub display: Display,

    display_size: Vector2F
}

impl Application {
    ///
    /// Attempts to create and initialize the application
    ///
    pub fn create(title: &str, width: u32, height: u32) -> Result<Application, ()> {

        if let Ok(graphics) = Graphics::new() {
            let window = Window::create(title, width, height);
            if let Ok(display) = graphics.create_display(window.get_handle()) {
                return Ok(Application {
                    window,
                    graphics,
                    display,
                    display_size: Vector2F::new(width as f32, height as f32)
                });
            }
        }

        Err(())
    }

    ///
    /// Displays the window and runs the application until the user exits or the application is otherwise told
    ///
    pub fn run(&mut self, interface: &mut impl AppInterface) -> Result<(), i32> {
        self.window.show(true);

        loop {
            if !self.window.process_messages() {
                break;
            }

            self.update(interface);
            self.render(interface);

            self.display.present();
        }

        Ok(())
    }

    ///
    /// Updates the application for the frame
    ///
    fn update(&mut self, interface: &mut impl AppInterface) {
        interface.update(self);
    }

    ///
    /// Performs rendering operations for the application for the frame
    ///
    fn render(&mut self, interface: &mut impl AppInterface) {
        // initialize graphics pipeline state for the frame
        self.graphics.set_viewport(0.0f32, 0.0f32, self.display_size.x, self.display_size.y, 0.0f32, 1.0f32);

        interface.render(self);
    }
}