use super::{Force, Value};

#[derive(Clone)]
pub struct Resize {
    dict: Vec<(&'static str, Value)>,
    dict_default: Vec<(&'static str, Value)>,
}

impl Resize {
    pub fn new() -> Self {
        let dict = vec![("Size", Value::Number(1.0, 0.1..=10.0))];

        Self {
            dict: dict.clone(),
            dict_default: dict,
        }
    }
}

impl Default for Resize {
    fn default() -> Self {
        let dict = vec![("Size", Value::Number(1.0, 0.1..=10.0))];

        Self {
            dict: dict.clone(),
            dict_default: dict,
        }
    }
}

impl<D: Clone> Force<D> for Resize {
    fn update(&self, _graph: &mut crate::ForceGraph<D>, _dt: f32) {
        println!("Update!")
    }

    fn dict_mut(&mut self) -> &mut [(&'static str, Value)] {
        &mut self.dict
    }

    fn dict(&self) -> &[(&'static str, Value)] {
        &self.dict
    }

    fn reset(&mut self) {
        self.dict = self.dict_default.clone()
    }

    fn name(&self) -> &'static str {
        "Resize"
    }

    fn continuous(&self) -> bool {
        false
    }
}
