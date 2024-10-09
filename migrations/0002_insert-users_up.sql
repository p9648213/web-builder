DO $$
BEGIN
   IF NOT EXISTS (SELECT 1 FROM users) THEN
      INSERT INTO users (username, password, email)
      VALUES 
        ('admin', 'password', 'admin@example.com'), 
        ('user1', 'password', 'user1@example.com'), 
        ('user2', 'password', 'user2@example.com');
   END IF;
END $$;