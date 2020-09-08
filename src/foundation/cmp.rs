use std::cmp::Ordering;

/// Constants that indicate sort order.
///
/// See [documentation](https://developer.apple.com/documentation/foundation/nscomparisonresult).
#[repr(isize)] // NSInteger
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum NSComparisonResult {
    /// The left operand is smaller than the right operand.
    OrderedAscending = -1,
    /// The operands are equal.
    OrderedSame = 0,
    /// The left operand is greater than the right operand.
    OrderedDescending = 1,
}

impl From<Ordering> for NSComparisonResult {
    #[inline]
    fn from(ordering: Ordering) -> Self {
        match ordering {
            Ordering::Less => Self::OrderedAscending,
            Ordering::Equal => Self::OrderedSame,
            Ordering::Greater => Self::OrderedDescending,
        }
    }
}

impl From<NSComparisonResult> for Ordering {
    #[inline]
    fn from(result: NSComparisonResult) -> Self {
        match result {
            NSComparisonResult::OrderedAscending => Self::Less,
            NSComparisonResult::OrderedSame => Self::Equal,
            NSComparisonResult::OrderedDescending => Self::Greater,
        }
    }
}

impl NSComparisonResult {
    /// Converts this comparison result into a Rust ordering.
    #[inline]
    pub fn into_ordering(self) -> Ordering {
        self.into()
    }
}
