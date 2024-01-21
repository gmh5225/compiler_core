use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use compiler_core::frontend::{ 
    ast::ast_struct::{ AST, ASTNode }, 
    ast::{syntax_element::SyntaxElement, sem_rule::SemanticRule, ast_struct::{ModAST, ModElement}}, 
    ast::{data_type::DataType, sem_rule::RulesConfig},
    analysis::{sem_analysis::*, symbol_table::{SymbolTable, SymbolTableStack}},
    utils::error::*,
};

fn init() -> RulesConfig {
    let rule = |element: &SyntaxElement, _symbol_table: &mut SymbolTable| -> Option<ErrorType> {
        if let SyntaxElement::BinaryExpression { left: _, operator, right } = element {
            if operator == "/" && matches!(right.get_element(), SyntaxElement::Literal { data_type: DataType::Integer, value: ref val } if val == "0") {
                return Some(ErrorType::DivisionByZero{operation: "Divisor is zero".to_string()});
            }
        }
        None
    };

    let sem_rule: SemanticRule = SemanticRule::new(rule);
    let sem_rules: Vec<SemanticRule> = vec![sem_rule];
    let mut rule_mapping: HashMap<SyntaxElement, Vec<SemanticRule>> = HashMap::new();
    rule_mapping.insert(SyntaxElement::BinaryExpression { left: Box::new(ASTNode::default()), operator: String::from("/"), right: Box::new(ASTNode::default()) }, sem_rules);

    RulesConfig::new(rule_mapping)
}

#[test]
fn basic_test() {
    let left_node: ASTNode = ASTNode::new(SyntaxElement::Literal{data_type: DataType::Integer, value: "5".to_string()});
    let right_node: ASTNode = ASTNode::new(SyntaxElement::Literal{data_type: DataType::Integer, value: "0".to_string()});

    let division_expr: ASTNode = ASTNode::new(SyntaxElement::BinaryExpression {
        left: Box::new(left_node),
        operator: "/".to_string(),
        right: Box::new(right_node),
    });

    let mut root_node: ASTNode = ASTNode::new(SyntaxElement::ModuleExpression);
    root_node.add_child(division_expr);

    let ast: AST = AST::new(root_node);
    let mut sym_table = SymbolTableStack::new();
    sym_table.push(SymbolTable::new());

    let sym_table_arc_mutex = Arc::new(Mutex::new(sym_table));

    let mut mod_ast: ModAST = ModAST::new();
    mod_ast.add_child(ModElement::new(ast, sym_table_arc_mutex, 0));

    let rules: RulesConfig = init();

    match SemAnalysis::sem_analysis(mod_ast, rules) {
        Ok(mod_ast) => {
            panic!("expected error")
        }
        Err(errors) => {
            assert!(errors.iter().any(|e| matches!(e, ErrorType::DivisionByZero { .. })),
            "Expected DivisionByZero error, but found {:?}", errors);
        }
    }

}
