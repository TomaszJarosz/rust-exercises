use std::collections::HashMap;
use std::hash::Hash;
use num::traits::{CheckedAdd, CheckedSub, Zero};
use crate::Trait;

pub struct BalanceModule<T: Trait> {
    balances: HashMap<T::AccountId, T::Balance>,
}

impl<T: Trait> BalanceModule<T> {
    pub fn new() -> Self {
        Self {
            balances: HashMap::new()
        }
    }

    pub fn set_balances(&mut self, who: T::AccountId, amount: T::Balance) {
        self.balances.insert(who, amount);
    }

    pub fn balance(&self, who: T::AccountId) -> T::Balance {
        let zero = &T::Balance::zero();
        *self.balances.get(&who).unwrap_or(zero)
    }

    pub fn transfer(&mut self, from: T::AccountId, to: T::AccountId, amount: T::Balance) -> Result<(), &'static str> {
        let from_balance = self.balances.get(&from).ok_or("from user does not exist")?;
        let zero = &T::Balance::zero();
        let to_balance = self.balances.get(&to).unwrap_or(zero);

        let new_from_balance = from_balance.checked_sub(&amount).ok_or("Not enough funds")?;
        let new_to_balance = to_balance.checked_add(&amount).ok_or("Overflow")?;

        self.balances.insert(from, new_from_balance);
        self.balances.insert(to, new_to_balance);
        Ok(())
    }
}

pub struct VotingModule<T: Trait> {
    votes: HashMap<(T::AccountId, T::VoteIndex), bool>,
}

impl<T: Trait> VotingModule<T> {
    pub fn new() -> Self {
        Self {
            votes: HashMap::new()
        }
    }

    pub fn vote(&mut self, who: T::AccountId, index: T::VoteIndex, vote: bool) {
        self.votes.insert((who, index), vote);
    }

    pub fn get_vote(self, who: T::AccountId, index: T::VoteIndex) -> bool {
        *self.votes.get(&(who, index)).unwrap_or(&false)
    }
}