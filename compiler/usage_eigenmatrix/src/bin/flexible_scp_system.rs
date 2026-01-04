use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ScpEntity {
    pub number: String,
    pub class: String,
    pub description: String,
    pub properties: HashMap<String, String>,
}

impl ScpEntity {
    pub fn new(number: &str, class: &str, description: &str) -> Self {
        Self {
            number: number.to_string(),
            class: class.to_string(),
            description: description.to_string(),
            properties: HashMap::new(),
        }
    }
    
    pub fn add_property(&mut self, key: &str, value: &str) -> &mut Self {
        self.properties.insert(key.to_string(), value.to_string());
        self
    }
    
    pub fn generate_code(&self, language: &str) -> String {
        match language {
            "rust" => self.rust_template(),
            "python" => self.python_template(),
            _ => format!("// Unsupported language: {}", language),
        }
    }
    
    fn rust_template(&self) -> String {
        format!(r#"
// {} - {} Implementation
use std::collections::HashMap;

pub struct {} {{
    pub active: bool,
    pub containment_level: u8,
}}

impl {} {{
    pub fn new() -> Self {{
        Self {{ active: true, containment_level: 1 }}
    }}
    
    pub fn status(&self) -> &str {{
        if self.active {{ "CONTAINED" }} else {{ "BREACH" }}
    }}
}}

fn main() {{
    let entity = {}::new();
    println!("{} Status: {{}}", entity.status());
}}
"#, self.number, self.description, self.safe_name(), self.safe_name(), self.safe_name(), self.number)
    }
    
    fn python_template(&self) -> String {
        format!(r#"
# {} - {} Implementation
class {}:
    def __init__(self):
        self.active = True
        self.containment_level = 1
    
    def status(self):
        return "CONTAINED" if self.active else "BREACH"

if __name__ == "__main__":
    entity = {}()
    print(f"{} Status: {{entity.status()}}")
"#, self.number, self.description, self.safe_name(), self.safe_name(), self.number)
    }
    
    fn safe_name(&self) -> String {
        self.number.replace("-", "_").replace(" ", "_")
    }
}

macro_rules! scp {
    ($num:literal, $class:literal, $desc:literal) => {{
        ScpEntity::new($num, $class, $desc)
    }};
    ($num:literal, $class:literal, $desc:literal, $($key:literal => $value:literal),*) => {{
        let mut entity = ScpEntity::new($num, $class, $desc);
        $(entity.add_property($key, $value);)*
        entity
    }};
}

fn main() {
    let scp173 = scp!("SCP-173", "Euclid", "The Sculpture");
    println!("{}", scp173.generate_code("rust"));
    
    let scp3008 = scp!("SCP-3008", "Euclid", "A Perfectly Normal IKEA", 
        "danger_level" => "moderate",
        "staff_required" => "12"
    );
    println!("{}", scp3008.generate_code("python"));
}
