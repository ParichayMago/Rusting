struct User {
    activate: bool,
    username: String,
    email: String,
}
fn main() {

    let mut user1 = build_user(String::from("name@gmail.com"), String::from("username"));

    let user2:User = User {
        email: String::from("new@gmail.com"),
        ..user1
    };
}
fn build_user(email:String, username:String)-> User {
    User {
        activate:true,
        username,
        email,
    }
}