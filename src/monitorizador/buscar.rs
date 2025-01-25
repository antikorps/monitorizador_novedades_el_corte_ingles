use std::env;

use futures::future::join_all;
use regex::Regex;
use reqwest::Client;

use crate::monitorizador::modelos::{DataLayerResponse, ProductosECI};

use super::crear::Monitorizador;

async fn visitar_url_extraer_productos(
    cliente_http: &Client,
    url: &str,
    exp_reg_data_layer: &Regex,
) -> Result<ProductosECI, String> {
    println!("INFO: Buscando productos en {}...", url);
    let r = match cliente_http.get(url).send().await {
        Err(error) => {
            let mensaje_error = format!("ha fallado la petición a {} {}", url, error);
            return Err(mensaje_error);
        }
        Ok(ok) => ok,
    };
    if !r.status().is_success() {
        let mensaje_error = format!(
            "la petición a {} ha devuelto un status code no deseado {}",
            url,
            r.status()
        );
        return Err(mensaje_error);
    }
    let texto = match r.text().await {
        Err(error) => {
            let mensaje_error = format!("la petición a {} no ha podido ser leída {}", url, error);
            return Err(mensaje_error);
        }
        Ok(ok) => ok,
    };

    let texto = texto.replace("\n", "");

    let coincidencias = match exp_reg_data_layer.captures(&texto) {
        None => {
            let mensaje_error = format!("la expresión regular no ha encontrado la variable dataLayer con la información de los productos en {}", url);
            return Err(mensaje_error);
        }
        Some(s) => s,
    };
    if coincidencias.len() != 2 {
        let mensaje_error = format!(
            "la expresión regular para dataLayer no ha encontrado 2 coincidencias en {}",
            url
        );
        return Err(mensaje_error);
    }


    let data_layer: DataLayerResponse = match serde_json::from_str(&coincidencias[1]) {
        Err(error) => {
            let mensaje_error = format!(
                "ha fallado la deserialización de dataLayer en {} {}",
                url, error
            );
            return Err(mensaje_error);
        }
        Ok(ok) => ok,
    };

    let mut productos = Vec::new();
    for v in data_layer {
        for mut p in v.products {
            p.url = Some(url.to_owned());
            productos.push(p);
        }       
    }
    Ok(productos)
}

/**
Modificar número de peticiones simultáneas (3) mediante variable entorno: MONITOR_ECI_SIMULTANEIDAD
 */
fn establecer_simultaneidad() -> usize {
    let mut simultaneidad = 3;
    match env::var("MONITOR_ECI_SIMULTANEIDAD") {
        Err(_) => (),
        Ok(ok) => {
            match ok.parse::<usize>() {
                Err(_) => (),
                Ok(ok) => {
                    simultaneidad = ok;
                }
            };
        }
    }
    simultaneidad
}

impl Monitorizador {
    pub async fn localizar_productos_urls(&mut self) {
        let exp_reg_data_layer = Regex::new(r#".*?dataLayer.*?(\[.*?);</script>"#)
            .expect("exp_reg_data_layer no ha podido crearse");
        let lotes_urls = self.configuracion.urls.chunks(establecer_simultaneidad());
        for lote in lotes_urls {
            let mut futuros = Vec::new();
            for url in lote {
                futuros.push(visitar_url_extraer_productos(
                    &self.cliente_http,
                    url,
                    &exp_reg_data_layer,
                ))
            }
            let resultados = join_all(futuros).await;
            for r in resultados {
                match r {
                    Err(error) => {
                        eprintln!("ERROR: {}", error)
                    }
                    Ok(ok) => {
                        for p in ok {
                            self.productos.push(p);
                        }
                    }
                }
            }
        }
        println!(
            "INFO: el análisis de las URL ha devuelto {} productos",
            self.productos.len()
        )
    }
}
