
use std::collections::VecDeque;

use crate::grid:: { GridPosition };

#[derive(Clone, Copy, Debug)]
pub struct Segment {
    pub pos: GridPosition,
}

impl Segment {
    pub fn new(pos: GridPosition) -> Self {
        Segment { pos }
    }
}

#[derive(Clone)]
pub struct Snek {
    pub body: VecDeque<Segment>,
}

impl Snek {
    pub fn new(pos: GridPosition) -> Self {
        let mut body = VecDeque::new();
        body.push_back(Segment::new( GridPosition { x: pos.x - 1, y: pos.y }));

        Snek {
            body: body,
        }
    }

    pub fn get_head(&self) -> GridPosition {
        (*self.body.front().unwrap()).pos
    }

    pub fn eat(&self, pos: GridPosition) -> Snek {
        let mut e = self.body.clone();
        e.push_front(Segment::new(pos));
        Snek { body: e }
    }

    pub fn is_occupied(&self, pos: &GridPosition) -> bool {
        match self.body.iter().find(|&&element| element.pos == *pos) {
            Some(_) => true,
            None => false
        }
    }

    pub fn go(&self, pos: &GridPosition) -> Snek {
        let mut e = self.body.clone();
        e.push_front(Segment::new(*pos));
        e.pop_back();
        Snek { body: e }
    }

}
