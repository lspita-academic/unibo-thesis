pub trait Network<Id: Ord + Hash + Copy + Serialize + for<'de> Deserialize<'de>> {
    fn get_local_id(&self) -> Id;
    fn prepare_outbound(&mut self, outbound_message: Vec<u8>);
    fn prepare_inbound(&mut self) -> InboundMessage<Id>;
}
