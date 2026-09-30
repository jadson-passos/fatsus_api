use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde_json::{json, Value};

use crate::{
    auth::AuthenticatedUser,
    db::DbService,
    models::Medico,
    services::{
        paginacao::{ListQuery, PaginaMedicos},
        MedicoService,
    },
};

#[utoipa::path(
    get,
    path = "/api/medicos",
    params(
        ListQuery,
        ("X-API-Key" = String, Header, description = "Chave da aplicação cliente", example = "fatsus-delphi-local-api-key-min-32-chars"),
        ("X-Access-Token" = String, Header, description = "Cole só o token do login (sem a palavra Bearer)", example = "")
    ),
    responses(
        (status = 200, description = "Página de médicos cadastrados", body = PaginaMedicos),
        (status = 401, description = "Token JWT não fornecido ou inválido"),
        (status = 403, description = "API Key inválida")
    ),
    security(("jwt_token" = [], "api_key" = [])),
    tag = "Médicos"
)]
pub async fn listar(
    _auth: AuthenticatedUser,
    State(db): State<DbService>,
    Query(query): Query<ListQuery>,
) -> Result<Json<PaginaMedicos>, StatusCode> {
    MedicoService::listar(&db, query)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

#[utoipa::path(
    get,
    path = "/api/medicos/{documento}",
    params(
        ("documento" = String, Path, description = "CNS (15 dígitos) ou CPF (11 dígitos) do médico"),
        ("X-API-Key" = String, Header, description = "Chave da aplicação cliente", example = "fatsus-delphi-local-api-key-min-32-chars"),
        ("X-Access-Token" = String, Header, description = "Cole só o token do login (sem a palavra Bearer)", example = "")
    ),
    responses(
        (status = 200, description = "Médico encontrado", body = Medico),
        (status = 400, description = "Documento inválido"),
        (status = 404, description = "Médico não encontrado")
    ),
    security(("jwt_token" = [], "api_key" = [])),
    tag = "Médicos"
)]
pub async fn buscar_por_cns_ou_cpf(
    _auth: AuthenticatedUser,
    State(db): State<DbService>,
    Path(documento): Path<String>,
) -> Result<Json<Medico>, StatusCode> {
    if !documento.chars().any(|c| c.is_ascii_digit()) {
        return Err(StatusCode::BAD_REQUEST);
    }

    match MedicoService::buscar_por_cns_ou_cpf(&db, &documento).await {
        Ok(Some(medico)) => Ok(Json(medico)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[utoipa::path(
    post,
    path = "/api/medicos",
    params(
        ("X-API-Key" = String, Header, description = "Chave da aplicação cliente", example = "fatsus-delphi-local-api-key-min-32-chars"),
        ("X-Access-Token" = String, Header, description = "Cole só o token do login (sem a palavra Bearer)", example = "")
    ),
    request_body = Medico,
    responses(
        (status = 201, description = "Médico cadastrado com sucesso"),
        (status = 500, description = "Erro interno")
    ),
    security(("jwt_token" = [], "api_key" = [])),
    tag = "Médicos"
)]
pub async fn criar(
    _auth: AuthenticatedUser,
    State(db): State<DbService>,
    Json(payload): Json<Medico>,
) -> Result<(StatusCode, Json<Value>), StatusCode> {
    match MedicoService::criar(&db, payload).await {
        Ok(id) => Ok((StatusCode::CREATED, Json(json!({ "id": id })))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[utoipa::path(
    put,
    path = "/api/medicos/{documento}",
    params(
        ("documento" = String, Path, description = "CNS ou CPF do médico a atualizar"),
        ("X-API-Key" = String, Header, description = "Chave da aplicação cliente", example = "fatsus-delphi-local-api-key-min-32-chars"),
        ("X-Access-Token" = String, Header, description = "Cole só o token do login (sem a palavra Bearer)", example = "")
    ),
    request_body = Medico,
    responses(
        (status = 200, description = "Médico atualizado com sucesso"),
        (status = 400, description = "Documento inválido"),
        (status = 404, description = "Médico não encontrado")
    ),
    security(("jwt_token" = [], "api_key" = [])),
    tag = "Médicos"
)]
pub async fn atualizar(
    _auth: AuthenticatedUser,
    State(db): State<DbService>,
    Path(documento): Path<String>,
    Json(payload): Json<Medico>,
) -> Result<StatusCode, StatusCode> {
    if !documento.chars().any(|c| c.is_ascii_digit()) {
        return Err(StatusCode::BAD_REQUEST);
    }

    match MedicoService::atualizar_por_cns_ou_cpf(&db, &documento, payload).await {
        Ok(true) => Ok(StatusCode::OK),
        Ok(false) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[utoipa::path(
    delete,
    path = "/api/medicos/{documento}",
    params(
        ("documento" = String, Path, description = "CNS ou CPF do médico a remover"),
        ("X-API-Key" = String, Header, description = "Chave da aplicação cliente", example = "fatsus-delphi-local-api-key-min-32-chars"),
        ("X-Access-Token" = String, Header, description = "Cole só o token do login (sem a palavra Bearer)", example = "")
    ),
    responses(
        (status = 204, description = "Médico removido com sucesso"),
        (status = 400, description = "Documento inválido"),
        (status = 404, description = "Médico não encontrado")
    ),
    security(("jwt_token" = [], "api_key" = [])),
    tag = "Médicos"
)]
pub async fn apagar(
    _auth: AuthenticatedUser,
    State(db): State<DbService>,
    Path(documento): Path<String>,
) -> Result<StatusCode, StatusCode> {
    if !documento.chars().any(|c| c.is_ascii_digit()) {
        return Err(StatusCode::BAD_REQUEST);
    }

    match MedicoService::deletar_por_cns_ou_cpf(&db, &documento).await {
        Ok(true) => Ok(StatusCode::NO_CONTENT),
        Ok(false) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
