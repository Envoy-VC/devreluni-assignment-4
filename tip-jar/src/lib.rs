#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use alloy_primitives::{Address, U256};
use alloy_sol_types::sol;

use stylus_sdk::{
    call::{call, Call, Error},
    evm, msg,
    prelude::*,
};

sol! {
    event TipUser(address indexed to, uint256 amount);
    event WithdrawTip(address indexed to, uint256 amount);

    error TipFailed(address to, uint256 amount);
    error InsufficientBalance(address from, uint256 amount);
    error WithdrawErrorNoFunds();
    error WithdrawError(address to, uint256 amount);
}

sol_storage! {
    #[entrypoint]
    pub struct TipJar {
        mapping(address => uint256) balances;
    }
}

#[derive(SolidityError)]
pub enum TipJarErrors {
    TipFailed(TipFailed),
    InsufficientBalance(InsufficientBalance),
    WithdrawErrorNoFunds(WithdrawErrorNoFunds),
    WithdrawError(WithdrawError),
}

#[public]
impl TipJar {
    pub fn get_balance(&self, address: Address) -> U256 {
        self.balances.get(address)
    }

    #[payable]
    pub fn tip(&mut self, to: Address, amount: U256) -> Result<(), TipJarErrors> {
        let msg_value: U256 = msg::value();
        if msg_value < amount {
            return Err(TipJarErrors::InsufficientBalance(InsufficientBalance {
                from: msg::sender(),
                amount,
            }));
        }
        let balance: U256 = self.balances.get(to);
        self.balances.insert(to, balance + amount);
        evm::log(TipUser { to, amount });

        Ok(())
    }

    pub fn withdraw(&mut self, user: Address) -> Result<(), TipJarErrors> {
        let balance: U256 = self.balances.get(user);
        if balance == U256::from(0) {
            return Err(TipJarErrors::WithdrawErrorNoFunds(WithdrawErrorNoFunds {}));
        }
        let result: Result<Vec<u8>, Error> = call(Call::new_in(self).value(balance), user, &[]);
        if result.is_err() {
            return Err(TipJarErrors::WithdrawError(WithdrawError {
                to: user,
                amount: balance,
            }));
        }
        self.balances.replace(user, U256::from(0));
        evm::log(WithdrawTip {
            to: user,
            amount: balance,
        });
        Ok(())
    }
}
