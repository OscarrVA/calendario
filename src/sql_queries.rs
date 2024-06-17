pub const BD_NOMBRE: &str = "lanzamientos_DB.db";

pub const CREAR_TABLA_FECHAS_QUERY: &str = 
        "CREATE TABLE IF NOT EXISTS fechas (
            id INTEGER PRIMARY KEY,
            id_div INTEGER NOT NULL,
            titulo TEXT NOT NULL,
            fecha_dia INTEGER NOT NULL,
            categoria TEXT NOT NULL,
            steam BOOLEAN NOT NULL,
            epic BOOLEAN NOT NULL,
            gog BOOLEAN NOT NULL,
            gamepass BOOLEAN NOT NULL,
            ubisoftplus BOOLEAN NOT NULL,
            eaplay BOOLEAN NOT NULL,
            enlace TEXT NOT NULL,
            descripcion TEXT NOT NULL
        )";

pub const INSERTAR_TABLA_FECHAS_QUERY: &str =
    "INSERT INTO fechas (id_div, titulo, fecha_dia, categoria, steam,
        epic, gog, gamepass, ubisoftplus, eaplay, enlace, descripcion)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)";

pub const CONSULTAR_FECHAS_BY_ID_DIV_QUERY: &str =
    "SELECT id, titulo, fecha_dia, categoria, steam, epic, gog, gamepass, 
        ubisoftplus, eaplay, enlace, descripcion 
        FROM fechas 
        WHERE id_div = ?1
        ORDER BY fecha_dia ASC";

pub const CONSULTAR_FECHA_BY_ID_QUERY: &str = "SELECT * FROM fechas WHERE id = ?1";

pub const ACTUALIZAR_FECHA_EN_BD_QUERY: &str = 
        "UPDATE fechas
        SET
            id_div = ?1,
            titulo = ?2,
            fecha_dia = ?3,
            categoria = ?4,
            steam = ?5,
            epic = ?6,
            gog = ?7,
            gamepass = ?8,
            ubisoftplus = ?9,
            eaplay = ?10,
            enlace = ?11,
            descripcion = ?12
        WHERE id = ?13";

pub const ELIMINAR_FECHA_POR_ID_EN_BD_QUERY: &str = "DELETE FROM fechas WHERE id = ?1";
