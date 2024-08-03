use crate::outcome::Outcome;
#[derive(Default, Debug, Clone, Copy)]
pub struct Score {
    wins: usize,
    losses: usize,
}

impl Score {
    pub fn update(&mut self, outcome: &Outcome) {
        match outcome {
            Outcome::Won => self.wins += 1,
            Outcome::Lost => self.losses += 1,
            _ => {} // draws can be calculated by Node.visited - Node.score.wins - Node.score.losses
        };
    }

    pub fn wins(&self) -> usize {
        self.wins
    }

    pub fn losses(&self) -> usize {
        self.losses
    }

    pub fn invert(&mut self) {
        std::mem::swap(&mut self.wins, &mut self.losses)
    }
}

impl std::ops::Neg for Score {
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        std::mem::swap(&mut self.wins, &mut self.losses);
        self
    }
}