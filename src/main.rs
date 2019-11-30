use std::env;

fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();
    let mut nflag = false;

    if args[0] == "-n" {
        nflag = true;
        args.remove(0);
    }
    
    print!("{}", args.join(" "));

    if !nflag {
        println!("");
    }
}
