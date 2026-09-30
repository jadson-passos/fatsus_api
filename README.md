# fatsus_api

API REST em **Rust** (framework **Axum**) com **PostgreSQL**. Serve cadastros de **médicos**, **pacientes**, **exames** e **clínicas**, pensada para ser consumida por um programa local desenvolvido em Delphi. Essa aplicação faz parte de um contexto maior, onde existe uma aplicação em Delphi, uma outra aplicação em NodeJS (cron) e essa aplicação em Rust que em sua versao 1.0 foi desenvolvida em Java e na versao 2.0 foi refatorada em NodeJS e agora em sua ultima versao em Rust. Esse serviços estão disponíveis para clinicas que faturam SUS e precisam integrar dados do SISREG os quais são persistidos no PostgresSQL pelo cron.

A segurança tem **duas camadas**:

1. **API Key** — identifica a aplicação cliente (header `X-API-Key`)
2. **JWT** — identifica o utilizador depois do login (header `Authorization: Bearer ...`)

---

## Serviços disponibilizados

- API em http://localhost:3000
- Documentação interativa (Swagger) em http://localhost:3000/swagger-ui/
- Base de dados com **20 registos de teste**

---

## 1. O que precisa ser instalado

- [Docker Desktop](https://www.docker.com/products/docker-desktop/) (Windows/macOS) **ou** Docker Engine + plugin Compose (Linux)
- Git

No Windows, pode usar o terminal do **WSL** (Ubuntu). Confirme que o Docker está rodando



---

## 2. Baixar o projeto

```bash
git clone <URL-DO-REPOSITORIO>
cd fatsus_api
```

Entre na pasta do projeto e execute passo seguinte.

---

## 3. Criar as pastas/aqruivos de configuração local

```bash
cp .env.example .env
cp .cargo/config.toml.example .cargo/config.toml
```

No Windows (PowerShell) se `cp` não existir:

```powershell
Copy-Item .env.example .env
Copy-Item .cargo\config.toml.example .cargo\config.toml
```

---

## 4. Subir a API e a base de dados

Na pasta do projeto:

```bash
docker compose up -d --build
```

A **primeira** vez demora alguns minutos pois o Docker compila o Rust.

Quando terminar:

```bash
docker compose ps
```

Devem aparecer `fatsus_api` e `fatsus_postgres` como **running**.

- API: http://localhost:3000
- Swagger: http://localhost:3000/swagger-ui/
- Postgres (se quiseres um cliente SQL): `localhost`, porta **5435**, utilizador `postgres`, senha `postgrespassword`, base `fatsus_db`

Para ver os logs se algo falhar:

```bash
docker compose logs -f api
```

Sair dos logs: `Ctrl+C`

---

## 5. Credenciais de demonstração (só ambiente local)

| Para quê | Valor |
| --- | --- |
| Header `X-API-Key` | `fatsus-delphi-local-api-key-min-32-chars` |
| Utilizador do login | `admin` |
| Senha do login | `fatsus-operador-local-dev` |

O **JWT não é uma chave fixa**. Obtem um token novo em cada login (válido 40 horas para evitar geração desnecearia).

---

## 6. Primeiro teste: login

No terminal:

```bash
curl -sS -X POST "http://localhost:3000/api/login" \
  -H "Content-Type: application/json" \
  -H "X-API-Key: fatsus-delphi-local-api-key-min-32-chars" \
  -d '{"usuario":"admin","senha":"fatsus-operador-local-dev"}'
```

Resposta:

```json
{"token":"eyJhbGciOiJIUzI1NiJ9..."}
```

Copia o valor de `token`. Nos pedidos seguintes envias **os dois** headers:

- `X-API-Key: fatsus-delphi-local-api-key-min-32-chars`
- `Authorization: Bearer COLE_O_TOKEN_AQUI`

Exemplo — listar médicos (primeira página):

```bash
curl -sS "http://localhost:3000/api/medicos?limit=20" \
  -H "X-API-Key: fatsus-delphi-local-api-key-min-32-chars" \
  -H "Authorization: Bearer COLE_O_TOKEN_AQUI"
```

A lista **não** devolve um array solto. Devolve uma **página**:

```json
{
  "data": [ ... ],
  "nextCursor": 12,
  "hasMore": true,
  "limit": 20
}
```

Se `hasMore` for `true`, a página seguinte é:

`GET /api/medicos?limit=20&cursor=12`  
(usa o número que veio em `nextCursor`)

`limit` padrão: 50. Máximo: 100.

---

## 7. Usar o Swagger

1. Abre http://localhost:3000/swagger-ui/
2. Abre **POST /api/login** → **Try it out**
3. No campo de header **X-API-Key**, cola: `fatsus-delphi-local-api-key-min-32-chars`
4. No corpo JSON usa `admin` e `fatsus-operador-local-dev` e clica **Execute**
5. Copia o `token` da resposta
6. Nos outros endpoints (médicos, pacientes, exames e clínicas), em **X-Access-Token**, cola **só o token** (não coloques `Bearer `)
7. Experimenta **GET /api/medicos**, **GET /api/pacientes**, **GET /api/exames** ou **GET /api/clinicas** (`cursor` vazio na primeira página)

---

## 8. Como funcionam as rotas

Todas as rotas abaixo (exceto o login) precisam de **API Key + JWT**.

| Recurso | Listar / criar | Localizar, editar, apagar por |
| --- | --- | --- |
| Médicos | `/api/medicos` | **CNS** ou **CPF** |
| Pacientes | `/api/pacientes` | **CNS** ou **CPF** |
| Clínicas | `/api/clinicas` | **CNES** ou **CNPJ** |
| Exames | `/api/exames` | **cod_interno** |

Pontos e traços nos documentos são ignorados (`100.000.000-01` funciona).

Exemplos da massa de teste (depois do login):

```bash
# Médico por CPF
curl -sS "http://localhost:3000/api/medicos/10000000001" \
  -H "X-API-Key: fatsus-delphi-local-api-key-min-32-chars" \
  -H "Authorization: Bearer COLE_O_TOKEN_AQUI"

# Clínica por CNPJ
curl -sS "http://localhost:3000/api/clinicas/11222333000101" \
  -H "X-API-Key: fatsus-delphi-local-api-key-min-32-chars" \
  -H "Authorization: Bearer COLE_O_TOKEN_AQUI"

# Exame pelo código interno
curl -sS "http://localhost:3000/api/exames/5001" \
  -H "X-API-Key: fatsus-delphi-local-api-key-min-32-chars" \
  -H "Authorization: Bearer COLE_O_TOKEN_AQUI"
```

- `GET` e `POST` na URL sem documento = listar (paginado) e criar
- `GET` / `PUT` / `DELETE` com documento ou `cod_interno` = um registo

---

## 9. Parar, recomeçar ou apagar tudo

```bash
# Parar (mantém a base de dados)
docker compose stop

# Volta rodar
docker compose start

# Parar e remover (mantém o volume da base)
docker compose down

# Apagar também os dados do Postgres (massa de teste volta a ser criada na próxima subida)
docker compose down -v
```

---

## 10. Testes automatizados (opcional)

Isto **já precisa de Rust**, além do Docker (o Postgres tem quer estar rodando).

```bash
# Instalar Rust: https://rustup.rs
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"

# Só a base, usei a porta 5435 pois havia outra seção do Postgres rodando em minha maquina
docker compose up -d db

cp .env.example .env
cp .cargo/config.toml.example .cargo/config.toml

cargo test
```

Esperado: todos os testes verde.

---

## 11. Rodar sem Docker (avançado)

Precisa do: Rust, PostgreSQL 15 (ou o  `db` do Compose) e as variáveis do `.env`.

```bash
cp .env.example .env
# DATABASE_URL deve apontar para o  Postgres (no Compose: porta 5435)
docker compose up -d db
cargo run --release
```

A API fica em http://localhost:3000. As migrations e a massa de teste rodam no build.

---


