use crate::*;

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn nft_burn(&mut self, token_id: TokenId) {
        assert_one_yocto();
        let owner_id = env::predecessor_account_id();

        let token = self.tokens_by_id.get(&token_id).expect("No token");

        assert_eq!(
            &token.owner_id, &owner_id,
            "The token owner and the receiver must match"
        );

        //verify granted permits
        let user_id = token.user_id.clone();
        if self.permits_granted.get(&owner_id) != Some(user_id) {
            env::panic_str("Unauthorized");
        }

        //we remove the token from it's current owner's set
        self.internal_remove_token_from_owner(&token.owner_id, &token_id);
        //call the internal method for removing the token for the user
        let user_id = token.user_id.clone();
        self.internal_remove_token_from_user(&user_id, &token_id);

        self.tokens_by_id.remove(&token_id);
        self.token_metadata_by_id.remove(&token_id);
    }
}
