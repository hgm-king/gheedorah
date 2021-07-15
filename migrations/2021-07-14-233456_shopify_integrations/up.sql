CREATE TABLE "shopify_integrations" (
  id SERIAL PRIMARY KEY,
  shop VARCHAR NOT NULL,
  nonce VARCHAR NOT NULL,
  access_token VARCHAR,
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP,
  deleted_at TIMESTAMP,
  active BOOLEAN NOT NULL
);
