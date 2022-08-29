mod utils {
    use std::collections::BTreeMap;

    use crate::client::Client;
    use crate::engine::TransactionWrapper;
    use crate::transactions::Charge;
    use crate::transactions::Transaction;
    use crate::types::ClientId;
    use crate::types::EngineResult;
    use crate::types::TransactionId;

    pub fn get_or_insert_client(
        clients: &mut BTreeMap<ClientId, Client>,
        client: ClientId,
    ) -> &mut Client {
        clients.entry(client).or_insert_with(|| Client::new(client))
    }

    pub(super) fn insert_transaction(
        transactions: &mut BTreeMap<TransactionId, TransactionWrapper>,
        tx: TransactionId,
        transaction: Transaction,
    ) {
        let transaction_wrapper = TransactionWrapper::new(transaction);
        transactions.insert(tx, transaction_wrapper);
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
}

use std::collections::BTreeMap;

use crate::client::Client;
use crate::transactions::Charge;
use crate::transactions::Record;
use crate::transactions::Transaction;
use crate::types::ClientId;
use crate::types::EngineResult;
use crate::types::TransactionId;

#[cfg_attr(test, derive(Debug))]
struct TransactionWrapper {
    pub(self) transaction: Transaction,
    // pub(self) disputed_by: BTreeSet<TransactionId>,
    pub(self) disputed: bool,
}

impl TransactionWrapper {
    fn new(transaction: Transaction) -> Self {
        Self {
            transaction,
            // disputed_by: BTreeSet::default(),
            disputed: false,
        }
    }
}

#[cfg_attr(test, derive(Debug))]
pub struct Engine {
    clients: BTreeMap<ClientId, Client>,
    transactions: BTreeMap<TransactionId, TransactionWrapper>,
}

impl Default for Engine {
    fn default() -> Self {
        Self {
            clients: BTreeMap::default(),
            transactions: BTreeMap::default(),
        }
    }
}

impl Engine {
    pub fn process(&mut self, transaction: Transaction) -> EngineResult<()> {
        let Self {
            clients,
            transactions,
        } = self;
        macro_rules! charge {
            ($charge_type:ident, $tx:ident, $client:ident, $amount:ident) => {{
                utils::assert_transaction_doesnt_exists(transactions, $tx)?;
                let client = utils::get_or_insert_client(clients, *$client);
                client.$charge_type(*$amount)?;
            }};
        };
        match &transaction {
            Transaction::Deposit(Charge { client, tx, amount }) => {
                charge!(deposit, tx, client, amount);
                // utils::assert_transaction_doesnt_exists(transactions, tx)?;
                // let client = utils::get_or_insert_client(clients, *client);
                // client.deposit(*amount)?;
            },
            Transaction::Withdrawal(Charge { client, tx, amount }) => {
                utils::assert_transaction_doesnt_exists(transactions, tx)?;
                let client = utils::get_or_insert_client(clients, *client);
                client.withdraw(*amount)?;
            },
            Transaction::Dispute(Record { tx, .. }) => {
                let TransactionWrapper {
                    transaction: prev_transaction,
                    disputed,
                } = utils::get_transaction_wrapper(transactions, tx)?;
                if !*disputed {
                    let Charge { client, amount, .. } =
                        utils::as_deposit(prev_transaction)?;
                    let client = utils::get_client(clients, client)?;
                    client.dispute(*amount)?;
                    *disputed = true;
                };
            },
            Transaction::Resolve(Record { tx, .. }) => {
                let TransactionWrapper {
                    transaction: prev_transaction,
                    disputed,
                } = utils::get_transaction_wrapper(transactions, tx)?;
                if *disputed {
                    let Charge { client, amount, .. } =
                        utils::as_deposit(prev_transaction)?;
                    let client = utils::get_client(clients, client)?;
                    client.resolve(*amount)?;
                    *disputed = false;
                };
            },
            Transaction::Chargeback(Record { tx, .. }) => {
                let TransactionWrapper {
                    transaction: prev_transaction,
                    disputed,
                } = utils::get_transaction_wrapper(transactions, tx)?;
                if !*disputed {
                    let Charge { client, amount, .. } =
                        utils::as_deposit(prev_transaction)?;
                    let client = utils::get_client(clients, client)?;
                    client.charge_back(*amount)?;
                    *disputed = false;
                };
            },
        };
        let tx = transaction.tx();
        utils::insert_transaction(transactions, tx, transaction);
        Ok(())
    }
}
