use std::ops::{Index, IndexMut};

pub struct Buffer<T> {
    pub width: usize,
    pub height: usize,
    data: Vec<Vec<T>>,
}

impl<T> Buffer<T> {
    pub fn new(default_value: T, width: usize, height: usize) -> Self
    where
        T: Copy,
    {
        Self {
            width,
            height,
            data: vec![vec![default_value; width]; height],
        }
    }
}

impl<T> Index<usize> for Buffer<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<usize> for Buffer<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
