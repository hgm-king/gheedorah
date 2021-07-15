use crate::schema::shopify_integrations;
use crate::utils::now;
use chrono::naive::NaiveDateTime;
use diesel::prelude::*;

#[derive(Debug, Identifiable, Queryable)]
#[table_name = "shopify_integrations"]
pub struct ShopifyIntegration {
    pub id: i32,
    pub shop: String,
    pub nonce: String,
    pub access_token: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub active: bool,
}

#[derive(Insertable)]
#[table_name = "shopify_integrations"]
pub struct NewShopifyIntegration {
    pub shop: String,
    pub nonce: String,
    pub access_token: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub active: bool,
}

impl NewShopifyIntegration {
    pub fn new(shop: String, nonce: String) -> Self {
        NewShopifyIntegration {
            shop,
            nonce,
            access_token: None,
            created_at: now(),
            updated_at: None,
            deleted_at: None,
            active: true,
        }
    }

    pub fn insert(&self, conn: &PgConnection) -> ShopifyIntegration {
        create(conn, self).unwrap()
    }
}

pub fn create(
    conn: &PgConnection,
    new_shopify_integration: &NewShopifyIntegration,
) -> Result<ShopifyIntegration, diesel::result::Error> {
    diesel::insert_into(shopify_integrations::table)
        .values(new_shopify_integration)
        .get_result(conn)
}

pub fn read(conn: &PgConnection) -> Result<Vec<ShopifyIntegration>, diesel::result::Error> {
    shopify_integrations::table.load::<ShopifyIntegration>(conn)
}

pub fn read_by_shop(
    conn: &PgConnection,
    shop: String,
) -> Result<Vec<ShopifyIntegration>, diesel::result::Error> {
    shopify_integrations::table
        .filter(shopify_integrations::shop.eq(shop))
        .load::<ShopifyIntegration>(conn)
}

pub fn read_by_shop_and_nonce(
    conn: &PgConnection,
    shop: String,
    nonce: String,
) -> Result<Vec<ShopifyIntegration>, diesel::result::Error> {
    shopify_integrations::table
        .filter(shopify_integrations::shop.eq(shop))
        .filter(shopify_integrations::nonce.eq(nonce))
        .load::<ShopifyIntegration>(conn)
}

pub fn update_access_token(
    conn: &PgConnection,
    shopify_integration: &ShopifyIntegration,
    access_token: String,
) -> QueryResult<usize> {
    diesel::update(shopify_integration)
        .set((
            shopify_integrations::access_token.eq(access_token),
            shopify_integrations::updated_at.eq(now()),
        ))
        .execute(conn)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::establish_test_connection;

    fn cleanup_table(conn: &PgConnection) {
        diesel::delete(shopify_integrations::table)
            .execute(conn)
            .unwrap();
    }

    fn mock_struct() -> NewShopifyIntegration {
        NewShopifyIntegration::new(
            String::from("ShopName"),
            String::from("00a329c0648769a73afac7f9381e08fb43dbea72"),
        )
    }

    #[test]
    fn it_creates_a_shopify_integration() {
        let conn = establish_test_connection();

        create(&conn, &mock_struct()).unwrap();

        let shopify_integration = shopify_integrations::table
            .load::<ShopifyIntegration>(&conn)
            .expect("Error loading shopify_integration");

        assert_eq!(1, shopify_integration.len());

        cleanup_table(&conn);
    }

    #[test]
    fn it_reads_a_shopify_integration() {
        let conn = establish_test_connection();

        let new_shopify_integration = mock_struct();

        let created_shopify_integration = diesel::insert_into(shopify_integrations::table)
            .values(&new_shopify_integration)
            .get_result::<ShopifyIntegration>(&conn)
            .expect("Error saving new shopify_integration");

        let shopify_integration = read(&conn).unwrap();

        assert!(0 < shopify_integration.len());

        let my_shopify_integration = shopify_integration
            .iter()
            .find(|&x| x.shop == new_shopify_integration.shop);
        assert!(
            my_shopify_integration.is_some(),
            "Could not find the created shopify_integration in the database!"
        );

        cleanup_table(&conn);
    }

    #[test]
    fn it_reads_a_shopify_integration_by_shop() {
        let conn = establish_test_connection();
        let shop = String::from("ShopNameBaby");

        // make 2 shopify_integrations, each with different categories
        let mut new_shopify_integration = mock_struct();
        create(&conn, &new_shopify_integration).unwrap();

        new_shopify_integration.shop = shop.clone();
        create(&conn, &new_shopify_integration).unwrap();

        let shopify_integration = read_by_shop(&conn, shop.clone()).unwrap();

        assert_eq!(1, shopify_integration.len());

        let my_shopify_integration = shopify_integration.iter().find(|x| x.shop == shop);
        assert!(
            my_shopify_integration.is_some(),
            "Could not find the created shopify_integration in the database!"
        );

        cleanup_table(&conn);
    }

    #[test]
    fn it_reads_a_shopify_integration_by_shop_and_nonce() {
        let conn = establish_test_connection();
        let nonce =
            String::from("0cd1136c6702de4410d06d3ae80f592c9b2132ea232011bcc78fb53862cbd9ee");

        // make 2 shopify_integrations, each with different categories
        let mut new_shopify_integration = mock_struct();
        create(&conn, &new_shopify_integration).unwrap();

        new_shopify_integration.nonce = nonce.clone();
        create(&conn, &new_shopify_integration).unwrap();

        let shopify_integration =
            read_by_shop_and_nonce(&conn, String::from("ShopName"), nonce.clone()).unwrap();

        assert_eq!(1, shopify_integration.len());

        let my_shopify_integration = shopify_integration.iter().find(|x| x.nonce == nonce);
        assert!(
            my_shopify_integration.is_some(),
            "Could not find the created shopify_integration in the database!"
        );

        cleanup_table(&conn);
    }

    #[test]
    fn it_updates_a_shopify_integration_access_token() {
        let conn = establish_test_connection();

        let shopify_integration = create(&conn, &mock_struct()).unwrap();
        let access_token = String::from("super ssssecret");

        update_access_token(&conn, &shopify_integration, access_token.clone());

        let shopify_integrations = read_by_shop(&conn, shopify_integration.shop).unwrap();

        assert_eq!(1, shopify_integrations.len());
        let my_shopify_integration = shopify_integrations
            .iter()
            .find(|x| x.access_token.as_ref().unwrap() == &access_token);
        assert!(
            my_shopify_integration.is_some(),
            "Could not find the created shopify_integration in the database!"
        );

        cleanup_table(&conn);
    }
}
