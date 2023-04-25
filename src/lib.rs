pub mod traits {
    use std::error::Error;

    pub trait QueueTrait<T> {
        fn dequeue(&mut self) -> Result<bool, Box<dyn Error>>;

        fn enqueue(&mut self) -> Result<bool, Box<dyn Error>>;

        fn reset(&mut self) -> Vec<T>;

        fn new() -> Self;
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
}

impl<T> traits::QueueTrait<T> for Queue<T> {
    fn new() -> Self {
        let inner: Vec<T> = Vec::new();

        Self { inner }
    }

    fn dequeue(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        todo!()
    }

    fn enqueue(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        todo!()
    }

    fn reset(&mut self) -> Vec<T> {
        std::mem::take(&mut self.inner)
    }
}
