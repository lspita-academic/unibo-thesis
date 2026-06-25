// The session is the center of the communication, and its trait also defines
// the concrete types used by the Zenoh implementation.
trait ZenohSession {
    type Err, Config, KeyExpr, Subscriber, Publisher;

    // The nodes id type must be the same in all implementations for consistency
    // in serialization of messages sent between nodes using different versions
    // of Zenoh.
    fn node_id(&self) -> ZenohNodeId;

    fn init(zenoh_config: Self::Config) -> Result<Self, Self::Err>;
    fn declare_subscriber<T>(&self, keyexpr: Self::KeyExpr, callback: fn(T, &NetworkContext), context: Arc<NetworkContext>) -> Result<Self::Subscriber, Self::Err>;
    fn declare_publisher(&self, keyexpr: Self::KeyExpr) -> Result<Self::Publisher, Self::Err>;
}

// Other traits can access the implementation types through the Session trait.
trait ZenohKeyExpr, ZenohSubscriber, ZenohPublisher;

// The main library module that defines the `ZenohNetwork` can than import the
// correct Zenoh implementation based on the target characteristics.
#[cfg(target_os = "espidf")]
#[path = "zenoh_pico/mod.rs"]
mod zenoh_impl;

#[cfg(not(any(target_os = "espidf", target_os = "none")))]
#[path = "zenoh_standard/mod.rs"]
mod zenoh_impl;
