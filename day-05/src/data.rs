use core::slice;
use std::{
    array, iter,
    num::NonZeroU8,
    ops::{BitOr, BitOrAssign, Range},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Page(NonZeroU8);

impl TryFrom<u8> for Page {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value >= 100 {
            return Err(());
        }
        NonZeroU8::new(value).map(Page).ok_or(())
    }
}

impl FromStr for Page {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value: u8 = s.parse().map_err(|_| ())?;
        Self::try_from(value)
    }
}

impl Page {
    pub fn to_u8(self) -> u8 {
        self.0.get()
    }

    pub fn to_u32(self) -> u32 {
        self.0.get().into()
    }

    fn to_u128(self) -> u128 {
        self.0.get().into()
    }

    fn to_usize(self) -> usize {
        self.0.get().into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PageSet {
    bits: u128,
}

impl PageSet {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn contains(self, page: Page) -> bool {
        self.bits & (1 << page.to_u128()) != 0
    }

    pub fn insert(&mut self, page: Page) {
        self.bits |= 1 << page.to_u128();
    }
}

pub struct PageSetIter {
    set: PageSet,
    indices: Range<u8>,
}

impl IntoIterator for PageSet {
    type Item = Page;
    type IntoIter = PageSetIter;
    fn into_iter(self) -> Self::IntoIter {
        PageSetIter {
            set: self,
            indices: 1..100,
        }
    }
}

impl Iterator for PageSetIter {
    type Item = Page;

    fn next(&mut self) -> Option<Self::Item> {
        self.indices
            .by_ref()
            .map(|idx| Page::try_from(idx).unwrap())
            .find(|&page| self.set.contains(page))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(99))
    }
}

impl BitOr for PageSet {
    type Output = PageSet;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            bits: self.bits | rhs.bits,
        }
    }
}

impl BitOrAssign for PageSet {
    fn bitor_assign(&mut self, rhs: Self) {
        self.bits |= rhs.bits;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PageMap<T> {
    slots: [Option<T>; 100],
}

impl<T> Default for PageMap<T> {
    fn default() -> Self {
        Self {
            slots: array::from_fn(|_| None),
        }
    }
}

impl<T> PageMap<T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get(&self, page: Page) -> Option<&T> {
        self.slots[page.to_usize()].as_ref()
    }

    pub fn get_mut(&mut self, page: Page) -> Option<&mut T> {
        self.slots[page.to_usize()].as_mut()
    }

    pub fn insert(&mut self, page: Page, value: T) -> &mut T {
        self.slots[page.to_usize()].insert(value)
    }

    pub fn get_or_insert(&mut self, page: Page, value: T) -> &mut T {
        self.slots[page.to_usize()].get_or_insert(value)
    }

    pub fn remove(&mut self, page: Page) -> Option<T> {
        self.slots[page.to_usize()].take()
    }
}

pub struct PageMapIter<'map, T> {
    slots: iter::Enumerate<slice::Iter<'map, Option<T>>>,
}

impl<'map, T> IntoIterator for &'map PageMap<T> {
    type Item = (Page, &'map T);
    type IntoIter = PageMapIter<'map, T>;

    fn into_iter(self) -> Self::IntoIter {
        PageMapIter {
            slots: self.slots.iter().enumerate(),
        }
    }
}

impl<'map, T> Iterator for PageMapIter<'map, T> {
    type Item = (Page, &'map T);

    fn next(&mut self) -> Option<Self::Item> {
        self.slots.find_map(|(idx, value)| {
            let value = value.as_ref()?;
            Some((Page::try_from(idx as u8).unwrap(), value))
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(99))
    }
}
