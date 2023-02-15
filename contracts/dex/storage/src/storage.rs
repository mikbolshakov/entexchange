multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi)]
pub struct Loan<M: ManagedTypeApi> {
  pub amount: BigUint<M>,
  pub token: EgldOrEsdtTokenIdentifier<M>,
  pub lender: ManagedAddress<M>,
}

#[multiversx_sc::module]
pub trait StorageModule {
    #[storage_mapper("loansById")]
    fn loans_by_id(&self, loan_id: usize) -> VecMapper<Loan<Self::Api>>;

    #[view(getLendersListByBool)]
    #[storage_mapper("lendersList")]
    fn lenders_list(&self, address: &ManagedAddress) -> UnorderedSetMapper<bool>;

    #[view(assignLoanId)]
    #[storage_mapper("assignLoanId")]
    fn assign_loan_id(&self) -> SingleValueMapper<u64>;
}

