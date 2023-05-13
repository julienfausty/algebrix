/// Trait for implementing structures representing sets
pub trait Set<KeyType, ElementType, SizeType = usize> {
    type SubsetType: Set<KeyType, ElementType, SizeType>;

    /// Get the size of the set
    fn get_cardinality(&self) -> SizeType;
    /// Get an element of the set
    fn get_element(&self, key: KeyType) -> ElementType;
    /// Get a subset of the set
    fn get_subset(&self, keys: &[KeyType]) -> Self::SubsetType;
}
