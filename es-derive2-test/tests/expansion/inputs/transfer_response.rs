use es_interface::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Transferred {
    pub transfer_id: String,
}
event!(Transferred);

#[derive(Debug, Clone, PartialEq)]
pub struct TransferFailed {
    pub transfer_id: String,
}
event!(TransferFailed);

#[derive(Debug, Clone, AwaitedSet)]
pub enum TransferResponse {
    Transferred(Transferred),
    Failed(TransferFailed),
}
