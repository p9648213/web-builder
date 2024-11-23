-- ENUM (EXCUTE BEFORE MIGRATION)
DROP TYPE IF EXISTS "Role";
DROP TYPE IF EXISTS "TemplateType";

CREATE TYPE "Role" AS ENUM (
    'User',
    'Admin'
);

CREATE TYPE "TemplateType" AS ENUM (
    'Ecommerce',
    'Custom'
);

-- FUNCTION (EXCUTE BEFORE MIGRATION)
CREATE OR REPLACE FUNCTION update_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- TRIGGER (EXCUTE AFTER MIGRATION)
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