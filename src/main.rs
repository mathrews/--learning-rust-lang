const SECONDS_IN_MINUTE: u32 = 60;
const MINUTES_IN_HOUR: u32 = 60;
const SECONDS_IN_HOUR: u32 = SECONDS_IN_MINUTE * MINUTES_IN_HOUR;

fn main() {
    {
        let mut total: i32 = 20;
        println!("Trabalhou {} horas", total);
    
        total = 44;
        println!("Trabalhou {} horas", total);
    }
    let total = 30;
    {
        let total = 30 + total;
        println!("Trabalhou {} horas", total);
    } // escopo interno

    {
        let total = "quarenta e quatro";
        println!("Trabalhou {} horas", total);
    }

    {
        let total: u32 = 30;
        let total_in_seconds: u32 = total * SECONDS_IN_HOUR;
        println!("Trabalhou {} minutos", total_in_seconds);
    }
}
