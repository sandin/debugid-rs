use std::process;
use std::path::Path;
use clap::Parser;
use symbolic::common::ByteView;
use symbolic::debuginfo::Object;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the input file
    #[arg()]
    input: String
}

fn main() {
    let args = Args::parse();
    let input = Path::new(&args.input);
    if !input.exists() {
        println!("`{}` file is not exists!", args.input);
        process::exit(-1); 
    }

    let buf = ByteView::open(&args.input).unwrap();
    let object = match Object::parse(&buf) {
        Ok(o) => o,
        Err(..) => {
            eprintln!("Can not parse the input file as object file");
            process::exit(-1); 
        }
    };

    println!("FileFormat: {}", object.file_format().name());
    println!("Arch: {}", object.arch());
    println!("HasSymbolTable: {}", object.has_symbols());
    println!("HasDebugInfo: {}", object.has_debug_info());
    println!("HasUnwindInfo: {}", object.has_unwind_info());
    println!("BuildId: {}", object.code_id().unwrap());
    println!("BreakpadUUID: {}", object.debug_id().breakpad());

    match object {
        Object::Pe(ref o) => {
            println!("PdbFileName: {}", o.debug_file_name().unwrap());
        },
        _ => {
            // pass
        }
    }
}


