use crate::*;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

static VERIFICATION_FEE_TEXT: &str = "0.1";
const BASE: u128 = 10;
const VERIFICATION_FEE_YOCTO: u128 = 100 * BASE.pow(21);

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Permit {
    user_id: UserId,
    account_id: AccountId,
}

pub trait PermitVerifier {
    fn permit_request(&mut self, user_id: UserId);
    fn permits_granted(&mut self, permits: Vec<Permit>);
    fn permits_rejected(&mut self, permit: Vec<Permit>);
    fn get_oracle_permits_to_verify(&self) -> Vec<Permit>;
    fn permit_for_user(&self, user_id: UserId) -> Option<Permit>;
}


#[near_bindgen]
impl PermitVerifier for Contract {
    #[payable]
    fn permit_request(&mut self, user_id: UserId) {
        // require enough amount for this and oracle actions
        if near_sdk::env::attached_deposit() != VERIFICATION_FEE_YOCTO {
            let deposit_msg = format!("Please deposit exactly {} NEAR for paying the verification process.", VERIFICATION_FEE_TEXT);
            near_sdk::env::panic_str(&deposit_msg);
        }
        let account_id = env::signer_account_id();
        self.permits_to_verify.insert(&user_id, &account_id);
    }

    #[payable]
    fn permits_granted(&mut self, permits: Vec<Permit>) {
        let oracle_id = env::signer_account_id();
        if oracle_id != self.owner_id {
			env::panic_str("Unauthorized");
		}
        for permit in permits.iter() {
            // TODO send event
            self.permits_granted.remove(&permit.user_id);
            self.permits_granted.insert(&permit.user_id, &permit.account_id);
            self.permits_to_verify.remove(&permit.user_id);
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
            self.permits_to_verify.remove(&permit.user_id);
        }
    }

    fn get_oracle_permits_to_verify(&self) -> Vec<Permit> {
        self.permits_to_verify.iter()
        .take(100)
        .map(|(user_id, account_id)| Permit{user_id: user_id, account_id: account_id})
        .collect()
    }

    //Query for all the granted permits
    fn permit_for_user(&self, user_id: UserId) -> Option<Permit> {
        //get the permit for the passed in user
        let account_id = self.permits_granted.get(&user_id);
        if account_id.is_none() {
            None
        } else {
            Some(Permit{user_id: user_id, account_id: account_id.unwrap()})
        }
    }
}
