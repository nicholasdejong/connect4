
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