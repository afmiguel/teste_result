use std::io;

fn main() {
    println!("Por favor, digite um número:");

    let mut entrada_usuario = String::new();
    io::stdin()
        .read_line(&mut entrada_usuario)
        .unwrap();  // Pode causar uma panique se a leitura da linha falhar

    let numero: i32 = entrada_usuario.trim().parse()
        .unwrap();  // Pode causar uma panique se a entrada não puder ser convertida para i32

    println!("Você digitou o número: {}", numero);
}
