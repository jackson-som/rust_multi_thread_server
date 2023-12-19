#![allow(unused_variables)]

use std::collections::HashMap;

#[derive(Debug)]
pub enum Value<'b> {
    Single(&'b str),
    Multi(Vec<&'b str>),
}

pub struct Query<'a> {
    data: HashMap<&'a str, Value<'a>>,
}

impl<'a> Query<'a> {
    pub fn get(&self, key: &'a str) -> Option<&Value<'a>> {
        self.data.get(key)
    }
}

impl<'a> From<&'a str> for Query<'a> {
    fn from(value: &'a str) -> Self {
        let mut data: HashMap<&'a str, Value<'a>> = HashMap::new();

        if !value.is_empty() {
            for split_value in value.split('&') {
                let mut k = "";
                let mut v = "";

                if let Some(i) = split_value.find('&') {
                    k = &split_value[..i]; // Key
                    v = &split_value[i + 1..]; // Value
                }

                data.entry(k)
                    .and_modify(|exist| {
                        match exist {
                            Value::Single(prev_value) => {
                                *exist = Value::Multi(vec![prev_value, v]);
                            }
                            Value::Multi(prev_vec) => {
                                prev_vec.push(v);
                            }
                        }
                    })
                    .or_insert(Value::Single(v));
            }
        }

        Query { data }
    }
}