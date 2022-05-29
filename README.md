# A NEAR Food Marketplace built using RUST and Web assembly

This NEAR dApp is used to simulate the purchase of Food and cuisine in the online market place.

It was built using these below technologies:
- RUST
- Web Assembly
- ReactJS

> I used **Rust** to build and compile the smart contract. It is used because it offers features like memory safety and small runtime. This made building a smart contract that doesn't have memory bugs and consumes less storage on the NEAR blockchain.

> I used **Web Assembly** because of its high perfomance. It facilittates small binaries to ship over the internet devices.

> I used **ReactJS** to build the front end and interact with the **NEAR-API-JS (JavaScript Library)**

## Challenges encoutered
- Serialization of the data using Borsh - Different data required different serialization and I encountered multiple serialization error.

## Installing and running the program
you can find the program from the following [githubpage:](www.githubpage.git). You could also choose to clone the program from the following [github repository](www.githubrepo.org).

This project was bootstrapped with [Create React App](https://github.com/facebook/create-react-app).

## Parts of my contract
---
The contract is made up of two modules:
- The model
- The lib

**The model** module is used to create an abstraction of a product and all its attributes.

**The lib** module the entry point of my program which is used to implement the product module and other functions as seen below.

## Methods and functions within the contract
---
My contract has four functions; **One** constructor function, **Two** view functions, and **One** call function.

* The **new()** function is automatically initialized upon the contract call and this is enabled by the **#[init]** attribute.

* The **getProducts()** function is used to return all the values withing the unordered map in form of a vector.

* The **getProduct()** function is used to return only one value within the unordered map.

* The **buyProduct()** function is used to create a trasaction call. This will prompt the user to buy a specific product.

# The program's front end
For the contract to be able to interact well with my fronten, I had to install the **near-api-js:** `npm install near-api-js`.

The contract was explicitly configured to run in a testent. Hence the environment would be "testnet".

Through the **window.contract** module we could interact with our contract's view and call functions within our interface.