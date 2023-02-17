use crate::*;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

static VERIFICATION_FEE_TEXT: &str = "0.1";
const base: u128 = 10;
const VERIFICATION_FEE_YOCTO: u128 = 100 * base.pow(21);

pub type UserId = String;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Permit {
    account_id: AccountId,
    user_id: UserId,
}

pub trait PermitVerifier {
    fn verify_permit(&mut self, user_id: UserId);
    fn permits_granted(&mut self, permits: Vec<Permit>);
    fn permits_rejected(&mut self, permit: Vec<Permit>);
    fn get_oracle_permits_to_verify(&self) -> Vec<Permit>;
}


#[near_bindgen]
impl PermitVerifier for Contract {
    #[payable]
    fn verify_permit(&mut self, user_id: UserId) {
        // require enough amount for this and oracle actions
        if near_sdk::env::attached_deposit() != VERIFICATION_FEE_YOCTO {
            let deposit_msg = format!("Please deposit exactly {} NEAR for paying the verification process.", VERIFICATION_FEE_TEXT);
            near_sdk::env::panic_str(&deposit_msg);
        }
        let account_id = env::signer_account_id();
        self.permits_to_verify.insert(&account_id, &user_id);
    }

    #[payable]
    fn permits_granted(&mut self, permits: Vec<Permit>) {
        let oracle_id = env::signer_account_id();
        if oracle_id != self.owner_id {
			env::panic_str("Unauthorized");
		}
        for permit in permits.iter() {
            // TODO send event
            self.permits_granted.insert(&permit.account_id, &permit.user_id);
            self.permits_to_verify.remove(&permit.account_id);
        }
    }

    #[payable]
    fn permits_rejected(&mut self, permits: Vec<Permit>) {
        let oracle_id = env::signer_account_id();
        if oracle_id != self.owner_id {
			env::panic_str("Unauthorized");
		}
        for permit in permits.iter() {
            // TODO send event
            self.permits_to_verify.remove(&permit.account_id);
        }
    }

    fn get_oracle_permits_to_verify(&self) -> Vec<Permit> {
        self.permits_to_verify.iter()
        .take(100)
        .map(|(account_id, user_id)| Permit{account_id: account_id, user_id: user_id})
        .collect()
    }
}
