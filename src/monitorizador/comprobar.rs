use rusqlite::params;

use super::crear::Monitorizador;

impl Monitorizador {
    pub fn revisar_notificacion_previa(&mut self) {
        let mut productos_por_notificar = 0;
        for producto in self.productos.iter_mut() {
            let insert_sql = "INSERT INTO registros (code_a) VALUES (?)";
            match self.conexion.execute(insert_sql, params![producto.code_a]) {
                Err(error) => {
                    let tipo_error = match error.sqlite_error_code() {
                        None => {
                            eprintln!("ERROR no esperado en el insert del registro {}", error);
                            continue;
                        }
                        Some(s) => s,

                    };
                    if tipo_error != rusqlite::ErrorCode::ConstraintViolation {
                        eprintln!("ERROR no esperado en el insert del registro {}", error);
                    }

                    producto.notificar = Some(false)
                }
                Ok(_) => {
                    producto.notificar = Some(true);
                    productos_por_notificar += 1;
                }
            }
        }
        println!(
            "INFO: se intentarán notificar {} nuevos productos",
            productos_por_notificar
        );
    }
}
