struct User {
    active: bool,
    username: String,
    email: String, 
    sign_in_count: u64,
}

struct Color(i32, i32, i32); 
struct Point3d(i32, i32, i32);

fn main() {
    let mut user1 = User {
        active: true, 
        username: String::from("someusername123"), 
        email: String::from("someone@example.com"),
        sign_in_count: 1
    };
    user1.email = String::from("modified@example.com");
    user1.sign_in_count = 5;
    print_user(&user1);

    let user2 = build_user(
        String::from("user2@example.com"), 
        String::from("username2")
    );
    print_user(&user2);

    let user3 = User {
        // user2 moved to user3, due to String doesn't implement copy() trait, no longer valid
        active: user2.active,  
        username: user2.username, 
        email: String::from("user3@example.com"),
        // user1 not be move to user3, due to u64 implements copy() trait, remain valid
        sign_in_count: user1.sign_in_count  
    }; 
    print_user(&user3);

    let user4 = User {
        email: String::from("user4@example.com"), 
        ..user3
    }; 
    print_user(&user4);

    let black = Color(0, 0, 0);
    let poin1 = Point3d(0, 0, 0);

}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, 
        email, 
        sign_in_count: 1
    }
}

fn print_user(user: &User) {
    println!("Username: {}, Email: {}, Active: {}, Sign-in Count: {}", user.username, user.email, user.active, user.sign_in_count);
}
