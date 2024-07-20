//! This crates provides the [Euui] struct.
//!
//! This struct holds a unique identifier which is **4 times bigger** than UUIDs and GUIDs. 
//! So, an EUUI is **512 bits** or 64 bytes. It can be read as 4x`u128`, 8x`u64` or 64x`u8`.
//! 
//! A raw hexadecimal string representing an EUUI is 128 characters wide.  
//! A formatted hexadecimal string representing an EUUI is 131 characters wide (two "-" and one "\n").
//!
//! You can create :
//!  - a zero Euui with [Euui::default],
//!  - or, a random one with [Euui::random].
//!
//! Then, use :
//!  - [Euui::format] to display it as 4 u128s or `.to_string()` to get the whole hexadecimal string,
//!  - or, [Euui::u128] to reach for individual u128s,
//!  - or, [Euui::u64] to reach for individual u64s,
//!  - or, [Euui::u8] or [Euui::to_be_bytes] to reach for individual u8s.
//!
//!
//!
//! ## An example
//! ### ::format()
//! ```txt
//! 2f8596cc2f3b3da9adf20cf9413104ab-1f8de1116aef039d12c80587e7551080
//! d43ed7632e94801a395a5454a382dff1-23decf62d51eafee3ec0bb98b1b90d15
//! ```
//!
//! ### ::to_string()
//! ```txt
//! 2f8596cc2f3b3da9adf20cf9413104ab1f8de1116aef039d12c80587e7551080d43ed7632e94801a395a5454a382dff123decf62d51eafee3ec0bb98b1b90d15
//! ```
//!

use std::fmt::{Display, Formatter};
use rand::random;

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
#[derive(Copy, Clone, Default, Eq, PartialEq, Debug)]
pub struct Euui([u128; 4]);

impl Euui {
    /// Provides a new random 512-bits Euui.
    pub fn random() -> Self {
        Self([random(), random(), random(), random()])
    }

    /// Gets one of the 4 u128 composing this Euui.
    ///
    /// Returns [None] if index >= 4.
    pub fn u128(&self, index: usize) -> Option<u128> {
        if index >= self.0.len() { None } else { Some(self.0[index]) }
    }

    /// Gets one of the 64 bytes composing this Euui.
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

    /// Returns the 64 bytes composing this Euui.
    pub fn to_be_bytes(&self) -> [u8; 64] {
        let mut bytes = [0u8; 64];
        for i in 0..64 {
            bytes[i] = self.u8(i).unwrap();
        }
        bytes
    }

    /// Gets one of the 8 u64 composing this Euui.
    ///
    /// Returns [None] if index >= 8.
    pub fn u64(&self, index: usize) -> Option<u64> {
        if index >= self.0.len() * 2 { None } else {
            let start = index * 8;
            let end = start + 8;
            Some(u64::from_be_bytes(self.to_be_bytes()[start..end].try_into().expect("Logic error")))
        }
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:032x}{:032x}{:032x}{:032x}",
            self.0[0], self.0[1], self.0[2], self.0[3]
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::Euui;

    #[test]
    fn test_zero() {
        let euui = Euui::default();
        assert_eq!(
            euui.to_string(),
            "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
        );
    }

    #[test]
    fn test_non_zero() {
        let euui = Euui::random();
        assert_ne!(
            euui.to_string(),
            "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
        );
    }

    #[test]
    fn format_random() {
        let euui = Euui::random();
        println!("{}\n\n{}", euui.format(), euui);
    }
}