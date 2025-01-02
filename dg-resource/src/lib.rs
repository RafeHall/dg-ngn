pub mod image;
pub mod text;
pub mod texture;


// - Must keep track of whether resource is still in use, IE. no manual allocate or free.
// - Cloneable

// Resource Identifier
pub struct RID<T> {
    _marker: std::marker::PhantomData<T>,
}

impl<T> Clone for RID<T> {
    fn clone(&self) -> Self {
        Self {
            _marker: self._marker.clone(),
        }
    }
}

// Resource Trait
pub trait Resource {

}