use std::borrow::Cow;

struct User {
    name: Cow<'static, str>
}

fn main() {
    let user_name = "User1";
    let other_user_name = "User10".to_string();

    let user1 = User {
        name: user_name.into(),
    };
    
    let user2 = User {
        name: other_user_name.into(),
    };
}
