use std::collections::HashMap;
use rusqlite::{Connection,Result,params};

use crate::models;
use models::Fecha;
use models::FechaBD;

use crate::sql_queries;
use sql_queries::BD_NOMBRE;
use sql_queries::CREAR_TABLA_FECHAS_QUERY;
use sql_queries::INSERTAR_TABLA_FECHAS_QUERY;
use sql_queries::CONSULTAR_FECHAS_BY_ID_DIV_QUERY;
use sql_queries::CONSULTAR_FECHA_BY_ID_QUERY;
use sql_queries::ACTUALIZAR_FECHA_EN_BD_QUERY;
use sql_queries::ELIMINAR_FECHA_POR_ID_EN_BD_QUERY;
use sql_queries::CONTAR_FECHAS_POR_MESES_2024;
use sql_queries::CONTAR_FECHAS_POR_MESES_2025;

/// Establece una conexión a la base de datos.
pub fn establecer_conexion() -> Result<Connection> {
    Connection::open(BD_NOMBRE)
}

/// Crear la tabla 'fechas' si no existe.
pub fn crear_tabla(conn: &Connection) -> Result<()> {
    conn.execute(CREAR_TABLA_FECHAS_QUERY, params![])?;

        Ok(())
}

/// Inserta en la tabla 'fechas' una lanzamiento.
pub fn insertar_fecha_en_bd(conn: &Connection, fecha: &Fecha, imagen_url: String) -> Result<usize> {

    let mut fecha_id: i32 = 0;
    let mut fecha_dia: i32 = 0;

    match separar_fecha(fecha) {
        Ok(fecha_array) => {
            //println!("ID del mes: {}, Día: {}", fecha_array[0], fecha_array[1]);
            fecha_id = fecha_array[0];
            fecha_dia = fecha_array[1];
        },
        Err(e) => println!("Error para generar fechas: {}", e),
    }

    conn.execute(INSERTAR_TABLA_FECHAS_QUERY, params![
        fecha_id,    
        fecha.titulo,
        fecha_dia,
        fecha.categoria,
        fecha.steam,
        fecha.epic,
        fecha.gog,
        fecha.gamepass,
        fecha.ubisoftplus,
        fecha.eaplay,
        fecha.enlace,
        fecha.descripcion,
        imagen_url
    ])
}


/// Saca el id_div y el dia de la fecha que viene del objeto Fecha.
pub fn separar_fecha(fecha: &Fecha) -> Result<[i32; 2], &'static str> {
    let id: i32;

    let partes: Vec<&str> = fecha.fecha.split('-').collect();
    if partes.len() != 3 {
        return Err("Formato de fecha incorrecto");
    }

    let str_numero_invalido: &str = "El año no es un número válido";

    let año = partes[0].parse::<i32>().map_err(|_| str_numero_invalido)?;
    let mes = partes[1].parse::<i32>().map_err(|_| str_numero_invalido)?;
    let dia = partes[2].parse::<i32>().map_err(|_| str_numero_invalido)?;

    // Asignar 'id' basado en el año y el mes
    id = match año {
        2024 => match mes {
            01 => 1,
            02 => 2,
            03 => 3,
            04 => 4,
            05 => 5,
            06 => 6,
            07 => 7,
            08 => 8,
            09 => 9,
            10 => 10,
            11 => 11,
            12 => 12, 
            _ => return Err("Mes no reconocido para el año 2024"),
        },
        2025 => match mes {
            01 => 13,
            02 => 14,
            03 => 15,
            04 => 16,
            05 => 17,
            06 => 18,
            07 => 19,
            08 => 20,
            09 => 21,
            10 => 22,
            11 => 23,
            12 => 24,
            _ => return Err("Mes no reconocido para el año 2025"),
        },
        _ => return Err("Año no reconocido"),
    };

    Ok([id, dia])
}

///Consultar en la BD, las fechas por el id del div seleccionado.
pub fn get_fechas_por_id_div(conn: &Connection, id_div: i32) -> Result<Vec<FechaBD>> {
    let mut stmt = conn.prepare(CONSULTAR_FECHAS_BY_ID_DIV_QUERY)?;
    let fecha_iter = stmt.query_map(params![id_div], |row| {
        Ok((
            row.get(0)?, // id
            row.get(1)?, // titulo
            row.get(2)?, // fecha_dia
            row.get(3)?, // categoria
            row.get(4)?, // steam
            row.get(5)?, // epic
            row.get(6)?, // gog
            row.get(7)?, // gamepass
            row.get(8)?, // ubisoftplus
            row.get(9)?, // eaplay
            row.get(10)?, // enlace
            row.get(11)?, // descripcion
            row.get(12)?, //imagen_url
        ))
    })?;

    let mut fechas = Vec::new();
    for fecha in fecha_iter {
        fechas.push(fecha?);
    }

    //println!("Fechas recibidas de BD: {:?}", fechas);

    Ok(fechas)
}

///Consultar en la BD, la fecha por el id.
pub fn get_fecha_por_id(conn: &Connection, id: i32) -> Result<Fecha>{
    conn.query_row(CONSULTAR_FECHA_BY_ID_QUERY, params![id],
        |row| {

            let id_div: i32 = row.get(1)?;  // Suponiendo que es un INTEGER
            let fecha_dia: i32 = row.get(3)?;  // Suponiendo que es un INTEGER

            // Convertir los INTEGER a string para pasar a unir_fecha
            let id_div_str = id_div.to_string();
            let fecha_dia_str = fecha_dia.to_string();

            // Unir las partes de la fecha
            let fecha_unida = unir_fecha(&id_div_str, &fecha_dia_str);
            let fecha_completa = fecha_unida.unwrap_or("Fecha no disponible".to_string());

            Ok(Fecha {
                titulo: row.get(2)?,
                fecha: fecha_completa,
                categoria: row.get(4)?,
                steam: row.get(5)?,
                epic: row.get(6)?,
                gog: row.get(7)?,
                gamepass: row.get(8)?,
                ubisoftplus: row.get(9)?,
                eaplay: row.get(10)?,
                enlace: row.get(11)?,
                descripcion: row.get(12)?
            })
        }
    )
}

///Toma el i_div y el fecha_dia para volver a formar una fecha completa de tipo string.
fn unir_fecha(id_div: &str, fecha_dia: &str) -> Result<String, ()> {
    let parte1: &str = match id_div {
        "1" => "2024-01",
        "2" => "2024-02",
        "3" => "2024-03",
        "4" => "2024-04",
        "5" => "2024-05",
        "6" => "2024-06",
        "7" => "2024-07",
        "8" => "2024-08",
        "9" => "2024-09",
        "10" => "2024-10",
        "11" => "2024-11",
        "12" => "2024-12",
        "13" => "2025-01",
        "14" => "2025-02",
        "15" => "2025-03",
        "16" => "2025-04",
        "17" => "2025-05",
        "18" => "2025-06",
        "19" => "2025-07",
        "20" => "2025-08",
        "21" => "2025-09",
        "22" => "2025-10",
        "23" => "2025-11",
        "24" => "2025-12",
        _ => return Err(()),  // Devuelve un error si el id_div no es válido
    };

    Ok(format!("{}-{}", parte1, fecha_dia))  // Devuelve la fecha formateada correctamente
}

///Actualiza en la BD la fecha por el id.
pub fn actualizar_fecha_en_db(conn: &Connection, fecha: &Fecha, imagen_url: String,id: i32) -> Result<()>{

    let mut fecha_id: i32 = 0;
    let mut fecha_dia: i32 = 0;

    match separar_fecha(fecha) {
        Ok(fecha_array) => {
            //println!("ID del mes: {}, Día: {}", fecha_array[0], fecha_array[1]);
            fecha_id = fecha_array[0];
            fecha_dia = fecha_array[1];
        },
        Err(e) => println!("Error para generar fechas: {}", e),
    }

    conn.execute(ACTUALIZAR_FECHA_EN_BD_QUERY, params![
            fecha_id,
            fecha.titulo,
            fecha_dia,
            fecha.categoria,
            fecha.steam,
            fecha.epic,
            fecha.gog,
            fecha.gamepass,
            fecha.ubisoftplus,
            fecha.eaplay,
            fecha.enlace,
            fecha.descripcion,
            imagen_url,
            id  // El ID es usado para identificar qué fila actualizar
        ],
    )?;

    Ok(())
}

///Elimina en la BD la fecha por el id.
pub fn eliminar_fecha_por_id_en_bd(conn: &Connection, id: i32) -> Result<()>{
    
    conn.execute(ELIMINAR_FECHA_POR_ID_EN_BD_QUERY,params![id],)?;

    Ok(())
}

/// Regrese un Hashmap con la cantidad de fechas de cada mes.
pub fn contar_todas_fechas_por_mes(conn: &Connection, year: i32)-> Result<HashMap<i32, i32>, rusqlite::Error> {
    
    let year_querie: &str;

    if year == 2024 {
        year_querie = CONTAR_FECHAS_POR_MESES_2024;
    }
    else {
        year_querie = CONTAR_FECHAS_POR_MESES_2025;
    }

    let mut stmt = conn.prepare(year_querie)?;
    let rows = stmt.query_map([], |row| {
        Ok((row.get(0)?, row.get(1)?))
    })?;

    let mut counts = HashMap::new();
    for count_result in rows {
        let (id_div, count): (i32, i32) = count_result?;
        counts.insert(id_div, count);
    }

    /* for (id_div, count) in &counts {
        println!("id mes: {}, cantidad: {}", id_div, count);
    } */

    Ok(counts)
}