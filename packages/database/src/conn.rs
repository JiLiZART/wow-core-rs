
use sqlx::{postgres::PgPoolOptions, Database, Pool};
use anyhow::Context;

struct ConnectionState {
    db: Pool<DB: Database>,
}

impl ConnectionState {
    pub async fn connect(url: &str) -> anyhow::Result<ConnectionState> {
        // We create a single connection pool for SQLx that's shared across the whole application.
        // This saves us from opening a new connection for every API call, which is wasteful.
        let db = PgPoolOptions::new()
            // The default connection limit for a Postgres server is 100 connections, minus 3 for superusers.
            // Since we're using the default superuser we don't have to worry about this too much,
            // although we should leave some connections available for manual access.
            //
            // If you're deploying your application with multiple replicas, then the total
            // across all replicas should not exceed the Postgres connection limit.
            .max_connections(50)
            .connect(url)
            .await
            .context(format!("could not connect to {}", url))?;

        // This embeds database migrations in the application binary so we can ensure the database
        // is migrated correctly on startup
        sqlx::migrate!().run(&db).await?;

        Ok(ConnectionState {
            db: db
        })
    }

    pub fn accounts(&self) {

    }
}