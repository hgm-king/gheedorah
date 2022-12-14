# Sidecar

## Setup
You will need to setup a `.env` file based on the `local.env` template. Ask HG for this file. For testing, a copy of the template will suffice; otherwise, fields are needed from the Shopify application setup as well as SSL certificate files.
#### To run tests:
- `docker-compose up`
- `sh scripts/setup-test-db.sh`
- `sh scripts/run-tests.sh`
- If a test that inserts into db were to fail, it will leave db in and invalid state; fix this with `sh scripts/setup-test-db.sh`
#### To setup Shopify application:
- App URL: `https://localhost:3030/shopify/install`
- Allowed redirection URL(s): `https://localhost:3030/shopify/confirm`

Object({"admin_graphql_api_id": String("gid://shopify/Order/3987524878525"), "app_id": Number(580111), "billing_address": Object({"address1": String("31-16 38th Street"), "address2": String(""), "city": String("Queens"), "company": Null, "country": String("United States"), "country_code": String("US"), "first_name": String(""), "last_name": String("King"), "latitude": Number(40.761323), "longitude": Number(-73.91848), "name": String("King"), "phone": Null, "province": String("New York"), "province_code": String("NY"), "zip": String("11103")}), "browser_ip": String("24.102.117.198"), "buyer_accepts_marketing": Bool(false), "cancel_reason": Null, "cancelled_at": Null, "cart_token": Null, "checkout_id": Number(22047589466301), "checkout_token": String("0771810c7c94822b1edc7da6a26f0e71"), "client_details": Object({"accept_language": String("en-US,en;q=0.5"), "browser_height": Number(792), "browser_ip": String("24.102.117.198"), "browser_width": Number(1440), "session_hash": Null, "user_agent": String("Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:89.0) Gecko/20100101 Firefox/89.0")}), "closed_at": Null, "confirmed": Bool(true), "contact_email": String("hgmaxwellking@gmail.com"), "created_at": String("2021-07-29T22:06:13-04:00"), "currency": String("USD"), "current_subtotal_price": String("1.00"), "current_subtotal_price_set": Object({"presentment_money": Object({"amount": String("1.00"), "currency_code": String("USD")}), "shop_money": Object({"amount": String("1.00"), "currency_code": String("USD")})}), "current_total_discounts": String("0.00"), "current_total_discounts_set": Object({"presentment_money": Object({"amount": String("0.00"), "currency_code": String("USD")}), "shop_money": Object({"amount": String("0.00"), "currency_code": String("USD")})}), "current_total_duties_set": Null, "current_total_price": String("1.00"), "current_total_price_set": Object({"presentment_money": Object({"amount": String("1.00"), "currency_code": String("USD")}), "shop_money": Object({"amount": String("1.00"), "currency_code": String("USD")})}), "current_total_tax": String("0.00"), "current_total_tax_set": Object({"presentment_money": Object({"amount": String("0.00"), "currency_code": String("USD")}), "shop_money": Object({"amount": String("0.00"), "currency_code": String("USD")})}), "customer": Object({"accepts_marketing": Bool(false), "accepts_marketing_updated_at": String("2021-07-13T18:12:20-04:00"), "admin_graphql_api_id": String("gid://shopify/Customer/5373193912509"), "created_at": String("2021-07-13T18:12:20-04:00"), "currency": String("USD"), "default_address": Object({"address1": String("31-16 38th Street"), "address2": String(""), "city": String("Queens"), "company": Null, "country": String("United States"), "country_code": String("US"), "country_name": String("United States"), "customer_id": Number(5373193912509), "default": Bool(true), "first_name": String(""), "id": Number(6662249119933), "last_name": String("King"), "name": String("King"), "phone": Null, "province": String("New York"), "province_code": String("NY"), "zip": String("11103")}), "email": String("hgmaxwellking@gmail.com"), "first_name": String("HG"), "id": Number(5373193912509), "last_name": String("King"), "last_order_id": Number(3987524878525), "last_order_name": String("#1005"), "marketing_opt_in_level": Null, "multipass_identifier": Null, "note": Null, "orders_count": Number(5), "phone": Null, "state": String("disabled"), "tags": String(""), "tax_exempt": Bool(false), "tax_exemptions": Array([]), "total_spent": String("5.00"), "updated_at": String("2021-07-29T22:06:14-04:00"), "verified_email": Bool(true)}), "customer_locale": String("en"), "device_id": Null, "discount_applications": Array([]), "discount_codes": Array([]), "email": String("hgmaxwellking@gmail.com"), "financial_status": String("paid"), "fulfillment_status": Null, "fulfillments": Array([]), "gateway": String("gift_card"), "id": Number(3987524878525), "landing_site": String("/wallets/checkouts.json"), "landing_site_ref": Null, "line_items": Array([Object({"admin_graphql_api_id": String("gid://shopify/LineItem/10231817732285"), "discount_allocations": Array([]), "duties": Array([]), "fulfillable_quantity": Number(1), "fulfillment_service": String("manual"), "fulfillment_status": Null, "gift_card": Bool(false), "grams": Number(0), "id": Number(10231817732285), "name": String("Rocket"), "origin_location": Object({"address1": String("33 Caton Pl"), "address2": String("#8F"), "city": String("Brooklyn"), "country_code": String("US"), "id": Number(3017731080381), "name": String("bdrocketstore"), "province_code": String("NY"), "zip": String("11218")}), "price": String("1.00"), "price_set": Object({"presentment_money": Object({"amount": String("1.00"), "currency_code": String("USD")}), "shop_money": Object({"amount": String("1.00"), "currency_code": String("USD")})}), "product_exists": Bool(true), "product_id": Number(6756816683197), "properties": Array([]), "quantity": Number(1), "requires_shipping": Bool(false), "sku": String(""), "tax_lines": Array([]), "taxable": Bool(true), "title": String("Rocket"), "total_discount": String("0.00"), "total_discount_set": Object({"presentment_money": Object({"amount": String("0.00"), "currency_code": String("USD")}), "shop_money": Object({"amount": String("0.00"), "currency_code": String("USD")})}), "variant_id": Number(39960092508349), "variant_inventory_management": String("shopify"), "variant_title": String(""), "vendor": String("bdrocketstore")})]), "location_id": Null, "name": String("#1005"), "note": Null, "note_attributes": Array([]), "number": Number(5), "order_number": Number(1005), "order_status_url": String("https://bdrocketstore.myshopify.com/57647497405/orders/ccdf6bd6ea53f26635b52ea7ed76c3e2/authenticate?key=aa4445f48a71fdfe9cc7af771ef415f1"), "original_total_duties_set": Null, "payment_gateway_names": Array([String("gift_card")]), "phone": Null, "presentment_currency": String("USD"), "processed_at": String("2021-07-29T22:06:12-04:00"), "processing_method": String("gift_cards_only"), "reference": Null, "referring_site": String("https://bdrocketstore.myshopify.com/products/rocket"), "refunds": Array([]), "shipping_lines": Array([]), "source_identifier": Null, "source_name": String("web"), "source_url": Null, "subtotal_price": String("1.00"), "subtotal_price_set": Object({"presentment_money": Object({"amount": String("1.00"), "currency_code": String("USD")}), "shop_money": Object({"amount": String("1.00"), "currency_code": String("USD")})}), "tags": String(""), "tax_lines": Array([]), "taxes_included": Bool(false), "test": Bool(false), "token": String("ccdf6bd6ea53f26635b52ea7ed76c3e2"), "total_discounts": String("0.00"), "total_discounts_set": Object({"presentment_money": Object({"amount": String("0.00"), "currency_code": String("USD")}), "shop_money": Object({"amount": String("0.00"), "currency_code": String("USD")})}), "total_line_items_price": String("1.00"), "total_line_items_price_set": Object({"presentment_money": Object({"amount": String("1.00"), "currency_code": String("USD")}), "shop_money": Object({"amount": String("1.00"), "currency_code": String("USD")})}), "total_outstanding": String("0.00"), "total_price": String("1.00"), "total_price_set": Object({"presentment_money": Object({"amount": String("1.00"), "currency_code": String("USD")}), "shop_money": Object({"amount": String("1.00"), "currency_code": String("USD")})}), "total_price_usd": String("1.00"), "total_shipping_price_set": Object({"presentment_money": Object({"amount": String("0.00"), "currency_code": String("USD")}), "shop_money": Object({"amount": String("0.00"), "currency_code": String("USD")})}), "total_tax": String("0.00"), "total_tax_set": Object({"presentment_money": Object({"amount": String("0.00"), "currency_code": String("USD")}), "shop_money": Object({"amount": String("0.00"), "currency_code": String("USD")})}), "total_tip_received": String("0.00"), "total_weight": Number(0), "updated_at": String("2021-07-29T22:06:17-04:00"), "user_id": Null})





Object(
  {
    "admin_graphql_api_id": String("gid://shopify/Product/6868469481661"),
    "body_html": String("Purchasing this product awards a specified amount of store credit for the client. This credit can be redeemed only within this store, and cannot be transferred to another site. The giftcard will be represented by a code, sent to the client's provided email. This code needs to be protected, though could be shared if preferred."), "created_at": String("2021-07-30T12:16:24-04:00"),
    "handle": String("sidecar-gift-card-1"),
    "id": Number(6868469481661),
    "image": Null,
    "images": Array([]),
    "options": Array(
      [Object(
        {"id": Number(8810588537021), "name": String("Card Title"), "position": Number(1), "product_id": Number(6868469481661), "values": Array([String("100 bucks"), String("1000 bucks")])}
      )]),
    "product_type": String(""),
    "published_at": String("2021-07-30T12:16:24-04:00"),
    "published_scope": String("web"),
    "status": String("active"),
    "tags": String("Sidecar Gift Card"),
    "template_suffix": String(""),
    "title": String("Sidecar Gift Card"),
    "updated_at": String("2021-08-02T19:36:46-04:00"),
    "variants":  Array([
      Object({
        "admin_graphql_api_id": String("gid://shopify/ProductVariant/40335394734269"),
        "barcode": Null,
        "compare_at_price": Null,
        "created_at": String("2021-07-30T12:16:24-04:00"),
        "fulfillment_service": String("manual"),
        "grams": Number(0),
        "id": Number(40335394734269),
        "image_id": Null,
        "inventory_item_id": Number(42430301864125),
        "inventory_management": Null,
        "inventory_policy": String("deny"),
        "inventory_quantity": Number(0),
        "old_inventory_quantity": Number(0),
        "option1": String("100 bucks"),
        "option2": Null,
        "option3": Null,
        "position": Number(1),
        "price": String("100.00"),
        "product_id": Number(6868469481661),
        "requires_shipping": Bool(false),
        "sku": String(""),
        "taxable": Bool(false),
        "title": String("100 bucks"),
        "updated_at": String("2021-07-30T12:16:24-04:00"),
        "weight": Number(0.0),
        "weight_unit": String("lb")
      }),
      Object({
        "admin_graphql_api_id": String("gid://shopify/ProductVariant/40335394767037"),
        "barcode": Null,
        "compare_at_price": Null,
        "created_at": String("2021-07-30T12:16:24-04:00"),
        "fulfillment_service": String("manual"),
        "grams": Number(0),
        "id": Number(40335394767037),
        "image_id": Null,
        "inventory_item_id": Number(42430301896893),
        "inventory_management": Null,
        "inventory_policy": String("deny"),
        "inventory_quantity": Number(0),
        "old_inventory_quantity": Number(0),
        "option1": String("1000 bucks"),
        "option2": Null,
        "option3": Null,
        "position": Number(2),
        "price": String("1000.00"),
        "product_id": Number(6868469481661),
        "requires_shipping": Bool(false),
        "sku": String(""),
        "taxable": Bool(false),
        "title": String("1000 bucks"),
        "updated_at": String("2021-08-02T19:36:46-04:00"),
        "weight": Number(0.0), "weight_unit": String("lb")
      })
    ]),
    "vendor": String("Sidecar")
})
