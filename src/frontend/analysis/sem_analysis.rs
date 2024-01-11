/*
Checks an AST for semantic correctness
 */

use crate::frontend::{ 
    ast::{ 
        ast_struct::{ AST, ASTNode }, 
        syntax_element::SyntaxElement, 
        data_type:: DataType 
    },
    utils::error::ErrorType,
    analysis::symbol_table::{ SymbolTableStack, SymbolTable } 
};

/// Checks a given AST for semantic correctness
pub struct SemAnalysis {
    input: AST,
    scope_stack: SymbolTableStack

}

impl SemAnalysis {
    fn new(input: AST) -> Self {
        Self {
            input,
            scope_stack: SymbolTableStack::new(),
        }
    }

    /// checks an ast for semantic correctness
    pub fn sem_analysis(input: AST) -> Vec<ErrorType> { 
        let mut semantic_analysis: SemAnalysis = SemAnalysis::new(input);
        semantic_analysis.scope_stack.push(SymbolTable::new());

        let mut errors: Vec<ErrorType> = Vec::new();
        let root: ASTNode = semantic_analysis.input.get_root().clone();

        if let SyntaxElement::ModuleExpression = root.get_element() {
            for child in &root.get_children() {
                semantic_analysis.node_analysis(child, &mut errors);
            }        
        }
        errors
    }

    /// analyzies each node, recursively, until it has checked all nodes, and appends errors
    fn node_analysis(&mut self, node: &ASTNode, errors: &mut Vec<ErrorType>) {
        match &node.get_element() {
            SyntaxElement::ModuleExpression => {
                errors.push(ErrorType::InvalidAssignment {
                    target: "ModuleExpression".to_string()
                })
            },
            SyntaxElement::BinaryExpression{left,
                                              operator, 
                                              right} => {

                if operator == "/" && self.is_zero(right) {
                    errors.push(ErrorType::DivisionByZero {
                        operation: format!("{}/{} is division by zero", left, right)
                    });
                }                                
                self.node_analysis(left, errors);
                self.node_analysis(right, errors);
            },
            SyntaxElement::IfStatement{condition, 
                                         then_branch, 
                                         else_branch} => {
                self.node_analysis(condition, errors);

                self.scope_stack.push(SymbolTable::new());
                self.node_analysis(then_branch, errors);
                self.scope_stack.pop();

                if let Some(else_branch) = else_branch {
                    self.scope_stack.push(SymbolTable::new());
                    self.node_analysis(else_branch, errors);
                    self.scope_stack.pop();
                }
            },
            SyntaxElement::Assignment{ variable, 
                                        value } => {
                if !self.is_variable_defined(variable) {
                    errors.push(ErrorType::UndefinedVariable {
                        variable_name: variable.clone()
                    })
                }
                self.node_analysis(value, errors);
            },
            _ => unimplemented!("Unimplemented syntactic check")
        }
        for child in &node.get_children() {
            self.node_analysis(child, errors);
        }
    }

    /// checks if a node is 0
    fn is_zero(&self, node: &ASTNode) -> bool {
        match &node.get_element() {
            SyntaxElement::Literal(DataType::Integer, value) => {
                value.parse::<i64>().map_or(false, |num| num == 0)
            },
            SyntaxElement::Literal(DataType::Float, value) => {
                value.parse::<f64>().map_or(false, |num| num == 0.0)
            },
            _ => false,  
        }
    }

    /// checks if the current symbol table (on top of the stack) defines a var
    fn is_variable_defined(&self, variable: &String) -> bool {
        if let Some(top_table) = self.scope_stack.peek() {
            return top_table.get(variable).is_some();
        }
        panic!("No scope defined");
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::{ 
        ast::ast_struct::{ AST, ASTNode }, 
        ast::syntax_element::SyntaxElement, 
        ast::data_type::DataType 
    };
    #[test]
    fn basic_test() {
        let left_node = ASTNode::new(SyntaxElement::Literal(DataType::Integer, "5".to_string()));
        let right_node = ASTNode::new(SyntaxElement::Literal(DataType::Integer, "0".to_string()));

        let division_expr = ASTNode::new(SyntaxElement::BinaryExpression {
            left: Box::new(left_node),
            operator: "/".to_string(),
            right: Box::new(right_node),
        });

        let mut root_node = ASTNode::new(SyntaxElement::ModuleExpression);
        root_node.add_child(division_expr);

        let ast = AST::new(root_node);

        let errors = SemAnalysis::sem_analysis(ast);

        assert!(errors.iter().any(|e| matches!(e, ErrorType::DivisionByZero { .. })),
                "Expected DivisionByZero error, but found {:?}", errors);
    }
}
