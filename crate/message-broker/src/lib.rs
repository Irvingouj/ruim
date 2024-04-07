use std::{
    collections::{HashSet, VecDeque},
    hash::Hash,
    sync::Arc,
};

use dashmap::DashMap;
use thiserror::Error;

pub trait Message<T>
where
    T: Send + Sync + Clone,
{
    fn id(&self) -> String;
    fn time(&self) -> chrono::DateTime<chrono::Utc>;
    fn payload(&self) -> &T;
}

#[async_trait::async_trait]
pub trait Subscriber<T, M>
where
    T: Send + Sync + Clone,
    M: Message<T>,
{
    async fn on_message(&self, message: &M) -> Result<(), Error>;
}

pub trait BrokerReceive<T, S, M>: Send + Clone
where
    T: Send + Sync + Clone,
    S: Subscriber<T, M>,
    M: Message<T>,
{
    fn create_channel(&self, topic: &str) -> Result<(), Error>;
    fn delete_channel(&self, topic: &str) -> Result<(), Error>;
    fn add_subscriber(&self, topic: &str, subscriber: S) -> Result<(), Error>;
}

#[async_trait::async_trait]
pub trait BrokerSend<T, M>: Send + Clone
where
    T: Send + Sync + Clone,
    M: Message<T>,
{
    async fn send_message(&self, topic: &str, message: M) -> Result<(), Error>;
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error: {0}")]
    Other(String),

    #[error("Error: {0}")]
    ChannelDoesNotExist(String),

    #[error("Error: {0}")]
    ChannelAlreadyExist(String),

    #[error("Subscriber is dead: {0}")]
    SubscriberGoneBad(String),
}

#[derive(Clone)]
pub struct InMemoryBroker<MsgInner, Sub, Msg>
where
    MsgInner: Send + Sync + Clone,
    Sub: Subscriber<MsgInner, Msg> + Hash + Eq,
    Msg: Message<MsgInner>,
{
    channels: Arc<DashMap<String, HashSet<Sub>>>,
    _phantom: std::marker::PhantomData<MsgInner>,
    _phantom2: std::marker::PhantomData<Msg>,
}

impl<T, S, M> Default for InMemoryBroker<T, S, M>
where
    T: Send + Sync + Clone,
    S: Subscriber<T, M> + Hash + Eq,
    M: Message<T>,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T, S, M> InMemoryBroker<T, S, M>
where
    T: Send + Sync + Clone,
    S: Subscriber<T, M> + Hash + Eq,
    M: Message<T>,
{
    pub fn new() -> Self {
        InMemoryBroker {
            channels: Arc::new(DashMap::new()),
            _phantom: std::marker::PhantomData,
            _phantom2: std::marker::PhantomData,
        }
    }
}

impl<T, S, M> BrokerReceive<T, S, M> for InMemoryBroker<T, S, M>
where
    T: Send + Sync + Clone,
    S: Subscriber<T, M> + Send + Sync + Clone + Hash + Eq,
    M: Message<T> + Send + Sync + Clone,
{
    fn create_channel(&self, topic: &str) -> Result<(), Error> {
        match self.channels.entry(topic.to_string()) {
            dashmap::mapref::entry::Entry::Occupied(_) => {
                return Err(Error::ChannelAlreadyExist(topic.to_string()));
            }
            dashmap::mapref::entry::Entry::Vacant(_) => {
                self.channels.insert(topic.to_string(), HashSet::new());
            }
        }

        Ok(())
    }

    fn delete_channel(&self, topic: &str) -> Result<(), Error> {
        match self.channels.remove(topic) {
            Some(_) => Ok(()),
            None => Err(Error::ChannelDoesNotExist(topic.to_string())),
        }
    }

    fn add_subscriber(&self, topic: &str, subscriber: S) -> Result<(), Error> {
        self.channels
            .get_mut(topic)
            .ok_or(Error::ChannelDoesNotExist(topic.to_string()))?
            .insert(subscriber);

        Ok(())
    }
}

#[async_trait::async_trait]
impl<T, S, M> BrokerSend<T, M> for InMemoryBroker<T, S, M>
where
    T: Send + Sync + Clone,
    S: Subscriber<T, M> + Send + Sync + Clone + Hash + Eq,
    M: Message<T> + Send + Sync + Clone,
{
    async fn send_message(&self, topic: &str, message: M) -> Result<(), Error> {
        let mut vec = VecDeque::new();
        let mut channel = self
            .channels
            .get_mut(topic)
            .ok_or(Error::ChannelDoesNotExist(topic.to_string()))?;

        // TODO: send message concurrently
        for subscriber in channel.iter() {
            if subscriber.on_message(&message).await.is_err() {
                vec.push_back(subscriber.clone());
            }
        }

        while let Some(subscriber) = vec.pop_front() {
            channel.remove(&subscriber);
        }

        Ok(())
    }
}
