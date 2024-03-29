// ---------------------------------------------------------
// This file was generated by parol.
// It is not intended for manual editing and changes will be
// lost after next build.
// ---------------------------------------------------------

#![allow(unused_imports)]
use crate::json_grammar::JsonGrammar;
use id_tree::Tree;
use log::trace;
use miette::{bail, miette, IntoDiagnostic, Result};
use parol_macros::{pop_and_reverse_item, pop_item};
use parol_runtime::lexer::Token;
use parol_runtime::parser::{ParseTreeStackEntry, ParseTreeType, UserActionsTrait};

/// Semantic actions trait generated for the user grammar
/// All functions have default implementations.
pub trait JsonGrammarTrait<'t> {
    /// Semantic action for non-terminal 'Json'
    fn json(&mut self, _arg: &Json<'t>) -> Result<()> {
        Ok(())
    }

    /// Semantic action for non-terminal 'Object'
    fn object(&mut self, _arg: &Object<'t>) -> Result<()> {
        Ok(())
    }

    /// Semantic action for non-terminal 'Pair'
    fn pair(&mut self, _arg: &Pair<'t>) -> Result<()> {
        Ok(())
    }

    /// Semantic action for non-terminal 'Array'
    fn array(&mut self, _arg: &Array<'t>) -> Result<()> {
        Ok(())
    }

    /// Semantic action for non-terminal 'Value'
    fn value(&mut self, _arg: &Value<'t>) -> Result<()> {
        Ok(())
    }

    /// Semantic action for non-terminal 'String'
    fn string(&mut self, _arg: &String<'t>) -> Result<()> {
        Ok(())
    }

    /// Semantic action for non-terminal 'Number'
    fn number(&mut self, _arg: &Number<'t>) -> Result<()> {
        Ok(())
    }
}

// -------------------------------------------------------------------------------------------------
//
// Output Types of productions deduced from the structure of the transformed grammar
//

///
/// Type derived for production 2
///
/// ObjectSuffix: Pair ObjectList /* Vec */ "\}"^ /* Clipped */;
///
#[allow(dead_code)]
#[derive(Builder, Debug, Clone)]
pub struct ObjectSuffix0<'t> {
    pub pair: Box<Pair<'t>>,
    pub object_list: Vec<ObjectList<'t>>,
}

///
/// Type derived for production 3
///
/// ObjectSuffix: "\}"^ /* Clipped */;
///
#[allow(dead_code)]
#[derive(Builder, Debug, Clone)]
pub struct ObjectSuffix1 {}

///
/// Type derived for production 8
///
/// ArraySuffix: Value ArrayList /* Vec */ "\]"^ /* Clipped */;
///
#[allow(dead_code)]
#[derive(Builder, Debug, Clone)]
pub struct ArraySuffix0<'t> {
    pub value: Box<Value<'t>>,
    pub array_list: Vec<ArrayList<'t>>,
}

///
/// Type derived for production 9
///
/// ArraySuffix: "\]"^ /* Clipped */;
///
#[allow(dead_code)]
#[derive(Builder, Debug, Clone)]
pub struct ArraySuffix1 {}

///
/// Type derived for production 12
///
/// Value: String;
///
#[allow(dead_code)]
#[derive(Builder, Debug, Clone)]
pub struct Value0<'t> {
    pub string: Box<String<'t>>,
}

///
/// Type derived for production 13
///
/// Value: Number;
///
#[allow(dead_code)]
#[derive(Builder, Debug, Clone)]
pub struct Value1<'t> {
    pub number: Box<Number<'t>>,
}

///
/// Type derived for production 14
///
/// Value: Object;
///
#[allow(dead_code)]
#[derive(Builder, Debug, Clone)]
pub struct Value2<'t> {
    pub object: Box<Object<'t>>,
}

///
/// Type derived for production 15
///
/// Value: Array;
///
#[allow(dead_code)]
#[derive(Builder, Debug, Clone)]
pub struct Value3<'t> {
    pub array: Box<Array<'t>>,
}

///
/// Type derived for production 16
///
/// Value: "true"^ /* Clipped */;
///
#[allow(dead_code)]
#[derive(Builder, Debug, Clone)]
pub struct Value4 {}

///
/// Type derived for production 17
///
/// Value: "false"^ /* Clipped */;
///
#[allow(dead_code)]
#[derive(Builder, Debug, Clone)]
pub struct Value5 {}

///
/// Type derived for production 18
///
/// Value: "null"^ /* Clipped */;
///
#[allow(dead_code)]
#[derive(Builder, Debug, Clone)]
pub struct Value6 {}

// -------------------------------------------------------------------------------------------------
//
// Types of non-terminals deduced from the structure of the transformed grammar
//

///
/// Type derived for non-terminal Array
///
#[allow(dead_code)]
#[derive(Builder, Debug, Clone)]
pub struct Array<'t> {
    pub array_suffix: Box<ArraySuffix<'t>>,
}

///
/// Type derived for non-terminal ArrayList
///
#[allow(dead_code)]
#[derive(Builder, Debug, Clone)]
pub struct ArrayList<'t> {
    pub value: Box<Value<'t>>,
}

///
/// Type derived for non-terminal ArraySuffix
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ArraySuffix<'t> {
    ArraySuffix0(ArraySuffix0<'t>),
    ArraySuffix1(ArraySuffix1),
}

///
/// Type derived for non-terminal Json
///
#[allow(dead_code)]
#[derive(Builder, Debug, Clone)]
pub struct Json<'t> {
    pub value: Box<Value<'t>>,
}

///
/// Type derived for non-terminal Number
///
#[allow(dead_code)]
#[derive(Builder, Debug, Clone)]
pub struct Number<'t> {
    pub number: Token<'t>, /* -?(?:0|[1-9][0-9]*)(?:\.[0-9]+)?(?:[eE][-+]?(?:0|[1-9][0-9]*)?)? */
}

///
/// Type derived for non-terminal Object
///
#[allow(dead_code)]
#[derive(Builder, Debug, Clone)]
pub struct Object<'t> {
    pub object_suffix: Box<ObjectSuffix<'t>>,
}

///
/// Type derived for non-terminal ObjectList
///
#[allow(dead_code)]
#[derive(Builder, Debug, Clone)]
pub struct ObjectList<'t> {
    pub pair: Box<Pair<'t>>,
}

///
/// Type derived for non-terminal ObjectSuffix
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ObjectSuffix<'t> {
    ObjectSuffix0(ObjectSuffix0<'t>),
    ObjectSuffix1(ObjectSuffix1),
}

///
/// Type derived for non-terminal Pair
///
#[allow(dead_code)]
#[derive(Builder, Debug, Clone)]
pub struct Pair<'t> {
    pub string: Box<String<'t>>,
    pub value: Box<Value<'t>>,
}

///
/// Type derived for non-terminal String
///
#[allow(dead_code)]
#[derive(Builder, Debug, Clone)]
pub struct String<'t> {
    pub string: Token<'t>, /* \u{0022}(?:\\[\u{0022}\\/bfnrt]|u[0-9a-fA-F]{4}|[^\u{0022}\\\u0000-\u001F])*\u{0022} */
}

///
/// Type derived for non-terminal Value
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Value<'t> {
    Value0(Value0<'t>),
    Value1(Value1<'t>),
    Value2(Value2<'t>),
    Value3(Value3<'t>),
    Value4(Value4),
    Value5(Value5),
    Value6(Value6),
}

// -------------------------------------------------------------------------------------------------

///
/// Deduced ASTType of expanded grammar
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ASTType<'t> {
    Array(Array<'t>),
    ArrayList(Vec<ArrayList<'t>>),
    ArraySuffix(ArraySuffix<'t>),
    Json(Json<'t>),
    Number(Number<'t>),
    Object(Object<'t>),
    ObjectList(Vec<ObjectList<'t>>),
    ObjectSuffix(ObjectSuffix<'t>),
    Pair(Pair<'t>),
    String(String<'t>),
    Value(Value<'t>),
}

/// Auto-implemented adapter grammar
///
/// The lifetime parameter `'t` refers to the lifetime of the scanned text.
/// The lifetime parameter `'u` refers to the lifetime of user grammar object.
///
#[allow(dead_code)]
pub struct JsonGrammarAuto<'t, 'u>
where
    't: 'u,
{
    // Mutable reference of the actual user grammar to be able to call the semantic actions on it
    user_grammar: &'u mut dyn JsonGrammarTrait<'t>,
    // Stack to construct the AST on it
    item_stack: Vec<ASTType<'t>>,
}

///
/// The `JsonGrammarAuto` impl is automatically generated for the
/// given grammar.
///
impl<'t, 'u> JsonGrammarAuto<'t, 'u> {
    pub fn new(user_grammar: &'u mut dyn JsonGrammarTrait<'t>) -> Self {
        Self {
            user_grammar,
            item_stack: Vec::new(),
        }
    }

    #[allow(dead_code)]
    fn push(&mut self, item: ASTType<'t>, context: &str) {
        trace!("push    {}: {:?}", context, item);
        self.item_stack.push(item)
    }

    #[allow(dead_code)]
    fn pop(&mut self, context: &str) -> Option<ASTType<'t>> {
        if !self.item_stack.is_empty() {
            let item = self.item_stack.pop();
            if let Some(ref item) = item {
                trace!("pop     {}: {:?}", context, item);
            }
            item
        } else {
            None
        }
    }

    #[allow(dead_code)]
    // Use this function for debugging purposes:
    // trace!("{}", self.trace_item_stack(context));
    fn trace_item_stack(&self, context: &str) -> std::string::String {
        format!(
            "Item stack at {}:\n{}",
            context,
            self.item_stack
                .iter()
                .rev()
                .map(|s| format!("  {:?}", s))
                .collect::<Vec<std::string::String>>()
                .join("\n")
        )
    }

    /// Semantic action for production 0:
    ///
    /// Json: Value;
    ///
    #[named]
    fn json(
        &mut self,
        _value: &ParseTreeStackEntry<'t>,
        _parse_tree: &Tree<ParseTreeType<'t>>,
    ) -> Result<()> {
        let context = function_name!();
        trace!("{}", self.trace_item_stack(context));
        let value = pop_item!(self, value, Value, context);
        let json_built = JsonBuilder::default()
            .value(Box::new(value))
            .build()
            .into_diagnostic()?;
        // Calling user action here
        self.user_grammar.json(&json_built)?;
        self.push(ASTType::Json(json_built), context);
        Ok(())
    }

    /// Semantic action for production 1:
    ///
    /// Object: "\{"^ /* Clipped */ ObjectSuffix;
    ///
    #[named]
    fn object(
        &mut self,
        _l_brace: &ParseTreeStackEntry<'t>,
        _object_suffix: &ParseTreeStackEntry<'t>,
        _parse_tree: &Tree<ParseTreeType<'t>>,
    ) -> Result<()> {
        let context = function_name!();
        trace!("{}", self.trace_item_stack(context));
        let object_suffix = pop_item!(self, object_suffix, ObjectSuffix, context);
        let object_built = ObjectBuilder::default()
            // Ignore clipped member 'l_brace'
            .object_suffix(Box::new(object_suffix))
            .build()
            .into_diagnostic()?;
        // Calling user action here
        self.user_grammar.object(&object_built)?;
        self.push(ASTType::Object(object_built), context);
        Ok(())
    }

    /// Semantic action for production 2:
    ///
    /// ObjectSuffix: Pair ObjectList /* Vec */ "\}"^ /* Clipped */;
    ///
    #[named]
    fn object_suffix_0(
        &mut self,
        _pair: &ParseTreeStackEntry<'t>,
        _object_list: &ParseTreeStackEntry<'t>,
        _r_brace: &ParseTreeStackEntry<'t>,
        _parse_tree: &Tree<ParseTreeType<'t>>,
    ) -> Result<()> {
        let context = function_name!();
        trace!("{}", self.trace_item_stack(context));
        let object_list = pop_and_reverse_item!(self, object_list, ObjectList, context);
        let pair = pop_item!(self, pair, Pair, context);
        let object_suffix_0_built = ObjectSuffix0Builder::default()
            .pair(Box::new(pair))
            .object_list(object_list)
            // Ignore clipped member 'r_brace'
            .build()
            .into_diagnostic()?;
        let object_suffix_0_built = ObjectSuffix::ObjectSuffix0(object_suffix_0_built);
        self.push(ASTType::ObjectSuffix(object_suffix_0_built), context);
        Ok(())
    }

    /// Semantic action for production 3:
    ///
    /// ObjectSuffix: "\}"^ /* Clipped */;
    ///
    #[named]
    fn object_suffix_1(
        &mut self,
        _r_brace: &ParseTreeStackEntry<'t>,
        _parse_tree: &Tree<ParseTreeType<'t>>,
    ) -> Result<()> {
        let context = function_name!();
        trace!("{}", self.trace_item_stack(context));
        let object_suffix_1_built = ObjectSuffix1Builder::default()
            // Ignore clipped member 'r_brace'
            .build()
            .into_diagnostic()?;
        let object_suffix_1_built = ObjectSuffix::ObjectSuffix1(object_suffix_1_built);
        self.push(ASTType::ObjectSuffix(object_suffix_1_built), context);
        Ok(())
    }

    /// Semantic action for production 4:
    ///
    /// ObjectList /* Vec<T>::Push */: ","^ /* Clipped */ Pair ObjectList;
    ///
    #[named]
    fn object_list_0(
        &mut self,
        _comma: &ParseTreeStackEntry<'t>,
        _pair: &ParseTreeStackEntry<'t>,
        _object_list: &ParseTreeStackEntry<'t>,
        _parse_tree: &Tree<ParseTreeType<'t>>,
    ) -> Result<()> {
        let context = function_name!();
        trace!("{}", self.trace_item_stack(context));
        let mut object_list = pop_item!(self, object_list, ObjectList, context);
        let pair = pop_item!(self, pair, Pair, context);
        let object_list_0_built = ObjectListBuilder::default()
            .pair(Box::new(pair))
            // Ignore clipped member 'comma'
            .build()
            .into_diagnostic()?;
        // Add an element to the vector
        object_list.push(object_list_0_built);
        self.push(ASTType::ObjectList(object_list), context);
        Ok(())
    }

    /// Semantic action for production 5:
    ///
    /// ObjectList /* Vec<T>::New */: ;
    ///
    #[named]
    fn object_list_1(&mut self, _parse_tree: &Tree<ParseTreeType<'t>>) -> Result<()> {
        let context = function_name!();
        trace!("{}", self.trace_item_stack(context));
        let object_list_1_built = Vec::new();
        self.push(ASTType::ObjectList(object_list_1_built), context);
        Ok(())
    }

    /// Semantic action for production 6:
    ///
    /// Pair: String ":"^ /* Clipped */ Value;
    ///
    #[named]
    fn pair(
        &mut self,
        _string: &ParseTreeStackEntry<'t>,
        _colon: &ParseTreeStackEntry<'t>,
        _value: &ParseTreeStackEntry<'t>,
        _parse_tree: &Tree<ParseTreeType<'t>>,
    ) -> Result<()> {
        let context = function_name!();
        trace!("{}", self.trace_item_stack(context));
        let value = pop_item!(self, value, Value, context);
        let string = pop_item!(self, string, String, context);
        let pair_built = PairBuilder::default()
            .string(Box::new(string))
            // Ignore clipped member 'colon'
            .value(Box::new(value))
            .build()
            .into_diagnostic()?;
        // Calling user action here
        self.user_grammar.pair(&pair_built)?;
        self.push(ASTType::Pair(pair_built), context);
        Ok(())
    }

    /// Semantic action for production 7:
    ///
    /// Array: "\["^ /* Clipped */ ArraySuffix;
    ///
    #[named]
    fn array(
        &mut self,
        _l_bracket: &ParseTreeStackEntry<'t>,
        _array_suffix: &ParseTreeStackEntry<'t>,
        _parse_tree: &Tree<ParseTreeType<'t>>,
    ) -> Result<()> {
        let context = function_name!();
        trace!("{}", self.trace_item_stack(context));
        let array_suffix = pop_item!(self, array_suffix, ArraySuffix, context);
        let array_built = ArrayBuilder::default()
            // Ignore clipped member 'l_bracket'
            .array_suffix(Box::new(array_suffix))
            .build()
            .into_diagnostic()?;
        // Calling user action here
        self.user_grammar.array(&array_built)?;
        self.push(ASTType::Array(array_built), context);
        Ok(())
    }

    /// Semantic action for production 8:
    ///
    /// ArraySuffix: Value ArrayList /* Vec */ "\]"^ /* Clipped */;
    ///
    #[named]
    fn array_suffix_0(
        &mut self,
        _value: &ParseTreeStackEntry<'t>,
        _array_list: &ParseTreeStackEntry<'t>,
        _r_bracket: &ParseTreeStackEntry<'t>,
        _parse_tree: &Tree<ParseTreeType<'t>>,
    ) -> Result<()> {
        let context = function_name!();
        trace!("{}", self.trace_item_stack(context));
        let array_list = pop_and_reverse_item!(self, array_list, ArrayList, context);
        let value = pop_item!(self, value, Value, context);
        let array_suffix_0_built = ArraySuffix0Builder::default()
            .value(Box::new(value))
            .array_list(array_list)
            // Ignore clipped member 'r_bracket'
            .build()
            .into_diagnostic()?;
        let array_suffix_0_built = ArraySuffix::ArraySuffix0(array_suffix_0_built);
        self.push(ASTType::ArraySuffix(array_suffix_0_built), context);
        Ok(())
    }

    /// Semantic action for production 9:
    ///
    /// ArraySuffix: "\]"^ /* Clipped */;
    ///
    #[named]
    fn array_suffix_1(
        &mut self,
        _r_bracket: &ParseTreeStackEntry<'t>,
        _parse_tree: &Tree<ParseTreeType<'t>>,
    ) -> Result<()> {
        let context = function_name!();
        trace!("{}", self.trace_item_stack(context));
        let array_suffix_1_built = ArraySuffix1Builder::default()
            // Ignore clipped member 'r_bracket'
            .build()
            .into_diagnostic()?;
        let array_suffix_1_built = ArraySuffix::ArraySuffix1(array_suffix_1_built);
        self.push(ASTType::ArraySuffix(array_suffix_1_built), context);
        Ok(())
    }

    /// Semantic action for production 10:
    ///
    /// ArrayList /* Vec<T>::Push */: ","^ /* Clipped */ Value ArrayList;
    ///
    #[named]
    fn array_list_0(
        &mut self,
        _comma: &ParseTreeStackEntry<'t>,
        _value: &ParseTreeStackEntry<'t>,
        _array_list: &ParseTreeStackEntry<'t>,
        _parse_tree: &Tree<ParseTreeType<'t>>,
    ) -> Result<()> {
        let context = function_name!();
        trace!("{}", self.trace_item_stack(context));
        let mut array_list = pop_item!(self, array_list, ArrayList, context);
        let value = pop_item!(self, value, Value, context);
        let array_list_0_built = ArrayListBuilder::default()
            .value(Box::new(value))
            // Ignore clipped member 'comma'
            .build()
            .into_diagnostic()?;
        // Add an element to the vector
        array_list.push(array_list_0_built);
        self.push(ASTType::ArrayList(array_list), context);
        Ok(())
    }

    /// Semantic action for production 11:
    ///
    /// ArrayList /* Vec<T>::New */: ;
    ///
    #[named]
    fn array_list_1(&mut self, _parse_tree: &Tree<ParseTreeType<'t>>) -> Result<()> {
        let context = function_name!();
        trace!("{}", self.trace_item_stack(context));
        let array_list_1_built = Vec::new();
        self.push(ASTType::ArrayList(array_list_1_built), context);
        Ok(())
    }

    /// Semantic action for production 12:
    ///
    /// Value: String;
    ///
    #[named]
    fn value_0(
        &mut self,
        _string: &ParseTreeStackEntry<'t>,
        _parse_tree: &Tree<ParseTreeType<'t>>,
    ) -> Result<()> {
        let context = function_name!();
        trace!("{}", self.trace_item_stack(context));
        let string = pop_item!(self, string, String, context);
        let value_0_built = Value0Builder::default()
            .string(Box::new(string))
            .build()
            .into_diagnostic()?;
        let value_0_built = Value::Value0(value_0_built);
        // Calling user action here
        self.user_grammar.value(&value_0_built)?;
        self.push(ASTType::Value(value_0_built), context);
        Ok(())
    }

    /// Semantic action for production 13:
    ///
    /// Value: Number;
    ///
    #[named]
    fn value_1(
        &mut self,
        _number: &ParseTreeStackEntry<'t>,
        _parse_tree: &Tree<ParseTreeType<'t>>,
    ) -> Result<()> {
        let context = function_name!();
        trace!("{}", self.trace_item_stack(context));
        let number = pop_item!(self, number, Number, context);
        let value_1_built = Value1Builder::default()
            .number(Box::new(number))
            .build()
            .into_diagnostic()?;
        let value_1_built = Value::Value1(value_1_built);
        // Calling user action here
        self.user_grammar.value(&value_1_built)?;
        self.push(ASTType::Value(value_1_built), context);
        Ok(())
    }

    /// Semantic action for production 14:
    ///
    /// Value: Object;
    ///
    #[named]
    fn value_2(
        &mut self,
        _object: &ParseTreeStackEntry<'t>,
        _parse_tree: &Tree<ParseTreeType<'t>>,
    ) -> Result<()> {
        let context = function_name!();
        trace!("{}", self.trace_item_stack(context));
        let object = pop_item!(self, object, Object, context);
        let value_2_built = Value2Builder::default()
            .object(Box::new(object))
            .build()
            .into_diagnostic()?;
        let value_2_built = Value::Value2(value_2_built);
        // Calling user action here
        self.user_grammar.value(&value_2_built)?;
        self.push(ASTType::Value(value_2_built), context);
        Ok(())
    }

    /// Semantic action for production 15:
    ///
    /// Value: Array;
    ///
    #[named]
    fn value_3(
        &mut self,
        _array: &ParseTreeStackEntry<'t>,
        _parse_tree: &Tree<ParseTreeType<'t>>,
    ) -> Result<()> {
        let context = function_name!();
        trace!("{}", self.trace_item_stack(context));
        let array = pop_item!(self, array, Array, context);
        let value_3_built = Value3Builder::default()
            .array(Box::new(array))
            .build()
            .into_diagnostic()?;
        let value_3_built = Value::Value3(value_3_built);
        // Calling user action here
        self.user_grammar.value(&value_3_built)?;
        self.push(ASTType::Value(value_3_built), context);
        Ok(())
    }

    /// Semantic action for production 16:
    ///
    /// Value: "true"^ /* Clipped */;
    ///
    #[named]
    fn value_4(
        &mut self,
        _true: &ParseTreeStackEntry<'t>,
        _parse_tree: &Tree<ParseTreeType<'t>>,
    ) -> Result<()> {
        let context = function_name!();
        trace!("{}", self.trace_item_stack(context));
        let value_4_built = Value4Builder::default()
            // Ignore clipped member 'r#true'
            .build()
            .into_diagnostic()?;
        let value_4_built = Value::Value4(value_4_built);
        // Calling user action here
        self.user_grammar.value(&value_4_built)?;
        self.push(ASTType::Value(value_4_built), context);
        Ok(())
    }

    /// Semantic action for production 17:
    ///
    /// Value: "false"^ /* Clipped */;
    ///
    #[named]
    fn value_5(
        &mut self,
        _false: &ParseTreeStackEntry<'t>,
        _parse_tree: &Tree<ParseTreeType<'t>>,
    ) -> Result<()> {
        let context = function_name!();
        trace!("{}", self.trace_item_stack(context));
        let value_5_built = Value5Builder::default()
            // Ignore clipped member 'r#false'
            .build()
            .into_diagnostic()?;
        let value_5_built = Value::Value5(value_5_built);
        // Calling user action here
        self.user_grammar.value(&value_5_built)?;
        self.push(ASTType::Value(value_5_built), context);
        Ok(())
    }

    /// Semantic action for production 18:
    ///
    /// Value: "null"^ /* Clipped */;
    ///
    #[named]
    fn value_6(
        &mut self,
        _null: &ParseTreeStackEntry<'t>,
        _parse_tree: &Tree<ParseTreeType<'t>>,
    ) -> Result<()> {
        let context = function_name!();
        trace!("{}", self.trace_item_stack(context));
        let value_6_built = Value6Builder::default()
            // Ignore clipped member 'null'
            .build()
            .into_diagnostic()?;
        let value_6_built = Value::Value6(value_6_built);
        // Calling user action here
        self.user_grammar.value(&value_6_built)?;
        self.push(ASTType::Value(value_6_built), context);
        Ok(())
    }

    /// Semantic action for production 19:
    ///
    /// String: "\u{0022}(?:\\[\u{0022}\\/bfnrt]|u[0-9a-fA-F]{4}|[^\u{0022}\\\u0000-\u001F])*\u{0022}";
    ///
    #[named]
    fn string(
        &mut self,
        string: &ParseTreeStackEntry<'t>,
        parse_tree: &Tree<ParseTreeType<'t>>,
    ) -> Result<()> {
        let context = function_name!();
        trace!("{}", self.trace_item_stack(context));
        let string = string.token(parse_tree)?.clone();
        let string_built = StringBuilder::default()
            .string(string)
            .build()
            .into_diagnostic()?;
        // Calling user action here
        self.user_grammar.string(&string_built)?;
        self.push(ASTType::String(string_built), context);
        Ok(())
    }

    /// Semantic action for production 20:
    ///
    /// Number: "-?(?:0|[1-9][0-9]*)(?:\.[0-9]+)?(?:[eE][-+]?(?:0|[1-9][0-9]*)?)?";
    ///
    #[named]
    fn number(
        &mut self,
        number: &ParseTreeStackEntry<'t>,
        parse_tree: &Tree<ParseTreeType<'t>>,
    ) -> Result<()> {
        let context = function_name!();
        trace!("{}", self.trace_item_stack(context));
        let number = number.token(parse_tree)?.clone();
        let number_built = NumberBuilder::default()
            .number(number)
            .build()
            .into_diagnostic()?;
        // Calling user action here
        self.user_grammar.number(&number_built)?;
        self.push(ASTType::Number(number_built), context);
        Ok(())
    }
}

impl<'t> UserActionsTrait<'t> for JsonGrammarAuto<'t, '_> {
    ///
    /// This function is implemented automatically for the user's item JsonGrammar.
    ///
    fn call_semantic_action_for_production_number(
        &mut self,
        prod_num: usize,
        children: &[ParseTreeStackEntry<'t>],
        parse_tree: &Tree<ParseTreeType<'t>>,
    ) -> Result<()> {
        match prod_num {
            0 => self.json(&children[0], parse_tree),
            1 => self.object(&children[0], &children[1], parse_tree),
            2 => self.object_suffix_0(&children[0], &children[1], &children[2], parse_tree),
            3 => self.object_suffix_1(&children[0], parse_tree),
            4 => self.object_list_0(&children[0], &children[1], &children[2], parse_tree),
            5 => self.object_list_1(parse_tree),
            6 => self.pair(&children[0], &children[1], &children[2], parse_tree),
            7 => self.array(&children[0], &children[1], parse_tree),
            8 => self.array_suffix_0(&children[0], &children[1], &children[2], parse_tree),
            9 => self.array_suffix_1(&children[0], parse_tree),
            10 => self.array_list_0(&children[0], &children[1], &children[2], parse_tree),
            11 => self.array_list_1(parse_tree),
            12 => self.value_0(&children[0], parse_tree),
            13 => self.value_1(&children[0], parse_tree),
            14 => self.value_2(&children[0], parse_tree),
            15 => self.value_3(&children[0], parse_tree),
            16 => self.value_4(&children[0], parse_tree),
            17 => self.value_5(&children[0], parse_tree),
            18 => self.value_6(&children[0], parse_tree),
            19 => self.string(&children[0], parse_tree),
            20 => self.number(&children[0], parse_tree),
            _ => Err(miette!("Unhandled production number: {}", prod_num)),
        }
    }
}
