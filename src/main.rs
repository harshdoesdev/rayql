// https://github.com/tursodatabase/libsql-client-rs

fn main() {
    let code = r#"
    
    # TODO: rules

    rule create user {
      true
    }

    rule update user {
      if(user.is_active, 
        and(
          not(matches(user.type, user_type.guest)),
          user.is_verified,
        )
      )
    }

    
    "#;

    let tokens = rayql::tokenizer::tokenize(code).unwrap();
    println!("{:#?}", tokens);
}
