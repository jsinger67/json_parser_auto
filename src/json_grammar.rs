use crate::json_grammar_trait::*;
use miette::Result;
use std::fmt::{Debug, Display, Error, Formatter};

impl Display for Json {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        write!(f, "{}", self.value_0)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        match self {
            Value::Value12(v) => write!(f, "{}", v.string_0.string_0.symbol),
            Value::Value13(v) => write!(f, "{}", v.number_0.number_0.symbol),
            Value::Value14(v) => write!(f, "{{{}}}", v.object_0.object_suffix_1),
            Value::Value15(v) => write!(f, "[{}]", v.array_0.array_suffix_1),
            Value::Value16(_) => write!(f, "true"),
            Value::Value17(_) => write!(f, "false"),
            Value::Value18(_) => write!(f, "null"),
        }
    }
}

impl Display for ObjectSuffix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        match self {
            ObjectSuffix::ObjectSuffix2(o) => write!(
                f,
                "{}{}",
                o.pair_0,
                o.object_list_1
                    .iter()
                    .map(|e| format!("{}", e))
                    .collect::<Vec<std::string::String>>()
                    .join("")
            ),
            ObjectSuffix::ObjectSuffix3(_) => Ok(()),
        }
    }
}

impl Display for ObjectList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        write!(f, "{} {}", self.comma_0.symbol, self.pair_1)
    }
}

impl Display for ArraySuffix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        match self {
            ArraySuffix::ArraySuffix8(a) => write!(
                f,
                "{}{}",
                a.value_0,
                a.array_list_1
                    .iter()
                    .map(|e| format!("{}", e))
                    .collect::<Vec<std::string::String>>()
                    .join("")
            ),
            ArraySuffix::ArraySuffix9(_) => Ok(()),
        }
    }
}

impl Display for ArrayList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        write!(f, "{} {}", self.comma_0.symbol, self.value_1)
    }
}

impl Display for Pair {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        write!(f, "{}: {}", self.string_0.string_0.symbol, self.value_2)
    }
}

///
/// Data structure used to build up a json structure during parsing
///
#[derive(Debug, Default)]
pub struct JsonGrammar {
    pub json: Option<Json>,
}

impl Display for JsonGrammar {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        match &self.json {
            Some(json) => write!(f, "{}", json),
            None => write!(f, "No parse result"),
        }
    }
}

impl JsonGrammar {
    pub fn new() -> Self {
        JsonGrammar::default()
    }
}

impl JsonGrammarTrait for JsonGrammar {
    fn json(&mut self, arg: Json) -> Result<()> {
        self.json = Some(arg);
        Ok(())
    }
}