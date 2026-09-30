use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::models::{Clinica, Exame, Medico, Paciente};

pub const LIMITE_PADRAO: i64 = 50;
pub const LIMITE_MAXIMO: i64 = 100;

#[derive(Debug, Default, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct ListQuery {
    /// Quantidade de registros por página (padrão 50, máximo 100)
    pub limit: Option<i64>,
    /// ID do último registro da página anterior (paginação keyset)
    pub cursor: Option<i32>,
}

impl ListQuery {
    pub fn limit(&self) -> i64 {
        self.limit.unwrap_or(LIMITE_PADRAO).clamp(1, LIMITE_MAXIMO)
    }
}

#[derive(Debug, Serialize, ToSchema)]
#[aliases(
    PaginaMedicos = Pagina<Medico>,
    PaginaPacientes = Pagina<Paciente>,
    PaginaExames = Pagina<Exame>,
    PaginaClinicas = Pagina<Clinica>
)]
#[serde(rename_all = "camelCase")]
pub struct Pagina<T> {
    pub data: Vec<T>,
    pub next_cursor: Option<i32>,
    pub has_more: bool,
    pub limit: i64,
}

pub fn montar_pagina<T>(mut rows: Vec<T>, limit: i64, id_de: impl Fn(&T) -> Option<i32>) -> Pagina<T> {
    let has_more = rows.len() as i64 > limit;
    if has_more {
        rows.truncate(limit as usize);
    }

    let next_cursor = if has_more {
        rows.last().and_then(id_de)
    } else {
        None
    };

    Pagina {
        data: rows,
        next_cursor,
        has_more,
        limit,
    }
}
