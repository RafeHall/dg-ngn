use std::{ops::Range, time::{Duration, Instant}};

use dg_structures::{Arena, Index};

// struct Node {
//     entries: Vec<Entry>,
// }

enum Node {
    Leaf {
        instant: Instant,
        index: Index,
    },
    Branch {
        range: Range<Instant>,
        children: Vec<Node>,
    },
}

pub struct DrawCalls<T> {
    values: Arena<T>,
    root: Option<Node>,
    last_time: Instant,
}

impl<T> DrawCalls<T> {
    const BRANCH_LIMIT: usize = 8;

    pub fn new() -> Self {
        Self {
            values: Arena::new(),
            root: None,
            last_time: Instant::now(),
        }
    }

    pub fn search(&self, range: Range<Instant>) -> Vec<Index> {
        if self.root.is_none() {
            return vec![];
        }

        let mut found = Vec::new();
        
        #[inline]
        fn _intersects(a: &Range<Instant>, b: &Range<Instant>) -> bool {
            !((a.start > b.start && a.start > b.end) &&
            (a.end > b.start && b.end > b.start) || // a > b
            (a.start <= b.start && a.start <= b.end) && // or
            (a.end <= b.start && a.end <= b.end)) // a <= b
        }

        fn _search(found: &mut Vec<Index>, test_range: &Range<Instant>, node: &Node) {
            match node {
                Node::Leaf { instant, index } => if test_range.contains(instant) {
                    found.push(*index);
                },
                Node::Branch { range, children } => if _intersects(range, test_range) {
                    for child in children {
                        _search(found, range, child);
                    }
                },
            }
        }

        _search(&mut found, &range, self.root.as_ref().unwrap());

        found
    }

    pub fn insert(&mut self, duration: Duration, value: T) -> Index {
        let now = Instant::now();
        let then = now + duration;
        let index = self.values.insert(value);

        if self.root.is_none() {
            self.root = Some(Node::Leaf { instant: then, index });
        } else {

        }

        index
    }

    pub fn remove_old(&mut self) -> Vec<Index> {
        let range = self.last_time..Instant::now();
        self.last_time = Instant::now();

        todo!()
    }
}