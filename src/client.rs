use std::cmp::Ordering;

use crate::types::Amount;
use crate::types::ClientId;
use crate::types::EngineResult;

#[cfg_attr(test, derive(Debug))]
pub struct Client {
    id: ClientId,
    available: Amount,
    held: Amount,
    locked: bool,
}

impl Client {
    pub fn new(id: u16) -> Self {
        Self {
            id,
            available: 0,
            held: 0,
            locked: false,
        }
    }

    pub fn id(&self) -> ClientId { self.id }

    pub fn available(&self) -> Amount { self.available }

    pub fn held(&self) -> Amount { self.held }

    pub fn locked(&self) -> bool { self.locked }

    pub fn deposit(&mut self, amount: Amount) -> EngineResult<()> {
        self.assert_not_locked()?;
        self.available = self.available + amount;
        Ok(())
    }

    pub fn withdraw(&mut self, amount: Amount) -> EngineResult<()> {
        self.assert_not_locked()?;
        let comparison = self.available.cmp(&amount);
        match comparison {
            Ordering::Equal | Ordering::Greater => {
                self.available = self.available - amount;
                Ok(())
            },
            Ordering::Less => Err("Oops, you cannot withdraw more money than what exists in your available funds."),
        }
    }

    pub fn dispute(&mut self, amount: Amount) -> EngineResult<()> {
        self.assert_not_locked()?;
        let comparison = self.available.cmp(&amount);
        match comparison {
            Ordering::Equal | Ordering::Greater => {
                self.available = self.available - amount;
                self.held = self.held + amount;
                Ok(())
            },
            Ordering::Less => Err("Oops, you cannot dispute a transaction which deals with more money than what exists in your available funds."),
        }
    }

    pub fn resolve(&mut self, amount: Amount) -> EngineResult<()> {
        self.assert_not_locked()?;
        let comparison = self.held.cmp(&amount);
        match comparison {
            Ordering::Equal | Ordering::Greater => {
                self.held = self.held - amount;
                self.available = self.available + amount;
                Ok(())
            },
            Ordering::Less => Err("Oops, this dispute is not able to be resolved because you don't have enough money in your held funds."),
        }
    }

    pub fn charge_back(&mut self, amount: Amount) -> EngineResult<()> {
        self.resolve(amount)?;
        self.withdraw(amount)?;
        self.lock();
        Ok(())
    }

    pub fn lock(&mut self) {
        self.locked = true;
    }

    fn assert_not_locked(&self) -> EngineResult<()> {
        match self.locked {
            true => Err("Oops, this account is locked and actions cannot be performed on it."),
            false => Ok(()),
        }
    }
}
