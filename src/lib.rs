pub mod traits {
    use std::error::Error;

    pub trait QueueTrait<T> {
        fn dequeue(&self) -> Result<bool, Box<dyn Error>>;

        fn enqueue(&self) -> Result<bool, Box<dyn Error>>;

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
    fn dequeue(&self) -> Result<bool, Box<dyn std::error::Error>> {
        todo!()
    }

    fn enqueue(&self) -> Result<bool, Box<dyn std::error::Error>> {
        todo!()
    }

    fn inner(&self) -> Result<Vec<T>, Box<dyn std::error::Error>> {
        todo!()
    }

    fn new() -> Self {
        let inner: Vec<T> = Vec::new();

        Self { inner }
    }
}
