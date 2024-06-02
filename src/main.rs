use error_chain::error_chain;
use std::io::Read;
use ansi_term::Colour;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

// fn fibonacci(posicao_termo: i32){
//     if (posicao_termo == 0){
//         return 1;
//     }

//     return fibonacci(posicao_termo - 1) +  fibonacci(posicao_termo - 2);

// }

fn print_azul(text: &str) {
    // println!("Texto: {text}");
    println!("Texto: {}", Colour::Blue.paint(text));
}

fn teste_print() {
    println!("This is {} in color, {} in color and {} in color",
             Colour::Red.paint("red"),
             Colour::Blue.paint("blue"),
             Colour::Green.paint("green"));
}

fn main() -> Result<()> {
    let mut res = reqwest::blocking::get("http://httpbin.org/get")?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", body);
    print_azul("Ola pessoas!");
    Ok(())
}