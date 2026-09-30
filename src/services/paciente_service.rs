use crate::db::DbService;
use crate::models::Paciente;
use crate::services::documento::apenas_digitos;
use crate::services::paginacao::{montar_pagina, ListQuery, Pagina};
use sqlx::Error;

pub struct PacienteService;

impl PacienteService {
    pub async fn listar(db: &DbService, query: ListQuery) -> Result<Pagina<Paciente>, Error> {
        let limit = query.limit();
        let rows = sqlx::query_as::<_, Paciente>(
            r#"
            SELECT id, nome, cpf, cns, data_nascimento, nome_mae, endereco, bairro, telefone, email
            FROM pacientes
            WHERE $1::int4 IS NULL OR id < $1
            ORDER BY id DESC
            LIMIT $2
            "#,
        )
        .bind(query.cursor)
        .bind(limit + 1)
        .fetch_all(&db.pool)
        .await?;

        Ok(montar_pagina(rows, limit, |paciente| paciente.id))
    }

    pub async fn buscar_por_cns_ou_cpf(
        db: &DbService,
        documento: &str,
    ) -> Result<Option<Paciente>, Error> {
        let documento = apenas_digitos(documento);
        if documento.is_empty() {
            return Ok(None);
        }

        sqlx::query_as::<_, Paciente>(
            r#"
            SELECT id, nome, cpf, cns, data_nascimento, nome_mae, endereco, bairro, telefone, email
            FROM pacientes
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

    pub async fn criar(db: &DbService, paciente: Paciente) -> Result<i32, Error> {
        let record: (i32,) = sqlx::query_as(
            r#"
            INSERT INTO pacientes (nome, cpf, cns, data_nascimento, nome_mae, endereco, bairro, telefone, email)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING id
            "#,
        )
        .bind(paciente.nome)
        .bind(paciente.cpf)
        .bind(paciente.cns)
        .bind(paciente.data_nascimento)
        .bind(paciente.nome_mae)
        .bind(paciente.endereco)
        .bind(paciente.bairro)
        .bind(paciente.telefone)
        .bind(paciente.email)
        .fetch_one(&db.pool)
        .await?;

        Ok(record.0)
    }

    pub async fn atualizar_por_cns_ou_cpf(
        db: &DbService,
        documento: &str,
        paciente: Paciente,
    ) -> Result<bool, Error> {
        let documento = apenas_digitos(documento);
        if documento.is_empty() {
            return Ok(false);
        }

        let result = sqlx::query(
            r#"
            UPDATE pacientes
            SET nome = $1, cpf = $2, cns = $3, data_nascimento = $4, nome_mae = $5,
                endereco = $6, bairro = $7, telefone = $8, email = $9
            WHERE id = (
                SELECT id FROM pacientes
                WHERE regexp_replace(COALESCE(cpf, ''), '[^0-9]', '', 'g') = $10
                   OR regexp_replace(COALESCE(cns, ''), '[^0-9]', '', 'g') = $10
                ORDER BY id DESC
                LIMIT 1
            )
            "#,
        )
        .bind(paciente.nome)
        .bind(paciente.cpf)
        .bind(paciente.cns)
        .bind(paciente.data_nascimento)
        .bind(paciente.nome_mae)
        .bind(paciente.endereco)
        .bind(paciente.bairro)
        .bind(paciente.telefone)
        .bind(paciente.email)
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
            DELETE FROM pacientes
            WHERE id = (
                SELECT id FROM pacientes
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
    use crate::models::Paciente;
    use crate::services::paginacao::ListQuery;
    use chrono::NaiveDate;
    use sqlx::PgPool;

    fn paciente_teste() -> Paciente {
        Paciente {
            id: None,
            nome: "Paciente Teste Unitário".to_string(),
            cpf: Some("80000000088".to_string()),
            cns: Some("800000000000088".to_string()),
            data_nascimento: Some(NaiveDate::from_ymd_opt(1990, 1, 15).unwrap()),
            nome_mae: Some("Mãe Teste".to_string()),
            endereco: Some("Rua Teste, 1".to_string()),
            bairro: Some("Centro".to_string()),
            telefone: Some("31999990000".to_string()),
            email: Some("paciente.teste@test".to_string()),
        }
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn listar_retorna_massa_inicial(pool: PgPool) {
        let db = DbService::from_pool(pool);
        let pagina = PacienteService::listar(&db, ListQuery::default()).await.unwrap();
        assert!(pagina.data.len() >= 20);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn buscar_por_cpf_e_cns_da_massa(pool: PgPool) {
        let db = DbService::from_pool(pool);

        let por_cpf = PacienteService::buscar_por_cns_ou_cpf(&db, "200.000.000-01")
            .await
            .unwrap()
            .unwrap();
        assert_eq!(por_cpf.nome, "Aline Souza Ramos");

        let por_cns = PacienteService::buscar_por_cns_ou_cpf(&db, "200000000000001")
            .await
            .unwrap()
            .unwrap();
        assert_eq!(por_cns.cpf.as_deref(), Some("20000000001"));
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn documento_vazio_nao_encontra(pool: PgPool) {
        let db = DbService::from_pool(pool);
        let resultado = PacienteService::buscar_por_cns_ou_cpf(&db, "---")
            .await
            .unwrap();
        assert!(resultado.is_none());
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn criar_atualizar_e_apagar_por_cns(pool: PgPool) {
        let db = DbService::from_pool(pool);
        let mut paciente = paciente_teste();

        let id = PacienteService::criar(&db, paciente.clone()).await.unwrap();
        assert!(id > 0);

        let encontrado = PacienteService::buscar_por_cns_ou_cpf(&db, "800000000000088")
            .await
            .unwrap()
            .unwrap();
        assert_eq!(encontrado.nome, "Paciente Teste Unitário");

        paciente.bairro = Some("Savassi".to_string());
        let atualizou =
            PacienteService::atualizar_por_cns_ou_cpf(&db, "80000000088", paciente)
                .await
                .unwrap();
        assert!(atualizou);

        let atualizado = PacienteService::buscar_por_cns_ou_cpf(&db, "800.000.000-88")
            .await
            .unwrap()
            .unwrap();
        assert_eq!(atualizado.bairro.as_deref(), Some("Savassi"));

        let apagou = PacienteService::deletar_por_cns_ou_cpf(&db, "80000000088")
            .await
            .unwrap();
        assert!(apagou);
        assert!(PacienteService::buscar_por_cns_ou_cpf(&db, "80000000088")
            .await
            .unwrap()
            .is_none());
    }
}
