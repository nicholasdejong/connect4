use std::{rc::{Rc, Weak}, cell::RefCell};
use crate::types::{r#move::Move, score::Score};


pub type Link = Rc<RefCell<Node>>;
pub type WeakLink = Weak<RefCell<Node>>;

#[derive(Default, Debug)]
pub struct Node {
    pub visited: usize,
    pub mv: Move,
    pub score: Score,
    pub children: Vec<Link>,
    pub parent: Option<WeakLink>,
}

impl From<Move> for Node {
    fn from(mv: Move) -> Self {
        Self {
            mv,
            children: Vec::new(),
            parent: None,
            ..Default::default()
        }
    }
}

impl Node {
    pub fn expected_score(&self) -> f32 {
        (self.score.wins() as f32 - self.score.losses() as f32) / self.visited as f32
    }
}
