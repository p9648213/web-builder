DO $$
BEGIN
   IF NOT EXISTS (SELECT 1 FROM users) THEN
      INSERT INTO users (username, password, email, role)
      VALUES 
        ('admin', 'password', 'admin@example.com', 'admin'), 
        ('user1', 'password', 'user1@example.com', 'user'), 
        ('user2', 'password', 'user2@example.com', 'user');
   END IF;
END $$;