use crate::types::Amount;
use crate::types::ClientId;
use crate::types::TransactionId;

#[cfg_attr(test, derive(Debug))]
pub struct Charge {
    pub client: ClientId,
    pub tx: TransactionId,
    pub amount: Amount,
}

#[cfg_attr(test, derive(Debug))]
pub struct Record {
    pub client: ClientId,
    pub tx: TransactionId,
}

#[cfg_attr(test, derive(Debug))]
pub enum Transaction {
    Deposit(Charge),
    Withdrawal(Charge),
    Dispute(Record),
    Resolve(Record),
    Chargeback(Record),
}

impl Transaction {
    pub fn as_deposit(&self) -> Option<&Charge> {
        match self {
            Self::Deposit(charge) => Some(charge),
            _ => None,
        }
    }

    pub fn tx(&self) -> TransactionId {
        match self {
            Self::Deposit(Charge { tx, .. })
            | Self::Withdrawal(Charge { tx, .. })
            | Self::Dispute(Record { tx, .. })
            | Self::Resolve(Record { tx, .. })
            | Self::Chargeback(Record { tx, .. }) => *tx,
        }
    }
}
