use anyhow::Result;
use connectgrid::*;
use klystron::{
    runtime_2d::{event::WindowEvent, launch, App2D},
    DrawType, Engine, FramePacket, Matrix4, Object, Vertex, WinitBackend, UNLIT_FRAG, UNLIT_VERT,
};

pub type Mesh = (Vec<Vertex>, Vec<u16>);

fn main() -> Result<()> {
    let mesh = circuit_mesh(todo!(), todo!());
    launch::<MyApp>(mesh)
}

fn circuit_mesh(circuit: &Circuit, layout: &Layout) -> Mesh {
    todo!();
}

struct MyApp {
    object: Object,
}

impl App2D for MyApp {
    const TITLE: &'static str = "2D example app";
    type Args = Mesh;

    fn new(engine: &mut WinitBackend, (vertices, indices): Self::Args) -> Result<Self> {
        let material = engine.add_material(UNLIT_VERT, UNLIT_FRAG, DrawType::Lines)?;

        let mesh = engine.add_mesh(&vertices, &indices)?;

        let object = Object {
            mesh,
            transform: Matrix4::identity(),
            material,
        };

        Ok(Self { object })
    }

    fn event(&mut self, _event: &WindowEvent, _engine: &mut WinitBackend) -> Result<()> {
        Ok(())
    }

    fn frame(&self) -> FramePacket {
        FramePacket {
            objects: vec![self.object],
        }
    }
}
