# RayQL

RayQL is a schema definition and query language for SQLite.

## Schema Definition

You can define your database schema by creating a RayQL file called `schema.rayql` and then running `rayql generate schema.rayql`.

For example, it may look something like this:

```rayql
# Enum for user types

enum user_type {
    admin
    developer
    normal
}

# Model declaration for 'user'

model user {
    id: int primary_key auto_increment,
    username: str unique,
    email: str unique, # This is an inline comment
    user_type: user_type default(user_type.normal)
}

# Model declaration for 'post'

model post {
    id: int primary_key auto_increment,
    title: str default('New Post'),
    content: str required,
    author_id: int foreign_key(user.id),
    created_at: timestamp default(now())
}
```

It will generate a SQL file in the migrations table, which should look something like this:

```sql
-- CREATE TABLE FOR MODEL `user`

CREATE TABLE IF NOT EXISTS user (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT UNIQUE,
    email TEXT UNIQUE,
    user_type TEXT CHECK(user_type IN ('admin', 'developer', 'normal')) DEFAULT 'normal'
);

-- CREATE TABLE FOR MODEL `post`

CREATE TABLE IF NOT EXISTS post (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT DEFAULT 'New Post',
    content TEXT NOT NULL,
    author_id INTEGER REFERENCES user(id),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```
