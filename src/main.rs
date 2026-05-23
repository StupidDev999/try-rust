use extabs::Expand;
use std::env;

fn main() {
    let s = String::from("Hewwo\tWorld!");
    let expanded = s.expandtabs(4);

    let args: Vec<String> = env::args().collect();

    let mut errorArgs = false;
    for arg in &args {
        if arg == "-h" || arg == "--help" {
            println!("Help prompt!");
            return;
        }
        else if arg == "-C" || arg == "--disable-colour" {
            println!("Disabled Colours!");
        }
        else if arg == "-w" || arg == "--enable-warn" {
            println!("Enabled Warning");
        }
        else if arg == "-v0" {
            println!("Verbosity 0");
        }
        else if arg == "-v1" {
            println!("Verbosity 1");
        }
        else if arg == "-v2" {
            println!("Verbosity 2");
        }
        else if arg == "-v3" {
            println!("Verbosity 3");
        }
        else {
            errorArgs = true;
        }
    }

    if errorArgs {
        println!("Unknown/Erroneous arguments found, ignored them.");
    }

    println!("{}", expanded)
}
