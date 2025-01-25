use std::time::Duration;

use reqwest::Client;
use serde::Serialize;
use tokio::time::sleep;

use super::{crear::Monitorizador, modelos::ProductoECI};

fn crear_mensaje_plantilla(plantilla: &str, producto: &ProductoECI) -> String {
    let mut mensaje = String::from(plantilla);

    mensaje = mensaje.replace("$$$NOMBRE$$$", &producto.name);
    let mut precio_sustitucion = String::new();
    match producto.price.f_price {
        None => (),
        Some(s) => {
            let moneda = producto.price.currency.clone().unwrap_or("💵".to_string());
            precio_sustitucion = format!("{:.2} {}", s, moneda);
        }
    }
    mensaje = mensaje.replace("$$$PRECIO$$$", &precio_sustitucion);

    let mut descuento_sustitucion = String::new();
    match producto.price.discount_percent {
        None => (),
        Some(s) => {
            descuento_sustitucion = format!("{:.2}%", s);
        }
    };
    mensaje = mensaje.replace("$$$DESCUENTO_PORCENTAJE$$$", &descuento_sustitucion);

    let mut precio_previo_sustitucion = String::new();
    match producto.price.o_price {
        None => (),
        Some(s) => {
            precio_previo_sustitucion = format!("{:.2}%", s);
        }
    }
    mensaje = mensaje.replace("$$$PRECIO_PREVIO$$$", &precio_previo_sustitucion);

    let mut url_sustitucion = producto.url.clone().unwrap();
    if !producto.hierarchy.is_empty() {
        url_sustitucion = format!(
            "https://www.elcorteingles.es/{}/{}",
            producto.hierarchy[0], producto.code_a
        );
    }
    mensaje = mensaje.replace("$$$URL$$$", &url_sustitucion);
    mensaje
}

#[derive(Serialize)]
struct SolicitudMensajeTelegram {
    chat_id: String,
    parse_mode: String,
    text: String,
}

async fn enviar_mensaje(cliente_http: &Client, bot_token: &str, chat_id: &str, mensaje: String) {
    let solicitud_mensaje_telegram = SolicitudMensajeTelegram {
        chat_id: chat_id.to_owned(),
        parse_mode: String::from("html"),
        text: mensaje.to_owned(),
    };
    let bot_endpoint = format!("https://api.telegram.org/bot{}/sendMessage", bot_token);
    if let Err(error) = cliente_http
        .post(bot_endpoint)
        .json(&solicitud_mensaje_telegram)
        .send()
        .await
    {
        eprintln!(
            "ERROR: ha fallado el envío de la notificación con el siguiente mensaje {} {}",
            mensaje, error
        )
    }
}

impl Monitorizador {
    pub async fn avisar_producto_telegram(&mut self) {
        if !self.configuracion.notificar_telegram {
            return;
        }
        for producto in &self.productos {
            if !producto.notificar.unwrap() {
                continue;
            }
            let mensaje =
                crear_mensaje_plantilla(&self.configuracion.plantilla_notificacion, producto);
            enviar_mensaje(
                &self.cliente_http,
                &self.configuracion.telegram_bot_token,
                &self.configuracion.telegram_chat_id,
                mensaje,
            )
            .await;
            sleep(Duration::from_secs(2)).await;
        }
    }
}
