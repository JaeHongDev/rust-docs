use crate::chapter5::User;

mod chapter5;

fn main()
{
    let mut user1 = chapter5::User {
        email:String::from("someone@example.com"),
        username:String::from("someusername123"),
        active:true,
        sign_in_count:1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = User{
        active:user1.active,
        username:user1.username,
        email:String::from("another@example.com"),
        sign_in_count:user1.sign_in_count,
    };

    let user3 = User{
        email:String::from("another@example.com"),
        ..user1
    };




}

