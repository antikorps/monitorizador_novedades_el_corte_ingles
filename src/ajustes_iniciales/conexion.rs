use std::path::PathBuf;

use rusqlite::Connection;

use crate::utilidades;

pub fn crear_bbdd_sqlite() -> Connection {
    let ruta_sqlite =
        utilidades::rutas::ruta_desde_ruta_raiz(&PathBuf::from("bbdd_mon_eci.sqlite"));
    let conexion = Connection::open(ruta_sqlite)
        .expect("ERROR FATAL: no se ha podido conectar con el archivo bbdd_mon_eci.sqlite");
    let create_sql = r#"
CREATE TABLE IF NOT EXISTS "registros" (
	"id"	INTEGER NOT NULL,
	"code_a"	TEXT NOT NULL UNIQUE,
	PRIMARY KEY("id" AUTOINCREMENT)
);"#;
    conexion
        .execute(create_sql, [])
        .expect("error en create_sql");
    conexion
}
