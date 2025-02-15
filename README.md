Prerequisite: rust 1.85, docker

From root run:

- docker compose build
- cargo make start

Setup Db:

- Login into pgadmin through port 15432 (admin@admin.com, Ad12345#)
- Connect pgadmin to postgres on port 5432 (POSTGRES_USER: postgres, POSTGRES_PASSWORD: Ad12345#, POSTGRES_DB: web-builder)
- CREATE DATABASE, SCHEMA, ENUM, TABLE in setup.sql from root
- Add FUNCTION and TRIGGER

Setup realestate data from resale-online through http://localhost:17001/builder/setup-data (need to obtain apikey)
Setup your own realestate data (To be implemented)

Access the app from localhost:17001
