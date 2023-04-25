pub mod traits {
    use std::error::Error;

    pub trait QueueTrait<T> {
        fn dequeue(&self) -> Result<bool, Box<dyn Error>>;

        fn enqueue(&self) -> Result<bool, Box<dyn Error>>;

        fn inner(&self) -> Result<Vec<T>, Box<dyn Error>>;

        fn new() -> Self;
    }
}
