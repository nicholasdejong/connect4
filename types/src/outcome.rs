
#[derive(Debug)]
pub enum Outcome {
    Won,
    Lost,
    Drawn,
}

impl std::ops::Neg for Outcome {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            Self::Won => Self::Lost,
            Self::Lost => Self::Won,
            Self::Drawn => Self::Drawn,
        }
    }
}

impl Outcome {
    pub fn reward(&self) -> f32 {
        match self {
            Outcome::Won => 1.,
            Outcome::Drawn => 0.,
            Outcome::Lost => -1.
        }
    }
}