/// # Chapter 4 - Enum & Patterns
///
/// Create an enum named `Json` that can work with arbitrary JSON data.
fn main() {
    let json = Json::Object(vec![
        (
            String::from("name"),
            Json::String(String::from("Peng Ding")),
        ),
        (String::from("age"), Json::Number(25.0)),
        (String::from("is_father"), Json::Bool(true)),
        (
            String::from("sons"),
            Json::Array(vec![Json::String(String::from("Echin Ding"))]),
        ),
        (String::from("daughters"), Json::Null),
    ]);

    println!("{}", json.to_string());
}

#[derive(Debug)]
enum Json {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Vec<(String, Json)>),
}

#[allow(dead_code, unused_variables)]
impl Json {
    fn to_string(&self) -> String {
        match self {
            Json::Null => "null".to_string(),
            Json::Bool(b) => b.to_string(),
            Json::Number(n) => n.to_string(),
            Json::String(s) => format!("\"{}\"", s),
            Json::Array(arr) => {
                let mut res = "[".to_string();
                for (i, item) in arr.iter().enumerate() {
                    if i != 0 {
                        res.push_str(", ");
                    }
                    res.push_str(&item.to_string());
                }
                res.push(']');
                res
            }
            Json::Object(obj) => {
                let mut res = "{".to_string();
                for (i, (key, value)) in obj.iter().enumerate() {
                    if i != 0 {
                        res.push_str(", ");
                    }
                    res.push_str(&format!("\"{}\": {}", key, value.to_string()));
                }
                res.push('}');
                res
            }
        }
    }

    fn from_string(s: &str) -> Json {
        unimplemented!()
    }
}
