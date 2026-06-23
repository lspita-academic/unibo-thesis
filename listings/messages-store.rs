// A store entity associates with a value the metadata necessary for the expiration
// calculations.
struct StoreEntity<T> {
    message: Option<T>,
    lifespan: Duration,
    timestamp: SystemTime,
}

impl<T> StoreEntity<T> {
    pub fn new(message: Option<T>, lifespan: Duration) -> Self {
        // The instant of creation is stored alongside the data.
        Self { timestamp: SystemTime::now(), ... }
    }

    // The timestamp can be manually reset to keep the object valid.
    pub fn keep_alive(&mut self) {
        self.timestamp = SystemTime::now();
    }

    // Each time the object is updated, the timestamp is reset.
    pub fn update_message(&mut self, message: T) {
        self.keep_alive();
        Self::update(&mut self.message, Some(message))
    }

    pub fn update_lifespan(&mut self, lifespan: Duration) {
        self.keep_alive();
        Self::update(&mut self.lifespan, lifespan)
    }

}

type Storage = HashMap<Id, StoreEntity<T>>;

pub struct MessagesStore<Id, T> {
    // The internal structure of the `MessagesStore` is an hash map behid a mutex
    // for thread safety.
    storage: Mutex<Storage<Id, T>>,
}

impl<Id: Eq + Hash + Clone, T> MessagesStore<Id, T> {
    pub fn new(default_lifespan: Duration) -> Self {
        // The map starts empty.
        Self { storage: Default::default(), ... }
    }

    // All the operations can acquire the lock on the storage when needed.
    fn acquire_storage(&self) -> Result<MutexGuard<'_, Storage<Id, T>>, ...> {
        self.storage.lock()
    }

    pub fn store_message(&self, id: Id, message: T) {
        // The lock acquisition guarantees that no other thread is modifying the storage.
        let mut storage = self.acquire_storage()?;

        // If the node was already in the map, the result is updated with the new one.
        // Otherwise, it's stored with the default lifespan.
        if let Some(e) = storage.get_mut(&id) {
            e.update_message(message);
        } else {
            storage.insert(id, StoreEntity::new(message, self.default_lifespan));
        }
    }

    // The same logic of `store_message` is used to update the lifespan used for
    // the results of a node.
    pub fn store_lifespan(&self, id: Id, lifespan: Duration)

    // And a result can be pinged to notify the existance of the node and reset
    // its timestamp of last interaction.
    pub fn keep_alive(&self, id: Id);

    // The nodes with outdated results can then be cleared from the map.
    pub fn clear_dead(&self);
}

// If the map is cloneable, we can create a snapshot of the current state.
impl<Id: Eq + Hash + Clone, T: Clone> MessagesStore<Id, T> {
    pub fn create_snapshot(&self) -> HashMap<Id, T>;
}
