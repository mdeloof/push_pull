use push_pull::*;

#[derive(PartialEq, Debug, Clone)]
struct User {
    first_name: String,
    last_name: String,
    age: u8,
}

impl User {
    // Write a generic function to update the user where the update is 
    // bounded by `PushInto`.
    fn update(&mut self, update: &impl PushInto<Self>) {
        update.push_into(self);
    }
}

#[derive(Default)]
struct PartialUser {
    first_name: Option<String>,
    last_name: Option<String>,
    age: Option<u8>
}

impl PullFrom<User> for User {
    fn pull_from(&mut self, user: &User) {
        self.first_name = user.first_name.clone();
        self.last_name = user.last_name.clone();
        self.age = user.age;
    }
}


impl PullFrom<PartialUser> for User {
    fn pull_from(&mut self, partial_user: &PartialUser) {
        if let Some(ref first_name) = partial_user.first_name {
            self.first_name = first_name.clone()
        }
        if let Some(ref last_name) = partial_user.last_name {
            self.last_name = last_name.clone()
        }
        if let Some(age) = partial_user.age {
            self.age = age
        }
    }
}

fn main() {
    let mut user = User {
        first_name: "Sean".to_string(),
        last_name: "Archer".to_string(),
        age: 43
    };
    let original_user = user.clone();

    let mut partial_user = PartialUser::default();
    partial_user.first_name = Some("Castor".to_string());
    partial_user.last_name = Some("Troy".to_string());
    
    user.update(&partial_user);
    assert_eq!(user, User { 
        first_name: "Castor".to_string(), 
        last_name: "Troy".to_string(), 
        age: 43
    });

    user.update(&original_user);
    assert_eq!(user, User {
        first_name: "Sean".to_string(),
        last_name: "Archer".to_string(),
        age: 43
    });
}