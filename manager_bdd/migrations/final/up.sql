CREATE TABLE level(
    id SERIAL PRIMARY KEY, 
    value VARCHAR(25) NOT NULL
);  
CREATE TABLE users(
    id SERIAL PRIMARY KEY, 
    id_admin INTEGER, 
    id_level INTEGER NOT NULL, 
    last_name VARCHAR(25) NOT NULL, 
    first_name VARCHAR(25) NOT NULL, 
    email VARCHAR(100) UNIQUE NOT NULL, 
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP, 
    password VARCHAR(150), 
    secret VARCHAR(150), 
    FOREIGN KEY(id_level) REFERENCES level(id), 
    FOREIGN KEY (id_admin) REFERENCES users(id) 
);
CREATE TABLE vm(
    id SERIAL PRIMARY KEY,
    id_creator INTEGER NOT NULL,
    title VARCHAR(75) UNIQUE NOT NULL,
    timestamp TIMESTAMP,
    link VARCHAR(500) NOT NULL,
    ip VARCHAR(200), 
    FOREIGN KEY (id_creator) REFERENCES users(id)
);
CREATE TABLE link_user_vm(
    id_vm INTEGER NOT NULL,
    id_user INTEGER NOT NULL,
    FOREIGN KEY (id_vm) REFERENCES vm(id), 
    FOREIGN KEY(id_user) REFERENCES users(id) ON DELETE CASCADE,
    PRIMARY KEY (id_vm, id_user)
);
CREATE TABLE session(
    uuid VARCHAR(200) NOT NULL DEFAULT md5(random()::text), 
    id_user INTEGER UNIQUE NOT NULL, 
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP, 
    path VARCHAR(200) NOT NULL DEFAULT md5(random()::text), 
    FOREIGN KEY(id_user) REFERENCES users(id) ON DELETE CASCADE,
    PRIMARY KEY (uuid, path)
);
CREATE TABLE temporaire(
    path VARCHAR(200) PRIMARY KEY DEFAULT MD5(random()::text), 
    id_user INTEGER UNIQUE NOT NULL, 
    timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP, 
    code VARCHAR(12) NOT NULL DEFAULT concat(left(md5(random()::text),3),'-',left(md5(random()::text),3),'-',left(md5(random()::text),3)),
    FOREIGN KEY (id_user) REFERENCES users(id) ON DELETE CASCADE
);
CREATE TABLE info(
    id SERIAL PRIMARY KEY, 
    id_user INTEGER NOT NULL,
    title VARCHAR(200) NOT NULL, 
    value VARCHAR(200) NOT NULL, 
    start TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    stop TIMESTAMP,
    FOREIGN KEY (id_user) REFERENCES users(id) ON DELETE CASCADE
);
CREATE TABLE vemmion(
    uuid VARCHAR(200) PRIMARY KEY DEFAULT MD5(random()::text),
    id_user INTEGER NOT NULL,
    id_vm INTEGER NOT NULL,
    timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    runing BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (id_user) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (id_vm) REFERENCES vm(id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX idx_vemmion ON vemmion(id_user, id_vm);


--INSERTION DE VALEURS DANS LA TABLE
INSERT INTO level (id,value) VALUES (0,'USER');
INSERT INTO level (id,value) VALUES (1,'ADMIN');