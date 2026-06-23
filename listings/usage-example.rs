// Create the Zenoh config.
#[cfg(target_os = "espidf")]
let zenoh_config_builder = {
    // The Zenoh pico implementation requires some extra options.
    let interface = "lo0";
    ZenohConfigBuilder::new(ZenohConfigBuilderInitOptions {
        interface: interface.into(),
    })
    // Once the builder is created, a method to get the default options is provided.
    .set_default_options()
};
#[cfg(not(target_os = "espidf"))]
// With standard Zenoh a default builder can be directly initialized.
let zenoh_config_builder = ZenohConfigBuilder::with_default_options();

let zenoh_config = zenoh_config_builder
    // Shared options can be set from the builder, while implementation-specific
    // ones can be set directly on the configuration when built.
    .scouting_timeout(Duration::from_secs(15))
    .build()
    .expect("Failed to build the zenoh config");

// The default network config can be created from the Zenoh one, by converting it
// directly or by using it as a base.
let network_config = ZenohNetworkConfig {
     lifespan: Duration::from_secs(10),
     ..zenoh_config.into(),
};

// Now the `ZenohNetwork` can be initialized.
let zenoh_network =
    ZenohNetwork::new(JsonSerializer, network_config).expect("Failed to create zenoh network");

// If the program needs some heartbeat publishers, they must be declared before
// giving the network to the `Engine`. Then they can be used freely.
let heartbeat_publishers = zenoh_network.declare_heartbeat_publisher();

// And the network it's ready to be used in YAAIR.
let engine = Engine::new(
    zenoh_network,
    // The serializer used by the engine must be the same of the network or a
    // compatible one.
    JsonSerializer,
    ...
);
