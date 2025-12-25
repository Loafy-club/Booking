-- Remove moderator role from the database
-- This role was reserved for future use but is no longer needed

-- First, update any users with moderator role to regular user role
UPDATE users
SET role_id = (SELECT id FROM roles WHERE name = 'user')
WHERE role_id = (SELECT id FROM roles WHERE name = 'moderator');

-- Then delete the moderator role
DELETE FROM roles WHERE name = 'moderator';
