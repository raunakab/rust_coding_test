#[cfg(test)]
mod tests;
mod utils;

use std::collections::BTreeMap;

use crate::client::Client;
use crate::transaction::Charge;
use crate::transaction::ChargeRef;
use crate::transaction::Transaction;
use crate::types::ClientId;
use crate::types::EngineResult;
use crate::types::TransactionId;

#[cfg_attr(test, derive(Debug))]
struct TransactionWrapper {
    transaction: Transaction,
    disputed: bool,
}

impl TransactionWrapper {
    fn new(transaction: Transaction) -> Self {
        Self {
            transaction,
            disputed: false,
        }
    }
}

#[cfg_attr(test, derive(Debug))]
pub struct Core {
    clients: BTreeMap<ClientId, Client>,
    transactions: BTreeMap<TransactionId, TransactionWrapper>,
}

impl Default for Core {
    fn default() -> Self {
        Self {
            clients: BTreeMap::default(),
            transactions: BTreeMap::default(),
        }
    }
}

impl Core {
    pub fn process(&mut self, transaction: Transaction) -> EngineResult<()> {
        let Self {
            clients,
            transactions,
        } = self;
        macro_rules! charge {
            ($action:ident @ [$client:ident, $tx:ident, $amount:ident]) => {{
                utils::assert_transaction_doesnt_exists(transactions, $tx)?;
                let client = utils::get_or_insert_client(clients, *$client);
                client.$action(*$amount)?;
            }};
        }
        macro_rules! charge_ref {
            ($action:ident @ $tx:ident, disputed := true) => {{
                let TransactionWrapper {
                    transaction: prev_transaction,
                    disputed,
                } = utils::get_transaction_wrapper(transactions, $tx)?;
                if !*disputed {
                    let Charge { client, amount, .. } =
                        utils::as_deposit(prev_transaction)?;
                    let client = utils::get_client(clients, client)?;
                    client.$action(*amount)?;
                    *disputed = true;
                };
            }};
            ($action:ident @ $tx:ident, disputed := false) => {{
                let TransactionWrapper {
                    transaction: prev_transaction,
                    disputed,
                } = utils::get_transaction_wrapper(transactions, $tx)?;
                if *disputed {
                    let Charge { client, amount, .. } =
                        utils::as_deposit(prev_transaction)?;
                    let client = utils::get_client(clients, client)?;
                    client.$action(*amount)?;
                    *disputed = false;
                };
            }};
        }
        match &transaction {
            Transaction::Deposit(Charge { client, tx, amount }) => {
                charge!(deposit @ [client, tx, amount])
            },
            Transaction::Withdrawal(Charge { client, tx, amount }) => {
                charge!(withdraw @ [client, tx, amount])
            },
            Transaction::Dispute(ChargeRef { tx, .. }) => {
                charge_ref!(dispute @ tx, disputed := true)
            },
            Transaction::Resolve(ChargeRef { tx, .. }) => {
                charge_ref!(resolve @ tx, disputed := false)
            },
            Transaction::Chargeback(ChargeRef { tx, .. }) => {
                charge_ref!(charge_back @ tx, disputed := false)
            },
        };
        transaction.charge_tx().map(|tx| {
            let transaction_wrapper = TransactionWrapper::new(transaction);
            transactions.insert(tx, transaction_wrapper);
        });
        Ok(())
    }

    pub fn clients(&self) -> Vec<&Client> {
        self.clients.values().collect()
    }
}
