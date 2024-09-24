#[cfg(feature = "speedy")]
use speedy::{Readable, Writable};

#[allow(non_camel_case_types)]
pub struct u7(u32);

/// A rust veriosn of the C# 7bit encoding scheme
impl u7 {
    /// Create a new `u7` from a `u32`.
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    /// Read a `u7` from a byte slice.
    pub fn from_encoded_bytes(bytes: &[u8]) -> (u7, usize) {
        let mut value = 0u32;
        let mut shift = 0;
        let mut i = 0;
        loop {
            let byte = bytes[i];
            value |= ((byte & 0x7F) as u32) << shift;
            shift += 7;
            i += 1;
            if byte & 0x80 == 0 || shift >= 32 {
                break;
            }
        }
        (u7(value), i)
    }

    /// Get the value of the `u7` as bytes
    pub fn to_encoded_bytes(&self) -> Vec<u8> {
        let mut value = self.0;
        let mut bytes = Vec::new();

        // println!("value: 0b{:b}", value);

        while value >= 0x80 {
            bytes.push((value & 0x7F) as u8 | 0x80);
            value >>= 7;
        }

        bytes.push(value as u8);

        bytes
    }
}

#[cfg(feature = "speedy")]
impl Writable<speedy::LittleEndian> for u7 {
    fn write_to<T: ?Sized + speedy::Writer<speedy::LittleEndian>>(
        &self,
        writer: &mut T,
    ) -> Result<(), <speedy::LittleEndian as speedy::Context>::Error> {
        let bytes = self.to_encoded_bytes();
        writer.write_value(&bytes)?;
        Ok(())
    }
}

#[cfg(feature = "speedy")]
impl<'a> Readable<'a, speedy::LittleEndian> for u7 {
    fn read_from<T: ?Sized + speedy::Reader<'a, speedy::LittleEndian>>(
        reader: &mut T,
    ) -> Result<Self, <speedy::LittleEndian as speedy::Context>::Error> {
        let mut value = 0u32;
        let mut shift = 0;
        loop {
            let byte = u8::read_from(reader);
            if byte.is_err() {
                return Err(byte.err().unwrap());
            }
            let byte = byte.unwrap();
            value |= ((byte & 0x7F) as u32) << shift;
            shift += 7;
            if byte & 0x80 == 0 || shift >= 32 {
                break;
            }
        }
        Ok(u7(value))
    }
}

impl std::cmp::PartialEq for u7 {
    fn eq(&self, other: &u7) -> bool {
        self.0 == other.0
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

// impl std::ops::Neg for u7 {
//     type Output = u7;

//     fn neg(self) -> u7 {
//         u7(-self.0)
//     }
// }

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

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_u7() {
        let a = u7::new(0x80);
        let b = u7::new(0x80);
        let c = a + b;
        assert_eq!(c, u7::new(0x100));
    }

    #[test]
    fn test_u7_to_bytes() {
        let a = u7::new(0x80);
        let bytes = a.to_encoded_bytes();
        assert_eq!(bytes, vec![0x80, 0x01]);
        let b = u7::new(1);
        let bytes = b.to_encoded_bytes();
        assert_eq!(bytes, vec![0x01]);
    }
}
