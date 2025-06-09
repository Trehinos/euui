//! This crate provides the [Euui] struct.
//!
//! This type represents a unique identifier which is **4 times bigger** than UUIDs and GUIDs.   
//! So, an EUUI is **512 bits** or 64 bytes. It can be read as 4x`u128`, 8x`u64` or 64x`u8`.
//!
//! A raw hexadecimal string representing an EUUI is 128 characters wide.  
//! A formatted hexadecimal string representing an EUUI is 131 characters wide (two "-" and one "\n").
//!
//! You can create :
//!  - a zero Euui with [Euui::default],
//!
//! Then, use :
//!  - [Euui::format] to display it as 4 u128s or `.to_string()` to get the whole hexadecimal string,
//!  - or, [Euui::u128] or [Euui::to_be_guids] to reach for individual u128s,
//!  - or, [Euui::u64] to reach for individual u64s,
//!  - or, [Euui::u8] or [Euui::to_be_bytes] to reach for individual u8s.
//!
//! ## Example
//!
//! ```rust
//! use euui::Euui;
//!
//! // Generate a zero-initialized Euui
//! let zero_euui = Euui::default(); // or Euui::zero()
//! println!("Zero Euui: {}", zero_euui);
//!
//! // Generate a random Euui
//! #[cfg(feature = "random")]
//! fn test_random() {
//!     let random_euui = Euui::random();
//!     println!("Random Euui: {}", random_euui);
//!
//!     // Format a Euui
//!     let formatted = random_euui.format();
//!     println!("Formatted Euui:\n{}", formatted);
//!
//!     // Access specific parts of the Euui
//!     if let Some(first_u128) = random_euui.u128(0) {
//!         println!("First u128 of random Euui: {:032x}", first_u128);
//!     }
//! }
//! ```
//!
//! ### Features
//!
//! The features of this crate add methods to the type [Euui].
//!
//! #### With the feature `random`
//!
//! - [Euui::random]
//! - [Euui::random_from_first]
//! - [Euui::random_from_second]
//! - [Euui::random_from_third]
//! - [Euui::random_from_fourth]
//! - [Euui::regenerate_first]
//! - [Euui::regenerate_second]
//! - [Euui::regenerate_third]
//! - [Euui::regenerate_fourth]
//!
//! #### With the feature `uuid`
//!
//! - [Euui::from_uuids]
//! - [Euui::with_uuid_part]
//! - [Euui::with_first]
//! - [Euui::with_second]
//! - [Euui::with_third]
//! - [Euui::with_fourth]
//! - [Euui::from_be_bytes]
//! - [Euui::uuid]
//!
//! #### With the feature `random_uuid`
//!
//! - [Euui::random_uuids]

#![no_std]
extern crate alloc;

use alloc::format;
use alloc::string::String;
use core::fmt::{Display, Formatter};

/// Extended Universal Unique Identifier
///
/// A 512-bits (64 bytes) identifier.
///
/// A formatted Euui follows this pattern (given #x is `self.0[x - 1]: u128`) :
/// ```txt
/// #1-#2
/// #3-#4
/// ```
///
/// The LF character (ASCII 0x0A) is used for new lines.
#[derive(Copy, Clone, Default, Eq, PartialEq, Debug, PartialOrd, Ord, Hash)]
pub struct Euui([u128; 4]);

impl Euui {
    /// Returns a zero-initialized `Euui`.
    ///
    /// ## Description
    ///
    /// This function generates a `Euui` instance with all components
    /// initialized to zero, effectively creating a blank or default `Euui`.
    ///
    /// ## Returns
    ///
    /// A `Euui` instance with all components set to `0`.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use euui::Euui;
    ///
    /// let zero_euui = Euui::zero();
    /// println!("{:?}", zero_euui); // Outputs: Euui([0, 0, 0, 0])
    /// ```
    pub fn zero() -> Self {
        Self([0, 0, 0, 0])
    }

    /// Creates a new Euui from a provided array of 4 big-endian `u128` GUIDs.
    ///
    /// ## Arguments
    ///
    /// * `guids` - An array of 4 `u128` values to initialize the Euui.
    ///
    /// ## Returns
    ///
    /// A new `Euui` instance containing the given GUIDs in big-endian format.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use euui::Euui;
    ///
    /// let guids = [
    ///     0x1234567890abcdef1234567890abcdef,
    ///     0xabcdef1234567890abcdef1234567890,
    ///     0x7890abcdef1234567890abcdef123456,
    ///     0x567890abcdef1234567890abcdef1234,
    /// ];
    /// let euui = Euui::from_be_guids(guids);
    ///
    /// assert_eq!(euui.to_be_guids(), guids);
    /// ```
    pub fn from_be_guids(guids: [u128; 4]) -> Self {
        Self(guids)
    }

    /// Creates a new Euui from a provided array of 64 big-endian bytes.
    ///
    /// ## Arguments
    ///
    /// * `bytes` - An array of 64 bytes to initialize the Euui.
    ///             Each 16-byte segment in the array is treated as a single `u128`
    ///             in big-endian format, resulting in a total of 4 `u128` values.
    ///
    /// ## Returns
    ///
    /// A new `Euui` instance containing the given bytes as 4 big-endian `u128` values.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use euui::Euui;
    ///
    /// let bytes = [
    ///     0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF,
    ///     0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF,
    ///     0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90,
    ///     0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90,
    ///     0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56,
    ///     0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56,
    ///     0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34,
    ///     0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34,
    /// ];
    /// let euui = Euui::from_be_bytes(bytes);
    ///
    /// assert_eq!(euui.to_be_bytes(), bytes);
    /// ```
    pub fn from_be_bytes(bytes: [u8; 64]) -> Self {
        let mut guids = [0u128; 4];
        for i in 0..4 {
            guids[i] =
                u128::from_be_bytes(bytes[i * 16..(i + 1) * 16].try_into().expect("Logic error"));
        }
        Self(guids)
    }

    /// Creates a new Euui from a provided array of 8 big-endian `u64` values.
    ///
    /// ## Arguments
    ///
    /// * `bytes` - An array of 8 `u64` values to initialize the Euui.
    ///             Each pair of `u64` values is concatenated to form a single `u128` value
    ///             in big-endian order, resulting in a total of 4 `u128` values.
    ///
    /// ## Returns
    ///
    /// A new `Euui` instance containing the given `u64` values as 4 big-endian `u128` values.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use euui::Euui;
    ///
    /// let longs = [
    ///     0x1234567890abcdef, 0xfedcba0987654321,
    ///     0xabcdef1234567890, 0x1234567890abcdef,
    ///     0x0987654321abcdef, 0xabcdef9876543210,
    ///     0x567890abcdef1234, 0x4321fedcba987654,
    /// ];
    /// let euui = Euui::from_be_longs(longs);
    ///
    /// assert_eq!(euui.u128(0).unwrap(), 0x1234567890abcdeffedcba0987654321);
    /// assert_eq!(euui.u128(1).unwrap(), 0xabcdef12345678901234567890abcdef);
    /// assert_eq!(euui.u128(2).unwrap(), 0x0987654321abcdefabcdef9876543210);
    /// assert_eq!(euui.u128(3).unwrap(), 0x567890abcdef12344321fedcba987654);
    /// ```
    pub fn from_be_longs(bytes: [u64; 8]) -> Self {
        let mut guids = [0u128; 4];
        for i in 0..4 {
            let long_a = bytes[i * 2].to_be_bytes();
            let long_b = bytes[i * 2 + 1].to_be_bytes();
            let mut buf = [0u8; 16];
            buf[..8].copy_from_slice(&long_a);
            buf[8..].copy_from_slice(&long_b);
            guids[i] = u128::from_be_bytes(buf);
        }
        Self(guids)
    }

    /// Gets one of the 4 u128s composing this Euui.
    ///
    /// Returns [None] if index >= 4.
    pub fn u128(&self, index: usize) -> Option<u128> {
        if index >= self.0.len() {
            None
        } else {
            Some(self.0[index])
        }
    }

    /// Gets one of the 8 u64s composing this Euui.
    ///
    /// Returns [None] if index >= 8.
    pub fn u64(&self, index: usize) -> Option<u64> {
        if index >= self.0.len() * 2 {
            None
        } else {
            let start = index * 8;
            let end = start + 8;
            Some(u64::from_be_bytes(
                self.to_be_bytes()[start..end]
                    .try_into()
                    .expect("Logic error"),
            ))
        }
    }

    /// Gets one of the 64 u8s composing this Euui.
    ///
    /// Returns [None] if index >= 64.
    pub fn u8(&self, index: usize) -> Option<u8> {
        if index >= self.0.len() * 16 {
            None
        } else {
            let wide_index = index / 16;
            let byte_index = index % 16;
            Some(self.u128(wide_index).unwrap().to_be_bytes()[byte_index])
        }
    }

    /// Returns the 64 u8s composing this Euui.
    pub fn to_be_bytes(&self) -> [u8; 64] {
        let mut bytes = [0u8; 64];
        for (i, guid) in self.0.iter().enumerate() {
            let part = guid.to_be_bytes();
            bytes[i * 16..(i + 1) * 16].copy_from_slice(&part);
        }
        bytes
    }

    /// Returns the 8 u64s that represent this Euui in big-endian order.
    ///
    /// ## Returns
    ///
    /// A fixed-size array of 8 `u64` values, where each value corresponds
    /// to an 8-byte segment of the 64 bytes composing this Euui.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use euui::Euui;
    ///
    /// let bytes = [0u8; 64]; // Example input
    /// let euui = Euui::from_be_bytes(bytes);
    /// let longs = euui.to_be_longs();
    ///
    /// assert_eq!(longs, [0u64; 8]);
    /// ```
    pub fn to_be_longs(&self) -> [u64; 8] {
        let mut longs = [0u64; 8];
        for (i, guid) in self.0.iter().enumerate() {
            let bytes = guid.to_be_bytes();
            longs[i * 2] = u64::from_be_bytes(bytes[0..8].try_into().expect("Logic error"));
            longs[i * 2 + 1] = u64::from_be_bytes(bytes[8..16].try_into().expect("Logic error"));
        }
        longs
    }

    /// Returns the 4 u128s composing this Euui.
    ///
    pub fn to_be_guids(&self) -> [u128; 4] {
        self.0
    }

    /// Returns a hexadecimal formatted Euui which follows this pattern (given #x is `self.0[x - 1]`) :
    /// ```txt
    /// #1-#2
    /// #3-#4
    /// ```
    ///
    /// ## Example
    ///
    /// ```txt
    /// 2f8596cc2f3b3da9adf20cf9413104ab-1f8de1116aef039d12c80587e7551080
    /// d43ed7632e94801a395a5454a382dff1-23decf62d51eafee3ec0bb98b1b90d15
    /// ```
    pub fn format(&self) -> String {
        format!(
            "{:032x}-{:032x}\n{:032x}-{:032x}",
            self.0[0], self.0[1], self.0[2], self.0[3]
        )
    }
}

impl Display for Euui {
    /// Returns a hexadecimal Euui in one block. It follows this pattern (given #x is `self.0[x - 1]`) :
    /// ```txt
    /// #1#2#3#4
    /// ```
    ///
    /// ## Example
    ///
    /// ```txt
    /// 2f8596cc2f3b3da9adf20cf9413104ab1f8de1116aef039d12c80587e7551080d43ed7632e94801a395a5454a382dff123decf62d51eafee3ec0bb98b1b90d15
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{:032x}{:032x}{:032x}{:032x}",
            self.0[0], self.0[1], self.0[2], self.0[3]
        )
    }
}

#[cfg(feature = "random")]
mod random;

#[cfg(feature = "uuid")]
mod uuid;

#[cfg(test)]
mod tests {
    use crate::Euui;
    use alloc::string::ToString;
    use alloc::format;
    #[cfg(feature = "uuid")]
    use uuid::Uuid;

    #[test]
    fn test_zero() {
        let euui = Euui::default();
        assert_eq!(
            euui.to_string(),
            "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
        );
    }

    #[test]
    #[cfg(feature = "random")]
    fn test_non_zero() {
        let euui = Euui::random();
        assert_ne!(
            euui.to_string(),
            "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
        );
    }

    #[test]
    fn test_from_be_guids() {
        let guids = [1u128, 2, 3, 4];
        let euui = Euui::from_be_guids(guids);
        assert_eq!(euui.to_be_guids(), guids);
    }

    #[test]
    fn test_from_be_bytes() {
        let mut bytes = [0u8; 64];
        for i in 0..64 {
            bytes[i] = i as u8;
        }
        let euui = Euui::from_be_bytes(bytes);
        assert_eq!(euui.to_be_bytes(), bytes);
    }

    #[test]
    fn test_from_be_longs() {
        let longs = [1u64, 2, 3, 4, 5, 6, 7, 8];
        let euui = Euui::from_be_longs(longs);
        assert_eq!(euui.to_be_longs(), longs);
    }

    #[test]
    fn test_accessors() {
        let mut bytes = [0u8; 64];
        for i in 0..64 {
            bytes[i] = i as u8;
        }
        let euui = Euui::from_be_bytes(bytes);
        for i in 0..4 {
            assert_eq!(euui.u128(i).unwrap(), u128::from_be_bytes(bytes[i * 16..(i + 1) * 16].try_into().unwrap()));
        }
        assert!(euui.u128(4).is_none());
        for i in 0..8 {
            let start = i * 8;
            let end = start + 8;
            assert_eq!(euui.u64(i).unwrap(), u64::from_be_bytes(bytes[start..end].try_into().unwrap()));
        }
        assert!(euui.u64(8).is_none());
        for i in 0..64 {
            assert_eq!(euui.u8(i).unwrap(), bytes[i]);
        }
        assert!(euui.u8(64).is_none());
    }

    #[test]
    fn test_format_and_display() {
        let guids = [1u128, 2, 3, 4];
        let euui = Euui::from_be_guids(guids);
        let formatted = euui.format();
        assert_eq!(
            formatted,
            format!(
                "{:032x}-{:032x}\n{:032x}-{:032x}",
                guids[0], guids[1], guids[2], guids[3]
            )
        );
        let display = euui.to_string();
        assert_eq!(
            display,
            format!("{:032x}{:032x}{:032x}{:032x}", guids[0], guids[1], guids[2], guids[3])
        );
    }

    #[test]
    #[cfg(feature = "random")]
    fn test_random_from_parts() {
        let first = 0x1u128;
        let second = 0x2u128;
        let third = 0x3u128;
        let fourth = 0x4u128;
        let e1 = Euui::random_from_first(first);
        assert_eq!(e1.u128(0).unwrap(), first);
        let e2 = Euui::random_from_second(second);
        assert_eq!(e2.u128(1).unwrap(), second);
        let e3 = Euui::random_from_third(third);
        assert_eq!(e3.u128(2).unwrap(), third);
        let e4 = Euui::random_from_fourth(fourth);
        assert_eq!(e4.u128(3).unwrap(), fourth);
    }

    #[test]
    #[cfg(feature = "random")]
    fn test_regenerate_parts() {
        let euui = Euui::random();
        let r1 = euui.regenerate_first();
        assert_ne!(r1.u128(0), euui.u128(0));
        assert_eq!(r1.u128(1), euui.u128(1));
        assert_eq!(r1.u128(2), euui.u128(2));
        assert_eq!(r1.u128(3), euui.u128(3));

        let r2 = euui.regenerate_second();
        assert_ne!(r2.u128(1), euui.u128(1));
        assert_eq!(r2.u128(0), euui.u128(0));
        assert_eq!(r2.u128(2), euui.u128(2));
        assert_eq!(r2.u128(3), euui.u128(3));

        let r3 = euui.regenerate_third();
        assert_ne!(r3.u128(2), euui.u128(2));
        assert_eq!(r3.u128(0), euui.u128(0));
        assert_eq!(r3.u128(1), euui.u128(1));
        assert_eq!(r3.u128(3), euui.u128(3));

        let r4 = euui.regenerate_fourth();
        assert_ne!(r4.u128(3), euui.u128(3));
        assert_eq!(r4.u128(0), euui.u128(0));
        assert_eq!(r4.u128(1), euui.u128(1));
        assert_eq!(r4.u128(2), euui.u128(2));
    }

    #[test]
    #[cfg(feature = "uuid")]
    fn test_uuid_functions() {
        let base = Euui::zero();
        let uuids = [
            Uuid::from_u128(1),
            Uuid::from_u128(2),
            Uuid::from_u128(3),
            Uuid::from_u128(4),
        ];
        let with_part = base.with_uuid_part(uuids[2], 2);
        assert_eq!(with_part.u128(2).unwrap(), uuids[2].as_u128());

        let f = base.with_first(uuids[0]);
        assert_eq!(f.u128(0).unwrap(), uuids[0].as_u128());
        let s = base.with_second(uuids[1]);
        assert_eq!(s.u128(1).unwrap(), uuids[1].as_u128());
        let t = base.with_third(uuids[2]);
        assert_eq!(t.u128(2).unwrap(), uuids[2].as_u128());
        let fo = base.with_fourth(uuids[3]);
        assert_eq!(fo.u128(3).unwrap(), uuids[3].as_u128());

        let from = Euui::from_uuids(uuids);
        assert_eq!(from.to_be_guids(), [1u128, 2, 3, 4]);

        #[cfg(feature = "random_uuid")]
        {
            let r = Euui::random_uuids();
            let bytes = r.to_be_bytes();
            assert_ne!(bytes, [0u8; 64]);
        }

        for i in 0..4 {
            assert_eq!(from.uuid(i).unwrap().as_u128(), uuids[i].as_u128());
        }
        assert!(from.uuid(4).is_none());
    }
}
