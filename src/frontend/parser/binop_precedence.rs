/*
Defines the precedence of binary operations
 */

use std::collections::HashMap;

pub fn binop_precedence() -> &'static once_cell::sync::Lazy<HashMap<char, i32>> {
    static PRECEDENCE: once_cell::sync::Lazy<HashMap<char, i32>> = 
    once_cell::sync::Lazy::new(|| {
        let mut p = HashMap::new();
        p.insert('<', 10);
        p.insert('+', 20);
        p.insert('-', 20);
        p.insert('*', 40);
        p
    });
    &PRECEDENCE
}