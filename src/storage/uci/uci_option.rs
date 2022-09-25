use std::collections::HashSet;

#[derive(PartialEq)]
pub struct UciOption {
    pub name: String,
    pub values: Vec<String>,
    pub opt_type: UciOptionType,
}

#[derive(PartialEq)]
pub enum UciOptionType {
    TypeOption,
    TypeList,
}

impl UciOption {
    pub fn new(name: String, opt_type: UciOptionType, values: Vec<String>) -> UciOption {
        UciOption {
            name: name,
            opt_type: opt_type,
            values: values,
        }
    }

    pub fn set_values(&mut self, values: Vec<String>) {
        self.values = values;
    }

    pub fn add_values(&mut self, value: String) {
        self.values.push(value);
    }

    pub fn merge_values(&mut self, values: Vec<String>) {
        let set: HashSet<String> = HashSet::from_iter([self.values, values].concat().into_iter());

        self.values = set.into_iter().collect();
    }
}