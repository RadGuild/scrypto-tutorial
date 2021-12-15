use scrypto::prelude::*;

blueprint! {
    struct CandyStore {
        // token address mapped to vault holding that token
        candy_vaults: HashMap<Address, Vault>,

        collected_xrd: Vault,

        // token address mapped to the price in XRD for that token
        prices: HashMap<Address, Decimal>,

        // secure selected functions with a badge
        owners_badge: Address,

        // track the total XRD claimed thus far
        total_claimed: Decimal,
    }

    impl CandyStore {

        pub fn new() -> (Bucket, Component) {
            // Create the badge
            let badge_bucket: Bucket = ResourceBuilder::new_fungible(DIVISIBILITY_NONE)
                .metadata("name", "Store Owner's Badge")
                .initial_supply_fungible(1);
            let component = Self {
                candy_vaults: HashMap::new(),
                collected_xrd: Vault::new(RADIX_TOKEN),
                prices: HashMap::new(),
                owners_badge: badge_bucket.resource_address(),
                total_claimed: 0.into(),
            }
            .instantiate();
            // Return the badge and the component
            (badge_bucket, component)
        }

        #[auth(owners_badge)]
        pub fn stock_candy(&mut self, candy: Bucket, new_price: Decimal ) {
            let candy_addr: Address = candy.resource_address();

            // We can write assertions. If it fails, the whole transaction is safely rolled back.
            // Here, we make sure that the provided bucket does not contain XRD
            // and that the price is greater than zero.
            assert!( candy_addr != self.collected_xrd.resource_address(), "cannot stock XRD as candy");
            assert!(new_price > 0.into(), "new price must be a positive value");

            // Try to find the vault with candy_addr as key.
            // If it does not exist, it creates a new empty vault.
            let v = self.candy_vaults.entry(candy_addr).or_insert(Vault::new(candy_addr));

            // Insert the candies in the vault
            v.put(candy);

            // Update the price
            self.prices.insert(candy_addr, new_price);
        }

        pub fn get_price(&self, candy_addr: Address) {
            // Make sure the candy_addr is not XRD
            assert!( candy_addr != self.collected_xrd.resource_address(), "XRD is priceless");

            // Display the price if present, display error otherwise
            match self.prices.get(&candy_addr) {
                Some(price) => info!("Price: {}", price),
                None => info!("Could not find candy in stock !")
            };
        }

        pub fn menu(&self) -> Vec<Bucket> {
            let mut buckets = Vec::new();
            for (addr, vault) in self.candy_vaults.iter() {
                buckets.push(vault.take(0));
            }
            buckets
        }

        pub fn buy_candy(&mut self, candy_addr: Address, payment: Bucket) -> (Bucket, Bucket) {
            // take our price in XRD out of the payment
            // if the caller has sent too few, or sent something other than XRD, they'll get a runtime error
            let price = match self.prices.get(&candy_addr) {
                Some(price) => price,
                None => {
                    info!("Candy not in stock !");
                    std::process::abort()
                }
            };

            self.collected_xrd.put(payment.take(*price));

            let candy_bucket: Bucket = match self.candy_vaults.get(&candy_addr) {
                Some(vault) => vault.take(1),
                None => {
                    info!("Candy not in stock !");
                    std::process::abort()
                }
            };

            (candy_bucket, payment)
        }

        #[auth(owners_badge)]
        pub fn claim (&mut self) -> Bucket {
            self.total_claimed += self.collected_xrd.amount();
            self.collected_xrd.take_all()
        }
    }
}
