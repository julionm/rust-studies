use proc_macro;

#[macro_export]
macro_rules! vec {
    ( $( $x:expr ), * ) => {
        let mut temp_vec = Vec::new();
        $(
            temp_vec.push($x);
        )*
        temp_vec
    }
}

// * Procedural Macros acts like functions
// * they accept some code as an input, operate on that code
// * and produce some code as an output

#[some_attribute] // * the kind of proc macro that will be created
pub fn some_name(input: TokenStream) -> TokenStream {

}

