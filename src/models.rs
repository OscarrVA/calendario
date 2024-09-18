use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize,FromForm)]
pub struct Fecha{
    pub titulo: String,
    pub fecha: String,
    pub categoria: String,
    pub steam: bool,
    pub epic: bool,
    pub gog: bool,
    pub gamepass: bool,
    pub ubisoftplus: bool,
    pub eaplay: bool,
    pub enlace: String,
    pub descripcion: String
}

pub type FechaBD = (
    i32, // id
    String, // titulo
    i32, // fecha_dia
    String, // categoria
    bool, // steam
    bool, // epic
    bool, // gog
    bool, // gamepass
    bool, // ubisoftplus
    bool, // eaplay
    String, // enlace
    String, // descripcion
    String //imagen_url
);

#[derive(Serialize)]
pub struct FechaJson {
    pub id: i32,
    pub titulo: String,
    pub fecha_dia: i32,
    pub categoria: String,
    pub steam: bool,
    pub epic: bool,
    pub gog: bool,
    pub gamepass: bool,
    pub ubisoftplus: bool,
    pub eaplay: bool,
    pub enlace: String,
    pub descripcion: String,
    pub imagen_url: String,
}
