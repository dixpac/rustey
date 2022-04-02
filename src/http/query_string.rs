use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'buff> {
    data: HashMap<&'buff str, Value<'buff>>,
}

#[derive(Debug)]
pub enum Value<'buff> {
    Single(&'buff str),
    Multiple(Vec<&'buff str>),
}

impl<'buff> QueryString<'buff> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'buff> From<&'buff str> for QueryString<'buff> {
    fn from(s: &'buff str) -> Self {
        let mut data = HashMap::new();
        for sub_string in s.split('&') {
            let mut key = sub_string;
            let mut value = "";

            if let Some(i) = sub_string.find('=') {
                key = &sub_string[..i];
                value = &sub_string[i + 1..];
            }

            data.entry(key)
            .and_modify(|existing: &mut Value| match existing {
                Value::Single(prev_value) => {
                    *existing = Value::Multiple(vec![prev_value, value]);
                },
                Value::Multiple(vec) => vec.push(value),
            })
            .or_insert(Value::Single(value));

        }


        QueryString { data }



    }
}