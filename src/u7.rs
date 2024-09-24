#[cfg(feature = "speedy")]
use speedy::{Readable, Writable};

pub struct u7(u32);

/// A rust veriosn of the C# 7bit encoding scheme
impl u7 {
    /// Create a new `u7` from a `u32`.
    pub fn new(value: u32) -> Self {
        Self(value)
    }
}

#[cfg(feature = "speedy")]
impl Readable<'a, speedy::LittleEndian> for u7 {
    fn read_from<R: std::io::Read>(reader: &mut R) -> Result<Self, speedy::Error> {
        let mut value = 0u32;
        let mut shift = 0;
        loop {
            let byte = u8::read_from(reader)?;
            value |= ((byte & 0x7F) as u32) << shift;
            shift += 7;
            if byte & 0x80 == 0 {
                break;
            }
        }
        Ok(u7(value))
    }
}

impl std::fmt::Debug for u7 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for u7 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u7> for u32 {
    fn from(value: u7) -> u32 {
        value.0
    }
}

impl From<u32> for u7 {
    fn from(value: u32) -> u7 {
        u7(value)
    }
}

impl std::ops::Add<u7> for u7 {
    type Output = u7;

    fn add(self, rhs: u7) -> u7 {
        u7(self.0 + rhs.0)
    }
}

impl std::ops::AddAssign<u7> for u7 {
    fn add_assign(&mut self, rhs: u7) {
        self.0 += rhs.0;
    }
}

impl std::ops::Sub<u7> for u7 {
    type Output = u7;

    fn sub(self, rhs: u7) -> u7 {
        u7(self.0 - rhs.0)
    }
}

impl std::ops::SubAssign<u7> for u7 {
    fn sub_assign(&mut self, rhs: u7) {
        self.0 -= rhs.0;
    }
}

impl std::ops::Mul<u7> for u7 {
    type Output = u7;

    fn mul(self, rhs: u7) -> u7 {
        u7(self.0 * rhs.0)
    }
}

impl std::ops::MulAssign<u7> for u7 {
    fn mul_assign(&mut self, rhs: u7) {
        self.0 *= rhs.0;
    }
}

impl std::ops::Div<u7> for u7 {
    type Output = u7;

    fn div(self, rhs: u7) -> u7 {
        u7(self.0 / rhs.0)
    }
}

impl std::ops::DivAssign<u7> for u7 {
    fn div_assign(&mut self, rhs: u7) {
        self.0 /= rhs.0;
    }
}

impl std::ops::Rem<u7> for u7 {
    type Output = u7;

    fn rem(self, rhs: u7) -> u7 {
        u7(self.0 % rhs.0)
    }
}

impl std::ops::RemAssign<u7> for u7 {
    fn rem_assign(&mut self, rhs: u7) {
        self.0 %= rhs.0;
    }
}

impl std::ops::Neg for u7 {
    type Output = u7;

    fn neg(self) -> u7 {
        u7(-self.0)
    }
}

impl std::ops::Not for u7 {
    type Output = u7;

    fn not(self) -> u7 {
        u7(!self.0)
    }
}

impl std::ops::BitAnd<u7> for u7 {
    type Output = u7;

    fn bitand(self, rhs: u7) -> u7 {
        u7(self.0 & rhs.0)
    }
}

impl std::ops::BitAndAssign<u7> for u7 {
    fn bitand_assign(&mut self, rhs: u7) {
        self.0 &= rhs.0;
    }
}

impl std::ops::BitOr<u7> for u7 {
    type Output = u7;

    fn bitor(self, rhs: u7) -> u7 {
        u7(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign<u7> for u7 {
    fn bitor_assign(&mut self, rhs: u7) {
        self.0 |= rhs.0;
    }
}
