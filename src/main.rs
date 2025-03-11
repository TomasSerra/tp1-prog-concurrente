use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("No se pudo iniciar el servidor");

    println!("Servidor corriendo en http://127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0; 1024];
                stream.read(&mut buffer).unwrap();

                let request = std::str::from_utf8(&buffer).unwrap();
                let response = handle_request(request);

                stream.write_all(response.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
            Err(e) => println!("Error de conexiÃ³n: {}", e),
        }
    }
}

fn leibniz_pi(iterations: u128) -> f64 {
    let mut pi = 0.0;
    for k in 0..iterations {
        let term = 4.0 * (-1.0_f64).powi(k as i32) / (2 * k + 1) as f64;
        pi += term;
    }
    pi
}

fn handle_request(request: &str) -> String {
    // Extrae la primera línea de la petición HTTP
    let first_line = request.lines().next().unwrap_or("");

    // Intenta extraer el número de la URL usando una expresión regular básica
    if let Some(num_str) = first_line.strip_prefix("GET /") {
        if let Some(num_str) = num_str.split_whitespace().next() {
            if let Ok(num) = num_str.parse::<u128>() {
                let resultado = leibniz_pi(num);
                return format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nEl resultado es: {}",
                    resultado
                );
            }
        }
    }

    // Si no se encontró un número válido, devolver un 400 Bad Request
    "HTTP/1.1 400 BAD REQUEST\r\nContent-Type: text/plain\r\n\r\nNúmero inválido.".to_string()
}