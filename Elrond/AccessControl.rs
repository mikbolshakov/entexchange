multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait AccessControl {
    #[init]
    fn init(&self) {
        self.admins().insert(self.blockchain().get_caller());
    }

    // ADMIN
    #[endpoint(addAdmins)]
    fn add_admins(&self, admins: MultiValueEncoded<ManagedAddress>) {
        self.require_admin();
        self.admins().extend(admins);
    }

    #[endpoint(removeAdmins)]
    fn remove_admins(&self, admins: MultiValueEncoded<ManagedAddress>) {
        self.require_admin();
        self.admins().remove_all(admins);
    }

    fn require_admin(&self) {
        require!(
            self.admins().contains(&self.blockchain().get_caller()),
            "caller is not an admin"
        );
    }

    // PAUSER
    #[endpoint(addPausers)]
    fn add_pausers(&self, pausers: MultiValueEncoded<ManagedAddress>) {
        self.require_admin();
        self.pausers().extend(pausers);
    }

    #[endpoint(removePausers)]
    fn remove_pausers(&self, pausers: MultiValueEncoded<ManagedAddress>) {
        self.require_admin();
        self.pausers().remove_all(pausers);
    }

    fn require_pauser(&self) {
        require!(
            self.pausers().contains(&self.blockchain().get_caller()),
            "caller is not an pauser"
        );
    }

    // DEPOSITER
    #[endpoint(addDepositers)]
    fn add_depositers(&self, depositers: MultiValueEncoded<ManagedAddress>) {
        self.require_admin();
        self.depositers().extend(depositers);
    }

    #[endpoint(removeDepositers)]
    fn remove_depositers(&self, depositers: MultiValueEncoded<ManagedAddress>) {
        self.require_admin();
        self.depositers().remove_all(depositers);
    }

    fn require_depositer(&self) {
        require!(
            self.depositers().contains(&self.blockchain().get_caller()),
            "caller is not an depositer"
        );
    }

    // BORROWER
    #[endpoint(addBorrowers)]
    fn add_borrowers(&self, borrowers: MultiValueEncoded<ManagedAddress>) {
        self.require_admin();
        self.borrowers().extend(borrowers);
    }

    #[endpoint(removeBorrowers)]
    fn remove_admins(&self, borrowers: MultiValueEncoded<ManagedAddress>) {
        self.require_admin();
        self.borrowers().remove_all(borrowers);
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
