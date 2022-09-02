#[cfg(test)]
mod tests;

use std::path::PathBuf;

use serde::Deserialize;

use crate::transaction::Charge;
use crate::transaction::ChargeRef;
use crate::transaction::Transaction;
use crate::types::Amount;
use crate::types::ClientId;
use crate::types::EngineResult;
use crate::types::TransactionId;

#[derive(Deserialize)]
#[cfg_attr(test, derive(Debug))]
struct RawTransaction<'a> {
    r#type: &'a str,
    client: ClientId,
    tx: TransactionId,
    amount: Option<Amount>,
}

impl<'a> TryFrom<RawTransaction<'a>> for Transaction {
    type Error = &'static str;

    fn try_from(
        RawTransaction {
            r#type,
            client,
            tx,
            amount,
        }: RawTransaction,
    ) -> Result<Self, Self::Error> {
        fn get_amount(amount: Option<Amount>) -> EngineResult<Amount> {
            amount.ok_or_else(|| {
                "Unable to get the amount for this transaction type."
            })
        }
        match &*r#type {
            "deposit" => get_amount(amount).map(|amount| {
                Transaction::Deposit(Charge { client, tx, amount })
            }),
            "withdrawal" => get_amount(amount).map(|amount| {
                Transaction::Withdrawal(Charge { client, tx, amount })
            }),
            "dispute" => Ok(Transaction::Dispute(ChargeRef { client, tx })),
            "resolve" => Ok(Transaction::Resolve(ChargeRef { client, tx })),
            "chargeback" => {
                Ok(Transaction::Chargeback(ChargeRef { client, tx }))
            },
            _ => Err(""),
        }
    }
}

pub fn deserialize<P, F>(src: P, mut callback: F) -> EngineResult<()>
where
    P: Into<PathBuf>,
    F: FnMut(Transaction),
{
    let src = src.into();
    let mut reader = csv::Reader::from_path(src)
        .map_err(|_| "Unable to read from the given source file.")?;
    let mut raw_record = csv::ByteRecord::new();
    let headers = reader
        .byte_headers()
        .map(|headers| {
            let mut headers = headers.clone();
            headers.trim();
            headers
        })
        .map_err(|_| "Unable to read headers for this csv file.")?;
    let headers = Some(&headers);
    while reader.read_byte_record(&mut raw_record).unwrap_or_default() {
        raw_record.trim();
        raw_record
            .deserialize::<RawTransaction>(headers)
            .ok()
            .and_then(|raw_transaction| raw_transaction.try_into().ok())
            .map(&mut callback);
    }
    Ok(())
}
