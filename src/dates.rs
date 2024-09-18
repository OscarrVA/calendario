use std::collections::{BTreeMap, HashMap};
use plotters::prelude::*;

/// Metodo para generar un grafico usando el crate 'plotters'.
pub fn grafico(data: &HashMap<i32, i32>, output_path: &str, year: i32) -> Result<(), Box<dyn std::error::Error>>{
    let root = BitMapBackend::new(output_path, (890,730)).into_drawing_area();
    root.fill(&BLACK)?;

    let max_count = *data.values().max().unwrap_or(&0);

    let caption: &str;

    //Colores
    let mut color_puntos = RGBColor(169, 29, 58);
    let mut color_lineas = RGBColor(199, 54, 89);
    
    if year == 2024 {
        caption = "Número de Fechas por Mes en 2024";
    }
    else {
        caption = "Número de Fechas por Mes en 2025";
        color_puntos = RGBColor(22, 160, 133);
        color_lineas = RGBColor(26, 188, 156);
    }

    let mut grafica = ChartBuilder::on(&root)
        .caption(caption, ("Courier New", 40).into_font().color(&WHITE))
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0u32..12u32, 0i32..max_count + 1)?;

    grafica.configure_mesh()
        .x_labels(12)
        .x_label_formatter(&|x| format!("{}", x+1))
        .x_desc("Meses")
        .y_desc("Lanzamientos")
        .disable_mesh()
        .axis_style(&WHITE)
        .axis_desc_style(("sans-serif",20).into_font().color(&WHITE))
        .label_style(("sans-serif",20).into_font().color(&WHITE))
        .draw()?;


    let mut points = BTreeMap::new();
    let i: i32;
    if year == 2025 {i = 13} else {i=1}
    for month in 0..12 {
        points.insert(month, *data.get(&(month + i)).unwrap_or(&0));
    }

    //println!("Points: {:?}", points);


    grafica.draw_series(
        data.iter().map(|(id_div, count)|{
            let mut pos = *id_div - 1;
            if year == 2025 {pos = pos - 12}
            Circle::new((pos as u32, *count), 5, color_puntos.filled())
        })

    )?;

    // Dibujar la línea que conecta todos los puntos
    grafica.draw_series(LineSeries::new(points.iter().map(|(&month, &count)| (month as u32, count)),color_lineas))?;

    root.present()?;
    Ok(())
}