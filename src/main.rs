use std::string::String;

mod memory;
mod z3;

fn main() {

    //let smtfile = "./assets/small-test.smt";
    let smtfile = "./assets/unsat-test.smt";

    match z3::z3::solve(String::from(smtfile)) {
        z3::z3::SMTResults::SAT => {
            println!("sat");
        }
        z3::z3::SMTResults::UNSAT => {
            println!("unsat");
        }
        z3::z3::SMTResults::UNDEF => {
            println!("undef");
        }
    }
}
