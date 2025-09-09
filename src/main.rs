use std::env;

#[derive(Debug)]
struct Sizes {
    bytes: String,
    kilobytes: String,
    megabytes: String,
    gigabytes: String,
}

enum Unit {
    Bytes,
    Kilobytes,
    Megabytes,
    Gigabytes,
}

impl Sizes {
    fn from(value: f64, unit: Unit) -> Self {
        // Convertimos todo a bytes primero
        let bytes = match unit {
            Unit::Bytes => value,
            Unit::Kilobytes => value * 1_000.0,
            Unit::Megabytes => value * 1_000_000.0,
            Unit::Gigabytes => value * 1_000_000_000.0,
        };

        // Calculamos todas las representaciones
        Sizes {
            bytes: format!("{} bytes", bytes as u64),
            kilobytes: format!("{} kilobytes", (bytes / 1_000.0) as u64),
            megabytes: format!("{} megabytes", (bytes / 1_000_000.0) as u64),
            gigabytes: format!("{} gigabytes", (bytes / 1_000_000_000.0) as u64),
        }
    }
}

fn main() {
    // Obtenemos los argumentos de consola
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Error: Debes proporcionar un tamaño y unidad. Ejemplo: cargo run \"24 mb\"");
        std::process::exit(1);
    }

    let input = &args[1];

    // Separamos el número y la unidad
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() != 2 {
        eprintln!("Error: Formato incorrecto. Usa algo como \"24 mb\" o \"300 kb\".");
        std::process::exit(1);
    }

    // Intentamos convertir el número
    let value: f64 = match parts[0].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: El primer valor debe ser un número válido.");
            std::process::exit(1);
        }
    };

    let unit_str = parts[1].to_lowercase();

    // Convertimos la unidad a nuestro enum
    let unit = match unit_str.as_str() {
        "b" | "bytes" => Unit::Bytes,
        "kb" => Unit::Kilobytes,
        "mb" => Unit::Megabytes,
        "gb" => Unit::Gigabytes,
        _ => {
            eprintln!("Error: Unidad no válida. Usa b, kb, mb o gb.");
            std::process::exit(1);
        }
    };

    // Creamos el struct con todas las representaciones
    let sizes = Sizes::from(value, unit);

    // Imprimimos en formato Debug
    println!("{:?}", sizes);
}
