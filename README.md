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
