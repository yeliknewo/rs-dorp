use glium::backend::glutin_backend::{GlutinFacade, PollEventsIter};
use glium::glutin::WindowBuilder as GlutinWindowBuilder;
use glium::glutin::{get_primary_monitor};
use glium::{Surface, DisplayBuild};
use glium::Frame as GliumFrame;

use logic::{Entity};
use err::DorpErr;
use graphics::{Renderers, RendererType, SyncData};

pub struct Frame {
    frame: GliumFrame,
    renderers: Renderers,
}

impl Frame {

    fn new(facade: & mut GlutinFacade, renderers: Renderers) -> Frame {
        let mut frame  = facade.draw();
        frame.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
        Frame {
            frame: frame,
            renderers: renderers,
        }
    }


    pub fn draw_entity<T: Entity<T>>(&mut self, entity: &T, sync_data: &SyncData) -> Result<(), DorpErr> {
        match entity.get_renderable() {
            Some(renderable) => {
                match renderable.get_renderer_type() {
                    RendererType::SolidColor => match self.renderers.get_mut_solid_color().render(&mut self.frame, renderable, sync_data) {
                        Ok(()) => Ok(()),
                        Err(err) => Err(DorpErr::Dorp("Self RendererSolidColor Render", Box::new(err))),
                    },
                    RendererType::VertexColor => match self.renderers.get_mut_vertex_color().render(&mut self.frame, renderable, sync_data) {
                        Ok(()) => Ok(()),
                        Err(err) => Err(DorpErr::Dorp("Self RendererVertexColor Render", Box::new(err))),
                    },
                    RendererType::Texture2d => match self.renderers.get_mut_texture2d().render(&mut self.frame, renderable, sync_data) {
                        Ok(()) => Ok(()),
                        Err(err) => Err(DorpErr::Dorp("Self Renderer Texture2d Render", Box::new(err))),
                    },
                    RendererType::Empty => return Err(DorpErr::Base("RendererType Was Empty")),
                }
            },
            None => Ok(()),
        }
    }


    pub fn end(self) -> Result<Renderers, DorpErr> {
        match self.frame.finish() {
            Ok(()) => Ok(self.renderers),
            Err(err) => Err(DorpErr::GliumSwapBuffers("Self Frame Finish", err)),
        }
    }
}

pub struct Window {
    facade: GlutinFacade,
}

impl<'a> Window {
    pub fn frame(&mut self, renderers: Renderers) -> Frame {
        Frame::new(&mut self.facade, renderers)
    }

    pub fn poll_events(&self) -> PollEventsIter {
        self.facade.poll_events()
    }

    pub fn get_facade(&self) -> &GlutinFacade {
        &self.facade
    }
}

pub struct WindowBuilder {
    windowed: Windowed,
    dimensions: (u32, u32),
    title: String,
}

impl WindowBuilder {
    pub fn new() -> WindowBuilder {
        WindowBuilder {
            windowed: Windowed::Windowed,
            dimensions: (640, 480),
            title: "Untitled".to_string(),
        }
    }

    pub fn with_windowed(mut self) -> WindowBuilder {
        self.windowed = Windowed::Windowed;
        self
    }

    pub fn with_borderless(mut self) -> WindowBuilder {
        self.windowed = Windowed::Borderless;
        self
    }

    pub fn with_dimensions(mut self, dimensions: (u32, u32)) -> WindowBuilder {
        self.dimensions = dimensions;
        self
    }

    pub fn with_title(mut self, title: String) -> WindowBuilder {
        self.title = title;
        self
    }

    pub fn build(self) -> Result<(Window, (u32, u32)), DorpErr> {
        let resolution: (u32, u32) = get_primary_monitor().get_dimensions();
        Ok(
            (
                Window {
                    facade: match self.windowed {
                        Windowed::Windowed => {
                            let facade = match GlutinWindowBuilder::new()
                                .with_title(self.title)
                                .with_dimensions(self.dimensions.0, self.dimensions.1)
                                .with_decorations(true)
                                .with_depth_buffer(24)
                                .with_vsync()
                                .build_glium() {
                                    Ok(facade) => facade,
                                    Err(err) => return Err(DorpErr::GliumCreation("GlutinWindowBuilder Build Glium", err)),
                                };
                            match facade.get_window() {
                                Some(window) => window,
                                None => return Err(DorpErr::Base("Facade Get Window was none")),
                            }.set_position(((resolution.0 - self.dimensions.0) / 2) as i32, ((resolution.1 - self.dimensions.1) / 2) as i32);
                            facade
                        },
                        // Windowed::Fullscreen => {
                        //
                        // },
                        Windowed::Borderless => {
                            let facade = match GlutinWindowBuilder::new()
                                .with_title(self.title)
                                .with_dimensions(resolution.0, resolution.1)
                                .with_decorations(false)
                                .with_depth_buffer(24)
                                .with_vsync()
                                .build_glium() {
                                    Ok(facade) => facade,
                                    Err(err) => return Err(DorpErr::GliumCreation("GlutinWindowBuilder Build Glium", err)),
                                };
                            match facade.get_window() {
                                Some(window) => window,
                                None => return Err(DorpErr::Base("Facade Get Window was none")),
                            }.set_position(0, 0);
                            facade
                        },
                    },

                },
                self.dimensions
            )
        )
    }
}

enum Windowed {
    Windowed,
    //Fullscreen,
    Borderless,
}
