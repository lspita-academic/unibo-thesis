// This enum represent the 3 possible nodes in the test, to allow the different
// binaries to share the aggregate program logic by just specifying the node
// they represent.
// In a normal scenario, all the nodes partecipating in an aggregate program
// should execute the same binary, but this is needed to be able to mock the
// environment using the fixed distances and connections between the nodes.
enum Node {
    Node1,
    Node2,
    Node3,
}

impl Node {
    // All nodes override the id in the network with a fixed value to be able
    // to recognize them
    fn node_id(&self) -> ZenohNodeId;
}

// This is the environment accessible during the execution of the aggregate
// program.
struct GradientEnv {
    node: Node,
}

impl GradientEnv {
    // Node 1 will act as the source of the gradient.
    fn is_source(&self) -> bool {
        self.node == Node::Node1
    }

    // All distances between nodes are predefined.
    fn distances(&self) -> Field<ZenohNodeId, f32> {
        match self.node {
            Node::Node1 => Field::new(0.0, HashMap::from([(Node::Node2.node_id(), 1.0)])),
            Node::Node2 => Field::new(
                0.0,
                HashMap::from([(Node::Node1.node_id(), 1.0), (Node::Node3.node_id(), 1.5)]),
            ),
            Node::Node3 => Field::new(0.0, HashMap::from([(Node::Node2.node_id(), 1.5)])),
        }
    }
}

// This is the actual function executed in a cycle of the aggregate program, that
// calculates the minimum distance from the other nodes.
// The source node has a fixed value of 0.
fn gradient(env: &GradientEnv, vm: &mut VM<ZenohNodeId, Serializer>) -> Result<f32, AggregateError> {
    let initial = f32::MAX;
    vm.share(&initial, |_, field| {
        let distances = field.aligned_map(&env.distances(), |a, b| a + b);
        let min_distance =
            *distances.min_by(|a, b| PartialOrd::partial_cmp(a, b).unwrap_or(Ordering::Greater));
        if env.is_source() { 0.0 } else { min_distance }
    })
}

fn main() {
    // Each binary executes the same program as a different node.
    let node = Node::Node1;

    // A Zenoh network is initialized with a fixed id for the node.
    let zenoh_config = ZenohConfigBuilder::with_default_options()
        .id(node.node_id())
        .build();
    let network_config = zenoh_config.into();
    let zenoh_network = ZenohNetwork::new(JsonSerializer, network_config);

    // Then the YAAIR engine can be started to execute the gradient.
    let env = GradientEnv { node };
    let mut engine = Engine::new(network, env, JsonSerializer, gradient);
    loop {
        match engine.cycle() {
            Ok(result) => log::info!("Gradient result: {result:?}"),
            Err(e) => log::warn!("Error during cycle: {e:?}"),
        }
        Timer::after(EmbassyDuration::from_secs(3)).await;
    }
}
