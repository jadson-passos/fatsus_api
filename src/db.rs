use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;

#[derive(Clone)]
pub struct DbService {
    pub pool: PgPool,
}

impl DbService {
    #[cfg(test)]
    pub fn from_pool(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn init() -> Self {
        let database_url = env::var("DATABASE_URL")
            .expect("A variável DATABASE_URL precisa estar configurada no .env");

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .expect("Erro ao conectar no banco de dados PostgreSQL");

        // Executa as migrações pendentes na pasta ./migrations automaticamente
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Falha ao executar as migrações no banco de dados");

        println!("✓ Migrações do banco de dados executadas com sucesso!");
        println!("✓ Massa de teste aplicada quando as tabelas estavam vazias (20 registros por entidade).");

        DbService { pool }
    }
}