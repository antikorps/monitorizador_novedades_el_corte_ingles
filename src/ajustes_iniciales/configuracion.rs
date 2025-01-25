use std::{fs::File, path::PathBuf};

use serde::Deserialize;

use crate::utilidades;

#[derive(Deserialize)]
pub struct Configuracion {
    pub telegram_bot_token: String,
    pub telegram_chat_id: String,
    pub plantilla_notificacion: String,
    pub notificar_telegram: bool,
    pub urls: Vec<String>,
}

fn error_deserializacion_configuracion(error: &str) {
    panic!(
        r###"ERROR FATAL: {}
    
Recuerda que para que funcione el programa es necesario el archivo
configuracion.json en el mismo directorio que el ejecutable.
Este archivo debe tener la siguiente estructura:
{{
    "telegram_bot_token": "",
    "telegram_chat_id": "",
    "plantilla_notificacion": "",
    "notificar_telegram": true,
    "urls": []
}}
Si tienes dudas, consulta el README.md del repositorio
    
    "###,
        error
    );
}

pub fn deserializar_configuracion() -> Configuracion {
    let ruta_configuracion =
        utilidades::rutas::ruta_desde_ruta_raiz(&PathBuf::from("configuracion.json"));
    let archivo_configuracion = match File::open(ruta_configuracion) {
        Err(error) => {
            let mensaje_error = format!(
                "no se ha podido abrir el archivo configuracion.json {}",
                error
            );
            error_deserializacion_configuracion(&mensaje_error);
            unreachable!();
        }
        Ok(ok) => ok,
    };
    match serde_json::from_reader(archivo_configuracion) {
        Err(error) => {
            let mensaje_error = format!("error deserializando configuracion.json {}", error);
            error_deserializacion_configuracion(&mensaje_error);
            unreachable!();
        }
        Ok(ok) => ok,
    }
}
