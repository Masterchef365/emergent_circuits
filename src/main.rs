use anyhow::Result;
use connectgrid::*;
use klystron::{
    runtime_2d::{event::WindowEvent, launch, App2D},
    DrawType, Engine, FramePacket, Matrix4, Object, Vertex, WinitBackend, UNLIT_FRAG, UNLIT_VERT,
};
use nalgebra::{Vector3, Vector4};

pub type Mesh = (Vec<Vertex>, Vec<u16>);
pub type Drawing = (Mesh, Matrix4<f32>);
pub type Animation = Vec<Drawing>;

fn main() -> Result<()> {
    let components = vec![
        chip((7, 2), true, false),
        chip((3, 18), false, true),
        chip((3, 3), true, true),
        chip((3, 3), true, true),
    ];
    /*let connections = vec![
        ((0, 0), (1, 0)),
        ((0, 1), (1, 1)),
        ((0, 2), (1, 2)),
        ((0, 3), (1, 3)),
    ];*/
    let connections = dense(&components);
    let board_size = (30, 30);
    let circuit = (components, connections, board_size);
    let (placements, routes) = layout(&circuit).expect("Circuit layout problem");

    let n_steps = 100;
    let mut animation = Vec::with_capacity(n_steps);
    for _ in 0..n_steps {
        let layout = (placements.clone(), routes.clone());
        let drawing = circuit_drawing(&circuit, &layout);
        animation.push(drawing);
    }

    launch::<MyApp>(animation)
}

fn dense(components: &[Component]) -> Vec<Connection> {
    let mut connections = Vec::new();

    let destinations = |(comp_idx, (terminals, _)): (usize, &Component)| {
        (0..terminals.len()).map(move |term_idx| (comp_idx, term_idx))
    };

    let mut cmp_iter = components.iter().enumerate().map(destinations);

    let mut a = match cmp_iter.next() {
        Some(i) => i,
        None => return connections,
    };

    let mut b = match cmp_iter.next() {
        Some(i) => i,
        None => return connections,
    };

    loop {
        let src = match a.next() {
            Some(term) => term,
            None => match cmp_iter.next() {
                Some(i) => {
                    a = i;
                    match a.next() {
                        Some(term) => term,
                        None => break,
                    }
                }
                None => break,
            },
        };

        let dst = match b.next() {
            Some(term) => term,
            None => match cmp_iter.next() {
                Some(i) => {
                    b = i;
                    match b.next() {
                        Some(term) => term,
                        None => break,
                    }
                }
                None => break,
            },
        };

        connections.push((src, dst))
    }

    connections
}

fn chip(size: Size, vertical_terms: bool, horizontal_terms: bool) -> Component {
    let (width, height) = size;
    assert!(width > 0);
    assert!(height > 0);
    assert!(vertical_terms || horizontal_terms);

    let mut terminals = Vec::new();

    if vertical_terms {
        for x in 0..width {
            terminals.push((x, -1));
            terminals.push((x, height));
        }
    }

    if horizontal_terms {
        for y in 0..height {
            terminals.push((-1, y));
            terminals.push((width, y));
        }
    }

    (terminals, size)
}

fn circuit_drawing(
    (components, _, (width, height)): &Circuit,
    (placements, routes): &Layout,
) -> Drawing {
    let mut mesh = ShapeBuilder::new();

    // Components
    let component_color = [1., 0., 0.];
    for ((_, (width, height)), (x, y)) in components.iter().zip(placements) {
        rectangle(
            &mut mesh,
            *x as f32 - 0.5,
            *y as f32 - 0.5,
            *width as f32,
            *height as f32,
            component_color,
        );
    }

    // Routes
    let route_color = [0.6133, 0.9333, 1.1244];
    for route in routes {
        for pair in route.windows(2) {
            line(
                &mut mesh,
                pair[0].0 as f32,
                pair[0].1 as f32,
                pair[1].0 as f32,
                pair[1].1 as f32,
                route_color,
            );
        }
    }

    // Border
    rectangle(
        &mut mesh,
        0.,
        0.,
        *width as f32,
        *height as f32,
        [1., 1., 1.],
    );

    let scale = 1. / *width.max(height) as f32;
    let scale = Matrix4::new_translation(&Vector3::new(-1., -1., 0.))
        * Matrix4::from_diagonal(&Vector4::new(2., 2., 1., 1.))
        * Matrix4::from_diagonal(&Vector4::new(scale, scale, 1., 1.));

    let ShapeBuilder { vertices, indices } = mesh;
    ((vertices, indices), scale)
}

fn line(mesh: &mut ShapeBuilder, x1: f32, y1: f32, x2: f32, y2: f32, color: [f32; 3]) {
    let base = mesh.vertices.len() as u16;
    mesh.vertices
        .extend_from_slice(&[vert2d(x1, y1, color), vert2d(x2, y2, color)]);
    mesh.indices.extend_from_slice(&[base, base + 1]);
}

fn rectangle(mesh: &mut ShapeBuilder, x: f32, y: f32, width: f32, height: f32, color: [f32; 3]) {
    let base = mesh.vertices.len() as u16;

    mesh.vertices.extend_from_slice(&[
        vert2d(x, y, color),
        vert2d(x + width, y, color),
        vert2d(x + width, y + height, color),
        vert2d(x, y + height, color),
    ]);

    mesh.indices
        .extend([0, 1, 1, 2, 2, 3, 3, 0].iter().map(|i| i + base));
}

fn vert2d(x: f32, y: f32, color: [f32; 3]) -> Vertex {
    Vertex {
        pos: [x, y, 0.],
        color,
    }
}

#[derive(Default)]
pub struct ShapeBuilder {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}

impl ShapeBuilder {
    pub fn new() -> Self {
        Default::default()
    }
}

struct MyApp {
    animation: Vec<Object>,
    frame: usize,
}

impl App2D for MyApp {
    const TITLE: &'static str = "2D example app";
    type Args = Animation;

    fn new(engine: &mut WinitBackend, animation: Self::Args) -> Result<Self> {
        let material = engine.add_material(UNLIT_VERT, UNLIT_FRAG, DrawType::Lines)?;
        let base_transform = Matrix4::from_diagonal(&Vector4::new(0.5, 0.5, 1., 1.));

        let animation = animation
            .into_iter()
            .map(|((vertices, indices), transform)| {
                Ok(Object {
                    mesh: engine.add_mesh(&vertices, &indices)?,
                    transform: base_transform * transform,
                    material,
                })
            })
            .collect::<Result<_>>()?;

        Ok(Self {
            animation,
            frame: 0,
        })
    }

    fn event(&mut self, _event: &WindowEvent, _engine: &mut WinitBackend) -> Result<()> {
        Ok(())
    }

    fn frame(&mut self) -> FramePacket {
        let rate = 30;
        if self.frame >= self.animation.len() * rate {
            self.frame = 0;
        }
        let frame = self.animation[self.frame / rate];

        FramePacket {
            objects: vec![frame],
        }
    }
}
