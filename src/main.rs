use anyhow::Result;
use connectgrid::walker::*;
use connectgrid::*;
use klystron::{
    runtime_2d::{event::WindowEvent, launch, App2D},
    DrawType, Engine, FramePacket, Matrix4, Object, Vertex, WinitBackend, UNLIT_FRAG, UNLIT_VERT,
};
use nalgebra::{Vector3, Vector4};

pub type Mesh = (Vec<Vertex>, Vec<u16>);
pub type Drawing = (Mesh, Matrix4<f32>);

fn dummy_eval(pt: Point, dest: Point, board: &Board) -> DirectionPrefs {
    [
        Direction::E,
        Direction::NE,
        Direction::N,
        Direction::NW,
        Direction::W,
        Direction::SW,
        Direction::S,
        Direction::SE,
    ]
}

fn better_eval(pt: Point, dest: Point, board: &Board) -> DirectionPrefs {
    let diff = point_sub(pt, dest);
    let dots = |dir: Direction| (point_dot(diff, dir.vector()), dir);
    let mut directions = [
        dots(Direction::E),
        dots(Direction::NE),
        dots(Direction::N),
        dots(Direction::NW),
        dots(Direction::W),
        dots(Direction::SW),
        dots(Direction::S),
        dots(Direction::SE),
    ];
    directions.sort_by_key(|k| k.0);
    [
        directions[0].1,
        directions[1].1,
        directions[2].1,
        directions[3].1,
        directions[4].1,
        directions[5].1,
        directions[6].1,
        directions[7].1,
    ]
}

fn main() -> Result<()> {
    let components = vec![
        chip((7, 2), true, false),
        chip((3, 18), false, true),
        chip((3, 3), true, true),
        chip((3, 3), true, true),
    ];
    let connections = dense(&components);
    /*
    let connections = vec![
        ((0, 0), (1, 0)),
        ((0, 1), (1, 9)),
        ((0, 2), (1, 19)),
        ((0, 3), (1, 23)),
    ];
    */
    let board_size = (30, 30);
    let circuit = (components, connections, board_size);

    let (placements, routes) = layout(&circuit).expect("Circuit layout problem");

    let mut game = Game::new(&circuit, &placements);

    let n_steps = 400;
    for _ in 0..n_steps {

        match game.step(better_eval) {
            Status::Running => (),
            Status::Stuck(idx) => {
                println!("Stuck at {}", idx);
                break;
            }
            Status::Finished => {
                println!("Finished!");
                break;
            }
        }
    }

    let routes = game.unfinished_routes();
    let layout = (placements.clone(), routes);
    let drawing = circuit_drawing(&circuit, &layout);

    launch::<MyApp>(drawing)
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
    //let route_color = [0.6133, 0.9333, 1.1244];
    for route in routes {
        for (idx, pair) in route.windows(2).enumerate() {
            let begin = idx as f32 / route.len() as f32;
            let end = (idx + 1) as f32 / route.len() as f32;
            line2(
                &mut mesh,
                pair[0].0 as f32,
                pair[0].1 as f32,
                pair[1].0 as f32,
                pair[1].1 as f32,
                [begin, begin, begin],
                [end, end, end],
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

fn line2(mesh: &mut ShapeBuilder, x1: f32, y1: f32, x2: f32, y2: f32, color_begin: [f32; 3], color_end: [f32; 3]) {
    let base = mesh.vertices.len() as u16;
    mesh.vertices
        .extend_from_slice(&[vert2d(x1, y1, color_begin), vert2d(x2, y2, color_end)]);
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
    frame: Object,
    anim: f32,
}

impl App2D for MyApp {
    const TITLE: &'static str = "2D example app";
    type Args = Drawing;

    fn new(engine: &mut WinitBackend, drawing: Self::Args) -> Result<Self> {
        let material = engine.add_material(
            UNLIT_VERT,
            &std::fs::read("./shaders/progressive.frag.spv")?, 
            DrawType::Lines
        )?;
        let base_transform = Matrix4::from_diagonal(&Vector4::new(0.5, 0.5, 1., 1.));

        let ((vertices, indices), transform) = drawing;
        let frame = Object {
            mesh: engine.add_mesh(&vertices, &indices)?,
            transform: base_transform * transform,
            material,
        };

        Ok(Self {
            frame,
            anim: 0.,
        })
    }

    fn event(&mut self, _event: &WindowEvent, _engine: &mut WinitBackend) -> Result<()> {
        Ok(())
    }

    fn frame(&mut self, engine: &mut WinitBackend) -> FramePacket {
        self.anim += 0.001;

        if self.anim > 1. {
            self.anim = 0.;
        }

        engine.update_time_value(self.anim);

        FramePacket {
            objects: vec![self.frame],
        }
    }
}

/*let connections = vec![
    ((0, 0), (1, 0)),
    ((0, 1), (1, 1)),
    ((0, 2), (1, 2)),
    ((0, 3), (1, 3)),
];*/
