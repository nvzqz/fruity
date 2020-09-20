use std::cmp::Ordering;

/// Constants returned by comparison functions, indicating sort order.
///
/// See [documentation](https://developer.apple.com/documentation/corefoundation/cfcomparisonresult).
#[repr(isize)] // NSIndex
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum CFComparisonResult {
    /// The first value is less than the second value.
    LessThan = -1,
    /// The first value is equal to the second value.
    EqualTo = 0,
    /// The first value is greater than the second value.
    GreaterThan = 1,
}

impl From<Ordering> for CFComparisonResult {
    #[inline]
    fn from(ordering: Ordering) -> Self {
        match ordering {
            Ordering::Less => Self::LessThan,
            Ordering::Equal => Self::EqualTo,
            Ordering::Greater => Self::GreaterThan,
        }
    }
}

impl From<CFComparisonResult> for Ordering {
    #[inline]
    fn from(result: CFComparisonResult) -> Self {
        match result {
            CFComparisonResult::LessThan => Self::Less,
            CFComparisonResult::EqualTo => Self::Equal,
            CFComparisonResult::GreaterThan => Self::Greater,
        }
    }
}

impl CFComparisonResult {
    /// Converts this comparison result into a Rust ordering.
    #[inline]
    pub fn into_ordering(self) -> Ordering {
        self.into()
    }
}
