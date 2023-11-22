CREATE TABLE computers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(100) NOT NULL,
    description VARCHAR(100) NOT NULL,
    cpu_id INTEGER NOT NULL,
    gpu_id INTEGER NULL,
    mobo_id INTEGER NOT NULL,
    psu_id INTEGER NOT NULL,
    cooler_id INTEGER NOT NULL,
    case_id INTEGER NOT NULL,
    FOREIGN KEY(cpu_id) REFERENCES cpus(id),
    FOREIGN KEY(gpu_id) REFERENCES gpus(id),
    FOREIGN KEY(mobo_id) REFERENCES mobos(id),
    FOREIGN KEY(psu_id) REFERENCES psus(id),
    FOREIGN KEY(cooler_id) REFERENCES coolers(id),
    FOREIGN KEY(case_id) REFERENCES cases(id)
);
