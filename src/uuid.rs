use crate::Euui;
use uuid::Uuid;

impl Euui {
    /// Modifies a specific part of the internal GUID array with the provided UUID.
    ///
    /// # Arguments
    ///
    /// * `uuid` - The `Uuid` instance to be inserted in the specified part of the array.
    /// * `part` - The index of the GUID array to update (must be between 0 and 3).
    ///
    /// # Returns
    ///
    /// A new instance of `Euui` with the updated GUID array.
    ///
    /// # Panics
    ///
    /// This function will panic if the provided `part` index is greater than 3.
    pub fn with_uuid_part(&self, uuid: Uuid, part: usize) -> Self {
        if part > 3 {
            panic!("Index out of bounds");
        }
        let mut guids = self.0;
        guids[part] = uuid.as_u128();
        Self(guids)
    }

    /// Shorthand method to update the first part of the GUID array with the provided UUID.
    ///
    /// # Arguments
    ///
    /// * `uuid` - The `Uuid` instance to be inserted as the first part of the GUID array.
    ///
    /// # Returns
    ///
    /// A new instance of `Euui` with the updated first part of the GUID array.
    pub fn with_first(&self, uuid: Uuid) -> Self {
        self.with_uuid_part(uuid, 0)
    }

    /// Shorthand method to update the second part of the GUID array with the provided UUID.
    ///
    /// # Arguments
    ///
    /// * `uuid` - The `Uuid` instance to be inserted as the second part of the GUID array.
    ///
    /// # Returns
    ///
    /// A new instance of `Euui` with the updated second part of the GUID array.
    pub fn with_second(&self, uuid: Uuid) -> Self {
        self.with_uuid_part(uuid, 1)
    }

    /// Shorthand method to update the third part of the GUID array with the provided UUID.
    ///
    /// # Arguments
    ///
    /// * `uuid` - The `Uuid` instance to be inserted as the third part of the GUID array.
    ///
    /// # Returns
    ///
    /// A new instance of `Euui` with the updated third part of the GUID array.
    pub fn with_third(&self, uuid: Uuid) -> Self {
        self.with_uuid_part(uuid, 2)
    }

    /// Shorthand method to update the fourth part of the GUID array with the provided UUID.
    ///
    /// # Arguments
    ///
    /// * `uuid` - The `Uuid` instance to be inserted as the fourth part of the GUID array.
    ///
    /// # Returns
    ///
    /// A new instance of `Euui` with the updated fourth part of the GUID array.
    pub fn with_fourth(&self, uuid: Uuid) -> Self {
        self.with_uuid_part(uuid, 3)
    }

    /// Creates a new `Euui` instance from an array of 4 `Uuid` values.
    ///
    /// # Arguments
    ///
    /// * `uuids` - An array containing four `Uuid` instances to initialize the internal GUID array.
    ///
    /// # Returns
    ///
    /// A new instance of `Euui` with the GUID array populated using the provided `Uuid` values.
    pub fn from_uuids(uuids: [Uuid; 4]) -> Self {
        Self([
            uuids[0].as_u128(),
            uuids[1].as_u128(),
            uuids[2].as_u128(),
            uuids[3].as_u128(),
        ])
    }

    /// Generates a new `Euui` instance with an array of random `Uuid/v4` values.
    ///
    /// # Returns
    ///
    /// A new `Euui` instance with all four parts of the internal GUID array
    /// populated using randomly generated `Uuid/v4` values.
    #[cfg(feature = "random_uuid")]
    pub fn random_uuids() -> Self {
        Self::from_uuids([
            Uuid::new_v4(),
            Uuid::new_v4(),
            Uuid::new_v4(),
            Uuid::new_v4(),
        ])
    }

    /// Retrieves a `Uuid` instance from the internal GUID array at the specified index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the GUID array to retrieve (must be between 0 and 3).
    ///
    /// # Returns
    ///
    /// An `Option<Uuid>` instance:
    /// - `Some(Uuid)` if the `index` is within bounds and a GUID exists at that index.
    /// - `None` if the `index` is out of bounds or the GUID cannot be retrieved.
    pub fn uuid(&self, index: usize) -> Option<Uuid> {
        self.u128(index).map(Uuid::from_u128)
    }
}
