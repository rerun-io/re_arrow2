use std::convert::TryFrom;
use std::ops::Neg;
use std::panic::RefUnwindSafe;

use bytemuck::{Pod, Zeroable};

pub use half::f16;

use super::PrimitiveType;

/// Sealed trait implemented by all physical types that can be allocated,
/// serialized and deserialized by this crate.
/// All O(N) allocations in this crate are done for this trait alone.
pub trait NativeType:
    super::private::Sealed
    + Pod
    + Send
    + Sync
    + Sized
    + RefUnwindSafe
    + std::fmt::Debug
    + std::fmt::Display
    + PartialEq
    + Default
{
    /// The corresponding variant of [`PrimitiveType`].
    const PRIMITIVE: PrimitiveType;

    /// Type denoting its representation as bytes.
    /// This is `[u8; N]` where `N = size_of::<T>`.
    type Bytes: AsRef<[u8]>
        + std::ops::Index<usize, Output = u8>
        + std::ops::IndexMut<usize, Output = u8>
        + for<'a> TryFrom<&'a [u8]>
        + std::fmt::Debug
        + Default;

    /// To bytes in little endian
    fn to_le_bytes(&self) -> Self::Bytes;

    /// To bytes in big endian
    fn to_be_bytes(&self) -> Self::Bytes;

    /// From bytes in little endian
    fn from_le_bytes(bytes: Self::Bytes) -> Self;

    /// From bytes in big endian
    fn from_be_bytes(bytes: Self::Bytes) -> Self;
}

macro_rules! native_type {
    ($type:ty, $primitive_type:expr) => {
        impl NativeType for $type {
            const PRIMITIVE: PrimitiveType = $primitive_type;

            type Bytes = [u8; std::mem::size_of::<Self>()];
            #[inline]
            fn to_le_bytes(&self) -> Self::Bytes {
                Self::to_le_bytes(*self)
            }

            #[inline]
            fn to_be_bytes(&self) -> Self::Bytes {
                Self::to_be_bytes(*self)
            }

            #[inline]
            fn from_le_bytes(bytes: Self::Bytes) -> Self {
                Self::from_le_bytes(bytes)
            }

            #[inline]
            fn from_be_bytes(bytes: Self::Bytes) -> Self {
                Self::from_be_bytes(bytes)
            }
        }
    };
}

native_type!(u8, PrimitiveType::UInt8);
native_type!(u16, PrimitiveType::UInt16);
native_type!(u32, PrimitiveType::UInt32);
native_type!(u64, PrimitiveType::UInt64);
native_type!(i8, PrimitiveType::Int8);
native_type!(i16, PrimitiveType::Int16);
native_type!(i32, PrimitiveType::Int32);
native_type!(i64, PrimitiveType::Int64);
native_type!(f32, PrimitiveType::Float32);
native_type!(f64, PrimitiveType::Float64);
native_type!(i128, PrimitiveType::Int128);

/// The in-memory representation of the DayMillisecond variant of arrow's "Interval" logical type.
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Hash, Zeroable, Pod)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct days_ms(pub i32, pub i32);

impl days_ms {
    /// A new [`days_ms`].
    #[inline]
    pub fn new(days: i32, milliseconds: i32) -> Self {
        Self(days, milliseconds)
    }

    /// The number of days
    #[inline]
    pub fn days(&self) -> i32 {
        self.0
    }

    /// The number of milliseconds
    #[inline]
    pub fn milliseconds(&self) -> i32 {
        self.1
    }
}

impl NativeType for days_ms {
    const PRIMITIVE: PrimitiveType = PrimitiveType::DaysMs;
    type Bytes = [u8; 8];
    #[inline]
    fn to_le_bytes(&self) -> Self::Bytes {
        let days = self.0.to_le_bytes();
        let ms = self.1.to_le_bytes();
        let mut result = [0; 8];
        result[0] = days[0];
        result[1] = days[1];
        result[2] = days[2];
        result[3] = days[3];
        result[4] = ms[0];
        result[5] = ms[1];
        result[6] = ms[2];
        result[7] = ms[3];
        result
    }

    #[inline]
    fn to_be_bytes(&self) -> Self::Bytes {
        let days = self.0.to_be_bytes();
        let ms = self.1.to_be_bytes();
        let mut result = [0; 8];
        result[0] = days[0];
        result[1] = days[1];
        result[2] = days[2];
        result[3] = days[3];
        result[4] = ms[0];
        result[5] = ms[1];
        result[6] = ms[2];
        result[7] = ms[3];
        result
    }

    #[inline]
    fn from_le_bytes(bytes: Self::Bytes) -> Self {
        let mut days = [0; 4];
        days[0] = bytes[0];
        days[1] = bytes[1];
        days[2] = bytes[2];
        days[3] = bytes[3];
        let mut ms = [0; 4];
        ms[0] = bytes[4];
        ms[1] = bytes[5];
        ms[2] = bytes[6];
        ms[3] = bytes[7];
        Self(i32::from_le_bytes(days), i32::from_le_bytes(ms))
    }

    #[inline]
    fn from_be_bytes(bytes: Self::Bytes) -> Self {
        let mut days = [0; 4];
        days[0] = bytes[0];
        days[1] = bytes[1];
        days[2] = bytes[2];
        days[3] = bytes[3];
        let mut ms = [0; 4];
        ms[0] = bytes[4];
        ms[1] = bytes[5];
        ms[2] = bytes[6];
        ms[3] = bytes[7];
        Self(i32::from_be_bytes(days), i32::from_be_bytes(ms))
    }
}

/// The in-memory representation of the MonthDayNano variant of the "Interval" logical type.
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Hash, Zeroable, Pod)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct months_days_ns(pub i32, pub i32, pub i64);

impl months_days_ns {
    /// A new [`months_days_ns`].
    #[inline]
    pub fn new(months: i32, days: i32, nanoseconds: i64) -> Self {
        Self(months, days, nanoseconds)
    }

    /// The number of months
    #[inline]
    pub fn months(&self) -> i32 {
        self.0
    }

    /// The number of days
    #[inline]
    pub fn days(&self) -> i32 {
        self.1
    }

    /// The number of nanoseconds
    #[inline]
    pub fn ns(&self) -> i64 {
        self.2
    }
}

impl NativeType for months_days_ns {
    const PRIMITIVE: PrimitiveType = PrimitiveType::MonthDayNano;
    type Bytes = [u8; 16];
    #[inline]
    fn to_le_bytes(&self) -> Self::Bytes {
        let months = self.months().to_le_bytes();
        let days = self.days().to_le_bytes();
        let ns = self.ns().to_le_bytes();
        let mut result = [0; 16];
        result[0] = months[0];
        result[1] = months[1];
        result[2] = months[2];
        result[3] = months[3];
        result[4] = days[0];
        result[5] = days[1];
        result[6] = days[2];
        result[7] = days[3];
        (0..8).for_each(|i| {
            result[8 + i] = ns[i];
        });
        result
    }

    #[inline]
    fn to_be_bytes(&self) -> Self::Bytes {
        let months = self.months().to_be_bytes();
        let days = self.days().to_be_bytes();
        let ns = self.ns().to_be_bytes();
        let mut result = [0; 16];
        result[0] = months[0];
        result[1] = months[1];
        result[2] = months[2];
        result[3] = months[3];
        result[4] = days[0];
        result[5] = days[1];
        result[6] = days[2];
        result[7] = days[3];
        (0..8).for_each(|i| {
            result[8 + i] = ns[i];
        });
        result
    }

    #[inline]
    fn from_le_bytes(bytes: Self::Bytes) -> Self {
        let mut months = [0; 4];
        months[0] = bytes[0];
        months[1] = bytes[1];
        months[2] = bytes[2];
        months[3] = bytes[3];
        let mut days = [0; 4];
        days[0] = bytes[4];
        days[1] = bytes[5];
        days[2] = bytes[6];
        days[3] = bytes[7];
        let mut ns = [0; 8];
        (0..8).for_each(|i| {
            ns[i] = bytes[8 + i];
        });
        Self(
            i32::from_le_bytes(months),
            i32::from_le_bytes(days),
            i64::from_le_bytes(ns),
        )
    }

    #[inline]
    fn from_be_bytes(bytes: Self::Bytes) -> Self {
        let mut months = [0; 4];
        months[0] = bytes[0];
        months[1] = bytes[1];
        months[2] = bytes[2];
        months[3] = bytes[3];
        let mut days = [0; 4];
        days[0] = bytes[4];
        days[1] = bytes[5];
        days[2] = bytes[6];
        days[3] = bytes[7];
        let mut ns = [0; 8];
        (0..8).for_each(|i| {
            ns[i] = bytes[8 + i];
        });
        Self(
            i32::from_be_bytes(months),
            i32::from_be_bytes(days),
            i64::from_be_bytes(ns),
        )
    }
}

impl std::fmt::Display for days_ms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}d {}ms", self.days(), self.milliseconds())
    }
}

impl std::fmt::Display for months_days_ns {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}m {}d {}ns", self.months(), self.days(), self.ns())
    }
}

impl Neg for days_ms {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        Self::new(-self.days(), -self.milliseconds())
    }
}

impl Neg for months_days_ns {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        Self::new(-self.months(), -self.days(), -self.ns())
    }
}

impl NativeType for f16 {
    const PRIMITIVE: PrimitiveType = PrimitiveType::Float16;
    type Bytes = [u8; 2];
    #[inline]
    fn to_le_bytes(&self) -> Self::Bytes {
        f16::to_le_bytes(*self)
    }

    #[inline]
    fn to_be_bytes(&self) -> Self::Bytes {
        f16::to_be_bytes(*self)
    }

    #[inline]
    fn from_be_bytes(bytes: Self::Bytes) -> Self {
        f16::from_be_bytes(bytes)
    }

    #[inline]
    fn from_le_bytes(bytes: Self::Bytes) -> Self {
        f16::from_le_bytes(bytes)
    }
}

/// Physical representation of a decimal
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct i256(pub ethnum::I256);

impl i256 {
    /// Returns a new [`i256`] from two `i128`.
    pub fn from_words(hi: i128, lo: i128) -> Self {
        Self(ethnum::I256::from_words(hi, lo))
    }
}

impl Neg for i256 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        let (a, b) = self.0.into_words();
        Self(ethnum::I256::from_words(-a, b))
    }
}

impl std::fmt::Debug for i256 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl std::fmt::Display for i256 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

unsafe impl Pod for i256 {}
unsafe impl Zeroable for i256 {}

impl NativeType for i256 {
    const PRIMITIVE: PrimitiveType = PrimitiveType::Int256;

    type Bytes = [u8; 32];

    #[inline]
    fn to_le_bytes(&self) -> Self::Bytes {
        let mut bytes = [0u8; 32];
        let (a, b) = self.0.into_words();
        let a = a.to_le_bytes();
        (0..16).for_each(|i| {
            bytes[i] = a[i];
        });

        let b = b.to_le_bytes();
        (0..16).for_each(|i| {
            bytes[i + 16] = b[i];
        });

        bytes
    }

    #[inline]
    fn to_be_bytes(&self) -> Self::Bytes {
        let mut bytes = [0u8; 32];
        let (a, b) = self.0.into_words();

        let a = a.to_be_bytes();
        (0..16).for_each(|i| {
            bytes[i] = a[i];
        });

        let b = b.to_be_bytes();
        (0..16).for_each(|i| {
            bytes[i + 16] = b[i];
        });

        bytes
    }

    #[inline]
    fn from_be_bytes(bytes: Self::Bytes) -> Self {
        let (a, b) = bytes.split_at(16);
        let a: [u8; 16] = a.try_into().unwrap();
        let b: [u8; 16] = b.try_into().unwrap();
        let a = i128::from_be_bytes(a);
        let b = i128::from_be_bytes(b);
        Self(ethnum::I256::from_words(a, b))
    }

    #[inline]
    fn from_le_bytes(bytes: Self::Bytes) -> Self {
        let (b, a) = bytes.split_at(16);
        let a: [u8; 16] = a.try_into().unwrap();
        let b: [u8; 16] = b.try_into().unwrap();
        let a = i128::from_le_bytes(a);
        let b = i128::from_le_bytes(b);
        Self(ethnum::I256::from_words(a, b))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_f16_to_f32() {
        let f = f16::from_f32(7.0);
        assert_eq!(f.to_f32(), 7.0f32);

        // 7.1 is NOT exactly representable in 16-bit, it's rounded
        let f = f16::from_f32(7.1);
        let diff = (f.to_f32() - 7.1f32).abs();
        // diff must be <= 4 * EPSILON, as 7 has two more significant bits than 1
        assert!(diff <= 4.0 * f16::EPSILON.to_f32());

        assert_eq!(f16::from_bits(0x0000_0001).to_f32(), 2.0f32.powi(-24));
        assert_eq!(f16::from_bits(0x0000_0005).to_f32(), 5.0 * 2.0f32.powi(-24));

        assert_eq!(f16::from_bits(0x0000_0001), f16::from_f32(2.0f32.powi(-24)));
        assert_eq!(
            f16::from_bits(0x0000_0005),
            f16::from_f32(5.0 * 2.0f32.powi(-24))
        );

        assert_eq!(format!("{}", f16::from_f32(7.0)), "7".to_string());
        assert_eq!(format!("{:?}", f16::from_f32(7.0)), "7.0".to_string());
    }
}
