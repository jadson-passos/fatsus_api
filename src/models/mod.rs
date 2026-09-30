use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Medico {
    pub id: Option<i32>,
    pub nome: String,
    pub cns: Option<String>,
    pub cbo: Option<String>,
    pub cpf: Option<String>,
    pub especialidade: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Paciente {
    pub id: Option<i32>,
    pub nome: String,
    pub cpf: Option<String>,
    pub cns: Option<String>,
    pub data_nascimento: Option<NaiveDate>,
    pub nome_mae: Option<String>,
    pub endereco: Option<String>,
    pub bairro: Option<String>,
    pub telefone: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Exame {
    pub id: Option<i32>,
    pub descricao: String,
    pub paciente_id: i32,
    pub medico_id: i32,
    pub cod_interno: Option<i32>,
    pub valor: f64,
    pub valor_fonte_100: f64,
    pub cid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Clinica {
    pub id: Option<i32>,
    pub razao: String,
    pub cnpj: String,
    pub cnes: Option<String>,
    pub endereco: Option<String>,
    pub bairro: Option<String>,
    pub telefone: Option<String>,
    pub email: Option<String>,
}