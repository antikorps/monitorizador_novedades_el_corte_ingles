use ajustes_iniciales::preparar_ejecucion::configuracion_bbdd_cliente;
use monitorizador::crear::nuevo_monitorizador;

mod ajustes_iniciales;
mod monitorizador;
mod utilidades;

#[tokio::main]
async fn main() {
    let (configuracion, conexion, cliente_http) = configuracion_bbdd_cliente().await;
    let mut monitorizador = nuevo_monitorizador(configuracion, conexion, cliente_http);
    monitorizador.localizar_productos_urls().await;
    monitorizador.revisar_notificacion_previa();
    monitorizador.avisar_producto_telegram().await;
}
