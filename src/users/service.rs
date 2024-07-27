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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_user() {
        let subject = UserService::build();

        let test_user_name = "test_user_name";
        let result = subject.create_user(test_user_name.to_string());

        assert_eq!(result.id, 42);
        assert_eq!(result.username, test_user_name);
    }

    #[test]
    fn test_get_user() {
        let subject = UserService::build();

        let test_user_id = 12u64;
        let result = subject.get_user(test_user_id);

        assert_eq!(result.id, test_user_id);
        assert_eq!(result.username, format!("test_user_{test_user_id}"))
    }
}
