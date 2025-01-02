use std::ops::Range;

use dg_math::Scalar;
use dg_structures::{Arena, Id};

struct Node {
    entries: Vec<Entry>,
}

enum Entry {
    Leaf {
        point: Scalar,
        id: Id,
    },
    Branch {
        range: Range<Scalar>,
        node: Node,
    },
}

pub struct RTree2D<T, const BRANCH_MAX: usize = 16> {
    root: Node,
    values: Arena<T>,
}

impl<T, const BRANCH_MAX: usize> RTree2D<T, BRANCH_MAX> {
    pub fn search(&self, test_range: Range<Scalar>) -> Vec<Id> {
        let mut found = Vec::new();

        fn _search(found: &mut Vec<Id>, test_range: &Range<Scalar>, node: &Node) {
            for entry in &node.entries {
                match entry {
                    Entry::Leaf { point, id } => if test_range.contains(point) {
                        found.push(*id);
                    },
                    Entry::Branch { range, node } => if test_range.start <= range.start && test_range.end >= range.end {
                        _search(found, test_range, node);
                    },
                }
            }
        }

        _search(&mut found, &test_range, &self.root);

        found
    }

    pub fn insert(&mut self, point: Scalar, value: T) -> Id {
        let id = self.values.insert(value);



        id
    }

    pub fn remove_exact(&mut self, point: Scalar) -> Id {
        todo!()
    }

    pub fn remove_within(&mut self, range: Range<Scalar>) -> Id {
        todo!()
    }

    fn add_leaf(&mut self, node: Node) {
        todo!()
    }

    fn add_branch(&mut self, node: Node) {
        todo!()
    }
    
    fn split(&mut self) {
        todo!()
    }
}