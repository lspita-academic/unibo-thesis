impl Network<ZenohNodeId> for ZenohNetwork {
    // The id of the node inside YAAIR is the same as in the Zenoh network.
    fn get_local_id(&self) -> ZenohNodeId {
        self.session.node_id()
    }

    fn prepare_outbound(&mut self, outbound_message: Vec<u8>) {
        // The new result is simply published on the network on the dedicated topic.
        self.messages_publisher.put_message(outbound_message)
    }

    fn prepare_inbound(&mut self) -> InboundMessage<ZenohNodeId> {
        // Get the `MessagesStore` instance used by the network.
        let messages = &self.context.messages;
        // Clear the obsolete results and create a snapshot of the current
        // state of the store.
        let snapshot = match messages.clear_dead().and_then(|| {
            messages.create_snapshot()
        }) {
            // If something fails, a default empty map is given to YAAIR.
            Ok(s) => s,
            Err(_) => return Default::default(),
        };
        // Create the YAAIR associative map from the snapshot of the current
        // node results.
        InboundMessage::new(snapshot)
    }
}
