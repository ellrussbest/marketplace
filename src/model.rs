/// Import `borsh` from `near_sdk` crate 
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
/// Import `serde` from `near_sdk` crate 
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::env;

/// Implements both `serde` and `borsh` serialization.
/// `serde` is typically useful when returning a struct in JSON format for a frontend.
// This Struct should act as a template definition to all my products
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Product {
    pub id: String, // they are publicly accessible
    pub name: String,
    pub description: String,
    pub image: String,
    pub location: String,
    pub price: u128,
    pub owner: String,
    pub sold: u32,
}


// Implementation of the above struct defitinition
impl Product {

    // increments the sold amount when a specific product is bought
    #[allow(non_snake_case)]
    pub fn incrementSoldAmount(&mut self) {
        self.sold += 1;
    }

    // initialization of a product through a function
    #[allow(non_snake_case)]
    pub fn fromPayLoad( payload: Product) -> Product {
        let product_util = Product {
            id: payload.id,
            name: payload.name,
            description: payload.description,
            image: payload.image,
            location: payload.location,
            price: payload.price,
            owner: env::signer_account_id(),
            sold: 0
        };
        product_util
    }
}