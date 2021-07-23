# Sidecar

## Setup
You will need to setup a `.env` file based on the `local.env` template. Ask HG for this file. For testing, a copy of the template will suffice; otherwise, fields are needed from the Shopify application setup as well as SSL certificate files.
#### To run tests:
- `docker-compose up`
- `sh scripts/setup-test-db.sh`
- `sh scripts/run-tests.sh`
