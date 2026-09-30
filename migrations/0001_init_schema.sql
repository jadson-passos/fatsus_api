-- Migration: Criação inicial do esquema fatsus_db

CREATE TABLE IF NOT EXISTS clinicas (
    id SERIAL PRIMARY KEY,
    razao VARCHAR(150) NOT NULL,
    cnpj VARCHAR(14) NOT NULL UNIQUE,
    cnes VARCHAR(20),
    endereco VARCHAR(200),
    bairro VARCHAR(100),
    telefone VARCHAR(20),
    email VARCHAR(100)
);

CREATE TABLE IF NOT EXISTS medicos (
    id SERIAL PRIMARY KEY,
    nome VARCHAR(150) NOT NULL,
    cns VARCHAR(20),
    cbo VARCHAR(10),
    cpf VARCHAR(11),
    especialidade VARCHAR(100)
);

CREATE TABLE IF NOT EXISTS pacientes (
    id SERIAL PRIMARY KEY,
    nome VARCHAR(150) NOT NULL,
    cpf VARCHAR(11),
    cns VARCHAR(20),
    data_nascimento DATE,
    nome_mae VARCHAR(150),
    endereco VARCHAR(200),
    bairro VARCHAR(100),
    telefone VARCHAR(20),
    email VARCHAR(100)
);

CREATE TABLE IF NOT EXISTS exames (
    id SERIAL PRIMARY KEY,
    descricao VARCHAR(200) NOT NULL,
    paciente_id INT NOT NULL REFERENCES pacientes(id) ON DELETE CASCADE,
    medico_id INT NOT NULL REFERENCES medicos(id) ON DELETE CASCADE,
    cod_interno INT,
    valor NUMERIC(10, 2) NOT NULL DEFAULT 0.00,
    valor_fonte_100 NUMERIC(10, 2) NOT NULL DEFAULT 0.00,
    cid VARCHAR(10)
);