-- Massa de teste: 20 registros por tabela (idempotente)

INSERT INTO clinicas (razao, cnpj, cnes, endereco, bairro, telefone, email)
SELECT v.razao, v.cnpj, v.cnes, v.endereco, v.bairro, v.telefone, v.email
FROM (
    VALUES
        ('Clínica São Lucas LTDA',           '11222333000101', '2560001', 'Rua das Palmeiras, 100',     'Centro',        '11940001001', 'contato@saolucas.test'),
        ('Instituto Vida Plena',             '11222333000102', '2560002', 'Av. Brasil, 250',            'Jardins',       '11940001002', 'contato@vidaplena.test'),
        ('Clínica Horizonte Saúde',          '11222333000103', '2560003', 'Rua Bahia, 45',              'Savassi',       '31940001003', 'contato@horizonte.test'),
        ('Centro Médico Boa Vista',          '11222333000104', '2560004', 'Av. Amazonas, 890',          'Boa Vista',     '31940001004', 'contato@boavista.test'),
        ('Clínica Santa Clara',              '11222333000105', '2560005', 'Rua Ceará, 12',              'Funcionários',  '31940001005', 'contato@santaclara.test'),
        ('Policlínica Norte',                '11222333000106', '2560006', 'Av. Antônio Carlos, 1500',   'Pampulha',      '31940001006', 'contato@norte.test'),
        ('Clínica Esperança',                '11222333000107', '2560007', 'Rua Goiás, 330',             'Lourdes',       '31940001007', 'contato@esperanca.test'),
        ('Instituto CardioVida',             '11222333000108', '2560008', 'Av. Afonso Pena, 2000',      'Centro',        '31940001008', 'contato@cardiovida.test'),
        ('Clínica Família e Saúde',          '11222333000109', '2560009', 'Rua Pernambuco, 77',         'Funcionários',  '31940001009', 'contato@familia.test'),
        ('Centro de Diagnóstico Alfa',       '11222333000110', '2560010', 'Av. do Contorno, 4400',      'Santa Efigênia','31940001010', 'contato@alfa.test'),
        ('Clínica Bem Estar',                '11222333000111', '2560011', 'Rua da Bahia, 1200',         'Lourdes',       '31940001011', 'contato@bemestar.test'),
        ('Instituto NeuroSul',               '11222333000112', '2560012', 'Av. Bias Fortes, 310',       'Barro Preto',   '31940001012', 'contato@neurosul.test'),
        ('Clínica Primavera',                '11222333000113', '2560013', 'Rua Espírito Santo, 88',     'Centro',        '31940001013', 'contato@primavera.test'),
        ('Centro Médico São José',           '11222333000114', '2560014', 'Av. Amazonas, 321',          'Centro',        '31940001014', 'contato@saojose.test'),
        ('Clínica Ouro Preto Saúde',         '11222333000115', '2560015', 'Rua Ouro Preto, 500',        'Santo Agostinho','31940001015','contato@ouropreto.test'),
        ('Instituto Visão Clara',            '11222333000116', '2560016', 'Av. Raja Gabaglia, 1001',    'Luxemburgo',    '31940001016', 'contato@visaoclara.test'),
        ('Clínica Cidade Nova',              '11222333000117', '2560017', 'Rua Itajubá, 210',           'Cidade Nova',   '31940001017', 'contato@cidadenova.test'),
        ('Policlínica Leste',                '11222333000118', '2560018', 'Av. Cristiano Machado, 2200','Cidade Nova',   '31940001018', 'contato@leste.test'),
        ('Clínica Monte Verde',              '11222333000119', '2560019', 'Rua Padre Eustáquio, 640',   'Padre Eustáquio','31940001019','contato@monteverde.test'),
        ('Centro de Saúde Atlântica',        '11222333000120', '2560020', 'Av. Pedro II, 90',           'Carlos Prates', '31940001020', 'contato@atlantica.test')
) AS v(razao, cnpj, cnes, endereco, bairro, telefone, email)
WHERE NOT EXISTS (SELECT 1 FROM clinicas);

INSERT INTO medicos (nome, cns, cbo, cpf, especialidade)
SELECT v.nome, v.cns, v.cbo, v.cpf, v.especialidade
FROM (
    VALUES
        ('Ana Beatriz Nogueira',     '100000000000001', '225125', '10000000001', 'Cardiologia'),
        ('Bruno Henrique Castro',    '100000000000002', '225265', '10000000002', 'Clínica Geral'),
        ('Camila Duarte Ferreira',   '100000000000003', '225142', '10000000003', 'Pediatria'),
        ('Diego Alves Monteiro',     '100000000000004', '225250', '10000000004', 'Ortopedia'),
        ('Eduarda Lima Rocha',       '100000000000005', '225133', '10000000005', 'Ginecologia'),
        ('Felipe Santos Oliveira',   '100000000000006', '225170', '10000000006', 'Neurologia'),
        ('Gabriela Costa Mendes',    '100000000000007', '225135', '10000000007', 'Dermatologia'),
        ('Henrique Souza Pinto',     '100000000000008', '225180', '10000000008', 'Oftalmologia'),
        ('Isabela Martins Araujo',   '100000000000009', '225310', '10000000009', 'Psiquiatria'),
        ('João Pedro Barros',        '100000000000010', '225255', '10000000010', 'Urologia'),
        ('Karina Ribeiro Campos',    '100000000000011', '225120', '10000000011', 'Endocrinologia'),
        ('Lucas Fernandes Dias',     '100000000000012', '225280', '10000000012', 'Otorrinolaringologia'),
        ('Mariana Teixeira Lopes',   '100000000000013', '225185', '10000000013', 'Oncologia'),
        ('Nicolas Almeida Cruz',     '100000000000014', '225275', '10000000014', 'Cirurgia Geral'),
        ('Olivia Carvalho Nunes',    '100000000000015', '225195', '10000000015', 'Reumatologia'),
        ('Paulo Ricardo Moura',      '100000000000016', '225210', '10000000016', 'Gastroenterologia'),
        ('Queila Barbosa Freitas',   '100000000000017', '225230', '10000000017', 'Infectologia'),
        ('Rafael Gomes Vieira',      '100000000000018', '225155', '10000000018', 'Pneumologia'),
        ('Sofia Almeida Prado',      '100000000000019', '225330', '10000000019', 'Radiologia'),
        ('Thiago Moreira Cunha',     '100000000000020', '225140', '10000000020', 'Nefrologia')
) AS v(nome, cns, cbo, cpf, especialidade)
WHERE NOT EXISTS (SELECT 1 FROM medicos);

INSERT INTO pacientes (nome, cpf, cns, data_nascimento, nome_mae, endereco, bairro, telefone, email)
SELECT v.nome, v.cpf, v.cns, v.data_nascimento::date, v.nome_mae, v.endereco, v.bairro, v.telefone, v.email
FROM (
    VALUES
        ('Aline Souza Ramos',        '20000000001', '200000000000001', '1988-03-12', 'Helena Souza Ramos',      'Rua das Acácias, 10',    'Centro',         '31988001001', 'aline.ramos@test'),
        ('Breno Carvalho Lima',      '20000000002', '200000000000002', '1975-11-02', 'Aparecida Carvalho Lima', 'Av. Amazonas, 55',       'Floresta',       '31988001002', 'breno.lima@test'),
        ('Cíntia Oliveira Dias',     '20000000003', '200000000000003', '1992-07-21', 'Lúcia Oliveira Dias',     'Rua Bahia, 200',         'Savassi',        '31988001003', 'cintia.dias@test'),
        ('Daniel Moreira Pinto',     '20000000004', '200000000000004', '1969-01-30', 'Terezinha Moreira Pinto', 'Rua Goiás, 18',          'Lourdes',        '31988001004', 'daniel.pinto@test'),
        ('Elisa Ferreira Castro',    '20000000005', '200000000000005', '2001-05-09', 'Marina Ferreira Castro',  'Av. Afonso Pena, 900',   'Centro',         '31988001005', 'elisa.castro@test'),
        ('Fábio Nogueira Reis',      '20000000006', '200000000000006', '1984-09-17', 'Célia Nogueira Reis',     'Rua Ceará, 77',          'Funcionários',   '31988001006', 'fabio.reis@test'),
        ('Giovana Martins Silva',    '20000000007', '200000000000007', '1996-12-04', 'Sandra Martins Silva',    'Av. do Contorno, 120',   'Santa Efigênia', '31988001007', 'giovana.silva@test'),
        ('Hugo Almeida Teixeira',    '20000000008', '200000000000008', '1979-04-25', 'Neusa Almeida Teixeira',  'Rua Espírito Santo, 33', 'Centro',         '31988001008', 'hugo.teixeira@test'),
        ('Íris Barbosa Mendes',      '20000000009', '200000000000009', '1990-08-14', 'Rita Barbosa Mendes',     'Rua da Bahia, 410',      'Lourdes',        '31988001009', 'iris.mendes@test'),
        ('Jorge Luiz Campos',        '20000000010', '200000000000010', '1964-02-08', 'Geraldina Luiz Campos',   'Av. Antônio Carlos, 80', 'Pampulha',       '31988001010', 'jorge.campos@test'),
        ('Lívia Costa Andrade',      '20000000011', '200000000000011', '1986-10-19', 'Fátima Costa Andrade',    'Rua Itajubá, 15',        'Cidade Nova',    '31988001011', 'livia.andrade@test'),
        ('Marcelo Duarte Rocha',     '20000000012', '200000000000012', '1972-06-28', 'Ivone Duarte Rocha',      'Av. Cristiano Machado, 9','Cidade Nova',    '31988001012', 'marcelo.rocha@test'),
        ('Natália Pires Azevedo',    '20000000013', '200000000000013', '1998-03-03', 'Sônia Pires Azevedo',     'Rua Ouro Preto, 60',     'Santo Agostinho','31988001013', 'natalia.azevedo@test'),
        ('Otávio Henrique Lopes',    '20000000014', '200000000000014', '1958-12-11', 'Benedita Henrique Lopes', 'Rua Padre Eustáquio, 22','Padre Eustáquio','31988001014', 'otavio.lopes@test'),
        ('Patrícia Gomes Vieira',    '20000000015', '200000000000015', '1981-07-07', 'Marlene Gomes Vieira',    'Av. Raja Gabaglia, 300', 'Luxemburgo',     '31988001015', 'patricia.vieira@test'),
        ('Renato Alves Prado',       '20000000016', '200000000000016', '1994-09-23', 'Creuza Alves Prado',      'Rua Itajubá, 401',       'Cidade Nova',    '31988001016', 'renato.prado@test'),
        ('Sabrina Melo Cardoso',     '20000000017', '200000000000017', '2003-01-15', 'Regina Melo Cardoso',     'Av. Pedro II, 48',       'Carlos Prates',  '31988001017', 'sabrina.cardoso@test'),
        ('Tiago Ribeiro Nunes',      '20000000018', '200000000000018', '1977-11-29', 'Dalva Ribeiro Nunes',     'Rua das Palmeiras, 8',   'Centro',         '31988001018', 'tiago.nunes@test'),
        ('Úrsula Fernandes Dias',    '20000000019', '200000000000019', '1967-05-05', 'Lourdes Fernandes Dias',  'Av. Brasil, 19',         'Jardins',        '31988001019', 'ursula.dias@test'),
        ('Vinícius Rocha Freitas',   '20000000020', '200000000000020', '1991-02-20', 'Paula Rocha Freitas',     'Rua Pernambuco, 140',    'Funcionários',   '31988001020', 'vinicius.freitas@test')
) AS v(nome, cpf, cns, data_nascimento, nome_mae, endereco, bairro, telefone, email)
WHERE NOT EXISTS (SELECT 1 FROM pacientes);

INSERT INTO exames (descricao, paciente_id, medico_id, cod_interno, valor, valor_fonte_100, cid)
SELECT
    v.descricao,
    p.id,
    m.id,
    v.cod_interno,
    v.valor,
    v.valor_fonte_100,
    v.cid
FROM (
    VALUES
        ('Hemograma completo',           '20000000001', '10000000001', 5001,  45.00,  80.00, 'D64'),
        ('Glicemia de jejum',            '20000000002', '10000000002', 5002,  22.50,  40.00, 'E11'),
        ('Raio-X de tórax',              '20000000003', '10000000018', 5003,  90.00, 150.00, 'J18'),
        ('Ultrassom abdominal',          '20000000004', '10000000016', 5004, 180.00, 260.00, 'K29'),
        ('Eletrocardiograma',            '20000000005', '10000000001', 5005,  70.00, 120.00, 'I10'),
        ('Ressonância de joelho',        '20000000006', '10000000004', 5006, 650.00, 890.00, 'M17'),
        ('Consulta dermatológica',       '20000000007', '10000000007', 5007, 150.00, 220.00, 'L30'),
        ('Exame de fundo de olho',       '20000000008', '10000000008', 5008, 120.00, 180.00, 'H52'),
        ('Avaliação psiquiátrica',       '20000000009', '10000000009', 5009, 200.00, 280.00, 'F32'),
        ('PSA total',                    '20000000010', '10000000010', 5010,  55.00,  95.00, 'N40'),
        ('TSH e T4 livre',               '20000000011', '10000000011', 5011,  85.00, 130.00, 'E03'),
        ('Audiometria tonal',            '20000000012', '10000000012', 5012, 110.00, 170.00, 'H90'),
        ('Mamografia digital',           '20000000013', '10000000013', 5013, 220.00, 310.00, 'C50'),
        ('Endoscopia digestiva',         '20000000014', '10000000016', 5014, 480.00, 640.00, 'K21'),
        ('Fator reumatoide',             '20000000015', '10000000015', 5015,  40.00,  70.00, 'M06'),
        ('Espirometria',                 '20000000016', '10000000018', 5016, 130.00, 190.00, 'J44'),
        ('Teste rápido HIV',             '20000000017', '10000000017', 5017,  35.00,  60.00, 'B24'),
        ('Tomografia de crânio',         '20000000018', '10000000006', 5018, 520.00, 740.00, 'G43'),
        ('Ureia e creatinina',           '20000000019', '10000000020', 5019,  38.00,  65.00, 'N18'),
        ('Ecocardiograma',               '20000000020', '10000000001', 5020, 310.00, 450.00, 'I50')
) AS v(descricao, paciente_cpf, medico_cpf, cod_interno, valor, valor_fonte_100, cid)
JOIN pacientes p ON p.cpf = v.paciente_cpf
JOIN medicos m ON m.cpf = v.medico_cpf
WHERE NOT EXISTS (SELECT 1 FROM exames);
