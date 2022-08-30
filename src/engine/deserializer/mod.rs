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
struct RawTransaction {
    r#type: String,
    client: ClientId,
    tx: TransactionId,
    amount: Option<Amount>,
}

impl TryFrom<RawTransaction> for Transaction {
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
            amount.ok_or_else(|| "")
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

pub fn deserialize(path: PathBuf) -> EngineResult<Vec<Transaction>> {
    let mut writer = csv::Reader::from_path(path)
        .map_err(|_| "Unable to read from the provided file.")?;
    let headers = writer.headers().ok().map(|headers| {
        let mut headers = headers.clone();
        headers.trim();
        headers
    });
    let headers = headers.as_ref();
    let transactions = writer.records().into_iter().fold(
        vec![],
        |mut transactions, string_record| {
            string_record
                .ok()
                .and_then(|mut string_record| {
                    string_record.trim();
                    string_record.deserialize::<RawTransaction>(headers).ok()
                })
                .and_then(|raw_transaction| raw_transaction.try_into().ok())
                .map(|transaction| transactions.push(transaction));
            transactions
        },
    );
    Ok(transactions)
}
