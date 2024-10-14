use std::env;
use std::fs;
use std::io;
use std::path::Path;


fn list_dir(path: &Path) -> io::Result<()>{
    if path.is_dir(){ // идем по путю
        for entry in fs::read_dir(path)? { // читаем директорию и идем по файлам
            let entry = entry?;
            let file_name = entry.file_name();
            println!("{}", file_name.to_string_lossy());
        }
    } else {
        println!("{} isn't a dir", path.display()); // обратботка ошибки в случаем, если директория не существует
    }
    Ok(()) // конец цикла
}


fn main() -> io::Result<()>{
    let args: Vec<String> = env::args().collect();
    let path = if args.len() > 1{ // проверяем задал ли пользователь саму директорию, если нет, то будет запущена директория по умолчанию
        Path::new(&args[1])
    } else {
        Path::new(".")
    };
    list_dir(path)
}

