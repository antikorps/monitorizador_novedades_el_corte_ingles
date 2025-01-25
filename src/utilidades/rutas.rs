use std::{env::current_exe, path::PathBuf};

pub fn ruta_desde_ruta_raiz(ruta: &PathBuf) -> PathBuf {
    let ruta_ejecutable = current_exe().expect("no se ha podido recuperar la ruta del ejecutable");
    let ruta_raiz = ruta_ejecutable
        .parent()
        .expect("no se ha podido recupera la ruta raíz");
    ruta_raiz.join(ruta).to_owned()
}
