use std::io::Read;

/// Types that have a known encoding size implement.
pub trait StaticSize {
    const SIZE: usize;
}

pub trait DynamicSize {
    fn size(&self) -> usize;
}

impl<T> DynamicSize for T
where
    T: StaticSize,
{
    fn size(&self) -> usize {
        Self::SIZE
    }
}