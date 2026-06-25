// A store entity associates with a value the metadata necessary for the expiration
// calculations.
// The timestamp is set to the current system time upon initialization, and each
// time the object is updated it is reset, making the check for the validity of
// the result a simple comparison between the time passed since the timestamp
// and the lifespan associated.
struct StoreEntity<T> {
    message: Option<T>,
    lifespan: Duration,
    timestamp: SystemTime,
}

struct MessagesStore<Id, T> {
    // The internal structure of the `MessagesStore` is an hash map behid a mutex
    // for thread safety.
    // Each operation that interacts with the values of the map automatically
    // updates the `StoreEntity` lifespan, keeping the node alive.
    storage: Mutex<HashMap<Id, StoreEntity<T>>>,
}
