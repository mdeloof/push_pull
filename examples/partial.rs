use push_pull::*;

#[derive(PartialEq, Debug)]
struct User {
    first_name: String,
    last_name: String,
    age: u8,
}

#[derive(Default)]
struct PartialUser {
    first_name: Option<String>,
    last_name: Option<String>,
    age: Option<u8>
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

    let mut partial_user = PartialUser::default();
    partial_user.first_name = Some("Castor".to_string());
    partial_user.last_name = Some("Troy".to_string());

    user.pull_from(&partial_user);

    assert_eq!(user, User { 
        first_name: "Castor".to_string(), 
        last_name: "Troy".to_string(), 
        age: 43
    });
}