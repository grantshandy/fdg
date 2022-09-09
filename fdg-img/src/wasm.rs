use super::*;

use fdg_sim::json;
use plotters::style::text_anchor::{HPos, VPos, Pos};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct ImageSettings {
    pub iterations: usize,
    pub dt: f32,
    pub node_size: u32,
    pub node_color: Color,
    pub edge_size: u32,
    pub edge_color: Color,
    pub background_color: Color,
    pub show_text: bool,
    pub text_size: i32,
    pub text_color: Color,
}

impl Default for ImageSettings {
    fn default() -> Self {
        let us: Settings<Value, Value> = Settings::default();

        Self {
            iterations: us.iterations,
            dt: us.dt,
            node_size: us.node_size,
            node_color: Color {
                r: us.node_color.0,
                g: us.node_color.1,
                b: us.node_color.2,
                a: us.node_color.3,
            },
            edge_size: us.edge_size,
            edge_color: Color {
                r: us.edge_color.0,
                g: us.edge_color.1,
                b: us.edge_color.2,
                a: us.edge_color.3,
            },
            background_color: Color {
                r: us.background_color.0,
                g: us.background_color.1,
                b: us.background_color.2,
                a: us.background_color.3,
            },
            show_text: true,
            text_size: 20,
            text_color: Color {
                r: 0,
                g: 0,
                b: 0,
                a: 1.0,
            }
        }
    }
}

impl ImageSettings {
    pub fn to_main(&self) -> Settings<Value, Value> {
        use plotters::style::Color;

        Settings {
            sim_parameters: SimulationParameters::default(),
            iterations: self.iterations,
            dt: self.dt,
            node_size: self.node_size,
            node_color: RGBAColor(self.node_color.r, self.node_color.g, self.node_color.b, self.node_color.a),
            edge_size: self.edge_size,
            edge_color: RGBAColor(self.edge_color.r, self.edge_color.g, self.edge_color.b, self.edge_color.a),
            background_color: RGBAColor(self.background_color.r, self.background_color.g, self.background_color.b, self.background_color.a),
            print_progress: false,
            text_style: {
                if self.show_text {
                    Some(TextStyle {
                        font: ("sans-serif", self.text_size).into_font(),
                        color: RGBAColor(self.text_color.r, self.text_color.g, self.text_color.b, self.text_color.a).to_backend_color(),
                        pos: Pos {
                            h_pos: HPos::Left,
                            v_pos: VPos::Center,
                        },
                    })
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f64,
}

#[wasm_bindgen]
pub fn generate_svg(jsongraph: String, settings: JsValue) -> Result<String, JsError> {
    let settings: ImageSettings = if settings != JsValue::NULL || settings != JsValue::UNDEFINED {
        match settings.into_serde() {
            Ok(settings) => settings,
            Err(err) => return Err(JsError::new(&format!("settings has invalid format: {err}"))),
        }
    } else {
        ImageSettings::default()
    };

    let graph = match json::graph_from_json(&jsongraph) {
        Ok(graph) => graph,
        Err(err) => return Err(JsError::new(&err.to_string())),
    };

    match gen_image(graph, Some(settings.to_main())) {
        Ok(svg) => Ok(svg),
        Err(err) => Err(JsError::new(&err.to_string())),
    }
}