// const SECONDS_IN_MINUTE: u32 = 60;
// const MINUTES_IN_HOUR: u32 = 60;
// const SECONDS_IN_HOUR: u32 = SECONDS_IN_MINUTE * MINUTES_IN_HOUR;

// fn main() {
//     {
//         let mut total: i32 = 20;
//         println!("Trabalhou {} horas", total);

//         total = 44;
//         println!("Trabalhou {} horas", total);
//     }
//     let total = 30;
//     {
//         let total = 30 + total;
//         println!("Trabalhou {} horas", total);
//     } // escopo interno

//     {
//         let total = "quarenta e quatro";
//         println!("Trabalhou {} horas", total);
//     }

//     {
//         let total: u32 = 30;
//         let total_in_seconds: u32 = total * SECONDS_IN_HOUR;
//         println!("Trabalhou {} minutos", total_in_seconds);
//     }
// }

fn main() {
    sum(10.0, 40.0);
    sub(10.0, 5.0);
    mult(1.5, 50.0);
    div(57.0, 3.0);
    raiz_quad(12.0);
    potencia(10.0, 2.0)
}

fn sum(a: f32, b: f32) {
    let sum = a + b;
    println!("{}", sum);
}

fn sub(a: f32, b: f32) {
    let sub = a - b;
    println!("{}", sub);
}

fn mult(a: f32, b: f32) {
    let mult = a * b;
    println!("{}", mult);
}

fn div(a: f32, b: f32) {
    let div = a / b;
    println!("{}", div);
}

fn raiz_quad(a: f32) {
    let raiz_quad = a * a;
    println!("{}", raiz_quad);
}

fn potencia(x: f32, pot: f32) {
    let mut i: f32 = 1.0;
    let mut res: f32 = x;
    while i < pot {
        res = res * x;
        i += 1.0;
    }
    println!("{}", res);
}
