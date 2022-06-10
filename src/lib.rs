use std::num::NonZeroUsize;
use vec_map::VecMap;

#[derive(Debug)]
pub struct IntMap<T> {
    indices: Vec<Option<NonZeroUsize>>,
    values: Vec<T>,
}

pub enum DynamicMap<T> {
    VecMap(VecMap<T>),
    IntMap(IntMap<T>),
}

impl<T> IntMap<T> {
    pub fn new() -> Self {
        Self {
            indices: vec![],
            values: vec![],
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let mut map = Self::new();
        map.expand_to(capacity);
        map
    }

    /// Expand the map so that it can store keys up to `len - 1`.
    fn expand_to(&mut self, len: usize) {
        self.indices.resize_with(len, Option::default);
    }

    pub fn insert(&mut self, key: usize, value: T) -> Option<T> {
        let index = match self.indices.get_mut(key) {
            Some(index) => index,
            None => {
                self.expand_to(key + 1);
                self.indices.get_mut(key)?
            }
        };

        match index {
            Some(index) => {
                let vindex = index.get() - 1;
                let value_mut = self.values.get_mut(vindex)?;
                Some(std::mem::replace(value_mut, value))
            }
            None => {
                // Store actual index in vector +1. If we push first, we get the correct result.
                self.values.push(value);
                *index = NonZeroUsize::new(self.values.len());
                None
            }
        }
    }

    pub fn get(&self, key: usize) -> Option<&T> {
        let index = self.indices.get(key)?.as_ref()?;
        let vindex = index.get() - 1;
        self.values.get(vindex)
    }
}

impl<T> DynamicMap<T> {
    pub fn new_vec_map(n: usize) -> Self {
        DynamicMap::VecMap(VecMap::with_capacity(n))
    }

    pub fn new_int_map(n: usize) -> Self {
        DynamicMap::IntMap(IntMap::with_capacity(n))
    }

    #[inline]
    pub fn insert(&mut self, key: usize, value: T) -> Option<T> {
        match self {
            DynamicMap::VecMap(map) => map.insert(key, value),
            DynamicMap::IntMap(map) => map.insert(key, value),
        }
    }

    #[inline]
    pub fn get(&self, key: usize) -> Option<&T> {
        match self {
            DynamicMap::VecMap(map) => map.get(key),
            DynamicMap::IntMap(map) => map.get(key),
        }
    }
}
