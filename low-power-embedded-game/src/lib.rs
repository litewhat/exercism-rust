// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    (dividend.div_euclid(divisor), dividend.rem_euclid(divisor))
}

struct EvensIterator<I>
where
    I: Iterator,
{
    iter: I,
}

impl<I> EvensIterator<I>
where
    I: Iterator,
{
    pub fn new(iter: I) -> Self {
        Self { iter }
    }
}

impl<I> Iterator for EvensIterator<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let original = self.iter.next();
        self.iter.next();
        original
    }
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    EvensIterator::new(iter)
}

pub struct Position(pub i16, pub i16);

impl Position {
    pub fn manhattan(&self) -> i16 {
        let Self(x0, y0) = Self(0, 0);
        let Self(ref x1, ref y1) = self;
        (x1 - x0).abs() + (y1 - y0).abs()
    }
}
