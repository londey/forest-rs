
#[derive(Default,Clone)]
pub struct Forest<T> {
    values : Vec<T>,
    offsets : Vec<isize>,
}

impl<T> Forest<T> {
    /// Pushes a value as a child of the last value in the forest.
    /// Pushes as the first root if the forest is empty.
    /// 
    /// # Examples
    ///
    /// ```
    /// let mut v = forest![1{2,3}];
    ///
    /// v.push_child(4);
    ///
    /// assert!(v.to_string() == "[1{2,3{4}}]");
    /// ```
    pub(crate) fn push_child(&mut self, value: T) {
        if self.is_empty() {
            self.values.push(value);
            self.offsets.push(self.values.len() as isize);
            return;
        }

        self.values.push(value);
        self.offsets.push(-1);

        let mut offset = &self.offsets[self.offsets.len() - 2];
        loop {
            if offset
        }
    }


    /// Clears the forest, removing all values.
    ///
    /// Note that this method has no effect on the allocated capacity
    /// of the forest.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut v = forest![1{2,3}];
    ///
    /// v.clear();
    ///
    /// assert!(v.is_empty());
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        self.values.clear();
        self.offsets.clear();
    }

    /// Returns the number of elements in the forest
    ///
    /// # Examples
    ///
    /// ```
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        assert!(self.values.len() == self.offsets.len());
        self.values.len()
    }

    /// Returns `true` if the forest contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut v = Forest::new();
    /// assert!(v.is_empty());
    ///
    /// v.push(1);
    /// assert!(!v.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        assert!(self.values.is_empty() == self.offsets.is_empty());
        self.values.is_empty()
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn construct_i32() {
        let x = Forest<i32>::new();
    }
}
