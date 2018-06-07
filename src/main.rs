extern crate modscript;

use modscript::*;
use std::env;
use std::io;


fn main() {
    // get args, compile into libs and attach
    let mut fns = FuncMap::new();
    let mut packs = Vec::new();

    for a in env::args().skip(1) {
        let p = match package_from_file(&a) {
            Ok(p) => p,
            Err(e) => panic!("Package {}: Compile error: {}", a, e),
        };

        fns.attach_package(&a, p.call_ref());
        packs.push(("pack".to_string(), a.clone()));

        println!("Attached package {}", a);
    }
    
    println!("Modscript");
    
    let mut state = Scope::new();

    loop {
        // accept a line of input
        // parse and run
        // print output
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(0) => break,
            Err(e) => panic!("Stdin error: {}", e),
            _ => {},
        }

        if !input.ends_with(";\n") {
            input = format!("return {};", input);
        }

        let s = match script_from_text(&packs, &input) {
            Ok(s) => s,
            Err(e) => {eprintln!("Compile error: {}", e); continue},
        };

        match s.repl_run(&mut state, &fns) {
            Signal::Done => {},
            Signal::Return(v) => println!("> {}", v),
            Signal::Error(e) => eprintln!("Runtime error: {}", e),
            Signal::Continue => eprintln!("Runtime error: Continue not allowed."),
            Signal::Break => eprintln!("Runtime error: Break not allowed."),
        }
    }
}
