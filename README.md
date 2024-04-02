# RayQL

<p align="center">
    <img src="./banner.png" alt="RayQL Logo">
    <br />
    <br />
    <b>RayQL is a schema definition and query language for SQLite.</b>
    <br />
    <br />
</p>

## Online Editor

You can try RayQL using the [Online RayQL editor](https://harshdoesdev.github.io/rayql-studio/).

## Schema Definition

You can define your database schema by creating a RayQL file called `schema.rayql`.

For example, it might look something like this:

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
    email: str unique, # this is an inline comment
    phone_number: str?,
    user_type: user_type default(user_type.normal)
}

# Model declaration for 'post'

model post {
    id: int primary_key auto_increment,
    title: str default('New Post'),
    content: str,
    author_id: int foreign_key(user.id),
    created_at: timestamp default(now()),
}
```

Then, when you run the `rayql print` command, it will generate and output the SQL equivalent of that model, which for the above example should look something like this:

```sql
CREATE TABLE IF NOT EXISTS user (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    phone_number TEXT NULL,
    user_type TEXT NOT NULL CHECK(user_type IN ('admin', 'developer', 'normal')) DEFAULT 'normal'
);

CREATE TABLE IF NOT EXISTS post (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL DEFAULT 'New Post',
    content TEXT NOT NULL,
    author_id INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (author_id) REFERENCES user(id)
);
```
