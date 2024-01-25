use sqlx::{Pool, Postgres};

struct AccountsRepository {
    pool: Pool<Postgres>,
}

struct Account {
    id: i32,
    username: String,
    salt: String,
    verifier: String,
    session_key_auth: String,
    locked: bool,
    last_ip: String,
}

impl AccountsRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn get_by_username(&self, username: &str) {
        let account = sqlx::query_as!(Account, "SELECT id, username, salt, verifier, session_key_auth, locked, last_ip FROM account WHERE username = $1")
            .fetch_one(&self.pool)
            .await
            .unwrap();

        Ok(account)
    }
}