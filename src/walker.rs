use crate::*;
use std::collections::HashSet;

#[derive(Clone)]
pub struct Walker {
    pub dest: Point,
    pub history: Vec<Point>,
}

impl Walker {
    pub fn new(position: Point, dest: Point) -> Self {
        Self {
            history: vec![position],
            dest,
        }
    }

    pub fn position(&self) -> Point {
        *self.history.last().unwrap()
    }

    pub fn finished(&self) -> bool {
        self.position() == self.dest
    }

    pub fn route(self) -> Route {
        self.history
    }
}

#[derive(Default)]
pub struct Board {
    tiles: HashSet<Point>,
    diagonals: HashSet<(Point, Point)>,
}

impl Board {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn try_insert(&mut self, src: Point, direction: Direction) -> bool {
        let dest = point_add(src, direction.vector());
        if self.tiles.contains(&dest) {
            return false;
        }

        if direction.diagonal() {
            if self.diagonals.contains(&(src, dest)) {
                return false;
            } else {
                let ((ax, ay), (bx, by)) = (src, dest);
                let first = (ax, by);
                let second = (bx, ay);
                self.diagonals.insert((first, second));
                self.diagonals.insert((second, first));
            }
        }
        self.tiles.insert(dest);
        true
    }

    pub fn init_point(&mut self, src: Point) -> bool {
        self.tiles.insert(src)
    }
}

pub struct Game {
    pub board: Board,
    pub walkers: Vec<Walker>,
    pub routes: Vec<Route>,
}

#[derive(Debug, Copy, Clone)]
pub enum Status {
    Running,
    /// The walker at this index became stuck
    Stuck(usize),
    Finished,
}

impl Game {
    pub fn new((components, connections, _size): &Circuit, placements: &[Placement]) -> Self {
        let mut board = Board::new();
        let mut walkers = Vec::new();
        for (src, dst) in connections {
            let deref_pt = |(c, t): (usize, usize)| point_add(components[c].0[t], placements[c]);
            let src = deref_pt(*src);
            let dst = deref_pt(*dst);
            board.init_point(src);
            walkers.push(Walker::new(src, dst));
        }

        Self {
            board,
            walkers,
            routes: Vec::new(),
        }
    }

    pub fn step(&mut self, mut evaluator: impl FnMut(Point, Point, &Board) -> DirectionPrefs) -> Status {
        'outer: for (idx, walker) in self.walkers.iter_mut().enumerate() {
            let position = walker.position();
            let direction_prefs = evaluator(position, walker.dest, &self.board);

            for direction in &direction_prefs {
                if self.board.try_insert(position, *direction) {
                    let next = point_add(position, direction.vector());
                    walker.history.push(next);
                    continue 'outer;
                }
            }
            // TODO: ENABLE ME
            return Status::Stuck(idx);
        }

        self.routes.extend(
            self.walkers
                .drain_filter(|w| w.finished())
                .map(Walker::route),
        );

        if self.walkers.is_empty() {
            Status::Finished
        } else {
            Status::Running
        }
    }

    pub fn unfinished_routes(&self) -> Vec<Route> {
        let mut routes = self.routes.clone();
        routes.extend(self.walkers.iter().map(|w| w.clone().route()));
        routes
    }
}

pub type DirectionPrefs = [Direction; 8];

#[repr(usize)]
#[derive(Debug, Copy, Clone)]
pub enum Direction {
    E = 0,
    NE = 1,
    N = 2,
    NW = 3,
    W = 4,
    SW = 5,
    S = 6,
    SE = 7,
}

impl Direction {
    pub fn vector(&self) -> (i32, i32) {
        match self {
            Direction::E => (1, 0),
            Direction::NE => (1, 1),
            Direction::N => (0, 1),
            Direction::NW => (-1, 1),
            Direction::W => (-1, 0),
            Direction::SW => (-1, -1),
            Direction::S => (0, -1),
            Direction::SE => (1, -1),
        }
    }
    
    pub fn diagonal(&self) -> bool {
        match self {
            Self::N | Self::S | Self::E | Self::W => false,
            _ => true,
        }
    }
}
