multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait PermissionsModule {
    #[init]
    fn init(&self) {
        self.admins().insert(self.blockchain().get_caller());
    }

    // ADMIN
    #[endpoint(addAdmin)]
    fn add_admin(&self, admin: ManagedAddress) {
        self.require_admin();
        self.admins().insert(admin);
    }

    #[endpoint(removeAdmin)]
    fn remove_admin(&self, admin: ManagedAddress) {
        self.require_admin();
        self.admins().swap_remove(admin);
    }

    fn require_admin(&self) {
        require!(
            self.admins().contains(&self.blockchain().get_caller()),
            "caller is not an admin"
        );
    }

    // PAUSER
    #[endpoint(addPauser)]
    fn add_pauser(&self, pauser: ManagedAddress) {
        self.require_admin();
        self.pausers().insert(pauser);
    }

    #[endpoint(removePauser)]
    fn remove_pauser(&self, pauser: ManagedAddress) {
        self.require_admin();
        self.pausers().swap_remove(pauser);
    }

    fn require_pauser(&self) {
        require!(
            self.pausers().contains(&self.blockchain().get_caller()),
            "caller is not an pauser"
        );
    }

    // DEPOSITER
    #[endpoint(addDepositer)]
    fn add_depositer(&self, depositer: ManagedAddress) {
        self.require_admin();
        self.depositers().insert(depositer);
    }

    #[endpoint(removeDepositer)]
    fn remove_depositer(&self, depositer: ManagedAddress) {
        self.require_admin();
        self.depositers().swap_remove(depositer);
    }

    fn require_depositer(&self) {
        require!(
            self.depositers().contains(&self.blockchain().get_caller()),
            "caller is not an depositer"
        );
    }

    // BORROWER
    #[endpoint(addBorrower)]
    fn add_borrower(&self, borrower: ManagedAddress) {
        self.require_admin();
        self.borrowers().insert(borrower);
    }

    #[endpoint(removeBorrower)]
    fn remove_admin(&self, borrower: ManagedAddress) {
        self.require_admin();
        self.borrowers().swap_remove(borrower);
    }

    fn require_borrower(&self) {
        require!(
            self.borrowers().contains(&self.blockchain().get_caller()),
            "caller is not an borrower"
        );
    }

    #[view]
    #[storage_mapper("admins")]
    fn admins(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[view]
    #[storage_mapper("pausers")]
    fn pausers(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[view]
    #[storage_mapper("depositers")]
    fn depositers(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[view]
    #[storage_mapper("borrowers")]
    fn borrowers(&self) -> UnorderedSetMapper<ManagedAddress>;
}
