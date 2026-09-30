use crate::db::DbService;
use crate::models::Exame;
use crate::services::paginacao::{montar_pagina, ListQuery, Pagina};
use sqlx::Error;

pub struct ExameService;

impl ExameService {
    pub async fn listar(db: &DbService, query: ListQuery) -> Result<Pagina<Exame>, Error> {
        let limit = query.limit();
        let rows = sqlx::query_as::<_, Exame>(
            r#"
            SELECT id, descricao, paciente_id, medico_id, cod_interno,
                   valor::float8 AS valor, valor_fonte_100::float8 AS valor_fonte_100, cid
            FROM exames
            WHERE $1::int4 IS NULL OR id < $1
            ORDER BY id DESC
            LIMIT $2
            "#,
        )
        .bind(query.cursor)
        .bind(limit + 1)
        .fetch_all(&db.pool)
        .await?;

        Ok(montar_pagina(rows, limit, |exame| exame.id))
    }

    pub async fn buscar_por_cod_interno(
        db: &DbService,
        cod_interno: i32,
    ) -> Result<Option<Exame>, Error> {
        sqlx::query_as::<_, Exame>(
            r#"
            SELECT id, descricao, paciente_id, medico_id, cod_interno,
                   valor::float8 AS valor, valor_fonte_100::float8 AS valor_fonte_100, cid
            FROM exames
            WHERE cod_interno = $1
            ORDER BY id DESC
            LIMIT 1
            "#,
        )
        .bind(cod_interno)
        .fetch_optional(&db.pool)
        .await
    }

    pub async fn criar(db: &DbService, exame: Exame) -> Result<i32, Error> {
        let record: (i32,) = sqlx::query_as(
            r#"
            INSERT INTO exames (descricao, paciente_id, medico_id, cod_interno, valor, valor_fonte_100, cid)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id
            "#,
        )
        .bind(exame.descricao)
        .bind(exame.paciente_id)
        .bind(exame.medico_id)
        .bind(exame.cod_interno)
        .bind(exame.valor)
        .bind(exame.valor_fonte_100)
        .bind(exame.cid)
        .fetch_one(&db.pool)
        .await?;

        Ok(record.0)
    }

    pub async fn atualizar_por_cod_interno(
        db: &DbService,
        cod_interno: i32,
        exame: Exame,
    ) -> Result<bool, Error> {
        let result = sqlx::query(
            r#"
            UPDATE exames
            SET descricao = $1, paciente_id = $2, medico_id = $3, cod_interno = $4,
                valor = $5, valor_fonte_100 = $6, cid = $7
            WHERE id = (
                SELECT id FROM exames
                WHERE cod_interno = $8
                ORDER BY id DESC
                LIMIT 1
            )
            "#,
        )
        .bind(exame.descricao)
        .bind(exame.paciente_id)
        .bind(exame.medico_id)
        .bind(exame.cod_interno)
        .bind(exame.valor)
        .bind(exame.valor_fonte_100)
        .bind(exame.cid)
        .bind(cod_interno)
        .execute(&db.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn deletar_por_cod_interno(db: &DbService, cod_interno: i32) -> Result<bool, Error> {
        let result = sqlx::query(
            r#"
            DELETE FROM exames
            WHERE id = (
                SELECT id FROM exames
                WHERE cod_interno = $1
                ORDER BY id DESC
                LIMIT 1
            )
            "#,
        )
        .bind(cod_interno)
        .execute(&db.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::DbService;
    use crate::models::Exame;
    use crate::services::{paginacao::ListQuery, MedicoService, PacienteService};
    use sqlx::PgPool;

    #[sqlx::test(migrations = "./migrations")]
    async fn listar_retorna_massa_inicial(pool: PgPool) {
        let db = DbService::from_pool(pool);
        let pagina = ExameService::listar(&db, ListQuery::default()).await.unwrap();
        assert!(pagina.data.len() >= 20);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn buscar_por_cod_interno_da_massa(pool: PgPool) {
        let db = DbService::from_pool(pool);
        let exame = ExameService::buscar_por_cod_interno(&db, 5001)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(exame.descricao, "Hemograma completo");
        assert_eq!(exame.cod_interno, Some(5001));
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn codigo_inexistente_nao_encontra(pool: PgPool) {
        let db = DbService::from_pool(pool);
        let resultado = ExameService::buscar_por_cod_interno(&db, 999999)
            .await
            .unwrap();
        assert!(resultado.is_none());
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn criar_atualizar_e_apagar_por_cod_interno(pool: PgPool) {
        let db = DbService::from_pool(pool);

        let medico = MedicoService::buscar_por_cns_ou_cpf(&db, "10000000001")
            .await
            .unwrap()
            .unwrap();
        let paciente = PacienteService::buscar_por_cns_ou_cpf(&db, "20000000001")
            .await
            .unwrap()
            .unwrap();

        let mut exame = Exame {
            id: None,
            descricao: "Exame Teste Unitário".to_string(),
            paciente_id: paciente.id.unwrap(),
            medico_id: medico.id.unwrap(),
            cod_interno: Some(9001),
            valor: 10.5,
            valor_fonte_100: 20.0,
            cid: Some("Z00".to_string()),
        };

        let id = ExameService::criar(&db, exame.clone()).await.unwrap();
        assert!(id > 0);

        let encontrado = ExameService::buscar_por_cod_interno(&db, 9001)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(encontrado.descricao, "Exame Teste Unitário");

        exame.descricao = "Exame Teste Atualizado".to_string();
        exame.valor = 15.0;
        let atualizou = ExameService::atualizar_por_cod_interno(&db, 9001, exame)
            .await
            .unwrap();
        assert!(atualizou);

        let atualizado = ExameService::buscar_por_cod_interno(&db, 9001)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(atualizado.descricao, "Exame Teste Atualizado");
        assert_eq!(atualizado.valor, 15.0);

        let apagou = ExameService::deletar_por_cod_interno(&db, 9001)
            .await
            .unwrap();
        assert!(apagou);
        assert!(ExameService::buscar_por_cod_interno(&db, 9001)
            .await
            .unwrap()
            .is_none());
    }
}
