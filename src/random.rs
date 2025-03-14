use crate::Euui;
use rand::random;

impl Euui {
    /// Generates a new random Euui.
    ///
    /// Each component of the Euui is generated using the `rand` crate's `random` function.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use euui::Euui;
    ///
    /// #[cfg(feature = "random")]
    /// fn test_random() {
    ///     let euui = Euui::random();
    ///     println!("{}", euui);
    /// }
    /// ```
    pub fn random() -> Self {
        Self([random(), random(), random(), random()])
    }

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

    /// Generates a new random Euui with the first `u128` component provided
    /// and the remaining three components generated randomly.
    ///
    /// ## Arguments
    ///
    /// * `first` - The first `u128` value to initialize the Euui.
    ///
    /// ## Returns
    ///
    /// A new `Euui` instance where the first `u128` value is set to the provided value,
    /// and the other three components are randomly generated.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use euui::Euui;
    ///
    /// #[cfg(feature = "random")]
    /// fn test_random() {
    ///     let first = 0x1234567890abcdef1234567890abcdef;
    ///     let euui = Euui::random_from_first(first);
    ///
    ///     println!("{:?}", euui);
    ///     assert_eq!(euui.to_be_guids().first().unwrap(), &first);
    /// }
    /// ```
    pub fn random_from_first(first: u128) -> Self {
        Self([first, random(), random(), random()])
    }

    /// Generates a new random Euui with the second `u128` component provided
    /// and the remaining three components generated randomly.
    ///
    /// See [Self::random_from_first].
    pub fn random_from_second(second: u128) -> Self {
        Self([random(), second, random(), random()])
    }

    /// Generates a new random Euui with the third `u128` component provided
    /// and the remaining three components generated randomly.
    ///
    /// See [Self::random_from_first].
    pub fn random_from_third(third: u128) -> Self {
        Self([random(), random(), third, random()])
    }

    /// Generates a new random Euui with the fourth `u128` component provided
    /// and the remaining three components generated randomly.
    ///
    /// See [Self::random_from_first].
    pub fn random_from_fourth(fourth: u128) -> Self {
        Self([random(), random(), random(), fourth])
    }

    /// Generates a new `Euui` with a randomly generated first component,
    /// leaving the remaining components unchanged.
    pub fn regenerate_first(&self) -> Self {
        Self([random(), self.0[1], self.0[2], self.0[3]])
    }

    /// Generates a new `Euui` with a randomly generated second component,
    /// leaving the remaining components unchanged.
    pub fn regenerate_second(&self) -> Self {
        Self([self.0[0], random(), self.0[2], self.0[3]])
    }

    /// Generates a new `Euui` with a randomly generated third component,
    /// leaving the remaining components unchanged.
    pub fn regenerate_third(&self) -> Self {
        Self([self.0[0], self.0[1], random(), self.0[3]])
    }

    /// Generates a new `Euui` with a randomly generated fourth component,
    /// leaving the remaining components unchanged.
    pub fn regenerate_fourth(&self) -> Self {
        Self([self.0[0], self.0[1], self.0[2], random()])
    }
}
