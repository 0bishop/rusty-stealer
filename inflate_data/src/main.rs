fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <path to file>", args[0]);
        std::process::exit(1);
    }

    let data = std::fs::read(&args[1]);
    match data {
        Ok(bytes) => {
            let inflated_data = inflate::inflate_bytes(&bytes);
            match inflated_data {
                Ok(inflated_bytes) => {
                    if std::fs::write("inflated_data.txt", &inflated_bytes).is_err() {
                        println!("Error writing inflated data to file");
                        std::process::exit(1);
                    }
                }
                Err(e) => {
                    println!("Error inflating data: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            println!("Error reading file: {}", e);
            std::process::exit(1);
        }
    }
    println!("Successfully inflated data to inflated_data.txt");
}

