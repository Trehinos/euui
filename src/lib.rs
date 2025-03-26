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
        for (i, byte) in bytes.iter_mut().enumerate() {
            *byte = self.u8(i).unwrap();
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
        for i in 0..8 {
            longs[i] = self.u64(i).unwrap();
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
}
