CREATE TABLE pessoa (
    pes_id SERIAL PRIMARY KEY,
    pes_nome VARCHAR(200) NOT NULL,
    pes_data_nascimento DATE NOT NULL,
    pes_sexo VARCHAR(9),
    pes_mae VARCHAR(200),
    pes_pai VARCHAR(200)
);

CREATE TABLE unidade (
    unid_id SERIAL PRIMARY KEY,
    unid_nome VARCHAR(200),
    unid_sigla VARCHAR(20)
);

CREATE TABLE foto_pessoa (
    fp_id SERIAL PRIMARY KEY,
    pes_id int,
    CONSTRAINT fk_pes FOREIGN KEY (pes_id) REFERENCES pessoa ON DELETE SET NULL,
    fp_data date,
    fp_bucket VARCHAR(50),
    fp_hash VARCHAR(50)
);

CREATE TABLE servidor_temporario (
    pes_id INT,
    CONSTRAINT fk_pes FOREIGN KEY (pes_id) REFERENCES pessoa,
    st_data_admissao date,
    st_data_demissao date
);

CREATE TABLE servidor_efetivo (
    pes_id INT,
    CONSTRAINT fk_pes FOREIGN KEY (pes_id) REFERENCES pessoa,
    se_matricula VARCHAR(20)
);

CREATE TABLE lotacao (
    lot_id SERIAL PRIMARY KEY,
    pes_id INT,
    CONSTRAINT fk_pes FOREIGN KEY (pes_id) REFERENCES pessoa,
    unid_id INT,
    CONSTRAINT fk_unid FOREIGN KEY (unid_id) REFERENCES unidade,
    lot_data_lotacao DATE,
    lot_data_remocao DATE,
    lot_portaria VARCHAR(100)
);

CREATE TABLE cidade (
    cid_id SERIAL PRIMARY KEY,
    cid_nome VARCHAR(200),
    cid_uf VARCHAR(2)
);

CREATE TABLE endereco (
    end_id SERIAL PRIMARY KEY,
    end_tipo_logradouro VARCHAR(50),
    end_logradouro VARCHAR(200),
    end_numero INT,
    end_bairro VARCHAR(100),
    cid_id INT,
    CONSTRAINT fk_cid FOREIGN KEY (cid_id) REFERENCES cidade,
);

CREATE TABLE pessoa_endereco (
    pes_id INT,
    end_id INT,
    CONSTRAINT fk_pes FOREIGN KEY (pes_id) REFERENCES alunos,
    CONSTRAINT fk_end FOREIGN KEY (end_id) REFERENCES disciplinas
);

CREATE TABLE unidade_endereco (
    unid_id INT,
    end_id INT,
    CONSTRAINT fk_unid FOREIGN KEY (unid_id) REFERENCES alunos,
    CONSTRAINT fk_end FOREIGN KEY (end_id) REFERENCES disciplinas
);