trait ZClosure {
    // This is the standard initialization method, that requires the callback
    // function and optionally a context.
    // The context is behind an Atomically Reference Counted (ARC) smart
    // pointer to allow sharing it in a thread-safe way.
    fn from_callback<T>(
        callback: unsafe extern "C" fn(*const Self::CallbackValue, *const T),
        context: Option<Arc<T>>,
    ) -> ZenohResult<Self>;

    // This auto-implemented method uses an embassy `Signal` as context to
    // notify any listener of the new value. Even if this implementation
    // depends on embassy, it works regardless of the async runtime used.
    fn from_signal<M: RawMutex, T: ZClone<Value = Self::CallbackValue>>(
        signal: Arc<Signal<M, T>>,
    ) -> ZenohResult<Self> {
        unsafe extern "C" fn zclosure_signal_callback<M: RawMutex, T: ZClone>(
            value: *const T::Value,
            context: *const Signal<M, T>,
        ) {
            let signal = unsafe { &*context };
            let value = T::zclone(value);
            signal.signal(value);
        }

        Self::from_callback(zclosure_signal_callback::<M, T>, Some(signal))
    }
}

// The closure types can be implemented like all the other types of the library
// using the `zwrap` macro.

#[zwrap(...)]
pub struct Sample;

#[zwrap(..., zclosure)]
struct SampleClosure;

// Then it can be used by other struct to offer a modern async/await API.
struct AsyncSubscriber {
    signal: Arc<Signal<Mutex, Sample>>,
}

impl AsyncSubscriber {
    fn new() {
        // ...
        Self { signal: Default::default() }
    }

    async fn recv_async(&self) -> T {
        self.signal.wait().await
    }
}

// Any async runtime is compatible
#[embassy_executor::main]
async fn main() {
    let subscriber = AsyncSubscriber::new();
    loop {
        let sample = subscriber.recv_async().await;
        // ...
    }
}
