use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, author = "Crispr", about = "A simple cli calculator")]

struct Args{
    operation: char,
    value1: i32,
    value2: i32,
}

fn main(){
    let args = Args::parse();
    let a = args.value1;
    let x = args.value2;

    match args.operation {
        'a' => addition(a, &x),
        _ => println!("missing operation")
    }
}

fn addition(a: i32, x: &i32){
    let output = a + x;
    println!("{}", output)
}
