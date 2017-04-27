mod asg;

mod test_grammar {
    //include!(concat!(env!("OUT_DIR"), "/test_grammar.rs"));
    include!("../temp/test_grammar.rs");
}

fn main() {
    match test_grammar::expression("314") {
        Ok(r) => println!("Parsed as: {:?}", r),
        Err(e) => println!("Parse error: {}", e),
    }
    
    match test_grammar::expression("3.14") {
        Ok(r) => println!("Parsed as: {:?}", r),
        Err(e) => println!("Parse error: {}", e),
    }
}
