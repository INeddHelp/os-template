use std::collections::HashMap;

// Enumeration of different input types
pub enum InputType {
    Text,
    Number,
    Email,
    Password,
}

// Structure of an input element
pub struct Input {
    input_type: InputType,
    label: String,
    value: String,
}

// Structure of a form containing input elements
pub struct Form {
    inputs: HashMap<String, Input>,
}

// Implementation of the Form structure
impl Form {
    // Method to add a new input element to the form
    pub fn add_input(&mut self, name: String, input: Input) {
        self.inputs.insert(name, input);
    }

    // Method to remove an input element from the form
    pub fn remove_input(&mut self, name: &str) {
        self.inputs.remove(name);
    }

    // Method to get an input element from the form
    pub fn get_input(&self, name: &str) -> Option<&Input> {
        self.inputs.get(name)
    }

    // Method to get the value of an input element from the form
    pub fn get_input_value(&self, name: &str) -> Option<&str> {
        self.inputs.get(name).map(|input| input.value.as_str())
    }

    // Method to set the value of an input element in the form
    pub fn set_input_value(&mut self, name: &str, value: String) {
        if let Some(input) = self.inputs.get_mut(name) {
            input.value = value;
        }
    }
}
