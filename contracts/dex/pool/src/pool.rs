#![no_std]

multiversx_sc::imports!();

pub mod storage;
mod pauser;

#[multiversx_sc::contract]
pub trait Pool: Pauser::Pauser {
    #[init]
    fn init(&self) {
        self.admins().insert(self.blockchain().get_caller());
    }

    #[payable("*")]
    #[endpoint(depositToken)]
    fn deposit_token(&self, amount: &BigUint, token: &EgldOrEsdtTokenIdentifier, opId: usize) {
        self.require_depositer();
        let (token, token_nonce, amount) = self.call_value().single_esdt().into_tuple();

        self.deposit_event(amount, token, opId);
    }

    #[endpoint(withdrawToken)]
    fn withdraw_token(
        &self,
        to: &ManagedAddress,
        token: &EgldOrEsdtTokenIdentifier,
        amount: &BigUint,
        opId: usize,
    ) {
        self.require_depositer();
        self.send().direct_esdt(to, token, 0, amount);

        self.withdraw_event(amount, token, opId);
    }

    #[event("depositEvent")]
    fn deposit_event(
        &self,
        #[indexed] amount: u32,
        #[indexed] token: &EgldOrEsdtTokenIdentifier,
        #[indexed] opId: u32,
    );

    #[event("withdrawEvent")]
    fn withdraw_event(
        &self,
        #[indexed] amount: u32,
        #[indexed] token: &EgldOrEsdtTokenIdentifier,
        #[indexed] opId: u32,
    );
}
