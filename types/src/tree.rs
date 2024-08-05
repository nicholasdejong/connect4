use core::f32;

use rand::{thread_rng, Rng};

use crate::{
    board::{choose_move, Board},
    node::{Node, NodeData},
    outcome::Outcome,
    r#move::Move,
};

/// Represents the Monte-Carlo search tree
pub struct Tree {
    head: Box<Zipper>,
}

impl Tree {
    /// Create a new game tree
    pub fn new() -> Self {
        let node: Node = Move::EMPTY.into();
        let zipper: Zipper = node.into();

        Self {
            head: Box::new(zipper),
        }
    }

    /// Obtain the corresponding zipper
    pub fn head(self) -> Zipper {
        *self.head
    }

    /// Returns the best move in the game tree
    pub fn best(&self) -> Move {
        let mut best = (f32::NEG_INFINITY, Move::EMPTY);
        for child in &self.head.node.children {
            // println!("{:?}", child.data.reward / child.data.visited as f32);
            let score = child.data.reward / child.data.visited as f32;
            // println!("{score:?} {}", child.data.visited);
            if score > best.0 {
                best = (score, child.data.mv);
            }
        }
        best.1
    }

    /// Flips the perspective from the one player to the other
    pub fn flip(&mut self) {
        for child in &mut self.head.node.children {
            child.data.reward = -child.data.reward;
        }
    }

    pub fn log(&self) {
        println!("--------------");
        for child in &self.head.node.children {
            println!("{child:?}");
        }
        println!("--------------");
    }
}

/// Provides focus for a certain node
#[derive(Default, Debug)]
pub struct Zipper {
    /// The node that is receiving the focus
    node: Node,
    /// The zipper's parent
    parent: Option<Box<Zipper>>,
    /// The index of this zipper in its parent's `children` vec
    child_index: usize,
}

impl Zipper {
    pub fn child(mut self, index: usize) -> Self {
        let child = self.node.children.swap_remove(index);

        Self {
            node: child,
            parent: Some(Box::new(self)),
            child_index: index,
        }
    }

    pub fn parent(self) -> Self {
        let Self {
            node,
            parent,
            child_index,
        } = self;

        let Self {
            node: mut parent_node,
            parent: parent_parent,
            child_index: parent_index,
        } = *parent.unwrap();

        parent_node.children.push(node);
        let len = parent_node.children.len();
        parent_node.children.swap(child_index, len - 1);

        Self {
            node: parent_node,
            parent: parent_parent,
            child_index: parent_index,
        }
    }

    /// An Upper Confidence Bound for Trees (UCT) algorithm.
    pub fn select(mut self, board: &mut Board, rounds: f32) -> Self {
        // println!("{}", self.node.children.len());
        self.visit();

        if self.node.children.len() < board.moves().len() as usize || board.game_over() {
            return self;
        }


        // Select the best node according to the UCB1 policy.
        let mut best = (f32::NEG_INFINITY, 0);
        for (idx, child) in self.node.children.iter().enumerate() {
            let expected_reward = -child.data.reward / child.data.visited;
            let score =
                expected_reward + 2. * ((rounds).ln() / child.data.visited).sqrt();
            // let score = 0.;
            if score > best.0 {
                best = (score, idx);
            }
        }

        let child = self.child(best.1);
        // let child = self.child(idx);
        board.play(child.data().mv);

        child.select(board, rounds)
    }

    /// Expands the selected node, creating a new Zipper if the game is not yet over.
    pub fn expand(mut self, board: &mut Board) -> Self {
        if board.game_over() {
            return self;
        }

        let len = self.node.children.len();
        let mv = choose_move(&board.moves(), len);
        board.play(mv);

        let node: Node = mv.into();
        self.node.children.push(node);
        self.child(len)
    }

    pub fn backpropagate(mut self, board: &mut Board, mut outcome: Outcome) -> Self {
        // const DISCOUNT: f32 = 0.9;

        loop {
            board.unplay(self.data().mv);
            self.data_mut().reward += outcome.reward(); // * DISCOUNT;
            outcome = -outcome;

            if self.parent.is_some() {
                self = self.parent();
            } else {
                break self;
            }
        }
    }

    pub fn has_parent(&self) -> bool {
        self.parent.is_some()
    }

    /// After consuming the tree's head, we can give back ownership
    pub fn into_tree(self) -> Tree {
        Tree {
            head: Box::new(self),
        }
    }

    pub fn visit(&mut self) {
        self.node.visit();
    }

    /// Receive a shared reference to the focused node's data
    pub fn data(&self) -> &NodeData {
        &self.node.data
    }

    /// Receive a mutable reference to the focused node's data
    pub fn data_mut(&mut self) -> &mut NodeData {
        &mut self.node.data
    }
}

impl From<Node> for Zipper {
    fn from(node: Node) -> Self {
        Self {
            node,
            parent: None,
            child_index: 0,
        }
    }
}
