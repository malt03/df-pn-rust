use std::{
    collections::HashMap,
    hash::Hash,
    ops::{Index, IndexMut},
};

#[derive(Debug, Clone, Hash)]
pub(crate) struct Set<T> {
    elements: Vec<T>,
}

impl<T> Set<T> {
    pub(crate) fn new() -> Set<T> {
        Set {
            elements: Vec::new(),
        }
    }

    pub(crate) fn push(&mut self, element: T) {
        self.elements.push(element);
    }

    pub(crate) fn len(&self) -> usize {
        self.elements.len()
    }

    pub(crate) fn map<F>(&self, f: F) -> Set<T>
    where
        F: Fn(&T) -> T,
    {
        Set {
            elements: self.elements.iter().map(f).collect(),
        }
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &T> {
        self.elements.iter()
    }
}

impl<T, I> From<I> for Set<T>
where
    I: IntoIterator<Item = T>,
{
    fn from(iter: I) -> Self {
        let mut set = Set::new();
        for i in iter {
            set.push(i);
        }
        set
    }
}

impl<T> Index<usize> for Set<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.elements[index]
    }
}

impl<T> IndexMut<usize> for Set<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.elements[index]
    }
}

impl<T> PartialEq for Set<T>
where
    T: Eq + Hash,
{
    fn eq(&self, other: &Self) -> bool {
        let mut map = HashMap::new();
        for e in &self.elements {
            *map.entry(e).or_insert(0) += 1;
        }
        for e in &other.elements {
            if let Some(v) = map.get_mut(e) {
                *v -= 1;
                if *v == 0 {
                    map.remove(e);
                }
            } else {
                return false;
            }
        }
        map.is_empty()
    }
}

impl<T> Eq for Set<T> where T: Eq + Hash {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq() {
        assert_eq!(Set::from([1, 1, 2, 3, 3]), Set::from([3, 2, 1, 3, 1]));
        assert_ne!(Set::from([1, 1, 2, 3, 3]), Set::from([3, 2, 1, 3, 2]));
    }
}
