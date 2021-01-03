use crate::*;
use std::collections::HashSet;

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
}

pub struct Game {
    pub board: HashSet<Point>,
    pub walkers: Vec<Walker>,
}

enum Status {
    Running,
    /// The walker at this index became stuck
    Stuck(usize),
    Finished,
}

impl Game {
    pub fn new((components, connections, _size): Circuit) -> Self {
        let mut board = HashSet::new();
        let mut walkers = Vec::new();
        for (src, dst) in connections {
            let deref_pt = |(c, t): (usize, usize)| components[c].0[t];
            let src = deref_pt(src);
            let dst = deref_pt(dst);
            board.insert(src);
            walkers.push(Walker::new(src, dst));
        }

        Self {
            board,
            walkers
        }
    }

    pub fn step(&mut self, evaluator: impl FnMut(&[f32]) -> &[f32]) -> Status {
        Status::Running
    }

    pub fn routes(self) -> Vec<Route> {
        self.walkers.into_iter().map(|w| w.history).collect()
    }
}
