mod position;
mod source;
mod token;
mod tokenizer;
mod ast;
mod error;

fn main() {

    let fname = std::env::args().nth(1).expect("Expecting a file name to parse !");
    let mut src = source::Source::from_file(&fname)
                .unwrap_or_else(|_| panic!("File not found!"));
    let mut ts = tokenizer::TokenStream::new(&mut src);
    let mut ast = ast::Ast::new();
    ast.build(&mut ts).unwrap_or_else(|e| println!("{:?}", e));
}
