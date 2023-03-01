use crate::*;

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn nft_mint(
        &mut self,
        token_id: TokenId,
        metadata: TokenMetadata,
        receiver_id: AccountId,
    ) {
        //measure the initial storage being used on the contract
        let initial_storage_usage = env::storage_usage();

        //specify the token struct that contains the owner ID
        let token = Token {
            //set the owner ID equal to the receiver ID passed into the function
            owner_id: receiver_id,
        };

        //insert the token ID and token struct and make sure that the token doesn't exist
        assert!(
            self.tokens_by_id.insert(&token_id, &token).is_none(),
            "Token already exists"
        );

        // TODO check what happens (or should happen) if someone tries to mint several NFTs for the same permit
        //verify granted permits
        let user_id = metadata.user_id.clone();
        if self.permits_granted.get(&token.owner_id) != Some(user_id) {
            env::panic_str("Unauthorized");
        }

        //insert the token ID and metadata
        self.token_metadata_by_id.insert(&token_id, &metadata);

        //call the internal method for adding the token to the owner
        self.internal_add_token_to_owner(&token.owner_id, &token_id);
        //call the internal method for adding the token to the user permanently
        let user_id = metadata.user_id.clone();
        self.internal_add_token_to_user(&user_id, &token_id);

        //calculate the required storage which was the used - initial
        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;

        //refund any excess storage if the user attached too much. Panic if they didn't attach enough to cover the required.
        refund_deposit(required_storage_in_bytes);
    }
}
