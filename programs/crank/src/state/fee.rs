use {
    super::Queue,
    anchor_lang::{prelude::*, AnchorDeserialize},
    std::convert::TryFrom,
};

pub const SEED_FEE: &[u8] = b"fee";

/**
 * Fee
 */

#[account]
#[derive(Debug)]
pub struct Fee {
    pub authority: Pubkey,
    pub admin_balance: u64,
    pub worker_balance: u64,
}

impl Fee {
    pub fn pubkey(authority: Pubkey) -> Pubkey {
        Pubkey::find_program_address(&[SEED_FEE, authority.as_ref()], &crate::ID).0
    }
}

impl TryFrom<Vec<u8>> for Fee {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Fee::try_deserialize(&mut data.as_slice())
    }
}

/**
 * FeeAccount
 */

pub trait FeeAccount {
    fn new(&mut self, authority: Pubkey) -> Result<()>;

    fn claim_admin_balance(&mut self, amount: u64, pay_to: &mut SystemAccount) -> Result<()>;

    fn claim_worker_balance(&mut self, amount: u64, pay_to: &mut SystemAccount) -> Result<()>;

    fn pay_to_admin(&mut self, amount: u64, queue: &mut Account<Queue>) -> Result<()>;

    fn pay_to_worker(&mut self, amount: u64, queue: &mut Account<Queue>) -> Result<()>;
}

impl FeeAccount for Account<'_, Fee> {
    fn new(&mut self, authority: Pubkey) -> Result<()> {
        self.authority = authority;
        self.admin_balance = 0;
        self.worker_balance = 0;
        Ok(())
    }

    fn claim_admin_balance(&mut self, amount: u64, pay_to: &mut SystemAccount) -> Result<()> {
        // Withdraw from the admin balance
        self.admin_balance = self.admin_balance.checked_sub(amount).unwrap();

        // Transfer lamports to the pay_to acccount
        **self.to_account_info().try_borrow_mut_lamports()? = self
            .to_account_info()
            .lamports()
            .checked_sub(amount)
            .unwrap();
        **pay_to.to_account_info().try_borrow_mut_lamports()? = pay_to
            .to_account_info()
            .lamports()
            .checked_add(amount)
            .unwrap();

        Ok(())
    }

    fn claim_worker_balance(&mut self, amount: u64, pay_to: &mut SystemAccount) -> Result<()> {
        // Withdraw from the worker amount
        self.worker_balance = self.worker_balance.checked_sub(amount).unwrap();

        // Transfer lamports to the pay_to acccount
        **self.to_account_info().try_borrow_mut_lamports()? = self
            .to_account_info()
            .lamports()
            .checked_sub(amount)
            .unwrap();
        **pay_to.to_account_info().try_borrow_mut_lamports()? = pay_to
            .to_account_info()
            .lamports()
            .checked_add(amount)
            .unwrap();

        Ok(())
    }

    fn pay_to_admin(&mut self, amount: u64, queue: &mut Account<Queue>) -> Result<()> {
        // Transfer balance from queue to fee account
        self.admin_balance = self.admin_balance.checked_add(amount).unwrap();

        // Transfer lamports
        **queue.to_account_info().try_borrow_mut_lamports()? = queue
            .to_account_info()
            .lamports()
            .checked_sub(amount)
            .unwrap();
        **self.to_account_info().try_borrow_mut_lamports()? = self
            .to_account_info()
            .lamports()
            .checked_add(amount)
            .unwrap();

        Ok(())
    }

    fn pay_to_worker(&mut self, amount: u64, queue: &mut Account<Queue>) -> Result<()> {
        // Transfer balance from queue to fee account
        self.worker_balance = self.worker_balance.checked_add(amount).unwrap();

        // Transfer lamports
        **queue.to_account_info().try_borrow_mut_lamports()? = queue
            .to_account_info()
            .lamports()
            .checked_sub(amount)
            .unwrap();
        **self.to_account_info().try_borrow_mut_lamports()? = self
            .to_account_info()
            .lamports()
            .checked_add(amount)
            .unwrap();

        Ok(())
    }
}
