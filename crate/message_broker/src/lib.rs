use std::hash::Hash;

use dashmap::DashMap;
use thiserror::Error;

pub trait Message<T>
where
    T: Send + Sync + Clone,
{
    fn id(&self) -> String;
    fn payload(&self) -> &T;
}

pub trait Subscriber<T>
where
    T: Send + Sync + Clone,
{
    fn subscribe(&mut self, topic: &str, callback: Box<dyn FnMut(&dyn Message<T>) -> ()>);
}

trait Broker<T>: Send + Sync + Clone
where
    T: Send + Sync + Clone,
{
    fn create_channel(&self, topic: &str) -> Result<(), Error>;
    fn delete_channel(&self, topic: &str) -> Result<(), Error>;
    fn send_channel(&self, topic: &str, message: dyn Message<T>) -> Result<(), Error>;
    fn add_subscriber(&self, topic: String, subscriber: Box<dyn Subscriber<T>>);
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error: {0}")]
    Other(String),
}

#[derive(Clone)]
pub struct InMemoryBroker<T, S>
where
    T: Send + Sync + Clone,
    S: Subscriber<T>,
{
    topics: DashMap<String, Vec<S>>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T, S> InMemoryBroker<T, S>
where
    T: Send + Sync + Clone,
    S: Subscriber<T>,
{
    pub fn new() -> Self {
        InMemoryBroker {
            topics: DashMap::new(),
            _phantom: std::marker::PhantomData,
        }
    }
}

// impl<T,P> Broker<T> for InMemoryBroker<T,P>
// where
//     T: Send + Sync + Clone,
// {
//     fn create_channel(&self, topic: &str) -> Result<(), String> {
//         todo!()
//     }

//     fn delete_channel(&self, topic: &str) -> Result<(), String> {
//         todo!()
//     }

//     fn add_publisher(&self, publisher: Box<dyn Publisher<T>>) {
//         todo!()
//     }

//     fn add_subscriber(&self, topic: String, subscriber: Box<dyn Subscriber<T>>) {
//         todo!()
//     }

//     fn distribute_messages(&self) {
//         todo!()
//     }
// }
