use std::error::Error;

pub mod prelude {
    pub use super::traits::*;
    pub use super::Queue;
}

pub mod traits {
    use std::error::Error;

    pub trait QueueTrait<T> {
        fn dequeue(&mut self) -> Result<T, Box<dyn Error>>;

        fn enqueue(&mut self, value: T);

        /// reset the queue and reallocate new memory for the queue,
        /// this method will return the previous queue memory back to the system.
        fn reset_realloc(&mut self) -> Vec<T>;

        /// reset the queue, this method will only clear all the elements inside the queue inner [Vector](Vec).
        fn reset_element_only(&mut self);
    }
}

pub struct Queue<T> {
    inner: Vec<T>,
}

impl<T> Queue<T> {
    pub fn inner(&self) -> &[T] {
        self.inner.as_ref()
    }

    pub fn inner_mut(&mut self) -> &mut Vec<T> {
        &mut self.inner
    }

    fn new() -> Self {
        let inner: Vec<T> = Vec::new();

        Self { inner }
    }
}

impl<T> traits::QueueTrait<T> for Queue<T> {
    fn dequeue(&mut self) -> Result<T, Box<dyn Error>> {
        if self.inner.is_empty() {
            let err = anyhow::anyhow!("queue is empty");

            return Err(err)?;
        }

        Ok(self.inner.remove(0))
    }

    fn enqueue(&mut self, value: T) {
        self.inner.push(value)
    }

    fn reset_realloc(&mut self) -> Vec<T> {
        let new_inner: Vec<T> = Vec::new();

        std::mem::replace(&mut self.inner, new_inner)
    }

    fn reset_element_only(&mut self) {
        self.inner.clear()
    }
}

#[cfg(test)]
mod tests {

    use super::prelude::*;

    #[test]
    fn should_enqueue() {
        let mut queue = Queue::new();

        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        let first_el = queue.inner()[0];
        assert_eq!(first_el, 1);

        let mid_el = queue.inner()[1];
        assert_eq!(mid_el, 2);

        let last_el = queue.inner().last().unwrap().clone();
        assert_eq!(last_el, 3);
    }

    #[test]
    fn should_dequeue() {
        todo!()
    }

    #[test]
    fn should_fail_dequeue_if_empty() {
        todo!()
    }

    #[test]
    fn should_reset_and_reallocate() {
        todo!()
    }

    #[test]
    fn should_reset_element_only() {
        todo!()
    }
}
