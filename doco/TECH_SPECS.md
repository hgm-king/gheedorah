# Technical Specifications
This application will be built from a handful of parts, linking different integrations that will involve very little custom logic on our part.

## Shopify Integration
We need to integrate with Shopify in order to help users bring our application into their e-commerce platform. Shopify will take care of many different parts: handling payments, managing orders, and providing an API to hook into these events. There are a number of different event flows that we need to handle on our side to fulfill this integration.

### Installation
To get get access to user's shop API, we need to handle the [Shopify OAuth flow](https://shopify.dev/apps/auth/oauth).

#### Algorithm
```
User clicks a special link to request an installation of our application
Our API receives the request with shop parameters and saves them in the database as a pending install request
Our API responds with a redirect to a confirmation url which requests to be allowed different scopes of permissions within the shop
User confirms the install
Our API receives the confirmation and does various checks to validate the request documented in the OAuth Flow
Our API fetches an Access Token from the store, updating the database entry with the token (the token being present in the database entry signifies a successful installation)
```

#### Related Endpoints
- `shopify/install` will accept the initial request, saving the params for the confirmation.
- `shopify/confirm` accepts the confirmation, grabs the access code, and saves it into the db. Also will setup any sort of hooks and products for the shop.

#### Related Models
- `ShopifyIntegration` defines the database entity for the installation request. It contains the shops access token that we need to use Shopify's API.

A [study](https://github.com/HGHimself/rust-oauth2-study) was done to explore the integration. This can be referred to in isolation from the larger codebase found in this project if need be.

### Gift Card Product
Once the shop has installed our application, we need to set it up to be in a state that will allow them to configure and sell gift cards. This is a typical Shopify operation, and is represented by [products](https://shopify.dev/api/admin/rest/reference/products/product). We will be able to generate a product named 'Gift Card' and use variations of this product to give the clients more than one option.

When the product is in a valid state to sell gift cards, we can leave things be. A valid state means that the product has attributes that make sense, i.e. it is not a physical product, has no type of inventory, is not taxable, etc. We do want the shopkeep to configure their gift cards though, so this introduces some instability.

Therefore, there will be two places where a shopkeep will be able to make these edits. First is within our UI that we provide. We will be sure to not enable the shopkeep to make any invalid edits. Second is the Shopify UI, where many more 'edit actions' are available to the shopkeep. We will need to have a list of checks to make sure the user hasn't made any changes that will invalidate the state of the product.

This raises the question, what would we do whenever this occurs?
- We could revert these most recent changes, and notify the shop keep somehow that the edit was ignored.
- We could delete the product and create a new one from template.
- We could disable the product until the user changes it back into a valid state.

Another question, what sort of model will we use to represent our gift cards? Do we want one row in the database to represent a variation of a shop's gift card, or one per product? Do we delete and create these entities whenever the shopkeep changes them, or do we update rows that already exist?

#### Algorithm
```
A shop installs our application
Our API creates a template product which represents a gift card
Our API saves the gift card into our database
Our API registers a PODUCT_EDIT webhook with the shop
If the product is edited in our UI
  Save data to database
  Update the product through Shopify API
If the product is edited in the Shopify API
  If the changes were valid
    Save changes into database
  Else the changes were not valid
    Take evasive action
```
#### Endpoints
- `shopify/product` will listen for when the shop user edits a product.

#### Models
- `ShopifyProduct` will represent all of the gift card entities our shops will have.

### Handling Gift Card Purchase
Now that the shop has a gift card product that a client can purchase, we need to handle the event. Luckily, we can use a webhook that will send all purchases to our API so that we can handle our scenario. We need a certain type of flag to let our app know that a gift card of ours is being purchased; this could be done with a product tag, a product type, or even saving the product id somewhere. When the user orders the gift card, we will credit them the value in the blockchain and email them a code that allows them to redeem the value.
#### Algorithm
```
User purchases a gift card
Our API receives an ORDER_CREATE webhook event
Our API identifies order of a gift card
Generate a unique code
Credit the value to the user in the blockchain
Email the user with their code
```
#### Endpoints
- `shopify/order` will listen for when the shop receives an order from a client.

#### Models
- `ShopifyOrder` will represent all of the gift cards that a user has. It needs to have a code and an email. Maybe a value? We may not, since it would make sense that the number only exists within the blockchain.

### Using Gift Cards
The part of the Shopify Integration I am not so enlightened on is the usage of the gift cards. To set the scenario, a user has a valid gift card code and is looking to purchase a product with it. Shopify will give our users choice of buttons for payment methods, one being 'other payment options'. This one will take them to a page which allows the user to insert a code and see the price of their purchase change. I assume there must be a sort of hook that will let an application respond with a discount. After reading the documentation, I cannot seem to find this hook. I need to ask around, I guess.

#### Algorithm
```
User checks out in the store
User enters gift card code into text area
Our API receives a request with the code
If code is valid
  Return the balance of the gift card
Else code is not valid
  Reject the request
User confirms the purchase
Our API receives an ORDER_CREATE webhook event
Our API identifies the order used a gift card
Debit the value from the user in the blockchain
Email the user with their remaining balance
```

https://www.alchemy.com/
