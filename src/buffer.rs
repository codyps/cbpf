
// Space to store a T, not yet allocated
pub struct Buffer<T> {

}

impl Buffer {

}

pub trait Buffer<T> : IndexMut <usize> {
    pub fn push(&mut self, val: T);
    pub unsafe fn set_len(&self, len: usize);
}

