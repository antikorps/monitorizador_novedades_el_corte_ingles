use reqwest::Client;
use rusqlite::Connection;

use super::{
    cliente_http::cliente_http_autentificado,
    conexion::crear_bbdd_sqlite,
    configuracion::{deserializar_configuracion, Configuracion},
};

pub async fn configuracion_bbdd_cliente() -> (Configuracion, Connection, Client) {
    let configuracion = deserializar_configuracion();
    let conexion = crear_bbdd_sqlite();
    let cliente = cliente_http_autentificado().await;
    (configuracion, conexion, cliente)
}
