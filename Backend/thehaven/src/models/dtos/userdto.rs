pub(crate) mod userdto {
    use serde::{Serialize, Deserialize};
    use crate::models::dbModels::user::user::UserDB;
    use chrono::{Utc};

    #[derive(Serialize, Deserialize)]
    pub struct UserDTO{
        pub id: String,
        pub username: String,
        pub password: String,
        pub email: String,
        pub created_at: String,
        pub updated_at: String,
        pub id_number: String,
        pub first_name: String,
        pub last_name: String,
        pub phone_number: String,
        pub address: String,
        pub gender: String
    }
    impl UserDTO {
        pub fn new(
            username: String,
            password: String,
            email: String,
            id_number: String,
            first_name: String,
            last_name: String,
            phone_number: String,
            address: String,
            gender: String,
        ) -> Self {
            UserDTO {
                id: "".to_string(),
                username,
                password,
                email,
                created_at: Utc::now().naive_utc().to_string(),
                updated_at: Utc::now().naive_utc().to_string(),
                id_number,
                first_name,
                last_name,
                phone_number,
                address,
                gender,
            }
        }
        pub fn new_full(
            id: String,
            username: String,
            password: String,
            email: String,
            created_at: String,
            updated_at: String,
            id_number: String,
            first_name: String,
            last_name: String,
            phone_number: String,
            address: String,
            gender: String,
        ) -> Self {
            UserDTO {
                id,
                username,
                password,
                email,
                created_at,
                updated_at,
                id_number,
                first_name,
                last_name,
                phone_number,
                address,
                gender,
            }
        }
    }
    impl From<UserDB> for UserDTO {
        fn from(t: UserDB) -> UserDTO {
            UserDTO::new_full(
                t.id,
                t.username,
                t.password,
                t.email,
                t.created_at,
                t.updated_at,
                t.id_number,
                t.first_name,
                t.last_name,
                t.phone_number,
                t.address,
                t.gender,
            )
        }
    }

}