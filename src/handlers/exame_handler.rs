use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde_json::{json, Value};

use crate::{
    auth::AuthenticatedUser,
    db::DbService,
    models::Exame,
    services::{
        paginacao::{ListQuery, PaginaExames},
        ExameService,
    },
};

#[utoipa::path(
    get,
    path = "/api/exames",
    params(
        ListQuery,
        ("X-API-Key" = String, Header, description = "Chave da aplicação cliente", example = "fatsus-delphi-local-api-key-min-32-chars"),
        ("X-Access-Token" = String, Header, description = "Cole só o token do login (sem a palavra Bearer)", example = "")
    ),
    responses(
        (status = 200, description = "Página de exames cadastrados", body = PaginaExames),
        (status = 401, description = "Token JWT não fornecido ou inválido"),
        (status = 403, description = "API Key inválida")
    ),
    security(("jwt_token" = [], "api_key" = [])),
    tag = "Exames"
)]
pub async fn listar(
    _auth: AuthenticatedUser,
    State(db): State<DbService>,
    Query(query): Query<ListQuery>,
) -> Result<Json<PaginaExames>, StatusCode> {
    ExameService::listar(&db, query)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

#[utoipa::path(
    get,
    path = "/api/exames/{cod_interno}",
    params(
        ("cod_interno" = i32, Path, description = "Código interno do exame"),
        ("X-API-Key" = String, Header, description = "Chave da aplicação cliente", example = "fatsus-delphi-local-api-key-min-32-chars"),
        ("X-Access-Token" = String, Header, description = "Cole só o token do login (sem a palavra Bearer)", example = "")
    ),
    responses(
        (status = 200, description = "Exame encontrado", body = Exame),
        (status = 404, description = "Exame não encontrado")
    ),
    security(("jwt_token" = [], "api_key" = [])),
    tag = "Exames"
)]
pub async fn buscar_por_cod_interno(
    _auth: AuthenticatedUser,
    State(db): State<DbService>,
    Path(cod_interno): Path<i32>,
) -> Result<Json<Exame>, StatusCode> {
    match ExameService::buscar_por_cod_interno(&db, cod_interno).await {
        Ok(Some(exame)) => Ok(Json(exame)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[utoipa::path(
    post,
    path = "/api/exames",
    params(
        ("X-API-Key" = String, Header, description = "Chave da aplicação cliente", example = "fatsus-delphi-local-api-key-min-32-chars"),
        ("X-Access-Token" = String, Header, description = "Cole só o token do login (sem a palavra Bearer)", example = "")
    ),
    request_body = Exame,
    responses(
        (status = 201, description = "Exame cadastrado com sucesso"),
        (status = 500, description = "Erro interno")
    ),
    security(("jwt_token" = [], "api_key" = [])),
    tag = "Exames"
)]
pub async fn criar(
    _auth: AuthenticatedUser,
    State(db): State<DbService>,
    Json(payload): Json<Exame>,
) -> Result<(StatusCode, Json<Value>), StatusCode> {
    match ExameService::criar(&db, payload).await {
        Ok(id) => Ok((StatusCode::CREATED, Json(json!({ "id": id })))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[utoipa::path(
    put,
    path = "/api/exames/{cod_interno}",
    params(
        ("cod_interno" = i32, Path, description = "Código interno do exame a atualizar"),
        ("X-API-Key" = String, Header, description = "Chave da aplicação cliente", example = "fatsus-delphi-local-api-key-min-32-chars"),
        ("X-Access-Token" = String, Header, description = "Cole só o token do login (sem a palavra Bearer)", example = "")
    ),
    request_body = Exame,
    responses(
        (status = 200, description = "Exame atualizado com sucesso"),
        (status = 404, description = "Exame não encontrado")
    ),
    security(("jwt_token" = [], "api_key" = [])),
    tag = "Exames"
)]
pub async fn atualizar(
    _auth: AuthenticatedUser,
    State(db): State<DbService>,
    Path(cod_interno): Path<i32>,
    Json(payload): Json<Exame>,
) -> Result<StatusCode, StatusCode> {
    match ExameService::atualizar_por_cod_interno(&db, cod_interno, payload).await {
        Ok(true) => Ok(StatusCode::OK),
        Ok(false) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[utoipa::path(
    delete,
    path = "/api/exames/{cod_interno}",
    params(
        ("cod_interno" = i32, Path, description = "Código interno do exame a remover"),
        ("X-API-Key" = String, Header, description = "Chave da aplicação cliente", example = "fatsus-delphi-local-api-key-min-32-chars"),
        ("X-Access-Token" = String, Header, description = "Cole só o token do login (sem a palavra Bearer)", example = "")
    ),
    responses(
        (status = 204, description = "Exame removido com sucesso"),
        (status = 404, description = "Exame não encontrado")
    ),
    security(("jwt_token" = [], "api_key" = [])),
    tag = "Exames"
)]
pub async fn apagar(
    _auth: AuthenticatedUser,
    State(db): State<DbService>,
    Path(cod_interno): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    match ExameService::deletar_por_cod_interno(&db, cod_interno).await {
        Ok(true) => Ok(StatusCode::NO_CONTENT),
        Ok(false) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
