use picoc089::{allocator, lexer, parser, selector, translator, typer};
use std::{env, fs, io::Write};

fn main() {
    println!(
        "
    ⠀⠀⠀⠀⠀⣼⣧⠀⠀⠀⠀⠀
    ⠀⠀⠀⠀⣼⣿⣿⣧⠀⠀⠀⠀
    ⠀⠀⠀⠾⠿⠿⠿⠿⠷⠀⠀⠀
    ⠀⠀⣼⣆⠀⠀⠀⠀⣰⣧⠀⠀
    ⠀⣼⣿⣿⣆⠀⠀⣰⣿⣿⣧⠀
    ⠾⠟⠿⠿⠿⠧⠼⠿⠿⠿⠻⠷
    picoc: aot optimizing C89 compiler
    "
    );

    let strat = env::args()
        .nth(1)
        .expect("picoc089-error: no strategy given");
    println!("picoc089-info: received strategy: {strat}");

    let src = env::args()
        .nth(2)
        .expect("picoc089-error: no source file given");
    println!("picoc089-info: received source: {src}");

    let chars = fs::read(src)
        .expect("picoc089-error: file dne`")
        .iter()
        .map(|b| *b as char)
        .collect::<Vec<_>>();
    let tokens = lexer::lex(&chars).unwrap();
    let src_tree = parser::parse_prg(&tokens).unwrap(); // recursive descent -> pratt parsing
    println!("picoc089-info: tree: {:?}", src_tree);

    let typ = typer::type_prg(&src_tree).unwrap();
    println!("picoc089-info: type: {:?}", typ);

    match strat.as_str() {
        // "interpretc0" => {
        //     let val = evaluator::eval_prg(&src_tree).unwrap();
        //     println!("picoc089-info: evaluated: {val}");
        // }
        "compilec89" => {
            let trgt_tree = translator::translate(&src_tree); // tree -> ssa -> son
                                                              // let abstract_assembly = selector::select(&trgt_tree).unwrap(); // maximal munch -> peephole
                                                              // let assembly = allocator::allocate(&abstract_assembly).unwrap(); // graph coloring -> linear-scan

            // let mut f = fs::File::create("./tmp.s").expect("picoc089-error: unable to create file");
            // f.write_all(assembly.join("\n").as_bytes())
            //     .expect("picoc089-error: unable to write data");
        }
        _ => {
            println!("picoc089-error: unknown strategy: {:?}", strat);
            std::process::exit(1);
        }
    }
}
