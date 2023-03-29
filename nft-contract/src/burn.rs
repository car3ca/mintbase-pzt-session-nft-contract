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
        let permit_account_id = self.permits_granted.get(&user_id).expect("Unauthorized");
        if permit_account_id != owner_id {
            env::panic_str("Unauthorized");
        }

        //we remove the token from it's current owner's set
        self.internal_remove_token_from_owner(&token.owner_id, &token_id);
        //call the internal method for removing the token for the user
        let user_id = token.user_id.clone();
        self.internal_remove_token_from_user(&user_id, &token_id);

        self.tokens_by_id.remove(&token_id);
        self.token_metadata_by_id.remove(&token_id);

        // Construct the mint log as per the events standard.
        let nft_burn_log: EventLog = EventLog {
            // Standard name ("nep171").
            standard: NFT_STANDARD_NAME.to_string(),
            // Version of the standard ("nft-1.0.0").
            version: NFT_METADATA_SPEC.to_string(),
            // The data related with the event stored in a vector.
            event: EventLogVariant::NftBurn(vec![NftBurnLog {
                // Owner of the token.
                owner_id: token.owner_id.to_string(),
                // Vector of token IDs that were minted.
                token_ids: vec![token_id.to_string()],
            }]),
        };

        // Log the serialized json.
        env::log_str(&nft_burn_log.to_string());
    }
}
