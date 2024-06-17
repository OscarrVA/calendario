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
    pub descripcion: String,
    
}

// Define el tipo para facilitar la referencia en la firma de la función.
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
    String // descripcion
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
    pub descripcion: String
}
