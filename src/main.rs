use crate::chapter5::User;

mod chapter5;
mod chapter5_1;
mod chapter5_2;

fn main()
{
    let mut user1 = chapter5::User {
        email:String::from("someone@example.com"),
        username:String::from("someusername123"),
        active:true,
        sign_in_count:1,
    };

    let user3 = User{
        email:String::from("another@example.com"),
        ..user1
    };

    chapter5_1::run();

}

