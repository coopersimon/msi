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
        //packs.push(("pack".to_string(), a.clone()));

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

        // Sanitise input
        if input.starts_with("import") {
            let v = input.split(char::is_whitespace).collect::<Vec<&str>>();

            if v.len() == 3 {
                packs.push((v[1].to_string(), v[1].to_string()));
            } else if v.len() == 5 && v[2] == "as" {
                packs.push((v[3].to_string(), v[1].to_string()));
            } else {
                eprintln!("Runtime error: Invalid import statement.");
            }

            continue;
        }

        if !input.ends_with(";\n") {
            input = format!("return {};", input);
        }

        let s = match script_from_text(&packs, &input) {
            Ok(s) => s,
            Err(e) => {eprintln!("{}", e); continue},
        };

        match s.repl_run(&mut state, &fns) {
            Signal::Done => {},
            Signal::Return(v) => println!("> {}", v),
            Signal::Error(e) => eprintln!("{}", e),
            Signal::Continue => eprintln!("Continue not allowed."),
            Signal::Break => eprintln!("Break not allowed."),
        }
    }
}
