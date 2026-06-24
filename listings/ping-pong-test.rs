// Since this test uses the Zenoh pico wrapper, it supports only embedded targets.
#![cfg(target_os = "espidf")]

fn main() {
    // A Zenoh config is created to initialize the Zenoh session.
    let zenoh_config = ConfigBuilder::default()
        .mode(ConfigMode::Peer)
        .scouting_timeout(Duration::from_secs(30))
        .build();
    let zenoh_session = Session::open(zenoh_config);

    // Then the different node binaries can start their version of the program.
    ping(zenoh_session);
}

async fn ping(session: Session) {
    // A publisher and a subscriber for the topics are declared from the session.
    let publisher = session.declare_publisher("ping/value");
    let subscriber = session.declare_subscriber("pong/value");

    let mut x = 0;
    loop {
        // Timers are used to delay the communication and prevent publishing
        // values while the other node it's not listening yet.
        Timer::after_secs(2).await;

        // Postcard is a binary serialization format used in this test to send
        // the value as a sequence of bytes.
        let bytes = postcard::to_allocvec(&x);

        // The value is published on the dedicated topic where the other node
        // is listening.
        publisher.put(bytes);

        // Then the node immediately starts listening for values on the
        // opposite topic used by the other node publisher.
        let sample = subscriber.recv_async().await;

        // The value is extracted and deserialized from the received sample
        // payload.
        let bytes = sample.payload().owned_bytes();
        x = postcard::from_bytes(&bytes);

        // The value is increased before the loop restarts and the next
        // communication cycle begins.
        x += 1;
    }
}

async fn pong(session: Session) {
    // The pong node has a specular logic to the ping one.
    let publisher = session.declare_publisher("pong/value");
    let subscriber = session.declare_subscriber("ping/value");

    loop {
        // It starts by listening instead of publishing to avoid a deadlock.
        let sample = subscriber.recv_async().await;
        let bytes = sample.payload().owned_bytes();
        let mut x = postcard::from_bytes(&bytes);

        // Once the value is received, it is increased and published back to
        // the ping node.
        x += 1;
        let bytes = postcard::to_allocvec(&x);
        Timer::after_secs(2).await;
        publisher.put(bytes);
    }
}
