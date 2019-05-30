mod position;
mod source;
mod token;
mod tokenizer;

fn main() {

    let fname = std::env::args().nth(1).expect("Expecting a file name to parse !");
    let mut src = source::Source::from_file(&fname)
                .unwrap_or_else(|_| panic!("File not found!"));
    let ts = tokenizer::TokenStream::new(&mut src);
    for t in ts {
        println!("{:?}",t);
    }
}
