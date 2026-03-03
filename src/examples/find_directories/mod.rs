use std::fs;

pub fn find_direrctories(directory: String) {
    let directories = fs::read_dir(&directory);

    match directories {
        Ok(dirs) => {
            for dir in dirs {
                match dir {
                    Ok(directory) => {
                        let folder = directory;
                        let path = folder.path();

                        if path.is_file() {
                            println!("{}", path.display());
                        } else if path.is_dir() {
                            println!("{}", path.display());

                            match path.to_str() {
                                Some(pathname) => find_direrctories(pathname.to_string()),
                                None => println!("Can't open the pathname."),
                            };
                        };
                    }
                    Err(err) => println!("Error al leer ruta: {}", err),
                };
            }
        },
        Err(err) => println!("Error: {}", err)
    }
}
