use std::collections::BTreeMap;

use crate::client::Client;
use crate::engine::core::TransactionWrapper;
use crate::transaction::Charge;
use crate::transaction::Transaction;
use crate::types::ClientId;
use crate::types::EngineResult;
use crate::types::TransactionId;

pub(super) fn get_or_insert_client(
    clients: &mut BTreeMap<ClientId, Client>,
    client: ClientId,
) -> &mut Client {
    clients.entry(client).or_insert_with(|| Client::new(client))
}

pub(super) fn assert_transaction_doesnt_exists(
    transactions: &BTreeMap<TransactionId, TransactionWrapper>,
    tx: &TransactionId,
) -> EngineResult<()> {
    let exists = transactions.contains_key(&tx);
    match exists {
        false => Ok(()),
        true => Err("Oops, a transaction with that id already exists."),
    }
}

pub(super) fn get_transaction_wrapper<'a>(
    transactions: &'a mut BTreeMap<TransactionId, TransactionWrapper>,
    tx: &'a TransactionId,
) -> EngineResult<&'a mut TransactionWrapper> {
    transactions.get_mut(tx).ok_or_else(|| {
        "Oops, a transaction with that id was not able to be found."
    })
}

pub(super) fn get_client<'a>(
    clients: &'a mut BTreeMap<ClientId, Client>,
    client: &'a ClientId,
) -> EngineResult<&'a mut Client> {
    clients.get_mut(client).ok_or_else(|| "Oops, a client with that id was not able to be found. You can only dispute transactions directed towards currently existing clients.")
}

pub(super) fn as_deposit<'a>(
    transaction: &'a mut Transaction,
) -> EngineResult<&'a Charge> {
    transaction.as_deposit().ok_or_else(|| {
        "Oops, only transactions of type 'deposit' can be disputable."
    })
}
