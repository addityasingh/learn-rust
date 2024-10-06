"""
Table schema
name, email
"""


"""
Phase 0
Create table
"""

CREATE TABLE users (name TEXT, email TEXT)

"""
Phase 1
Simple insert and read queries
"""

INSERT INTO users (name, email) VALUES ('John Doe', 'johndoe@example.com')
SELECT * FROM users

"""
Phase 2
Read and write column validation
"""


"""
Phase 2.1
Add autoincrementing index
"""


"""
Phase 3
Update queries based on id
"""

UPDATE users SET email = '"'"'johndoe@example.com'"'"' WHERE id = 1"


"""
Phase 4
Update queries based on name
"""

UPDATE users SET email = '"'"'johndoe@example.com'"'"' WHERE name = 'John Doe'"


