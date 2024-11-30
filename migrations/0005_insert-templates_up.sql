DO $$
BEGIN
   IF NOT EXISTS (SELECT 1 FROM templates) THEN
      INSERT INTO templates (template_type, description)
      VALUES 
        ('RealEstate', 'Default template for real estate website');
   END IF;
END $$;