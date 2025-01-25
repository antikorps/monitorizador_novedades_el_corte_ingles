# Monitorizador de novedades para El Corte Inglés
Monitoriza diferentes URLs de El Corte Inglés y recibe un mensaje por Telegram cada vez que se incorpore un producto nuevo.
## Uso y ejecución
Descarga desde la sección **Releases** del repositorio la versión que corresponda a tu arquitectura y sistema operativo. \
Junto al ejecutable (mismo directorio) debe existir el archivo **configuracion.json** con el siguiente esquema:
```json
{
    "telegram_bot_token": "XXX",
    "telegram_chat_id": "XXX",
    "plantilla_notificacion": "📢❗🚨 $$$NOMBRE$$$\n🛒🏷️ PRECIO: $$$PRECIO$$$\n PRECIO PREVIO: $$$PRECIO_PREVIO$$$, ahorra un $$$DESCUENTO_PORCENTAJE$$$\n$$$URL$$$",
    "notificar_telegram": true,
    "urls": ["XXX", "XXX", "XXX"]
}
```
- **telegram_bot_token**: es el token de tu bot que te proporciona @BotFather
- **telegram_chat_id**: el identificador del grupo, usuario, canal, al que el bot enviará la notificación
- **plantilla_notificacion**: plantilla para personalizar el mensaje de notificación. El texto de composición es libre y se puede utilizar distintas variables:
   - **$$$NOMBRE$$$**: sustituye el texto por el nombre del producto.
   - **$$$PRECIO$$$**: sustituye el texto por el precio de venta del producto.
   - **$$$PRECIO_PREVIO$$$**: sustituye el texto por el precio previo del producto antes del descuento (en caso de que aplique)
   - **$$$DESCUENTO_PORCENTAJE$$$**: sustituye el texto por el porcentaje de descuento que se ahorra con el nuevo precio.
   - **$$$URL$$$**: sustituye el texto por la URL del producto.
- **notificar_telegram**: indica si en la ejecución debe notificarse por Telegram de la existencia de los productos encontrados. Se aconseja que la primera ejecución esté en **false** para no recibir un aviso con todos los productos.
- **urls**: colección de URLs que deben monitorizarse.

Si a la hora de crear el archivo configuracion.json tienes alguna duda, recuerda que puedes verificar su formato en páginas como [JSON Lint](https://jsonlint.com/) o con la ayuda de IA.