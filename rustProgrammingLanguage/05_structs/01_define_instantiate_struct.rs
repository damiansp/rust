fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("Bob Dobolina"),
        email: String::from("bob@the_dob.com"),
        sign_in_count: 1
    };
    user1.email = String::from("bobdob@dobolina.com");
    let mut user2 = build_user(String::from("Jimmy Dean"), String::from("sausage@jd.com"));
    let mut user3 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("bob@dobbin.org"),
        sign_in_count: user1.sign_in_count
    };
    let mut user4 = User {
        email: String::from("ham@jd.com"),
        ..user2  // everything else copies
    };
}


fn build_user_verbose(username: String, email: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1
    }
}


fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}