use std::marker::PhantomData;

pub trait VecEntryMethods<'a, K, V>: Sized {
    fn key(&self) -> &K;

    fn take(self) -> (VacantVecEntry<'a, K, V>, Option<V>);

    fn insert_entry(self, value: V) -> OccupiedVecEntry<'a, K, V>;

    fn and_modify<F>(self, closure: F) -> Self
    where
        F: FnOnce(&mut V);

    fn or_insert_with_key<F>(self, closure: F) -> &'a mut V
    where
        F: FnOnce(&K) -> V;

    fn or_insert(self, value: V) -> &'a mut V {
        self.or_insert_with(|| value)
    }

    fn or_insert_with<F>(self, value: F) -> &'a mut V
    where
        F: FnOnce() -> V,
    {
        self.or_insert_with_key(|_| value())
    }

    fn or_default(self) -> &'a mut V
    where
        V: Default,
    {
        self.or_insert(Default::default())
    }
}

pub enum VecEntry<'a, K: 'a, V: 'a> {
    Occupied(OccupiedVecEntry<'a, K, V>),
    Vacant(VacantVecEntry<'a, K, V>),
}

impl<'a, K, V> VecEntryMethods<'a, K, V> for VecEntry<'a, K, V> {
    fn key(&self) -> &K {
        match self {
            Self::Occupied(ove) => ove.key(),
            Self::Vacant(vve) => vve.key(),
        }
    }

    fn take(self) -> (VacantVecEntry<'a, K, V>, Option<V>) {
        match self {
            Self::Occupied(ove) => ove.take(),
            Self::Vacant(vve) => vve.take(),
        }
    }

    fn insert_entry(self, value: V) -> OccupiedVecEntry<'a, K, V> {
        match self {
            Self::Occupied(ove) => ove.insert_entry(value),
            Self::Vacant(vve) => vve.insert_entry(value),
        }
    }

    fn and_modify<F>(self, closure: F) -> Self
    where
        F: FnOnce(&mut V),
    {
        match self {
            Self::Occupied(ove) => Self::Occupied(ove.and_modify(closure)),
            Self::Vacant(vve) => Self::Vacant(vve.and_modify(closure)),
        }
    }

    fn or_insert_with_key<F>(self, closure: F) -> &'a mut V
    where
        F: FnOnce(&K) -> V,
    {
        match self {
            Self::Occupied(ove) => ove.or_insert_with_key(closure),
            Self::Vacant(vve) => vve.or_insert_with_key(closure),
        }
    }
}

pub struct OccupiedVecEntry<'a, K: 'a, V: 'a> {
    pub(crate) index: usize,
    pub(crate) vec: &'a mut Vec<(K, V)>,
    pub(crate) _marker: PhantomData<&'a mut (K, V)>,
}

impl<'a, K, V> OccupiedVecEntry<'a, K, V> {
    fn into_mut_ref(self) -> &'a mut V {
        &mut self.vec[self.index].1
    }
}

impl<'a, K, V> VecEntryMethods<'a, K, V> for OccupiedVecEntry<'a, K, V> {
    fn key(&self) -> &K {
        &self.vec[self.index].0
    }

    fn take(self) -> (VacantVecEntry<'a, K, V>, Option<V>) {
        let (k, v) = self.vec.remove(self.index);
        (
            VacantVecEntry {
                key: k,
                vec: self.vec,
                _marker: PhantomData,
            },
            Some(v),
        )
    }

    fn insert_entry(self, value: V) -> OccupiedVecEntry<'a, K, V> {
        self.vec[self.index].1 = value;
        self
    }

    fn and_modify<F>(self, closure: F) -> Self
    where
        F: FnOnce(&mut V),
    {
        closure(&mut self.vec[self.index].1);
        self
    }

    fn or_insert_with_key<F>(self, closure: F) -> &'a mut V
    where
        F: FnOnce(&K) -> V,
    {
        &mut self.vec[self.index].1
    }
}

pub struct VacantVecEntry<'a, K: 'a, V: 'a> {
    pub(crate) key: K,
    pub(crate) vec: &'a mut Vec<(K, V)>,
    pub(crate) _marker: PhantomData<&'a mut (K, V)>,
}

impl<'a, K, V> VecEntryMethods<'a, K, V> for VacantVecEntry<'a, K, V> {
    fn key(&self) -> &K {
        &self.key
    }

    fn take(self) -> (VacantVecEntry<'a, K, V>, Option<V>) {
        (self, None)
    }

    fn insert_entry(self, value: V) -> OccupiedVecEntry<'a, K, V> {
        let index = self.vec.len();
        self.vec.push((self.key, value));
        OccupiedVecEntry {
            index,
            vec: self.vec,
            _marker: PhantomData,
        }
    }

    fn and_modify<F>(self, closure: F) -> Self
    where
        F: FnOnce(&mut V),
    {
        self
    }

    fn or_insert_with_key<F>(self, closure: F) -> &'a mut V
    where
        F: FnOnce(&K) -> V,
    {
        let res = closure(&self.key);
        self.insert_entry(res).into_mut_ref()
    }
}

pub(crate) trait VecEntryExt<K, V>: Sized {
    fn entry(&mut self, key: K) -> VecEntry<K, V>;
}

impl<K, V> VecEntryExt<K, V> for Vec<(K, V)>
where
    K: PartialEq,
{
    fn entry(&mut self, key: K) -> VecEntry<K, V> {
        if let Some((i, _)) = self.iter().enumerate().find(|(i, (k, v))| k == &key) {
            VecEntry::Occupied(OccupiedVecEntry {
                index: i,
                vec: self,
                _marker: PhantomData,
            })
        } else {
            VecEntry::Vacant(VacantVecEntry {
                key: key,
                vec: self,
                _marker: PhantomData,
            })
        }
    }
}

pub trait IterExt: Iterator + Sized {
    fn collvect(self) -> Vec<Self::Item> {
        self.collect::<Vec<Self::Item>>()
    }
}

impl<IT: Iterator> IterExt for IT {}
