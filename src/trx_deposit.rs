#[derive(Clone, PartialEq, ::prost::Message)]
#[my_service_bus_macros::my_sb_entity_protobuf_model(topic_id = "trx-deposit")]
pub struct TrxDeposit {
    #[prost(string, tag = "1")]
    pub wallet_id: String,
    #[prost(string, tag = "2")]
    pub tx_id: String,
    #[prost(sint64, tag = "3")]
    pub trx_amount: i64,
}
