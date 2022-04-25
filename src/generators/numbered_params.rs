pub struct NumberedParams {
    pub params: Vec<String>,
}

impl NumberedParams {
    pub fn new() -> Self {
        Self { params: Vec::new() }
    }

    pub fn add_or_get(&mut self, name: &str) -> i32 {
        let mut param_no = 1;
        for param_name in &self.params {
            if param_name == name {
                return param_no;
            }

            param_no += 1;
        }

        self.params.push(name.to_string());

        param_no
    }
}
