pub struct SelectStatement {
    pub columns: Vec<String>,
    pub table: String,
    pub condition: Option<Condition>,
}

pub enum Condition {
    Comparison {
        left: String,
        op: String,
        right: Value,
    },
    And(Box<Condition>, Box<Condition>),
    Or(Box<Condition>, Box<Condition>),
}

pub enum Value {
    String(String),
    Number(i64),
}
