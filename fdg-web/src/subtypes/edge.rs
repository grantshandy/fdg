use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ForceGraphEdge {
    source: String,
    target: String,
    weight: JsValue,
}

#[wasm_bindgen]
impl ForceGraphEdge {
    #[wasm_bindgen(method, getter)]
    pub fn source(&self) -> String {
        self.source.to_owned()
    }

    #[wasm_bindgen(method, getter)]
    pub fn target(&self) -> String {
        self.target.to_owned()
    }

    #[wasm_bindgen(method, getter)]
    pub fn weight(&self) -> JsValue {
        self.weight.to_owned()
    }
}

impl ForceGraphEdge {
    pub fn new(source: String, target: String, weight: JsValue) -> Self {
        Self {
            source,
            target,
            weight,
        }
    }
}
