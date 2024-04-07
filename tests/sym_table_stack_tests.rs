use compiler_core::frontend::{
    ast::{
        ast_struct::{ASTNode, AST}, data_type::DataType, syntax_element::SyntaxElement
    }, 
    symbol_table::symbol_table_struct::{SymbolInfo, SymbolTable, SymbolTableStack, SymbolValue}, utils::error::ErrorType,
};

/// --- UTILITIES SECTION --- ///
/// cargo test --test sym_table_stack_tests 

#[test]
fn test_function_declaration_sym_table() {
    let mut expected_sts: SymbolTableStack = SymbolTableStack::new();
    let mut global_scope: SymbolTable = SymbolTable::new();
    let fn_value: SymbolValue = SymbolValue::FunctionValue { parameters: vec![], return_type: None };

    let fn_info: SymbolInfo = SymbolInfo::new(DataType::Function, fn_value);
    global_scope.add("test_function".to_string(), fn_info);
    expected_sts.push(global_scope);

    expected_sts.push(SymbolTable::new());  // fn scope

    let fn_id: ASTNode = ASTNode::new(SyntaxElement::Identifier("test_function".to_string()));
    let block_exp: ASTNode = ASTNode::new(SyntaxElement::BlockExpression);

    let mut root: ASTNode = ASTNode::new(SyntaxElement::FunctionDeclaration);
    root.add_child(fn_id);
    root.add_child(block_exp);

    let ast: AST = AST::new(root);
    let result_sts_gen: Result<(AST, SymbolTableStack), Vec<ErrorType>> = SymbolTableStack::gen_sym_table_stack(ast);
    match result_sts_gen {
        Ok((_ast, sts)) => {
            assert_eq!(sts, expected_sts);
        }
        Err(e) => panic!("Couldn't parse AST, error: {:?}", e)
    }
}

#[test]
fn test_struct_declaration_sym_table() {
    let mut expected_sts: SymbolTableStack = SymbolTableStack::new();
    let mut global_scope: SymbolTable = SymbolTable::new();

    let struct_value: SymbolValue = SymbolValue::StructValue { 
        fields: vec![("field1".to_string(), DataType::Integer)],
    };

    let struct_info: SymbolInfo = SymbolInfo::new(DataType::Struct, struct_value);
    global_scope.add("test_struct".to_string(), struct_info);
    expected_sts.push(global_scope);

    let struct_id: ASTNode = ASTNode::new(SyntaxElement::Identifier("test_struct".to_string()));

    let mut field_node: ASTNode = ASTNode::new(SyntaxElement::Field);
    let field_id: ASTNode = ASTNode::new(SyntaxElement::Identifier("field1".to_string()));
    let field_type: ASTNode = ASTNode::new(SyntaxElement::Type(DataType::Integer));
    field_node.add_child(field_id);
    field_node.add_child(field_type);

    let mut root: ASTNode = ASTNode::new(SyntaxElement::StructDeclaration);
    root.add_child(struct_id);
    root.add_child(field_node);

    let ast: AST = AST::new(root);
    let result_sts_gen: Result<(AST, SymbolTableStack), Vec<ErrorType>> = SymbolTableStack::gen_sym_table_stack(ast);

    match result_sts_gen {
        Ok((_ast, sts)) => {
            assert_eq!(sts, expected_sts);
        },
        Err(e) => panic!("Couldn't parse AST, error: {:?}", e),
    }
}

#[test]
fn test_enum_declaration_sym_table() {
    let mut expected_sts: SymbolTableStack = SymbolTableStack::new();
    let mut global_scope: SymbolTable = SymbolTable::new();
    let enum_value: SymbolValue = SymbolValue::EnumValue { 
        variants: vec!["Variant1".to_string(), "Variant2".to_string()],
    };

    let enum_info: SymbolInfo = SymbolInfo::new(DataType::Enum, enum_value);
    global_scope.add("test_enum".to_string(), enum_info);
    expected_sts.push(global_scope);

    let enum_id: ASTNode = ASTNode::new(SyntaxElement::Identifier("test_enum".to_string()));

    let mut variant1: ASTNode = ASTNode::new(SyntaxElement::Variant);
    let mut variant2: ASTNode = ASTNode::new(SyntaxElement::Variant);
    let variant1_id: ASTNode = ASTNode::new(SyntaxElement::Identifier("Variant1".to_string()));
    let variant2_id: ASTNode = ASTNode::new(SyntaxElement::Identifier("Variant2".to_string()));
    variant1.add_child(variant1_id);
    variant2.add_child(variant2_id);

    let mut root: ASTNode = ASTNode::new(SyntaxElement::EnumDeclaration);
    root.add_child(enum_id);
    root.add_child(variant1);
    root.add_child(variant2);
    
    let ast: AST = AST::new(root);
    let result_sts_gen: Result<(AST, SymbolTableStack), Vec<ErrorType>> = SymbolTableStack::gen_sym_table_stack(ast);

    match result_sts_gen {
        Ok((_ast, sts)) => {
            assert_eq!(sts, expected_sts);
        },
        Err(e) => panic!("Couldn't parse AST, error: {:?}", e),
    }
}


#[test]
fn test_variable_initialization_sym_table() {
    let mut expected_sts: SymbolTableStack = SymbolTableStack::new();
    let mut global_scope: SymbolTable = SymbolTable::new();
    let var_value: SymbolValue = SymbolValue::StrValue("true".to_string().into()); 

    let var_info: SymbolInfo = SymbolInfo::new(DataType::Boolean, var_value);
    global_scope.add("test_var".to_string(), var_info);
    expected_sts.push(global_scope);

    let var_id: ASTNode = ASTNode::new(SyntaxElement::Identifier("test_var".to_string()));
    let var_type: ASTNode = ASTNode::new(SyntaxElement::Type(DataType::Boolean));
    let mut var_value: ASTNode = ASTNode::new(SyntaxElement::Variable);
    var_value.add_child(var_id);
    var_value.add_child(var_type);

    let mut assigned_value_node: ASTNode = ASTNode::new(SyntaxElement::AssignedValue);
    let assigned_value_literal: ASTNode = ASTNode::new(SyntaxElement::Literal("true".to_string()));
    assigned_value_node.add_child(assigned_value_literal);

    let mut root: ASTNode = ASTNode::new(SyntaxElement::Initialization);
    root.add_child(var_value);
    root.add_child(assigned_value_node);

    let ast: AST = AST::new(root);
    let result_sts_gen: Result<(AST, SymbolTableStack), Vec<ErrorType>> = SymbolTableStack::gen_sym_table_stack(ast);

    match result_sts_gen {
        Ok((_ast, sts)) => {
            assert_eq!(sts, expected_sts);
        },
        Err(e) => panic!("Couldn't parse AST, error: {:?}", e),
    }
}

