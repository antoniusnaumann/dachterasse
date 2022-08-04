CREATE TABLE lectures {
    title VARCHAR(255) NOT NULL,
    url TEXT NOT NULL, 
    degree VARCHAR(16) NOT NULL,
    PRIMARY KEY(url, degree)
}
