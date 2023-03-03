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
        let extra_string = metadata.extra.as_ref().ok_or("Unauthorized").unwrap();

        //measure the initial storage being used on the contract
        let initial_storage_usage = env::storage_usage();

        let extra: TokenMetadataExtra = serde_json::from_str(&extra_string).unwrap();
        let user_id = extra.user_id.clone();

        //specify the token struct that contains the owner ID
        let token = Token {
            //set the owner ID equal to the receiver ID passed into the function
            owner_id: receiver_id,
            user_id: extra.user_id.clone()
        };

        //insert the token ID and token struct and make sure that the token doesn't exist
        assert!(
            self.tokens_by_id.insert(&token_id, &token).is_none(),
            "Token already exists"
        );

        // TODO check what happens (or should happen) if someone tries to mint several NFTs for the same permit
        //verify granted permits
        if self.permits_granted.get(&token.owner_id) != Some(user_id) {
            env::panic_str("Unauthorized");
        }

        //insert the token ID and metadata
        self.token_metadata_by_id.insert(&token_id, &metadata);

        //call the internal method for adding the token to the owner
        self.internal_add_token_to_owner(&token.owner_id, &token_id);
        //call the internal method for adding the token to the user permanently
        let user_id = extra.user_id.clone();
        self.internal_add_token_to_user(&user_id, &token_id);

        //calculate the required storage which was the used - initial
        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;

        //refund any excess storage if the user attached too much. Panic if they didn't attach enough to cover the required.
        refund_deposit(required_storage_in_bytes);
    }

    fn nft_index_for_owner(&self, owner_id: AccountId) -> u64 {
        let tokens_for_owner_set = self.tokens_per_owner.get(&owner_id);
        if let Some(tokens_for_owner_set) = tokens_for_owner_set {
            tokens_for_owner_set.len()
        } else {
            //if there isn't a set of tokens for the passed in account ID, we'll return 0
            0
        }
    }

    #[payable]
    pub fn nft_batch_mint(
        &mut self,
        metadata: TokenMetadata,
        owner_id: AccountId,
        num_to_mint: i8
    ) {
        if num_to_mint > 1 {
            env::panic_str("Only one NFT mint per call allowed");
        }

        let extra_string = metadata.extra.as_ref().ok_or("Unauthorized").unwrap();

        let owner_id_string: String = owner_id.to_string();
        let nft_index = self.nft_index_for_owner(owner_id.clone());
        let token_id = format!("{}-{}", owner_id_string, nft_index);

        //measure the initial storage being used on the contract
        let initial_storage_usage = env::storage_usage();

        let extra: TokenMetadataExtra = serde_json::from_str(&extra_string).unwrap();
        let user_id = extra.user_id.clone();

        //specify the token struct that contains the owner ID
        let token = Token {
            //set the owner ID equal to the receiver ID passed into the function
            owner_id: owner_id,
            user_id: user_id
        };

        //insert the token ID and token struct and make sure that the token doesn't exist
        assert!(
            self.tokens_by_id.insert(&token_id, &token).is_none(),
            "Token already exists"
        );

        // TODO check what happens (or should happen) if someone tries to mint several NFTs for the same permit
        //verify granted permits
        let user_id = extra.user_id.clone();
        if self.permits_granted.get(&token.owner_id) != Some(user_id) {
            env::panic_str("Unauthorized");
        }

        //insert the token ID and metadata
        self.token_metadata_by_id.insert(&token_id, &metadata);

        //call the internal method for adding the token to the owner
        self.internal_add_token_to_owner(&token.owner_id, &token_id);
        //call the internal method for adding the token to the user permanently
        let user_id = extra.user_id.clone();
        self.internal_add_token_to_user(&user_id, &token_id);

        //calculate the required storage which was the used - initial
        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;

        //refund any excess storage if the user attached too much. Panic if they didn't attach enough to cover the required.
        refund_deposit(required_storage_in_bytes);
    }
}
