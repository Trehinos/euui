# Euui - Extended Universal Unique Identifier

## Description

`Euui` is a Rust library (`no_std` compatible) that provides a 512-bit (64 bytes) unique identifier, which is **4 times
larger** than traditional UUIDs or GUIDs. This allows for enhanced uniqueness and adaptability in scenarios where more
significant identifiers are required.
The `Euui` type can be utilized for various applications, offering readable formats and flexible access to its
components (`u128`, `u64`, `u8`), making it a powerful alternative for unique identification in embedded or
resource-constrained environments.

### Key Features

- **Size**: A single `Euui` consists of 512 bits (64 bytes), making it exceptionally unique.
- **Formatting**:
    - A raw hexadecimal string representation of 128 characters.
    - A formatted string with 131 characters, including separators (`-`) and line breaks (`\n`).

- **Components Access**:
    - Retrieve identifiers as 4×`u128`, 8×`u64`, or 64×`u8`.

- **Generation**:
    - Create a zero-initialized `Euui` using `default()`.
    - Generate a random `Euui` using `random()`.

## Installation

Add the following dependency to your `Cargo.toml`:

```toml
[dependencies]
euui = "1.1.0"
rand = "0.9.0"
```

## Usage

Here are examples of how to use the `Euui` library:

### Creation and Basic Operations

```rust
use euui::Euui;

fn test() {
    // Generate a zero-initialized Euui
    let zero_euui = Euui::default();
    println!("Zero Euui: {}", zero_euui);

    // Generate a random Euui
    let random_euui = Euui::random();
    println!("Random Euui: {}", random_euui);

    // Format the Euui into a readable structure
    println!("Formatted Euui:\n{}", random_euui.format());
}
```

### Accessing Parts of an Euui

You can retrieve specific components (`u128`, `u64`, or `u8`) of the `Euui` as needed:

```rust
// Access one of the u128 components

fn test() {
    if let Some(first_u128) = random_euui.u128(0) {
        println!("First u128: {:032x}", first_u128);
    }

    // Access one of the u64 components
    if let Some(second_u64) = random_euui.u64(1) {
        println!("Second u64: {:016x}", second_u64);
    }

    // Access one of the u8 components
    if let Some(last_u8) = random_euui.u8(63) {
        println!("Last u8: {:02x}", last_u8);
    }
}
```

### Advanced Initialization

You can initialize an `Euui` using custom GUIDs or bytes:

#### From GUIDs (`u128` array)

```rust
pub fn test() {
    let guids = [
        0x1234567890abcdef1234567890abcdef,
        0xabcdef1234567890abcdef1234567890,
        0x7890abcdef1234567890abcdef123456,
        0x567890abcdef1234567890abcdef1234,
    ];
    let euui = Euui::from_be_guids(guids);
}
```

#### From Bytes (`u8` array)

```rust
pub fn test() {
    let bytes = [0x12, 0x34, /* 61 other bytes... */ 0xef];
    let euui = Euui::from_be_bytes(bytes);
}
```

## API Overview

The main functionalities of the `Euui` type are:

### Creation Methods

- **`Euui::default()`**
  Creates a zero-initialized `Euui`.
- **`Euui::random()`**
  Generates a random `Euui`.
- **`Euui::from_be_guids([u128; 4])`**
  Initializes an `Euui` from an array of 4×`u128`.
- **`Euui::from_be_bytes([u8; 64])`**
  Initializes an `Euui` from a 64-byte array.
- **`Euui::random_from_first(u128)`**
  Generates a new random Euui with the first `u128` component provided.
- **`Euui::random_from_second(u128)`**
  Generates a new random Euui with the second `u128` component provided.
- **`Euui::random_from_third(u128)`**
  Generates a new random Euui with the third `u128` component provided.
- **`Euui::random_from_fourth(u128)`**
  Generates a new random Euui with the fourth `u128` component provided.
- **`Euui::regenerate_first(&self)`**
  Generates a new `Euui` with a randomly generated first component.
- **`Euui::regenerate_second(&self)`**
  Generates a new `Euui` with a randomly generated second component.
- **`Euui::regenerate_third(&self)`**
  Generates a new `Euui` with a randomly generated third component.
- **`Euui::regenerate_fourth(&self)`**
  Generates a new `Euui` with a randomly generated fourth component.

### Accessor Methods

- **`u128(index: usize) -> Option<u128>`**
  Retrieve a specific `u128` component. Index must be in the range `[0, 3]`.
- **`u64(index: usize) -> Option<u64>`**
  Retrieve a specific `u64` component. Index must be in the range `[0, 7]`.
- **`u8(index: usize) -> Option<u8>`**
  Retrieve a specific `u8` component. Index must be in the range `[0, 63]`.
- **`to_be_bytes() -> [u8; 64]`**
  Retrieve the entire `Euui` as an array of 64 bytes.
- **`to_be_guids() -> [u128; 4]`**
  Retrieve the entire `Euui` as an array of 4×`u128`.

### Display Methods

- **`to_string()`**
  Converts the `Euui` to a single hexadecimal string representation.
- **`format() -> String`**
  Formats the `Euui` into a structured string, following the pattern:
  #1-#2
  #3-#4.

## Use Cases : Large-Scale Unique ID Generation

With 512 bits of entropy, `Euui` can be useful for applications where traditional 128-bit UUIDs are insufficient to
guarantee uniqueness:

- Distributed systems,
- Cryptographic key identifiers,
- Unique identifiers in high-throughput environments,
- Large enough to have UUID/GUID parts.

## License

&copy; 2024-2025 [Sébastien GELDREICH](mailto:dev@trehinos.eu)  
This project is licensed under the MIT License. See [LICENSE]() for more details.