use super::model::User;

pub struct UserService {}

impl UserService {
    pub fn build() -> Self {
        UserService {}
    }

    pub fn create_user(&self, user_name: String) -> User {
        User {
            id: 42,
            username: user_name,
        }
    }

    pub fn get_user(&self, id: u64) -> User {
        User {
            id: id,
            username: format!("test_user_{id}"),
        }
    }
}
