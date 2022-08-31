use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, author = "Crispr", about = "A simple cli calculator")]

struct Args{
    value1: f32,
    operation: char,
    value2: f32,
}

// main funktion
fn main(){
    let args = Args::parse();
    let a = args.value1;
    let x = args.value2;

    match args.operation {
        '+' => addition(a, &x),
        '-' => division(a, &x),
        '*' => multiplication(a, &x),
        '/' => subtraction(a, &x),
        _ => println!("missing operation")
    }
}

fn addition(a: f32, x: &f32){
    let result = a + x;
    println!("{}", result)
}

fn division(a: f32, x: &f32){
    let result = a - x;
    println!("{}", result)
}

fn multiplication(a: f32, x: &f32){
    let result = a * x;
    println!("{}", result)
}

fn subtraction(a: f32, x: &f32){
    let result = a / x;
    println!("{}", result)
}
