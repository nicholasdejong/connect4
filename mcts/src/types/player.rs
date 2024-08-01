
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Player {
    #[default]
    Yellow,
    Red,
}

impl std::ops::Not for Player {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Player::Red => Player::Yellow,
            Player::Yellow => Player::Red,
        }
    }
}