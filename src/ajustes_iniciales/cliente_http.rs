use core::panic;

use regex::Regex;
use reqwest::header::{HeaderMap, USER_AGENT};
use reqwest::{Client, ClientBuilder};
use serde::Serialize;

fn crear_cliente() -> Client {
    let mut cabeceras = HeaderMap::new();
    cabeceras.insert(
        USER_AGENT,
        "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:133.0) Gecko/20100101 Firefox/133.0"
            .parse()
            .unwrap(),
    );
    cabeceras.insert(
        "Accept",
        "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8"
            .parse()
            .unwrap(),
    );
    cabeceras.insert(
        "Accept-Language",
        "es-ES,es;q=0.8,en-US;q=0.5,en;q=0.3".parse().unwrap(),
    );
    cabeceras.insert("Connection", "keep-alive".parse().unwrap());
    cabeceras.insert("Upgrade-Insecure-Requests", "1".parse().unwrap());
    cabeceras.insert("Sec-Fetch-Dest", "document".parse().unwrap());
    cabeceras.insert("Sec-Fetch-Mode", "navigate".parse().unwrap());
    cabeceras.insert("Sec-Fetch-Site", "none".parse().unwrap());
    cabeceras.insert("Sec-Fetch-User", "?1".parse().unwrap());
    cabeceras.insert("Priority", "u=0, i".parse().unwrap());
    cabeceras.insert("Pragma", "no-cache".parse().unwrap());
    cabeceras.insert("Cache-Control", "no-cache".parse().unwrap());
    ClientBuilder::new()
        .cookie_store(true)
        .default_headers(cabeceras)
        .build()
        .expect("no se ha podido crear el cliente_http")
}

#[derive(Serialize)]
struct SolicitudVerificacion {
    #[serde(rename = "bm-verify")]
    bm_verify: String,
    pow: i64,
}

trait Autentificar {
    async fn autentificar(&self);
}
impl Autentificar for Client {
    async fn autentificar(&self) {
        let r = self
            .get("https://www.elcorteingles.es/electronica/reacondicionados/videojuegos/consolas/")
            .send()
            .await
            .expect("ERROR FATAL: ha fallado la petición base de autentificación");
        if !r.status().is_success() {
            panic!(
                "ERROR FATAL: la petición base de autentificación ha devuelto un status code no esperado {}",
                r.status()
            )
        }
        let texto = r
            .text()
            .await
            .expect("ERROR FATAL: imposible autentificar el cliente_http, no se ha podido leer el contenido de la respuesta base de autentificación");
        let texto = texto.replace("\n", "");
        let exp_reg_script_data =
            Regex::new(r#".*?var i = (.*?);.*?Number\("(.*?)".*?"(.*?)".*?"bm-verify": "(.*?)""#)
                .expect("error construyendo exp_reg_script_data");
        let coincidencias = exp_reg_script_data.captures(&texto).expect("exp_reg_script_data no ha devuelto ninguna coincidencia, no se puede identificar el cliente http");
        if coincidencias.len() != 5 {
            panic!("ERROR FATAL: imposible autentificar el cliente_http, exp_reg_script_data no ha capturado las 4 coincidencias")
        }

        let i = match &coincidencias[1].parse::<i64>() {
            Err(error) => {
                panic!(
                    "ERROR FATAL: imposible autentificar el cliente_http i {} no es un número: {}",
                    &coincidencias[1], error
                )
            }
            Ok(ok) => ok.to_owned(),
        };
        let numero_1 = match &coincidencias[2].parse::<i64>() {
            Err(error) => {
                panic!(
                    "ERROR FATAL: imposible autentificar el cliente_http, numero_1 {} no es un número: {}",
                    &coincidencias[2], error
                )
            }
            Ok(ok) => ok.to_owned(),
        };
        let numero_2 = match &coincidencias[3].parse::<i64>() {
            Err(error) => {
                panic!(
                    "ERROR FATAL: imposible autentificar el cliente_http, numero_2 {} no es un número: {}",
                    &coincidencias[3], error
                )
            }
            Ok(ok) => ok.to_owned(),
        };
        let pow = i + numero_1 + numero_2;
        let mut bm_verify = String::new();
        let _ = &coincidencias[4].clone_into(&mut bm_verify);

        let solicitud_verificacion = SolicitudVerificacion { bm_verify, pow };

        let endpoint_verify = "https://www.elcorteingles.es/_sec/verify?provider=interstitial";

        let _ = self
            .post(endpoint_verify)
            .json(&solicitud_verificacion)
            .send()
            .await
            .expect("ERROR FATAL: ha fallado la petición post de verificación del cliente");
    }
}

pub async fn cliente_http_autentificado() -> Client {
    let cliente = crear_cliente();
    cliente.autentificar().await;
    cliente
}
