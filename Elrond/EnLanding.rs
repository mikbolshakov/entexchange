#![no_std]

multiversx_sc::imports!();

mod Pauser;

struct Loan {
    amount: &BigUint,
    token: &EgldOrEsdtTokenIdentifier,
    lender: MultiValueEncoded<ManagedAddress>,
}

#[multiversx_sc::contract]
pub trait EnLanding: Pauser::Pauser {
    #[init]
    fn init(&self) {
        self.admins().insert(self.blockchain().get_caller());
    }

    fn getLoan(loanId: usize) -> &struct {
      loans.get(loanId);
    }

    fn borrow(
        amount: &BigUint,
        token: &EgldOrEsdtTokenIdentifier,
        lender: MultiValueEncoded<ManagedAddress>,
        receiver: MultiValueEncoded<ManagedAddress>,
        opId: usize,
    ) {
      require!(lenders.contains(lender), "Lender is not authorized");

    }

    fn repay() {}

    fn authorizeLender(lender: MultiValueEncoded<ManagedAddress>) -> bool {
      self.require_admin();
      lenders.insert(lender);
    }

    fn disableLender(lender: MultiValueEncoded<ManagedAddress>) -> bool {
      self.require_admin();
      require!(lenders.contains(lender), "Already disabled");
      lenders.remove(lender);
    }

    #[event("getLoan")]
    fn get_loan(&self, #[indexed] loanId: u32, #[indexed] opId: u32);

    #[event("repayLoan")]
    fn repay_loan(&self, #[indexed] loanId: u32, #[indexed] opId: u32);

    #[view]
    #[storage_mapper("lenders")]
    fn lenders(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[view]
    #[storage_mapper("loans")]
    fn loans(&self, loanId: usize) -> VecMapper<&struct>;
}
