use std::collections::{btree_map, linked_list, BTreeMap, LinkedList};

pub(crate) trait MultiSetValue {
    type MultiSetOrderValue: Ord;
    fn multi_set_order_value(&self) -> Self::MultiSetOrderValue;
}

pub(crate) struct MultiSet<T: MultiSetValue> {
    inner: BTreeMap<T::MultiSetOrderValue, LinkedList<T>>,
}

pub(crate) struct MultiSetIterator<'a: 'b, 'b, T: MultiSetValue> {
    map_values: btree_map::Values<'a, T::MultiSetOrderValue, LinkedList<T>>,
    set_iter: Option<linked_list::Iter<'b, T>>,
}

impl<'a, 'b, T: MultiSetValue> Iterator for MultiSetIterator<'a, 'b, T> {
    type Item = &'b T;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(i) = self.set_iter.as_mut() {
            if let Some(v) = i.next() {
                return Some(v);
            }
        }
        let Some(set) = self.map_values.next() else {
            return None;
        };
        self.set_iter = Some(set.iter());
        return self.next();
    }
}

pub(crate) struct MultiSetIteratorMut<'a: 'b, 'b, T: MultiSetValue> {
    map_values: btree_map::ValuesMut<'a, T::MultiSetOrderValue, LinkedList<T>>,
    set_iter: Option<linked_list::IterMut<'b, T>>,
}

impl<'a, 'b, T: MultiSetValue> Iterator for MultiSetIteratorMut<'a, 'b, T> {
    type Item = &'b mut T;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(i) = self.set_iter.as_mut() {
            if let Some(v) = i.next() {
                return Some(v);
            }
        }
        let Some(set) = self.map_values.next() else {
            return None;
        };
        self.set_iter = Some(set.iter_mut());
        return self.next();
    }
}

impl<T: MultiSetValue> MultiSet<T> {
    pub(crate) fn new() -> Self {
        Self {
            inner: BTreeMap::new(),
        }
    }

    pub(crate) fn iter(&self) -> MultiSetIterator<'_, '_, T> {
        return MultiSetIterator {
            map_values: self.inner.values(),
            set_iter: None,
        };
    }

    #[allow(dead_code)]
    pub(crate) fn iter_mut(&mut self) -> MultiSetIteratorMut<'_, '_, T> {
        return MultiSetIteratorMut {
            map_values: self.inner.values_mut(),
            set_iter: None,
        };
    }

    pub(crate) fn push_back(&mut self, value: T) {
        let multi_set_order_value = value.multi_set_order_value();
        let list = self
            .inner
            .entry(multi_set_order_value)
            .or_insert_with(|| LinkedList::new());
        list.push_back(value);
    }

    pub(crate) fn pop_front(&mut self) -> Option<T> {
        let (key, mut list) = self.inner.pop_first()?;
        let value = list.pop_front()?;
        if !list.is_empty() {
            self.inner.insert(key, list);
        }
        return Some(value);
    }

    pub(crate) fn pop_same_key_fronts(&mut self) -> Option<LinkedList<T>> {
        Some(self.inner.pop_first()?.1)
    }

    pub(crate) fn peak_front(&self) -> Option<&T> {
        let (_, list) = self.inner.iter().next()?;
        return list.front();
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
    struct S {
        id: &'static str,
        n: u64,
    }

    impl MultiSetValue for S {
        type MultiSetOrderValue = u64;
        fn multi_set_order_value(&self) -> u64 {
            self.n
        }
    }

    #[test]
    fn test_multi_set() {
        let mut set = MultiSet::new();
        set.push_back(S { id: "d", n: 2 });
        set.push_back(S { id: "a", n: 1 });
        set.push_back(S { id: "e", n: 3 });
        set.push_back(S { id: "c", n: 2 });
        set.push_back(S { id: "f", n: 3 });
        set.push_back(S { id: "b", n: 1 });
        assert!(!set.is_empty());
        assert_eq!(set.pop_front(), Some(S { id: "a", n: 1 }));
        assert_eq!(set.pop_front(), Some(S { id: "b", n: 1 }));
        assert_eq!(set.pop_front(), Some(S { id: "d", n: 2 }));
        assert_eq!(set.pop_front(), Some(S { id: "c", n: 2 }));
        assert_eq!(set.pop_front(), Some(S { id: "e", n: 3 }));
        assert_eq!(set.pop_front(), Some(S { id: "f", n: 3 }));
        assert_eq!(set.pop_front(), None);
        assert!(set.is_empty());
    }
}
