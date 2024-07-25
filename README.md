This crate provides the `Euui` struct.

This type represents a unique identifier which is **4 times bigger** than UUIDs and GUIDs.  
So, an EUUI is **512 bits** or 64 bytes. It can be read as 4x`u128`, 8x`u64` or 64x`u8`.

A raw hexadecimal string representing an EUUI is 128 characters wide.  
A formatted hexadecimal string representing an EUUI is 131 characters wide (two "-" and one "\n").

You can create :

- a zero Euui with `Euui::default`,
- or, a random one with `Euui::random`.

Then, use :

- `Euui::format` to display it as 4 u128s or `Euui::.to_string` to get the whole hexadecimal string,
- or, `Euui::u128` or `Euui::to_be_guids` to reach for individual u128s,
- or, `Euui::u64` to reach for individual u64s,
- or, `Euui::u8` or `Euui::to_be_bytes` to reach for individual u8s.

## An example

### ::format()

 ```txt
 2f8596cc2f3b3da9adf20cf9413104ab-1f8de1116aef039d12c80587e7551080
 d43ed7632e94801a395a5454a382dff1-23decf62d51eafee3ec0bb98b1b90d15
 ```

### ::to_string()

 ```txt
 2f8596cc2f3b3da9adf20cf9413104ab1f8de1116aef039d12c80587e7551080d43ed7632e94801a395a5454a382dff123decf62d51eafee3ec0bb98b1b90d15
 ```
