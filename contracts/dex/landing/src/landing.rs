#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub mod storage;
use storage::Loan;
mod pauser;

#[multiversx_sc::contract]
pub trait EnLanding {
    #[init]
    fn init(&self) {
        self.admins().insert(self.blockchain().get_caller());
    }

    fn get_loan(&self, loan_id: usize) -> &Loan {
      loans_by_id.get(loan_id);
    }

    // fn borrow(
    //     &self,
    //     amount: &BigUint,
    //     token: &EgldOrEsdtTokenIdentifier,
    //     lender: MultiValueEncoded<ManagedAddress>,
    //     receiver: MultiValueEncoded<ManagedAddress>,
    //     opId: usize,
    // ) {
    //   require!(lenders_list.contains(lender), "Lender is not authorized");
    //   let loan_id = assign_loan_id();
    //   let loan = Loan {
    //     amount,
    //     token,
    //     lender
    //   };
    //   loans_by_id(loan_id);
    //   assign_loan_id() += 1;
    // }

    // fn repay() {}

    fn authorize_lender(&self, lender: MultiValueEncoded<ManagedAddress>) -> bool {
      self.require_admin();
      lenders_list.insert(lender);
    }

    fn disable_lender(&self, lender: MultiValueEncoded<ManagedAddress>) -> bool {
      self.require_admin();
      require!(lenders_list.contains(lender), "Already disabled");
      lenders_list.remove(lender);
    }

    #[event("getLoan")]
    fn event_get_loan(&self, #[indexed] loanId: u32, #[indexed] opId: u32);

    #[event("repayLoan")]
    fn event_repay_loan(&self, #[indexed] loanId: u32, #[indexed] opId: u32);
}
