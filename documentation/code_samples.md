# {Insert name of programming lanaguage}
    - C like language
    - Top level expressions:
        - Structs
        - Enums
        - Functions
        - Imports
    - Other expressions:
        - if/elif/else
        - initialization/assignment of variables
        - match
        - while 
        - for 
        - do-while 
    - Ideas
        - Memory safety without garbage collection
        - Null safety


```rust
pub fn foo(a: int, b: bool, c: str): bool {
    if (a == 0) {
        bar();
        return false;
    }
    elif (a == 1) {
        while (true) {
            break;
        }
    }
    else {
        match c {
            "abc" => {

            }
            "a" => {

            }
            _ => {}
        }
    }
    true
}
fn bar() {
    let mut x = 10;
    x = 15;
}
```

    - Could be interesting:
        - Structs having custom pass-by traits
            -ie: 
                ```rust
                    [#traits{pass-by: value}]
                    struct foo {}
                ```
        - Inherit block (can only be inherited, not initialized)
            -ie:
                ```rust
                    inherit foo {
                        let el = "abc"
                    }
                    #[traits{pass-by: ownership, inherits: foo}]
                    struct bar {}
                    bar.el = "abc";
                ```
        - Polymorphism
            -ie:
                ```rust
                    struct foo {
                        el: str,
                    }
                    struct foo {
                        el: str,
                        next_el: str,
                    }
                ```
        - Store functions in variabes, passed as arguments, returned form other functions
        - Variables immutable by default, unless specified as mutable
