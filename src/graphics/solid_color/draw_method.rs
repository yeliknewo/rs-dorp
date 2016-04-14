use glium::{DrawParameters};
use glium;

#[derive(Clone)]
pub enum DrawMethod {
    Both(DepthTestMethod, CullingMethod),
    Depth(DepthTestMethod),
    Culling(CullingMethod),
    Neither,
}

#[derive(Clone)]
pub enum DepthTestMethod {
    IfLess,
}

#[derive(Clone)]
pub enum CullingMethod {
    Clockwise,
    CounterClockwise,
}


pub fn method_to_parameters(method: DrawMethod) -> DrawParameters<'static> {
    match method {
        DrawMethod::Both(depth, cull) => {
            let depth_glium = match depth {
                DepthTestMethod::IfLess => glium::draw_parameters::DepthTest::IfLess,
            };
            let cull_glium = match cull {
                CullingMethod::Clockwise => glium::draw_parameters::BackfaceCullingMode::CullClockwise,
                CullingMethod::CounterClockwise => glium::draw_parameters::BackfaceCullingMode::CullCounterClockwise,
            };
            glium::DrawParameters {
                depth: glium::Depth {
                    test: depth_glium,
                    write: true,
                    .. Default::default()
                },
                backface_culling: cull_glium,
                .. Default::default()
            }
        },
        DrawMethod::Depth(depth) => {
            let depth_glium = match depth {
                DepthTestMethod::IfLess => glium::draw_parameters::DepthTest::IfLess,
            };
            glium::DrawParameters {
                depth: glium::Depth {
                    test: depth_glium,
                    write: true,
                    .. Default::default()
                },
                .. Default::default()
            }
        },
        DrawMethod::Culling(cull) => {
            let cull_glium = match cull {
                CullingMethod::Clockwise => glium::draw_parameters::BackfaceCullingMode::CullClockwise,
                CullingMethod::CounterClockwise => glium::draw_parameters::BackfaceCullingMode::CullCounterClockwise,
            };
            glium::DrawParameters {
                backface_culling: cull_glium,
                .. Default::default()
            }
        },
        DrawMethod::Neither => {
            glium::DrawParameters {
                .. Default::default()
            }
        },
    }
}
