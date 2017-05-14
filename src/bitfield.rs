use {Epoch, Index, StorageId};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PointerData(u64);

const INDEX_BITS: u8 = 40;
const EPOCH_BITS: u8 = 16;
const STORAGE_ID_BITS: u8 = 8;
const INDEX_MASK: u64 = (1 << INDEX_BITS) - 1;
const EPOCH_OFFSET: u8 = INDEX_BITS;
const EPOCH_MASK: u64 = ((1 << EPOCH_BITS) - 1) << EPOCH_OFFSET;
const STORAGE_ID_OFFSET: u8 = EPOCH_OFFSET + EPOCH_BITS;
const STORAGE_ID_MASK: u64 = ((1 << STORAGE_ID_BITS) - 1) << STORAGE_ID_OFFSET;

impl PointerData {
    #[inline]
    pub fn new(index: Index, epoch: Epoch, storage: StorageId) -> Self {
        debug_assert_eq!(index >> INDEX_BITS, 0);
        PointerData(index as u64 + ((epoch as u64) << EPOCH_OFFSET) +
            ((storage as u64) << STORAGE_ID_OFFSET))
    }

    #[inline]
    pub fn get_index(&self) -> Index {
        (self.0 & INDEX_MASK) as Index
    }

    #[inline]
    pub fn get_epoch(&self) -> Epoch {
        ((self.0 & EPOCH_MASK) >> EPOCH_OFFSET) as Epoch
    }

    #[inline]
    pub fn get_storage_id(&self) -> StorageId {
        ((self.0 & STORAGE_ID_MASK) >> STORAGE_ID_OFFSET) as StorageId
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn sizes() {
        assert_eq!(INDEX_BITS + EPOCH_BITS + STORAGE_ID_BITS, 64);
        assert!(size_of::<Index>() * 8 >= INDEX_BITS as usize);
        assert!(size_of::<Epoch>() * 8 >= EPOCH_BITS as usize);
        assert!(size_of::<StorageId>() * 8 >= STORAGE_ID_BITS as usize);
    }

    #[test]
    fn new() {
        let pd = PointerData::new(1, 2, 3);
        assert_eq!(pd.get_index(), 1);
        assert_eq!(pd.get_epoch(), 2);
        assert_eq!(pd.get_storage_id(), 3);
    }
}