use crate::Set;

use std::ops::Fn;

/// A trait definition for a map between two sets
pub trait Map<SrcKeyType, SrcElementType, SrcSizeType, SrcSetType, DstKeyType, DstElementType, DstSizeType, DstSetType> : Fn(SrcElementType) -> DstElementType
where
    SrcSetType: Set<SrcKeyType, SrcElementType, SrcSizeType>,
    DstSetType: Set<DstKeyType, DstElementType, DstSizeType>
{
}
