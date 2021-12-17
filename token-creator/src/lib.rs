use scrypto::prelude::*;

blueprint! {
    struct TokenCreator {
        token_vault: Vault
    }

    impl TokenCreator {
        pub fn new() -> (Component, Bucket) {
            // Create a badge with fixed supply of 1
            let badge : Bucket = ResourceBuilder::new_fungible(DIVISIBILITY_NONE)
                .metadata("name", "Access Badge")
                .metadata("symbol", "TB")
                .metadata("icon_url", "https://badge_website.com/icon.ico")
                .metadata("url", "https://badge_website.com")
                .initial_supply_fungible(1);

            // Create tokens with fixed supply of 1 000 000 000
            let tokens = ResourceBuilder::new_fungible(DIVISIBILITY_MAXIMUM)
                .metadata("name", "Really Cool Token")
                .metadata("symbol", "RCT")
                .initial_supply_fungible(1_000_000_000);

            let component = Self {
                token_vault: Vault::with_bucket(tokens)
            }
            .instantiate();

            // Don't forget! At the end of a function of method,
            // all buckets must be stored in a vault or returned.
            (component, badge)
        }

        pub fn describe_resource(&self, addr: Address) {
            // Send up the badge or token address to learn more about it.
            // Of course you could just do "resim show $my_address" but
            // this shows how to get the metadata with Scrypto.

            let this_res_def = ResourceDef::from(addr);
            let metadata = this_res_def.metadata();
            if metadata.contains_key("name") && metadata.contains_key("symbol") {
                info!( "name: {}, symbol: {}", metadata["name"], metadata["symbol"]);
            }
            if metadata.contains_key("description") {
                info!( "description: {}", metadata["description"]);
            }
            if metadata.contains_key("icon_url") && metadata.contains_key("url") {
                info!( "icon_url: {}, url: {}", metadata["icon_url"], metadata["url"]);
            }
            info!( "supply: {}", this_res_def.total_supply());
        }
    }
}
