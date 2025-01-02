use crate::{Arena, Index};

pub struct HalfEdge {
    twin: Index,
    next: Index,
    prev: Index,
    vert: Index,
    edge: Index,
    face: Index,
}

pub struct HalfEdgeMesh<V, E, F> {
    half_edges: Arena<HalfEdge>,
    faces: Arena<F>,
    edges: Arena<E>,
    vertices: Arena<V>,
}

impl<V, E, F> HalfEdgeMesh<V, E, F> {
    pub fn new() -> Self {
        Self {
            half_edges: Arena::new(),
            faces: Arena::new(),
            edges: Arena::new(),
            vertices: Arena::new(),
        }
    }
}