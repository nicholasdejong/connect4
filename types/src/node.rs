use crate::r#move::Move;


#[derive(Debug, Default)]
pub struct NodeData {
    /// How many times this node has been visited
    pub visited: f32,
    /// The move taken to arrive at this node
    pub mv: Move,
    /// The expected reward for choosing this node
    pub reward: f32
}

#[derive(Debug, Default)]
pub struct Node {
    /// The node's data
    pub data: NodeData,
    /// The nodes that are accessible from the current position
    pub children: Vec<Node>
}

impl Node {
    pub fn visit(&mut self) {
        self.data.visited += 1.;
    }
}

impl From<Move> for Node {
    fn from(mv: Move) -> Self {
        Self {
            data: NodeData { visited: 1., mv, reward: 0. },
            ..Default::default()
        }
    }
}
