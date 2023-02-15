multiversx_sc::imports!();

pub mod permissions;

mod pause_proxy {
    multiversx_sc::imports!();

    #[multiversx_sc::proxy]
    pub trait Pausable {
        #[endpoint]
        fn pause(&self);

        #[endpoint]
        fn unpause(&self);
    }
}

#[multiversx_sc::module]
pub trait PausableModule: permissions::PermissionsModule {
    #[init]
    fn init(&self) {
        self.admins().insert(self.blockchain().get_caller());
    }

    #[endpoint(addContracts)]
    fn add_contracts(&self, contracts: MultiValueEncoded<ManagedAddress>) {
        self.require_admin();
        self.contracts().insert(contracts);
    }

    #[endpoint(removeContracts)]
    fn remove_contracts(&self, contracts: MultiValueEncoded<ManagedAddress>) {
        self.require_admin();
        self.contracts().swap_remove(contracts);
    }

    fn for_each_contract<F>(&self, f: F)
    where
        F: Fn(pause_proxy::Proxy<Self::Api>),
    {
        for contract_address in self.contracts().iter() {
            f(self.pausable_contract().contract(contract_address));
        }
    }

    #[endpoint]
    fn pause(&self) {
        self.require_pauser();
        self.for_each_contract(|mut contract| contract.pause().execute_on_dest_context());
    }

    #[endpoint]
    fn unpause(&self) {
        self.require_pauser();
        self.for_each_contract(|mut contract| contract.unpause().execute_on_dest_context());
    }

    #[view]
    #[storage_mapper("contracts")]
    fn contracts(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[proxy]
    fn pausable_contract(&self) -> pause_proxy::Proxy<Self::Api>;
}
