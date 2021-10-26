use std::env;

fn main() {
    let args: Vec<String> = env::args().into_iter().collect();
    if args.contains(&"-help".to_string()) || args.contains(&"-h".to_string()) {
        println!("Press -help or -h to show this menu")
    } else if args.contains(&"-coffee".to_string()) {
        let num = match &args.len() {
            3.. => args[2].to_owned() + " cups",
            _ => "".to_string(),
        };
        println!("Making coffee: {}", num);
    } else {
        println!("Invalid arguments")
    }
    println!("Exiting...")
}
