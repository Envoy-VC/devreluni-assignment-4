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

    error InvalidTipAmount();
    error NoFundsToWithdraw();
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
    InvalidTipAmount(InvalidTipAmount),
    NoFundsToWithdraw(NoFundsToWithdraw),
    WithdrawError(WithdrawError),
}

#[public]
impl TipJar {
    pub fn get_balance(&self, address: Address) -> U256 {
        self.balances.get(address)
    }

    #[payable]
    pub fn tip(&mut self, to: Address) -> Result<(), TipJarErrors> {
        let msg_value: U256 = msg::value();
        if msg_value <= U256::from(0) {
            return Err(TipJarErrors::InvalidTipAmount(InvalidTipAmount {}));
        }
        let balance: U256 = self.balances.get(to);
        self.balances.insert(to, balance + msg_value);
        evm::log(TipUser {
            to,
            amount: msg_value,
        });

        Ok(())
    }

    pub fn withdraw(&mut self, user: Address) -> Result<(), TipJarErrors> {
        let balance: U256 = self.balances.get(user);
        if balance == U256::from(0) {
            return Err(TipJarErrors::NoFundsToWithdraw(NoFundsToWithdraw {}));
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
