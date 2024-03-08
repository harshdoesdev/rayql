// https://github.com/tursodatabase/libsql-client-rs

fn main() {
    let code = r#"
    # Enum for user types
    # Enum for user types
    enum user_type {
      admin
      developer
      normal
      guest
    }
    
    # Model declaration for 'user'
    model user {
      id: int primary_key auto_increment,
      username: str unique,
      email: str unique, # this is an inline comment
      age: int min(12),
      is_active: bool default(false),
    }
    
    # Model declaration for 'post'
    model post {
      id: int primary_key auto_increment,
      title: str default('New Post'),
      content: str,
      rating: real default(-0.0),
      author_id: int foreign_key(user.id),
      created_at: timestamp default(now()),
    }
    "#;

    let schema = rayql::parser::parse(code).unwrap();
    println!("{:#?}", schema);
}
