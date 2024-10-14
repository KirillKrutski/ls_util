use std::env;
use std::fs;
use std::io;
use std::path::Path;


fn list_dir(path: &Path) -> io::Result<()>{
    if path.is_dir(){
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let file_name = entry.file_name();
            println!("{}", file_name.to_string_lossy());
        }
    } else {
        println!("{} isn't a dir", path.display());
    }
    Ok(())
}


fn main() -> io::Result<()>{
    let args: Vec<String> = env::args().collect();
    let path = if args.len() > 1{
        Path::new(&args[1])
    } else {
        Path::new(".")
    };
    list_dir(path)
}

