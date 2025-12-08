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
