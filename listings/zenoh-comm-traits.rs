// The session is the center of the communication, it defines the concrete types
// used by the Zenoh implementation.
pub trait ZenohSession {
    type Err;
    type Config;
    type KeyExpr: ZenohKeyExpr<Self>;
    type Subscriber: ZenohSubscriber<Self>;
    type Publisher: ZenohPublisher<Self>;

    fn init(zenoh_config: Self::Config) -> Result<Self, Self::Err>;

    // The nodes id type must be the same in all implementations for consistency
    // in serialization of messages sent between nodes using different versions
    // of Zenoh.
    fn node_id(&self) -> ZenohNodeId;

    fn declare_subscriber<T>(&self, keyexpr: Self::KeyExpr, options: ZenohSubscriberOptions<T>) -> Result<Self::Subscriber, Self::Err>;
    fn declare_publisher(&self, keyexpr: Self::KeyExpr) -> Result<Self::Publisher, Self::Err>;
}

// Traits can access the implementation types through the session trait.
pub trait ZenohKeyExpr<Session: ZenohSession> {
    fn declare_topic(topic: &str) -> Result<Self, Session::Err>;
    fn join_topics(&self, other: &Self) -> Result<Self, Session::Err>;
}

// The subscriber closure options are defined at creation in a way compatible with
// the special closure-based API of Zenoh pico, that is also useable in the standard
// versione of the library.
pub struct ZenohSubscriberOptions<T> {
    pub callback: fn(T, &NetworkContext),
    pub context: Arc<NetworkContext>,
}

// The subscriber than becomes just an handle to keep it alive in the background.
pub trait ZenohSubscriber<Session: ZenohSession> {}

pub trait ZenohPublisher<Session: ZenohSession> {
    fn put_message<M: AsRef<[u8]>>(&self, message: M) -> Result<(), Session::Err>;
}

// The main library module that defines the `ZenohNetwork` can than import the
// correct Zenoh implementation through a custom configuration attribute which
// value is set based on different conditions on the target characteristics.
// Both the modules expose their implementations under the same name for easier
// usage, while the methods themselves can be called through the different traits.
#[cfg(zenoh_impl = "standard")]
#[path = "zenoh_standard/mod.rs"]
mod zenoh_impl;

// Only one of the two cfg conditions can be true at the same time, so there is
// no conflict between module names.
#[cfg(zenoh_impl = "pico")]
#[path = "zenoh_pico/mod.rs"]
mod zenoh_impl;
