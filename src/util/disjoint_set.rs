use std::{
    ops::{Index, IndexMut},
    usize,
};

pub struct DisjointSet {
    reps: Vec<usize>,
    sizes: Vec<u32>,
}

impl DisjointSet {
    pub fn new() -> Self {
        Self {
            reps: vec![],
            sizes: vec![],
        }
    }

    pub fn make_set(&mut self) -> usize {
        let idx = self.reps.len();
        self.reps.push(idx);
        self.sizes.push(1);
        idx
    }

    pub fn find(&mut self, idx: usize) -> usize {
        if self.reps[idx] == idx {
            idx
        } else {
            let rep = self.find(self.reps[idx]);
            if self.reps[idx] != rep {
                self.sizes[rep] += 1;
            }

            self.reps[idx] = rep;
            rep
        }
    }

    pub fn union(&mut self, a_idx: usize, b_idx: usize) {
        let a_rep = self.find(a_idx);
        let b_rep = self.find(b_idx);
        if a_rep != b_rep {
            let (mut a_rep, mut b_rep) = (a_rep, b_rep);
            if self.sizes[a_idx] < self.sizes[b_idx] {
                (a_rep, b_rep) = (b_rep, a_rep);
            }

            self.reps[b_rep] = a_rep;
            self.sizes[a_idx] += 1;
        }
    }

    pub fn is_joined(&mut self, a_idx: usize, b_idx: usize) -> bool {
        self.find(a_idx) == self.find(b_idx)
    }

    pub fn sets(&mut self) -> Vec<Vec<usize>> {
        let mut possible_sets = vec![vec![]; self.reps.len()];
        for i in 0..self.reps.len() {
            possible_sets[self.find(i)].push(i);
        }

        possible_sets.into_iter().filter(|v| v.len() > 0).collect()
    }

    pub fn num_sets(&mut self) -> usize {
        let mut seen = vec![];
        for i in 0..self.reps.len() {
            let rep = self.find(i);
            if seen.iter().position(|&r| r == rep).is_none() {
                seen.push(rep);
            }
        }

        seen.iter().count()
    }
}

pub struct DisjointSetData<T> {
    sets: DisjointSet,
    values: Vec<T>,
}

impl<T> DisjointSetData<T> {
    pub fn new() -> Self {
        Self {
            sets: DisjointSet::new(),
            values: vec![],
        }
    }

    pub fn values(&self) -> &Vec<T> {
        &self.values
    }

    pub fn push(&mut self, val: T) -> usize {
        self.values.push(val);
        self.sets.make_set()
    }

    pub fn find(&mut self, idx: usize) -> usize {
        self.sets.find(idx)
    }

    pub fn union(&mut self, a_idx: usize, b_idx: usize) {
        self.sets.union(a_idx, b_idx)
    }

    pub fn is_joined(&mut self, a_idx: usize, b_idx: usize) -> bool {
        self.sets.is_joined(a_idx, b_idx)
    }

    pub fn sets(&mut self) -> Vec<Vec<usize>> {
        self.sets.sets()
    }

    pub fn num_sets(&mut self) -> usize {
        self.sets.num_sets()
    }
}

impl<T> Index<usize> for DisjointSetData<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl<T> IndexMut<usize> for DisjointSetData<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.values[index]
    }
}

impl<T> FromIterator<T> for DisjointSetData<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut ds = DisjointSetData::new();
        let mut elems = iter.into_iter();
        while let Some(val) = elems.next() {
            ds.push(val);
        }

        ds
    }
}
