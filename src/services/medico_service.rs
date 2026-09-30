use crate::db::DbService;
use crate::models::Medico;
use crate::services::documento::apenas_digitos;
use crate::services::paginacao::{montar_pagina, ListQuery, Pagina};
use sqlx::Error;

pub struct MedicoService;

impl MedicoService {
    pub async fn listar(db: &DbService, query: ListQuery) -> Result<Pagina<Medico>, Error> {
        let limit = query.limit();
        let rows = sqlx::query_as::<_, Medico>(
            r#"
            SELECT id, nome, cns, cbo, cpf, especialidade
            FROM medicos
            WHERE $1::int4 IS NULL OR id < $1
            ORDER BY id DESC
            LIMIT $2
            "#,
        )
        .bind(query.cursor)
        .bind(limit + 1)
        .fetch_all(&db.pool)
        .await?;

        Ok(montar_pagina(rows, limit, |medico| medico.id))
    }

    pub async fn buscar_por_cns_ou_cpf(
        db: &DbService,
        documento: &str,
    ) -> Result<Option<Medico>, Error> {
        let documento = apenas_digitos(documento);
        if documento.is_empty() {
            return Ok(None);
        }

        sqlx::query_as::<_, Medico>(
            r#"
            SELECT id, nome, cns, cbo, cpf, especialidade
            FROM medicos
            WHERE regexp_replace(COALESCE(cpf, ''), '[^0-9]', '', 'g') = $1
               OR regexp_replace(COALESCE(cns, ''), '[^0-9]', '', 'g') = $1
            ORDER BY id DESC
            LIMIT 1
            "#,
        )
        .bind(documento)
        .fetch_optional(&db.pool)
        .await
    }

    pub async fn criar(db: &DbService, medico: Medico) -> Result<i32, Error> {
        let record: (i32,) = sqlx::query_as(
            r#"
            INSERT INTO medicos (nome, cns, cbo, cpf, especialidade)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id
            "#,
        )
        .bind(medico.nome)
        .bind(medico.cns)
        .bind(medico.cbo)
        .bind(medico.cpf)
        .bind(medico.especialidade)
        .fetch_one(&db.pool)
        .await?;

        Ok(record.0)
    }

    pub async fn atualizar_por_cns_ou_cpf(
        db: &DbService,
        documento: &str,
        medico: Medico,
    ) -> Result<bool, Error> {
        let documento = apenas_digitos(documento);
        if documento.is_empty() {
            return Ok(false);
        }

        let result = sqlx::query(
            r#"
            UPDATE medicos
            SET nome = $1, cns = $2, cbo = $3, cpf = $4, especialidade = $5
            WHERE id = (
                SELECT id FROM medicos
                WHERE regexp_replace(COALESCE(cpf, ''), '[^0-9]', '', 'g') = $6
                   OR regexp_replace(COALESCE(cns, ''), '[^0-9]', '', 'g') = $6
                ORDER BY id DESC
                LIMIT 1
            )
            "#,
        )
        .bind(medico.nome)
        .bind(medico.cns)
        .bind(medico.cbo)
        .bind(medico.cpf)
        .bind(medico.especialidade)
        .bind(documento)
        .execute(&db.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn deletar_por_cns_ou_cpf(db: &DbService, documento: &str) -> Result<bool, Error> {
        let documento = apenas_digitos(documento);
        if documento.is_empty() {
            return Ok(false);
        }

        let result = sqlx::query(
            r#"
            DELETE FROM medicos
            WHERE id = (
                SELECT id FROM medicos
                WHERE regexp_replace(COALESCE(cpf, ''), '[^0-9]', '', 'g') = $1
                   OR regexp_replace(COALESCE(cns, ''), '[^0-9]', '', 'g') = $1
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
    use crate::models::Medico;
    use crate::services::paginacao::ListQuery;
    use sqlx::PgPool;

    fn medico_teste() -> Medico {
        Medico {
            id: None,
            nome: "Médico Teste Unitário".to_string(),
            cns: Some("900000000000099".to_string()),
            cbo: Some("225125".to_string()),
            cpf: Some("90000000099".to_string()),
            especialidade: Some("Teste".to_string()),
        }
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn listar_retorna_massa_inicial(pool: PgPool) {
        let db = DbService::from_pool(pool);
        let pagina = MedicoService::listar(&db, ListQuery::default()).await.unwrap();
        assert!(pagina.data.len() >= 20);
        assert!(!pagina.has_more);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn listar_pagina_por_cursor(pool: PgPool) {
        let db = DbService::from_pool(pool);
        let primeira = MedicoService::listar(
            &db,
            ListQuery {
                limit: Some(5),
                cursor: None,
            },
        )
        .await
        .unwrap();
        assert_eq!(primeira.data.len(), 5);
        assert!(primeira.has_more);
        assert!(primeira.next_cursor.is_some());

        let segunda = MedicoService::listar(
            &db,
            ListQuery {
                limit: Some(5),
                cursor: primeira.next_cursor,
            },
        )
        .await
        .unwrap();
        assert_eq!(segunda.data.len(), 5);
        assert_ne!(primeira.data[0].id, segunda.data[0].id);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn buscar_por_cpf_e_cns_da_massa(pool: PgPool) {
        let db = DbService::from_pool(pool);

        let por_cpf = MedicoService::buscar_por_cns_ou_cpf(&db, "100.000.000-01")
            .await
            .unwrap()
            .unwrap();
        assert_eq!(por_cpf.nome, "Ana Beatriz Nogueira");

        let por_cns = MedicoService::buscar_por_cns_ou_cpf(&db, "100000000000001")
            .await
            .unwrap()
            .unwrap();
        assert_eq!(por_cns.cpf.as_deref(), Some("10000000001"));
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn documento_vazio_nao_encontra(pool: PgPool) {
        let db = DbService::from_pool(pool);
        let resultado = MedicoService::buscar_por_cns_ou_cpf(&db, "abc")
            .await
            .unwrap();
        assert!(resultado.is_none());
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn criar_atualizar_e_apagar_por_cpf(pool: PgPool) {
        let db = DbService::from_pool(pool);
        let mut medico = medico_teste();

        let id = MedicoService::criar(&db, medico.clone()).await.unwrap();
        assert!(id > 0);

        let encontrado = MedicoService::buscar_por_cns_ou_cpf(&db, "90000000099")
            .await
            .unwrap()
            .unwrap();
        assert_eq!(encontrado.nome, "Médico Teste Unitário");

        medico.especialidade = Some("Cardiologia Teste".to_string());
        let atualizou = MedicoService::atualizar_por_cns_ou_cpf(&db, "90000000099", medico)
            .await
            .unwrap();
        assert!(atualizou);

        let atualizado = MedicoService::buscar_por_cns_ou_cpf(&db, "900.000.000-99")
            .await
            .unwrap()
            .unwrap();
        assert_eq!(atualizado.especialidade.as_deref(), Some("Cardiologia Teste"));

        let apagou = MedicoService::deletar_por_cns_ou_cpf(&db, "900000000000099")
            .await
            .unwrap();
        assert!(apagou);
        assert!(MedicoService::buscar_por_cns_ou_cpf(&db, "90000000099")
            .await
            .unwrap()
            .is_none());
    }
}
