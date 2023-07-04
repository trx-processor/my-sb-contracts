#[my_service_bus_macros::my_sb_entity_protobuf_model(topic_id = "trx-deposit")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrxDepositSbContract {
    #[prost(string, tag = "1")]
    pub wallet_id: String,
    #[prost(string, tag = "2")]
    pub tx_id: String,
    #[prost(enumeration = "CurrencySbModel", tag = "3")]
    pub currency: i32,
    #[prost(uint64, tag = "4")]
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq, ::prost::Enumeration)]
pub enum CurrencySbModel {
    Trx = 0,
    Usdt = 1,
}
