use scrypto::prelude::*;

blueprint! {
    struct TokenCreator {
        minter_badge: Vault,
        token_vault: Vault
    }

    impl TokenCreator {
        pub fn new() -> (Component, Bucket) {
            let minter_badge = ResourceBuilder::new()
                .new_badge_fixed(1);

            // Create a mutable supply token and specify
            // the resource definition of the badge allowed to mint and burn.
            // Notice that new_token_mutable does not return a bucket but only a resource_definition.
            let token_resource_def = ResourceBuilder::new()
                .metadata("name", "Really Cool Token - but mutable")
                .metadata("symbol", "RCTM")
                .new_token_mutable(minter_badge.resource_def());

            // Now we can mint tokens
            let tokens = token_resource_def.mint(1000, minter_badge.borrow());

            // It's the same when creating mutable badges
            let badge_resource_def = ResourceBuilder::new()
                .metadata("name", "Mutable Badge")
                .new_badge_mutable(minter_badge.resource_def());

            let badge = badge_resource_def.mint(1, minter_badge.borrow());

            let component = Self {
                minter_badge: Vault::with_bucket(minter_badge),
                token_vault: Vault::with_bucket(tokens)
            }
            .instantiate();

            (component, badge)
        }

        pub fn burn_badge(&mut self, badge_to_burn: Bucket) {
            // Take the minter badge out of the vault
            let badge_bucket = self.minter_badge.take(1);

            // Burn the provided badge
            badge_to_burn.burn(badge_bucket.borrow());

            // Put the badge back into its vault
            self.minter_badge.put(badge_bucket);
        }

        pub fn mint_tokens(&mut self) -> Bucket {
            // Take the badge out of the vault
            let badge_bucket = self.minter_badge.take(1);

            // Let's mint 100 RCTM
            let bucket = self.token_vault.resource_def().mint(100, badge_bucket.borrow());

            // Put the badge back into its vault
            self.minter_badge.put(badge_bucket);

            // Return the minted tokens
            bucket
        }

        pub fn auth_burn_badge(&mut self, badge_to_burn: Bucket) {
            self.minter_badge.authorize(|badge| {
                badge_to_burn.burn(badge);
            });
        }

        pub fn auth_mint_tokens(&mut self) -> Bucket {
            self.minter_badge.authorize(|badge| {
                self.token_vault.resource_def().mint(100, badge)
            })
            // Notice, there is no semicolon. The created bucket will be returned.
        }
    }
}
