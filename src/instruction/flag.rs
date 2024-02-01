#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Flags {
    IsNop = 0,
    IsInteger = 1,
    IsFloating = 2,
    IsVector = 3,
    IsVectorElem = 4,
    IsMatrix = 5,
    IsLoad = 6,
    IsStore = 7,
    IsAtomic = 8,
    IsStoreConditional = 9,
    IsInstPrefetch = 10,
    IsDataPrefetch = 11,
    IsControl = 12,
    IsDirectControl = 13,
    IsIndirectControl = 14,
    IsCondControl = 15,
    IsUncondControl = 16,
    IsCall = 17,
    IsReturn = 18,
    IsSerializing = 19,
    IsSerializeBefore = 20,
    IsSerializeAfter = 21,
    IsWriteBarrier = 22,
    IsReadBarrier = 23,
    IsNonSpeculative = 24,
    IsQuiesce = 25,
    IsUnverifiable = 26,
    IsSyscall = 27,
    IsMacroop = 28,
    IsMicroop = 29,
    IsDelayedCommit = 30,
    IsLastMicroop = 31,
    IsFirstMicroop = 32,
    IsSquashAfter = 33,
    IsHtmStart = 34,
    IsHtmStop = 35,
    IsHtmCancel = 36,
    NumFlags = 37,
}

impl Flags {
    pub fn contains_flag(&self, flags: u64) -> bool {
        (flags & (1 << (*self as u64))) != 0
    }
}
