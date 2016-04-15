#![allow(dead_code)]
#[macro_use]
extern crate glium;
extern crate time;
extern crate image;
extern crate scoped_threadpool;
extern crate yaml_rust;

mod math;
mod input;
mod logic;
pub mod graphics;
mod components;
mod err;

pub use self::math::{Mat4, Vec2, Vec3, Vec4, DEG_TO_RAD};
pub use self::input::{Keyboard, Mouse, Display};
pub use self::logic::{TickCount, IdManager, Game, World, Id, IdType, Entity, OptErr};
pub use self::graphics::{WindowBuilder, Window, SyncData, Renderers, RendererType};
pub use self::components::{
    Transform,
    Renderable,
    RenderableTex2, RenderableSolidColor, RenderableVertexColor,
    Named,
    Map2d, Map2dCoords,
    Map3d, Map3dCoords,
    Scene
};
pub use self::err::DorpErr;
