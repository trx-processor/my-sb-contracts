#[my_service_bus_macros::my_sb_entity_protobuf_model(topic_id = "trx-deposit")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrxDeposit {
    #[prost(string, tag = "1")]
    pub wallet_id: String,
    #[prost(string, tag = "2")]
    pub tx_id: String,
    #[prost(enumeration = "Currency", tag = "3")]
    pub currency: i32,
    #[prost(sint64, tag = "4")]
    pub trx_amount: i64,
}
#[derive(Clone, Debug, PartialEq, ::prost::Enumeration)]
pub enum Currency {
    Trx = 0,
    Usdt = 1,
}
