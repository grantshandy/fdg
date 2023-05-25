use fdg_sim::Node;
use js_sys::Number;
use serde_json::Value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone)]
pub struct ForceGraphNode {
    name: String,
    label: JsValue,
    location: Vec<f32>,
    metadata: JsValue,
}

impl ForceGraphNode {
    pub fn new(node: &Node<JsValue>) -> Self {
        let name = node.name.to_owned();
        let location = vec![node.location.x, node.location.y, node.location.z];

        let data: Value = match serde_wasm_bindgen::from_value(node.data.to_owned()) {
            Ok(data) => data,
            Err(_) => Value::Null,
        };

        let label = match data.get("label") {
            Some(label) => match serde_wasm_bindgen::to_value(label) {
                Ok(label) => label,
                Err(_) => JsValue::NULL,
            },
            None => JsValue::NULL,
        };

        let metadata = match data.get("metadata") {
            Some(metadata) => match serde_wasm_bindgen::to_value(metadata) {
                Ok(metadata) => metadata,
                Err(_) => JsValue::NULL,
            },
            None => JsValue::NULL,
        };

        Self {
            name,
            label,
            location,
            metadata,
        }
    }
}

#[wasm_bindgen]
impl ForceGraphNode {
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.to_owned()
    }

    #[wasm_bindgen(getter)]
    pub fn label(&self) -> JsValue {
        self.label.to_owned()
    }

    #[wasm_bindgen(getter)]
    pub fn location(&self) -> Vec<Number> {
        self.location.iter().map(|x| Number::from(*x)).collect()
    }

    #[wasm_bindgen(getter)]
    pub fn metadata(&self) -> JsValue {
        self.metadata.to_owned()
    }
}
