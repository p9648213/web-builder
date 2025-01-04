-- CREATE DATABASE
DROP DATABASE IF EXISTS "web-builder";
CREATE DATABASE IF NOT EXISTS "web-builder";

-- CREATE SCHEMA
DROP SCHEMA public CASCADE;
CREATE SCHEMA public;

-- ENUM
DROP TYPE IF EXISTS "Role";
DROP TYPE IF EXISTS "TemplateType";

CREATE TYPE "Role" AS ENUM (
    'User',
    'Admin'
);

CREATE TYPE "TemplateType" AS ENUM (
    'RealEstate',
    'Custom'
);

-- CREATE TABLE
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    role "Role" NOT NULL DEFAULT 'User',
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS "templates" (
    id SERIAL PRIMARY KEY,
    template_type "TemplateType" NOT NULL,
    description VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS websites (
    id SERIAL PRIMARY KEY,
    template_id INTEGER,
    user_id INTEGER,
    name VARCHAR(255) NOT NULL,
    domain VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_template FOREIGN KEY (template_id) REFERENCES templates (id) ON DELETE CASCADE,
    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS rso_data (
    id SERIAL,
    user_id INTEGER UNIQUE,
    identifier_id INTEGER NOT NULL,
    api_key VARCHAR(255) NOT NULL,
    filter_id_sale INTEGER NOT NULL,
    filter_id_long INTEGER NOT NULL,
    filter_id_short INTEGER NOT NULL,
    filter_id_featured INTEGER NOT NULL,
    status BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id, user_id),
    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS website_settings (
    id SERIAL PRIMARY KEY,
    website_id INTEGER UNIQUE,
    header_theme INTEGER NOT NULL DEFAULT 1,
    footer_theme INTEGER NOT NULL DEFAULT 1,
    home_theme INTEGER NOT NULL DEFAULT 1,
    search_theme INTEGER NOT NULL DEFAULT 1,
    property_theme INTEGER NOT NULL DEFAULT 1,
    contact_theme INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_website FOREIGN KEY (website_id) REFERENCES websites (id) ON DELETE CASCADE
);

DO $$
BEGIN
   IF NOT EXISTS (SELECT 1 FROM users) THEN
      INSERT INTO users (username, password, email, role)
      VALUES 
        ('admin', '$argon2id$v=19$m=19456,t=2,p=1$gAxoCqrlM1uQPwjKHSgvXg$xggG9Lb4QxVE96eQ70egNJ3TEMZPBU/obPIZRYZnK48', 'admin@gmail.com', 'Admin'), 
        ('user1', '$argon2id$v=19$m=19456,t=2,p=1$yq7xlPTQfDoOXoIXa3cS2Q$8+/bwyWtFtJ/wA6CHNXV5VgxMLrQwXEVHbK1kdtx/YE', 'user1@gmail.com', 'User'), 
        ('user2', '$argon2id$v=19$m=19456,t=2,p=1$u1A0MgTC8wB+JkNQJAbmPg$eqshS0CQlElLfI4UxIXb3tue1dfymH1s2uC3HaN0ls8', 'user2@gmail.com', 'User');
   END IF;
END $$;

DO $$
BEGIN
   IF NOT EXISTS (SELECT 1 FROM templates) THEN
      INSERT INTO templates (template_type, description)
      VALUES 
        ('RealEstate', 'Default template for real estate website');
   END IF;
END $$;

-- FUNCTION
CREATE OR REPLACE FUNCTION update_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- TRIGGER
DO
$$
DECLARE
    tbl RECORD;
BEGIN
    FOR tbl IN
        SELECT table_name
        FROM information_schema.columns
        WHERE table_schema = 'public' 
          AND column_name = 'updated_at'
    LOOP
        -- Drop the trigger if it already exists to avoid duplication
        EXECUTE format('
            DROP TRIGGER IF EXISTS trigger_update_timestamp ON %I
        ', tbl.table_name);

        -- Create the trigger to update `updated_at` on each update
        EXECUTE format('
            CREATE TRIGGER trigger_update_timestamp
            BEFORE UPDATE ON %I
            FOR EACH ROW
            EXECUTE FUNCTION update_timestamp()
        ', tbl.table_name);
    END LOOP;
END
$$;