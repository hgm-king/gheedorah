# Decentralized Gift Card Service
The goal of this service is to integrate with various e-commerce platforms in order to allow online stores to issue gift cards and store credit that is backed by blockchain technology.

## ECommerce Integration
For a minimum viable product, our aim is to integrate only with *Shopify*. In order to do this, we will need a [Shopify Partners](https://www.shopify.com/partners) account which will allow us to generate a custom application as well as the necessary API keys.

When a shop keeper is looking to use our service, we will need to follow the **authentication flow** outlined in the [Shopify documentation](https://shopify.dev/apps/auth/oauth). We will need to decide what sort of *scopes/permissions* we will need to request from the shop.

After we are authenticated, our service will receive an *access token* from Shopify that needs to be saved. It will be valid as long as the shop has our service installed. When it comes time to do any sort of operations with the shop's API, we will use this token in the form of a header.

Once we have access, we can then get our hands on everything within the requested scopes for the shop's [Admin API](https://shopify.dev/api/admin). Personally, a

Throughout this main flow, we will need to handle various scenarios in which something may go wrong. Here is a working list of things that could go wrong:
- The shop uninstalls the application (we would not be able to get a working session token)
- The shop does not complete the authentication process (database entries will be left incomplete)

### User Facing Flow
1. Shop keeper **signs up** for the Service
1. Service will **create gift card** products through a UI page/form
1. User will **purchase** a gift card, or shop keep can credit a user
1. Once order is considered complete, user will **receive an email** with special code
1. Shop keep can resend email if gift card is valid
1. Upon checkout, **special code** will be entered in gift card location
1. Charge is **discounted**, gift card value is **debited**
1. Upon usage of credit through payment, our job is **done**

### Questions
##### How does our system get notified that a gift card is purchased?
We do not want to have to poll the Shopify API repetitively to find out a card has been bought. Maybe we could have a hook attached to our gift card products and the hook will notify our service.

##### Is giving the user a code the best possible way to handle customer authentication?
How do we validate a credited user when they are attempting to redeem their store credit? One way is to provide them with a code, preferably though email, this will be entered into the Shopify checkout page. Maybe there is a more modern way that utilizes a cookie or something? We will have to keep sharing gift cards in mind.

##### How do we get some sort of actionable element/button in the Shopify checkout page?
We will need to make sure the user can recognize their ability to use a gift card when checking out. Hopefully this will take them to a checkout page that comes out of the box with Shopify.

##### What entities need to exist within Shopify to utilize the Discount/Gift Card code feature?
It appears that within the checkout page, you have a text field to enter a code in order to redeem a discount or code. Is there some sort of hook that we can use to accept the value coming through from that field? We would need to accept and process the input and somehow return a *discount* object that would be understood by Shopify to lower the price.

When a purchase that utilizes a gift card occurs, we will need to identify this and respond accordingly (by debiting the account, that is). Again, we want to avoid polling the API to find an event; a hook would better serve our purpose.

## Blockchain Technology
We will use a popular blockchain, called **Ethereum**, to store our user's store credit in a persistent, decentralized manner. To do this, we will need to write a smart contract that will handle the logic of keeping a balance, as well as crediting and debiting appropriately. From there, we will have an API that exposes the contract functions to the outside world.

Our plan is to utilize [Truffle](https://www.trufflesuite.com/truffle) to manage our contracts throughout testing, compilation, and deployment.

To interact with our contract, we use a [Rust crate](https://github.com/gnosis/ethcontract-rs) from Gnosis. This will give us a Rust `struct` that wraps our contract and abstracts away all network calls.

To complete the blockchain slice of the stack, we will need an API that exposes these functions found in the contract. GraphQL is one approach that I am fond of, although regular REST is valid as well.

Here are the following functions that we require (note they are already in Solidity):
- `function balanceOf(string memory storeId, string memory clientId) public view returns (uint256)`
- `function credit(string memory storeId, string memory clientId, uint256 credits) public returns (bool)`
- `function redeem(string memory storeId, string memory clientId, uint256 credits) public returns (bool)`

### Questions
##### Is Ethereum the best platform to build upon?
Smart contracts are exactly what we need. Being able to codify the logic into an immutable, distributed *contract* is perfect. An issue, though, is that Ethereum is hot at the moment and will be expensive. Each `write` transaction to the blockchain will incur a fee, and the pricing may be an issue; it is [low at the moment](https://www.theblockcrypto.com/post/108471/ethereum-eth-gas-fees-six-month-low-why) but has been upwards of 45$.

##### Do we need to look into more methods in our contract?
We will be checking to see if a user is able to use credit on a purchase. It would make sense to be able to make one call that would see if a transaction would be valid. This would be followed by another subsequent call which would do the actual transaction. Seeing as we can do an initial check to see if they have a credit entity, we can get away with just the debit and credit functions, due to the following logic:

- If the customer is purchasing something with a price that is greater than or equal to the balance of their store credit, the credit will be deducted from the price and their balance would become 0.
- If the customer is purchasing something with a price that is less than the balance of their store credit, the price will be deducted from their credit balance and the price would become 0.
