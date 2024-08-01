#[derive(Clone, Copy, PartialEq, Default)]
pub struct BitBoard(pub u64);

impl BitBoard {
    pub const EMPTY: BitBoard = BitBoard(0);
    // pub const FULL: BitBoard = BitBoard(!0);
    pub const NOT_A: BitBoard = BitBoard(0xfefefefefefefefe);
    pub const NOT_H: BitBoard = BitBoard(0x7f7f7f7f7f7f7f7f);
    pub const A: BitBoard = BitBoard(0x101010101010101);
    pub const COLUMNS: usize = 8;

    pub const fn idx(idx: usize) -> Self {
        BitBoard(1 << idx)
    }

    pub const fn len(&self) -> u32 {
        self.0.count_ones()
    }

    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub const fn shl(&self, bits: usize) -> BitBoard {
        BitBoard(self.0 << bits)
    }

    pub const fn shr(&self, bits: usize) -> BitBoard {
        BitBoard(self.0 >> bits)
    }

    pub const fn is_full(&self) -> bool {
        self.0 == !0
    }

    /// Returns all bits in the specified column
    pub fn column(self, column: usize) -> BitBoard {
        self & BitBoard::A.shl(column)
    }
}

impl std::fmt::Debug for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

impl std::ops::Not for BitBoard {
    type Output = Self;
    fn not(self) -> Self::Output {
        BitBoard(!self.0)
    }
}

impl std::ops::Sub<u64> for BitBoard {
    type Output = Self;
    fn sub(self, rhs: u64) -> Self::Output {
        BitBoard(self.0 - rhs)
    }
}

macro_rules! bitops {
    ($($t:ident::$f:ident),*) => {
        $(impl std::ops::$t for BitBoard {
            type Output = Self;
            fn $f(self, rhs: Self) -> Self::Output {
                BitBoard(std::ops::$t::$f(self.0, rhs.0))
            }
        })*
    };
}

bitops!(BitAnd::bitand, BitOr::bitor, BitXor::bitxor);

macro_rules! bitassign {
    ($($t:ident::$f:ident),*) => {
        $(impl std::ops::$t for BitBoard {
            fn $f(&mut self, rhs: Self) {
                std::ops::$t::$f(&mut self.0, rhs.0);
            }
        })*
    };
}

bitassign!(BitAndAssign::bitand_assign, BitOrAssign::bitor_assign, BitXorAssign::bitxor_assign);

pub fn horizontal(b: BitBoard) -> BitBoard {
    let mut tmp = b;

    for _ in 0..3 {
        tmp &= (tmp.shl(1)) & BitBoard::NOT_A;
    }

    tmp
}

pub fn vertical(b: BitBoard) -> BitBoard {
    let mut tmp = b;

    for _ in 0..3 {
        tmp &= tmp.shl(8);
    }

    tmp
}

pub fn a1_h8(b: BitBoard) -> BitBoard {
    let mut tmp = b;

    for _ in 0..3 {
        tmp &= (tmp.shl(9)) & BitBoard::NOT_A;
    }

    tmp
}

pub fn h1_a8(b: BitBoard) -> BitBoard {
    let mut tmp = b;

    for _ in 0..3 {
        tmp &= (tmp.shl(7)) & BitBoard::NOT_H;
    }

    tmp
}