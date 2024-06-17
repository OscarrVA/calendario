#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use rocket::{form::Form, response::Redirect};
use rocket_dyn_templates::{context, Template};

mod models;
use models::Fecha;
use models::FechaJson;

mod connection;
use connection::establecer_conexion;
use connection::crear_tabla;
use connection::insertar_fecha_en_bd;
use connection::get_fechas_por_id_div;
use connection::get_fecha_por_id;
use connection::actualizar_fecha_en_db;
use connection::eliminar_fecha_por_id_en_bd;

mod sql_queries;

#[get("/")]
fn inicio() -> Template {
    Template::render("inicio", context! { titulo: "Mi Calendario" })
}

#[get("/agregarFecha")]
fn agregar_fecha() -> Template {
    Template::render("agregar_fecha", context! { titulo: "Agregar Juego"})
}

#[post("/guardarFecha", data = "<fecha_form>")]
fn guardar_fecha(fecha_form: Form<Fecha>) -> Redirect {
    let fecha_obj = fecha_form.into_inner();

    let conn = establecer_conexion().expect("Error al hacer la conexion.");
    let save_success = match insertar_fecha_en_bd(&conn, &fecha_obj) {
        Ok(_) => {
            println!("Fecha guardada correctamente");
            true},
        Err(e) => {
            println!("No se pudo guardar la fecha: {:?}", e);
            false
        }
    };

    if save_success {
        Redirect::to(uri!(inicio))
    } else {
        Redirect::to(uri!(error_al_guardar))
    }
}

#[get("/fechasPorMes/<id_div>")]
fn fechas_por_mes(id_div: i32) -> Json<Vec<FechaJson>> {
    let conn = establecer_conexion().expect("Error al hacer la conexion.");
    let fechas = match get_fechas_por_id_div(&conn, id_div) {
        Ok(fechas) => fechas.iter().map(|fecha_vector| FechaJson {
            id: fecha_vector.0,
            titulo: fecha_vector.1.clone(),
            fecha_dia: fecha_vector.2,
            categoria: fecha_vector.3.clone(),
            steam: fecha_vector.4,
            epic: fecha_vector.5,
            gog: fecha_vector.6,
            gamepass: fecha_vector.7,
            ubisoftplus: fecha_vector.8,
            eaplay: fecha_vector.9,
            enlace: fecha_vector.10.clone(),
            descripcion: fecha_vector.11.clone()
        }).collect(),
        Err(_) => Vec::new(),
    };

    Json(fechas)
}

#[get("/editarFecha/<id>")]
fn editar_fecha(id: i32) -> Template {
    let conn = establecer_conexion().expect("Error al hacer la conexion.");
    let fecha = get_fecha_por_id(&conn,id).unwrap();
    Template::render(
        "editar_fecha",
        context! { 
            titulo : "Editar Juego", 
            id_fecha: id,
            fecha: fecha },
    )
}

#[post("/actualizarFecha/<id>", data="<fecha_form>")]
fn actualizar_fecha(fecha_form: Form<Fecha>, id: i32) -> Redirect{

    let fecha_obj = fecha_form.into_inner();

    let conn = establecer_conexion().expect("Error al hacer la conexion.");
    let update_success = match actualizar_fecha_en_db(&conn, &fecha_obj,id) {
        Ok(_) => {
            println!("Fecha actualizada correctamente");
            true},
        Err(e) => {
            println!("No se pudo actualizar la fecha: {:?}", e);
            false
        }
    };

    if update_success {
        Redirect::to(uri!(inicio))
    } else {
        Redirect::to(uri!(error_al_guardar))
    }
}

#[get("/eliminarFecha/<id>")]
fn eliminar_fecha(id: i32) -> Redirect{
    let conn = establecer_conexion().expect("No se pudo conectar con la base de datos");
    match eliminar_fecha_por_id_en_bd(&conn, id) {
        Ok(_) => println!("Fecha eliminada correctamente"),
        Err(e) => println!("No se pudo eliminar la fecha: {:?}", e)
    }
    Redirect::to(uri!(inicio))
}

#[get("/errorAlGuardar")]
fn error_al_guardar() -> &'static str {
    "Error al guardar la fecha"
}

#[launch]
fn rocket() -> _ {

    let conn = establecer_conexion().expect("No se pudo conectar con la base de datos");
    crear_tabla(&conn).expect("No se pudo crear la tabla 'fechas'");

    rocket::build()
        .mount(
            "/",
            routes![inicio, agregar_fecha, guardar_fecha, editar_fecha,error_al_guardar,fechas_por_mes,actualizar_fecha, eliminar_fecha],
        )
        .attach(Template::fairing())
}

//TODO
//Terminar de dar diseño a las paginas.