use std::collections::HashMap;

use async_trait::async_trait;

use axum_login::{
    AuthUser, 
    AuthnBackend,
    UserId
};

#[derive(Debug, Clone)]
struct User {
    id: i64,
    pw_hash: Vec<u8>,
}

impl AuthUser for User {
    type Id = i64;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        &self.pw_hash
    }
}

#[derive(Clone, Default)]
struct Backend {
    users: HashMap<i64, User>,
}

#[derive(Clone)]
struct Credentials {
    user_id: i64
}

#[async_trait]
impl AuthnBackend for Backend {
    type User = User;
    type Credentials = Credentials;
    type Error = std::convert::Infallible;

    async fn authenticate(&self, Credentials{user_id}: Self::Credentials)
        -> Result<Option<Self::User>, Self::Error> {
            todo!("Still need to implement proper authentication using axum_auth");
            Ok(self.users.get(&user_id).cloned())
    }

    async fn get_user(&self, user_id: &UserId<Self>)
        -> Result<Option<Self::User>, Self::Error> {
            todo!("Still need to implement proper authentication using axum_auth");
            Ok(self.users.get(user_id).cloned())
    }
}
