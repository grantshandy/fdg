use fdg_sim::Node;
use js_sys::Number;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ForceGraphNode {
    name: String,
    location: Vec<f32>,
    data: JsValue,
}

impl ForceGraphNode {
    pub fn new(node: &Node<JsValue>) -> Self {
        let name = node.name.to_owned();
        let location = vec![node.location.x, node.location.y, node.location.z];
        let data = node.data.to_owned();

        Self {
            name,
            location,
            data,
        }
    }
}

#[wasm_bindgen]
impl ForceGraphNode {
    #[wasm_bindgen(method, getter)]
    pub fn name(&self) -> String {
        self.name.to_owned()
    }

    #[wasm_bindgen(method, getter)]
    pub fn location(&self) -> Vec<Number> {
        self.location.iter().map(|x| Number::from(*x)).collect()
    }

    #[wasm_bindgen(method, getter)]
    pub fn data(&self) -> JsValue {
        self.data.to_owned()
    }
}
