mod model;
use model::Product;

/// Import `borsh` from `near_sdk` crate 
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, PanicOnDefault};
use near_sdk::collections::UnorderedMap;

/// Main contract structure serialized with Borsh
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct ProductList {
    pub product_list: UnorderedMap<String, Product>,
}

#[near_bindgen]
impl ProductList {

    // A constructor function that gets executed when the contract is loaded
    #[init]
    pub fn new() -> ProductList {
        
        // Declaration and initialization of an unordered map variable that wil be used to store products and their id's
        let mut productlist_util: UnorderedMap<String, Product> = UnorderedMap::new(b"s".to_vec());

                // List of products that will be stored in the hashmap
                let product1 = Product {
                    id: String::from("product1"),
                    name: String::from("Lasagna"),
                    description: String::from("Lean ground beef, ricotta cheese with fresh mozzarella"),
                    image: String::from("https://i.imgur.com/b3IMhwX.jpeg"),
                    location: String::from("Nairobi, Kenya"),
                    price: 1000000000000000000000000,
                    owner: env::signer_account_id(),
                    sold: 0,
                };
        
                let product2 = Product {
                    id: String::from("product2"),
                    name: String::from("Filet Mignon"),
                    description: String::from("Rosemary, Butter, olive oil, black pepper"),
                    image: String::from("https://i.imgur.com/B7HwmYG.jpeg"),
                    location: String::from("Nairobi, Kenya"),
                    price: 1000000000000000000000000,
                    owner: env::signer_account_id(),
                    sold: 0,
                };
        
                let product3 = Product {
                    id: String::from("product3"),
                    name: String::from("Brisket"),
                    description: String::from("Beef brisket, apple cider, vinegar, brown sugar"),
                    image: String::from("https://i.imgur.com/Qsj69aw.jpg"),
                    location: String::from("Nairobi, Kenya"),
                    price: 1000000000000000000000000,
                    owner: env::signer_account_id(),
                    sold: 0,
                };
        
                let product4 = Product {
                    id: String::from("product4"),
                    name: String::from("Hamburger"),
                    description: String::from("High quality beef medium to well with cheese and hacon on a multigrain bun"),
                    image: String::from("https://i.imgur.com/cxSwwus.jpeg"),
                    location: String::from("Nairobi, Kenya"),
                    price: 1000000000000000000000000,
                    owner: env::signer_account_id(),
                    sold: 0,
                };
        
                let product5 = Product {
                    id: String::from("product5"),
                    name: String::from("Pizza"),
                    description: String::from("Maple syrup, wheat flour, mozzarella cheese"),
                    image: String::from("https://i.imgur.com/3eQIKiw.jpg"),
                    location: String::from("Nairobi, Kenya"),
                    price: 1000000000000000000000000,
                    owner: env::signer_account_id(),
                    sold: 0,
                };
        
                let product6 = Product {
                    id: String::from("product6"),
                    name: String::from("Chicken wings"),
                    description: String::from("Butter, flour chicken wingettes, garlic powder, black pepper"),
                    image: String::from("https://i.imgur.com/nWJPyff.jpg"),
                    location: String::from("Nairobi, Kenya"),
                    price: 1000000000000000000000000,
                    owner: env::signer_account_id(),
                    sold: 0,
                };
        
                let product7 = Product {
                    id: String::from("product7"),
                    name: String::from("Muffin"),
                    description: String::from("Dried cherries, self raising flour, semi skimmed milk, golden"),
                    image: String::from("https://i.imgur.com/XnwLZEw.jpeg"),
                    location: String::from("Nairobi, Kenya"),
                    price: 1000000000000000000000000,
                    owner: env::signer_account_id(),
                    sold: 0,
                };
        
                let product8 = Product {
                    id: String::from("product8"),
                    name: String::from("Sushi"),
                    description: String::from("Smoked salmon, sushi rice, soy sauce, wasabi, rice wine"),
                    image: String::from("https://i.imgur.com/QwEq1g2.jpeg"),
                    location: String::from("Nairobi, Kenya"),
                    price: 1000000000000000000000000,
                    owner: env::signer_account_id(),
                    sold: 0,
                };
        
                let product9 = Product {
                    id: String::from("product9"),
                    name: String::from("BBQ"),
                    description: String::from("Grilled chicken and beef served with vegetables and chips."),
                    image: String::from("https://i.imgur.com/yPreV19.png"),
                    location: String::from("Nairobi, Kenya"),
                    price: 1000000000000000000000000,
                    owner: env::signer_account_id(),
                    sold: 0,
                };
        
                let product10 = Product {
                    id: String::from("product10"),
                    name: String::from("Pasta"),
                    description: String::from("Olive oil, sea salt, all purpose flour"),
                    image: String::from("https://i.imgur.com/0zn0yRS.jpg"),
                    location: String::from("Nairobi, Kenya"),
                    price: 1000000000000000000000000,
                    owner: env::signer_account_id(),
                    sold: 0,
                };
        
                let product11 = Product {
                    id: String::from("product11"),
                    name: String::from("Salami"),
                    description: String::from("Ground beef, liquid smoke flavoring, red pepper flakes"),
                    image: String::from("https://i.imgur.com/hrbtods.jpeg"),
                    location: String::from("Nairobi, Kenya"),
                    price: 1000000000000000000000000,
                    owner: env::signer_account_id(),
                    sold: 0,
                };
        
                let product12 = Product {
                    id: String::from("product12"),
                    name: String::from("Waffles"),
                    description: String::from("Maple syrup, eggs, baking powder, sugar, all purpose flour"),
                    image: String::from("https://i.imgur.com/VXvenZW.jpg"),
                    location: String::from("Nairobi, Kenya"),
                    price: 1000000000000000000000000,
                    owner: env::signer_account_id(),
                    sold: 0,
                };
        
                let product13 = Product {
                    id: String::from("product13"),
                    name: String::from("Milk Shake"),
                    description: String::from("Vanilla ice cream, maraschino cherry, milk, whipped topping"),
                    image: String::from("https://i.imgur.com/ytUBumj.jpeg"),
                    location: String::from("Nairobi, Kenya"),
                    price: 1000000000000000000000000,
                    owner: env::signer_account_id(),
                    sold: 0,
                };
        
                let product14 = Product {
                    id: String::from("product14"),
                    name: String::from("Shawarma"),
                    description: String::from("Pita bread, chicken thigh fillets, cheese, hot sauce, smoked"),
                    image: String::from("https://i.imgur.com/3HcUUmV.jpeg"),
                    location: String::from("Nairobi, Kenya"),
                    price: 1000000000000000000000000,
                    owner: env::signer_account_id(),
                    sold: 0,
                };
        
                let product15 = Product {
                    id: String::from("product15"),
                    name: String::from("Thai food"),
                    description: String::from("Lecademie"),
                    image: String::from("https://i.imgur.com/dhZsowZ.jpeg"),
                    location: String::from("Nairobi, Kenya"),
                    price: 1000000000000000000000000,
                    owner: env::signer_account_id(),
                    sold: 0,
                };
        
                let product16 = Product {
                    id: String::from("product16"),
                    name: String::from("Cheese cake"),
                    description: String::from("Cream cheese, lemon curd, sour cream, graham cracker"),
                    image: String::from("https://i.imgur.com/kLpdqpz.gif"),
                    location: String::from("Nairobi, Kenya"),
                    price: 1000000000000000000000000,
                    owner: env::signer_account_id(),
                    sold: 0,
                };
        
                let product17 = Product {
                    id: String::from("product17"),
                    name: String::from("Fish Balls"),
                    description: String::from("Small fish fillets, eggs, garlic, flour, cilantro"),
                    image: String::from("https://i.imgur.com/R5LzMw0.jpg"),
                    location: String::from("Nairobi, Kenya"),
                    price: 1000000000000000000000000,
                    owner: env::signer_account_id(),
                    sold: 0,
                };
        
                let product18 = Product {
                    id: String::from("product18"),
                    name: String::from("Crispy Fried chicken"),
                    description: String::from("Chicken thighs, chicken drumsticks, buffalo hot sauce"),
                    image: String::from("https://i.imgur.com/LNYeC11.jpeg"),
                    location: String::from("Nairobi, Kenya"),
                    price: 1000000000000000000000000,
                    owner: env::signer_account_id(),
                    sold: 0,
                };
        
                let product19 = Product {
                    id: String::from("product19"),
                    name: String::from("Chicken curry"),
                    description: String::from("Skinless chicken breast, fresh tomatoes, chicken borth"),
                    image: String::from("https://i.imgur.com/8rLCoRt.jpeg"),
                    location: String::from("Nairobi, Kenya"),
                    price: 1000000000000000000000000,
                    owner: env::signer_account_id(),
                    sold: 0,
                };
                
                let product20 = Product {
                    id: String::from("product20"),
                    name: String::from("Chinese food"),
                    description: String::from("Lean ground beef, ricotta cheese with fresh mozzarella"),
                    image: String::from("https://i.imgur.com/Mufznwt.jpeg"),
                    location: String::from("Nairobi, Kenya"),
                    price: 1000000000000000000000000,
                    owner: env::signer_account_id(),
                    sold: 0,
                };
        
                // Inserting products into the unordered Hashmap to be stored in the Near Blockchain
                productlist_util.insert(&"product1".to_owned(), &product1);
                productlist_util.insert(&"product2".to_owned(), &product2);
                productlist_util.insert(&"product3".to_owned(), &product3);
                productlist_util.insert(&"product4".to_owned(), &product4);
                productlist_util.insert(&"product5".to_owned(), &product5);
                productlist_util.insert(&"product6".to_owned(), &product6);
                productlist_util.insert(&"product7".to_owned(), &product7);
                productlist_util.insert(&"product8".to_owned(), &product8);
                productlist_util.insert(&"product9".to_owned(), &product9);
                productlist_util.insert(&"product10".to_owned(), &product10);
                productlist_util.insert(&"product11".to_owned(), &product11);
                productlist_util.insert(&"product12".to_owned(), &product12);
                productlist_util.insert(&"product13".to_owned(), &product13);
                productlist_util.insert(&"product14".to_owned(), &product14);
                productlist_util.insert(&"product15".to_owned(), &product15);
                productlist_util.insert(&"product16".to_owned(), &product16);
                productlist_util.insert(&"product17".to_owned(), &product17);
                productlist_util.insert(&"product18".to_owned(), &product18);
                productlist_util.insert(&"product19".to_owned(), &product19);
                productlist_util.insert(&"product20".to_owned(), &product20); 
        
        // Returning a struct of type Product list that contains all the above inserted products
        ProductList {
            product_list: productlist_util
        }
    }

    // A getter function used to get a specific product from the Hashmap
    #[allow(non_snake_case)]
    pub fn getProduct(&self, id: String) -> Product {
        let product = match self.product_list.get(&id) {
            Some(product) => product,
            None => {
                let msg = format!("There is no such product");
                env::panic(msg.as_bytes());
            } 
        };
        product 
    }

    // A setter function for setting products
    #[allow(non_snake_case)]
    pub fn setProduct(&mut self, id: String,  product: Product) {
        self.product_list.insert(&id, &product);
    }
    // #[allow(non_snake_case)]
    // pub fn setProduct(&mut self, id: String, name: String, description: String, image: String, location: String, price: u128,) {
    //     let product = Product {
    //         id,
    //         name,
    //         description,
    //         image,
    //         location,
    //         price,
    //         owner: env::signer_account_id(),
    //         sold: 0,
    //     };
    //     self.product_list.insert(&product.id, &product);
    // }


    // function used to execute the buying of the product
    #[allow(non_snake_case)]
    pub fn buyProduct(&mut self, id: String) {
        let mut product = self.getProduct(id);
        if product.price > env::account_balance() {
            let log_message = format!("You don't have enough cash, your account balance is {}", env::account_balance());
            env::log(log_message.as_bytes());
            env::panic(log_message.as_bytes());
        }
        product.incrementSoldAmount();
        self.product_list.insert(&product.id, &product); 
        env::promise_batch_create(&product.owner);     
    }

    // A getter function used to get all the products from the blockchain
    #[allow(non_snake_case)]
    pub fn getProducts(&self) -> Vec<model::Product> {
        if self.product_list.is_empty() {
            let msg = format!("The list is empty!! Nothing to be returned");
            env::panic(msg.as_bytes());
        } else {
            let input = self.product_list.values_as_vector().to_vec();
            let mut output = Vec::new();

            for s in input {
                output.push(s);
            }

            output
        }
    }
}


// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "robert.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "jane.testnet".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 300,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }
    
    #[test] 
    #[should_panic]
    fn buy_product() {
        let context = get_context(vec![], false);
        testing_env!(context);

        let mut productlist = ProductList::new();
        productlist.buyProduct("product1".to_string());
        let p = productlist.getProduct("product1".to_string());
        assert_eq!(1, p.sold);
    }

    #[test]
    fn get_products() {
        let context = get_context(vec![], false);
        testing_env!(context);

        let productlist = ProductList::new();
        let x = productlist.getProducts();

        assert_eq!(20, x.len());
    }

    #[test]
    fn set_product() {
        let context = get_context(vec![], false);
        testing_env!(context);

        let mut productlist = ProductList::new();

        let product1 = Product {
            id: String::from("product22"),
            name: String::from("Lasagna"),
            description: String::from("Lean ground beef, ricotta cheese with fresh mozzarella"),
            image: String::from("https://i.imgur.com/b3IMhwX.jpeg"),
            location: String::from("Nairobi, Kenya"),
            price: 1000000000000000000000000,
            owner: env::signer_account_id(),
            sold: 0,
        };

        productlist.setProduct("product22".to_string(), product1);


        // productlist.setProduct(String::from("product22"), String::from("Chinese food"), String::from("Lean ground beef, ricotta cheese with fresh mozzarella"),
        // String::from("https://i.imgur.com/Mufznwt.jpeg"), String::from("Nairobi, Kenya"), 1000000000000000000000000);

        let prod = productlist.getProduct("product22".to_string());

        assert_eq!(prod.id, "product22".to_string());
        
    }
}