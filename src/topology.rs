use crate::Set;

///Trait for defining a topology on a set
pub trait Topology<KeyType, SizeType, SetKeyType, SetElementType, SetSizeType, SetType>:
    Set<KeyType, SetType::SubsetType, SizeType>
where
    SetType: Set<SetKeyType, SetElementType, SetSizeType>,
{
    type BasisType: Set<KeyType, SetType::SubsetType, SizeType>;
    /// Get a minimal set of subsets with which one is able to reconstruct the entire topology
    /// through unions and intersections
    fn get_basis(&self) -> Self::BasisType;
}
