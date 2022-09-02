use serde::Deserialize;

use crate::types::Amount;
use crate::types::ClientId;
use crate::types::TransactionId;

#[derive(Deserialize, PartialEq)]
#[cfg_attr(test, derive(Debug))]
pub struct Charge {
    pub client: ClientId,
    pub tx: TransactionId,
    pub amount: Amount,
}

#[derive(Deserialize, PartialEq)]
#[cfg_attr(test, derive(Debug))]
pub struct ChargeRef {
    pub client: ClientId,
    pub tx: TransactionId,
}

#[derive(Deserialize, PartialEq)]
#[cfg_attr(test, derive(Debug))]
pub enum Transaction {
    Deposit(Charge),
    Withdrawal(Charge),
    Dispute(ChargeRef),
    Resolve(ChargeRef),
    Chargeback(ChargeRef),
}

impl Transaction {
    pub fn as_deposit(&self) -> Option<&Charge> {
        match self {
            Self::Deposit(charge) => Some(charge),
            _ => None,
        }
    }

    pub fn charge_tx(&self) -> Option<TransactionId> {
        match self {
            Self::Deposit(Charge { tx, .. })
            | Self::Withdrawal(Charge { tx, .. }) => Some(*tx),
            Self::Dispute(ChargeRef { .. })
            | Self::Resolve(ChargeRef { .. })
            | Self::Chargeback(ChargeRef { .. }) => None,
        }
    }
}
