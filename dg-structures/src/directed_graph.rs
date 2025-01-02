use std::collections::{HashMap, HashSet};

use crate::{Arena, Id, Index};

struct Vertex<V> {
    id: Id,
    value: V,
}

struct Edge<E> {
    id: Id,
    value: E,
}

pub struct DirectedGraph<V, E> {
    vertices: Arena<Vertex<V>>,
    edges: Arena<Edge<E>>,
}

impl<V, E> DirectedGraph<V, E> {
    pub fn new() -> Self {
        Self {
            vertices: Default::default(),
            edges: Default::default(),
        }
    }

    pub fn connected(&self, a: Id, b: Id) -> bool {
        todo!()
    }

    pub fn strongly_connected(&self, a: Id, b: Id) -> bool {
        todo!()
    }

    pub fn neighbors(&self, v: Id) -> Vec<Id> {
        todo!()
    }

    pub fn add_vertex(&mut self, vertex: V) -> Id {
        todo!()
    }

    pub fn remove_vertex(&mut self, id: Id) -> Option<V> {
        todo!()
    }

    pub fn add_edge(&mut self, a: Id, b: Id, value: E) {
        todo!()
    }

    pub fn remove_edge(&mut self, a: Id, b: Id) -> Option<E> {
        todo!()
    }

    pub fn set_vertex(&mut self, id: Id, value: V) -> Option<V> {
        todo!()
    }

    pub fn get_vertex(&self, id: Id) -> Option<&V> {
        todo!()
    }

    pub fn get_vertex_mut(&mut self, id: Id) -> Option<&mut V> {
        todo!()
    }

    pub fn set_edge(&mut self, a: Id, b: Id, edge: E) -> Option<E> {
        todo!()
    }

    pub fn get_edge(&self, a: Id, b: Id) -> Option<&E> {
        todo!()
    }

    pub fn get_edge_mut(&mut self, a: Id, b: Id) -> Option<&mut E> {
        todo!()
    }
}
