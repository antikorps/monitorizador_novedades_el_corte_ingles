use reqwest::Client;
use rusqlite::Connection;

use crate::ajustes_iniciales::configuracion::Configuracion;

use super::modelos::ProductosECI;

pub struct Monitorizador {
    pub configuracion: Configuracion,
    pub conexion: Connection,
    pub cliente_http: Client,
    pub productos: ProductosECI,
}

pub fn nuevo_monitorizador(
    configuracion: Configuracion,
    conexion: Connection,
    cliente_http: Client,
) -> Monitorizador {
    Monitorizador {
        configuracion,
        conexion,
        cliente_http,
        productos: Vec::new(),
    }
}
