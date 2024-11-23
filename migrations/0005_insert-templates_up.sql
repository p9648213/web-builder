DO $$
BEGIN
   IF NOT EXISTS (SELECT 1 FROM templates) THEN
      INSERT INTO templates (template_type, description)
      VALUES 
        ('Ecommerce', 'Default template for ecommerce website');
   END IF;
END $$;