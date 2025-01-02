use dg_math::rect::Rect;
use dg_structures::Id;

struct Node {
    level: u32,
    entries: Vec<Entry>,
}

enum Entry {
    Leaf {
        rect: Rect,
        id: Id,
    },
    Branch {
        rect: Rect,
        node: Node,
    },
}

pub struct RTree2D<const BRANCH_MAX: usize = 16> {
    root: Node,
}

impl<const BRANCH_MAX: usize> RTree2D<BRANCH_MAX> {
    pub fn search(&self, rect: Rect) -> Vec<Id> {
        let mut found = Vec::new();

        fn _search(found: &mut Vec<Id>, test_rect: Rect, node: &Node) {
            for entry in &node.entries {
                match entry {
                    Entry::Leaf { rect, id } => if rect.intersects(test_rect) {
                        found.push(*id);
                    },
                    Entry::Branch { rect, node } => if rect.intersects(test_rect) {
                        _search(found, test_rect, node);
                    },
                }
            }
        }

        _search(&mut found, rect, &self.root);

        found
    }

    pub fn insert(&mut self, rect: Rect) -> Id {


        todo!()
    }

    pub fn remove_exact(&mut self, rect: Rect) -> Id {
        todo!()
    }

    pub fn remove_intersection(&mut self, rect: Rect) -> Id {
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