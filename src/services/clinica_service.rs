use crate::db::DbService;
use crate::models::Clinica;
use crate::services::documento::apenas_digitos;
use crate::services::paginacao::{montar_pagina, ListQuery, Pagina};
use sqlx::Error;

pub struct ClinicaService;

impl ClinicaService {
    pub async fn listar(db: &DbService, query: ListQuery) -> Result<Pagina<Clinica>, Error> {
        let limit = query.limit();
        let rows = sqlx::query_as::<_, Clinica>(
            r#"
            SELECT id, razao, cnpj, cnes, endereco, bairro, telefone, email
            FROM clinicas
            WHERE $1::int4 IS NULL OR id < $1
            ORDER BY id DESC
            LIMIT $2
            "#,
        )
        .bind(query.cursor)
        .bind(limit + 1)
        .fetch_all(&db.pool)
        .await?;

        Ok(montar_pagina(rows, limit, |clinica| clinica.id))
    }

    pub async fn buscar_por_cnes_ou_cnpj(
        db: &DbService,
        documento: &str,
    ) -> Result<Option<Clinica>, Error> {
        let documento = apenas_digitos(documento);
        if documento.is_empty() {
            return Ok(None);
        }

        sqlx::query_as::<_, Clinica>(
            r#"
            SELECT id, razao, cnpj, cnes, endereco, bairro, telefone, email
            FROM clinicas
            WHERE regexp_replace(COALESCE(cnpj, ''), '[^0-9]', '', 'g') = $1
               OR regexp_replace(COALESCE(cnes, ''), '[^0-9]', '', 'g') = $1
            ORDER BY id DESC
            LIMIT 1
            "#,
        )
        .bind(documento)
        .fetch_optional(&db.pool)
        .await
    }

    pub async fn criar(db: &DbService, clinica: Clinica) -> Result<i32, Error> {
        let record: (i32,) = sqlx::query_as(
            r#"
            INSERT INTO clinicas (razao, cnpj, cnes, endereco, bairro, telefone, email)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id
            "#,
        )
        .bind(clinica.razao)
        .bind(clinica.cnpj)
        .bind(clinica.cnes)
        .bind(clinica.endereco)
        .bind(clinica.bairro)
        .bind(clinica.telefone)
        .bind(clinica.email)
        .fetch_one(&db.pool)
        .await?;

        Ok(record.0)
    }

    pub async fn atualizar_por_cnes_ou_cnpj(
        db: &DbService,
        documento: &str,
        clinica: Clinica,
    ) -> Result<bool, Error> {
        let documento = apenas_digitos(documento);
        if documento.is_empty() {
            return Ok(false);
        }

        let result = sqlx::query(
            r#"
            UPDATE clinicas
            SET razao = $1, cnpj = $2, cnes = $3, endereco = $4, bairro = $5, telefone = $6, email = $7
            WHERE id = (
                SELECT id FROM clinicas
                WHERE regexp_replace(COALESCE(cnpj, ''), '[^0-9]', '', 'g') = $8
                   OR regexp_replace(COALESCE(cnes, ''), '[^0-9]', '', 'g') = $8
                ORDER BY id DESC
                LIMIT 1
            )
            "#,
        )
        .bind(clinica.razao)
        .bind(clinica.cnpj)
        .bind(clinica.cnes)
        .bind(clinica.endereco)
        .bind(clinica.bairro)
        .bind(clinica.telefone)
        .bind(clinica.email)
        .bind(documento)
        .execute(&db.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn deletar_por_cnes_ou_cnpj(db: &DbService, documento: &str) -> Result<bool, Error> {
        let documento = apenas_digitos(documento);
        if documento.is_empty() {
            return Ok(false);
        }

        let result = sqlx::query(
            r#"
            DELETE FROM clinicas
            WHERE id = (
                SELECT id FROM clinicas
                WHERE regexp_replace(COALESCE(cnpj, ''), '[^0-9]', '', 'g') = $1
                   OR regexp_replace(COALESCE(cnes, ''), '[^0-9]', '', 'g') = $1
                ORDER BY id DESC
                LIMIT 1
            )
            "#,
        )
        .bind(documento)
        .execute(&db.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::DbService;
    use crate::models::Clinica;
    use crate::services::paginacao::ListQuery;
    use sqlx::PgPool;

    fn clinica_teste() -> Clinica {
        Clinica {
            id: None,
            razao: "Clínica Teste Unitário LTDA".to_string(),
            cnpj: "99888777000166".to_string(),
            cnes: Some("2599999".to_string()),
            endereco: Some("Rua Teste, 10".to_string()),
            bairro: Some("Centro".to_string()),
            telefone: Some("31940009999".to_string()),
            email: Some("clinica.teste@test".to_string()),
        }
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn listar_retorna_massa_inicial(pool: PgPool) {
        let db = DbService::from_pool(pool);
        let pagina = ClinicaService::listar(&db, ListQuery::default()).await.unwrap();
        assert!(pagina.data.len() >= 20);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn buscar_por_cnpj_e_cnes_da_massa(pool: PgPool) {
        let db = DbService::from_pool(pool);

        let por_cnpj = ClinicaService::buscar_por_cnes_ou_cnpj(&db, "11.222.333/0001-01")
            .await
            .unwrap()
            .unwrap();
        assert_eq!(por_cnpj.razao, "Clínica São Lucas LTDA");

        let por_cnes = ClinicaService::buscar_por_cnes_ou_cnpj(&db, "2560001")
            .await
            .unwrap()
            .unwrap();
        assert_eq!(por_cnes.cnpj, "11222333000101");
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn documento_vazio_nao_encontra(pool: PgPool) {
        let db = DbService::from_pool(pool);
        let resultado = ClinicaService::buscar_por_cnes_ou_cnpj(&db, "xyz")
            .await
            .unwrap();
        assert!(resultado.is_none());
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn criar_atualizar_e_apagar_por_cnes(pool: PgPool) {
        let db = DbService::from_pool(pool);
        let mut clinica = clinica_teste();

        let id = ClinicaService::criar(&db, clinica.clone()).await.unwrap();
        assert!(id > 0);

        let encontrada = ClinicaService::buscar_por_cnes_ou_cnpj(&db, "2599999")
            .await
            .unwrap()
            .unwrap();
        assert_eq!(encontrada.razao, "Clínica Teste Unitário LTDA");

        clinica.bairro = Some("Lourdes".to_string());
        let atualizou =
            ClinicaService::atualizar_por_cnes_ou_cnpj(&db, "99888777000166", clinica)
                .await
                .unwrap();
        assert!(atualizou);

        let atualizada = ClinicaService::buscar_por_cnes_ou_cnpj(&db, "99.888.777/0001-66")
            .await
            .unwrap()
            .unwrap();
        assert_eq!(atualizada.bairro.as_deref(), Some("Lourdes"));

        let apagou = ClinicaService::deletar_por_cnes_ou_cnpj(&db, "2599999")
            .await
            .unwrap();
        assert!(apagou);
        assert!(ClinicaService::buscar_por_cnes_ou_cnpj(&db, "99888777000166")
            .await
            .unwrap()
            .is_none());
    }
}
