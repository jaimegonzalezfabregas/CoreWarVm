use core::fmt;
use std::ops::*;

use rand::Rng;

pub fn modulo<T: TryInto<isize>, Q: TryInto<isize>>(dividend: T, divisor: Q) -> usize {
    let i_divisor = unsafe { TryInto::<isize>::try_into(divisor).unwrap_unchecked() };
    let i_dividend = unsafe { TryInto::<isize>::try_into(dividend).unwrap_unchecked() };

    return (((i_dividend % i_divisor) + i_divisor) % i_divisor) as usize;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ModUsize {
    pub val: usize,
    pub congruence: usize,
}

impl SubAssign<isize> for ModUsize {
    fn sub_assign(&mut self, rhs: isize) {
        self.val = modulo(self.val + modulo(-rhs, self.congruence), self.congruence);
    }
}

impl AddAssign<isize> for ModUsize {
    fn add_assign(&mut self, rhs: isize) {
        self.val = modulo(self.val + modulo(rhs, self.congruence), self.congruence);
    }
}

impl Add for ModUsize {
    type Output = ModUsize;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            val: modulo(self.val + rhs.val, self.congruence),
            congruence: self.congruence,
        }
    }
}

impl Add<usize> for ModUsize {
    type Output = ModUsize;

    fn add(self, rhs: usize) -> Self::Output {
        Self {
            val: modulo(self.val + rhs, self.congruence),
            congruence: self.congruence,
        }
    }
}

impl Add<isize> for ModUsize {
    type Output = ModUsize;

    fn add(self, rhs: isize) -> Self::Output {
        Self {
            val: modulo(self.val as isize + rhs, self.congruence),
            congruence: self.congruence,
        }
    }
}

impl Sub for ModUsize {
    type Output = ModUsize;

    fn sub(self, rhs: ModUsize) -> Self::Output {
        Self {
            val: modulo(self.val as isize - rhs.val as isize, self.congruence),
            congruence: self.congruence,
        }
    }
}

impl Sub<usize> for ModUsize {
    type Output = ModUsize;

    fn sub(self, rhs: usize) -> Self::Output {
        Self {
            val: modulo(self.val as isize - rhs as isize, self.congruence),
            congruence: self.congruence,
        }
    }
}

impl Sub<isize> for ModUsize {
    type Output = ModUsize;

    fn sub(self, rhs: isize) -> Self::Output {
        Self {
            val: modulo(self.val as isize - rhs, self.congruence),
            congruence: self.congruence,
        }
    }
}

impl Mul for ModUsize {
    type Output = ModUsize;

    fn mul(self, rhs: ModUsize) -> Self::Output {
        Self {
            val: modulo(self.val * rhs.val, self.congruence),
            congruence: self.congruence,
        }
    }
}

impl Mul<usize> for ModUsize {
    type Output = ModUsize;

    fn mul(self, rhs: usize) -> Self::Output {
        Self {
            val: modulo(self.val * rhs, self.congruence),
            congruence: self.congruence,
        }
    }
}

impl Mul<isize> for ModUsize {
    type Output = ModUsize;

    fn mul(self, rhs: isize) -> Self::Output {
        Self {
            val: modulo(self.val as isize * rhs, self.congruence),
            congruence: self.congruence,
        }
    }
}

impl Div for ModUsize {
    type Output = ModUsize;

    fn div(self, rhs: ModUsize) -> Self::Output {
        Self {
            val: modulo(self.val / rhs.val, self.congruence),
            congruence: self.congruence,
        }
    }
}

impl Div<usize> for ModUsize {
    type Output = ModUsize;

    fn div(self, rhs: usize) -> Self::Output {
        Self {
            val: modulo(self.val / rhs, self.congruence),
            congruence: self.congruence,
        }
    }
}

impl Div<isize> for ModUsize {
    type Output = ModUsize;

    fn div(self, rhs: isize) -> Self::Output {
        Self {
            val: modulo(self.val as isize / rhs, self.congruence),
            congruence: self.congruence,
        }
    }
}

impl ModUsize {
    pub fn new(val: isize, core_size: usize) -> ModUsize {
        Self {
            val: modulo(val, core_size),
            congruence: core_size,
        }
    }

    pub fn inc(&self, x: isize) -> ModUsize {
        Self {
            val: modulo(self.val + modulo(x, self.congruence), self.congruence),
            congruence: self.congruence,
        }
    }

    pub(crate) fn rand(congruence: usize, range: Range<usize>) -> ModUsize {
        Self {
            val: rand::thread_rng().gen_range(range),
            congruence: congruence,
        }
    }
}

impl PartialEq<usize> for ModUsize {
    fn eq(&self, other: &usize) -> bool {
        self.val == *other
    }
}

impl fmt::Display for ModUsize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let res = if self.val > self.congruence / 2 {
            (self.val as isize - self.congruence as isize) as isize
        } else {
            self.val as isize
        };

        write!(f, "{res}")
    }
}
