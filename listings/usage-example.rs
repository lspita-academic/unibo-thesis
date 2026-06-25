// The config builder may require different initialization options based
// on the Zenoh version used.
let zenoh_config = ZenohConfigBuilder::with_default_options()
    // Shared options can be set from the builder, while implementation-specific
    // ones can be set directly on the configuration when built.
    .scouting_timeout(Duration::from_secs(15))
    .build();

// The default network config can be created from the Zenoh one, by converting it
// directly or by using it as a base.
let network_config = ZenohNetworkConfig {
     lifespan: Duration::from_secs(10),
     ..zenoh_config.into(),
};
let zenoh_network = ZenohNetwork::new(network_config, JsonSerializer);

// If the program needs some heartbeat publishers, they must be declared before
// giving the network to the `Engine`.
let heartbeat_publishers = zenoh_network.declare_heartbeat_publisher();

// And the network it's ready to be used in YAAIR.
// The serializer used by the engine must be the same of the network or a
// compatible one.
let engine = Engine::new(zenoh_network, JsonSerializer, ...);
