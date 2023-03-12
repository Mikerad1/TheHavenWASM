pub(crate) mod authentication_service {
    use std::collections::BTreeMap;
    use std::error::Error;

    use crate::models::dbModels::user::user::{userdto::userdto::UserDTO, UserDB};
    use crate::models::dtos::userdto;
    type Db = SqlDbSender<WasmHost>;
    use hmac::{Hmac, Mac};
    use jwt::SignWithKey;
    use sha2::Sha256;
    use wasmbus_rpc::actor::prelude::*;
    use wasmbus_rpc::minicbor::decode;
    use wasmbus_rpc::{actor::prelude::WasmHost, common::Context};
    use wasmcloud_interface_numbergen::{generate_guid, random_in_range};
    use wasmcloud_interface_sqldb::{minicbor, SqlDb, SqlDbError, SqlDbSender};

    pub async fn register_user(
        ctx: &Context,
        client: &Db,
        input: UserDTO,
    ) -> Result<UserDTO, SqlDbError> {
        let id = generate_guid().await.unwrap();
        let salt = random_in_range(1, 6).await.unwrap().to_string();
        let hashed_password = argon2::hash_encoded(
            input.password.as_bytes(),
            salt.as_bytes(),
            &argon2::Config::default(),
        )
        .unwrap();
        let resp = client.execute(ctx, &format!(
            r#"
            insert into {} (id, username, password, email, created_at, updated_at, id_number, first_name, last_name, phone_number, address, gender)
            values ('{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}');
            "#, "Users", id, input.username, hashed_password, input.email, input.created_at, input.updated_at, input.id_number, input.first_name, input.last_name, input.phone_number, input.address, input.gender
        ).into())
        .await?;

        Ok(input.into())
    }
    pub async fn login(
        ctx: &Context,
        client: &Db,
        username: String,
        password: String,
    ) -> Result<String, SqlDbError> {
        let resp = client
            .query(
                ctx,
                &format!(
                    r#"
            select * from {} where username = '{}';
            "#,
                    "Users", username
                )
                .into(),
            )
            .await?;
        if resp.num_rows == 0 {
            return Err(SqlDbError::new("notFound", "User was not found".to_string()));
        }
        let mut rows: Vec<UserDB> = decode(&resp.rows)?;
        let user = rows.remove(0);
        let passwordResult = argon2::verify_encoded(&user.password, password.as_bytes())
            .map_err(|_| SqlDbError::new("invalidPassword", "Invalid password".to_string()));
        if passwordResult.unwrap() {
            let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();
            let mut claims = BTreeMap::new();
            claims.insert("username", user.username);
            claims.insert("sub", user.id);
            let token = claims.sign_with_key(&key);
            if let token_string = token.unwrap()  {
                Ok(token_string)
            } else {
                Err(SqlDbError::new("jwtFailed", "Could not generate jwt".to_string()))
            }
        } else {
            return Err(SqlDbError::new("invalidPassword", "Invalid password".to_string()));
        }
    }
}
