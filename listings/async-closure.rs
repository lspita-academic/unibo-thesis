// The closure types are composed of a callback function and a context,
// that can be put behind an Atomically Reference Counted (Arc) smart pointer
// to allow sharing it in a thread-safe way.
// Any object using a closure can leverage an embassy `Signal` to get notified
// whenever a new value is received and provide a modern async/await API.
struct AsyncSubscriber {
    signal: Arc<Signal<Mutex, Sample>>,
    sample_closure: SampleClosure,
}

impl AsyncSubscriber {
    async fn recv_async(&self) -> T {
        self.signal.wait().await
    }
}

let subscriber = AsyncSubscriber::new();
loop {
    // This waits for the next value from the subscriber callback.
    let sample = subscriber.recv_async().await;
    // ...
}
