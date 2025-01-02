use rand::prelude::*;

fn main() {
    // Classy message
    println!("Hello algorithms!");

    /* Variables (int, float, string) */

    let one_signed_integer8: i8 = -1; /* (2^8)-1 - Zero sum */
    let one_signed_int32: i32 = 2;

    let one_unsigned_i8: u8 = 1; /* (2^8)-1 - Positive sum */

    let is_active = true;

    let one_slice = if is_active { "running" } else { "stopped" };

    println!("With {one_signed_integer8}.{one_signed_int32} and {one_unsigned_i8} {one_slice}");

    let one_string = String::from("Be rusty!");

    println!("Growable string. Just {one_string}");

    // Ciclos

    for element in [1, 2, 3, 4] {
        println!("item {:?}", element);
    }

    let mut condition: bool;
    let mut rng = rand::thread_rng();
    loop {
        condition = rng.gen();
        print!("looping! ");
        if condition {
            break;
        }

        // Random number and character
        let rnd_num: i32 = rng.gen_range(1..=100);
        let rnd_char = rng.gen_range('A'..='Z');
    }

    // Error handling

    match ratio(17.0, 2.5) {
        Ok(res) => println!("Ratio result {res}"),
        Err(msg) => println!("Error: {msg}"),
    }
    
}

fn ratio(num: f32, den: f32) -> Result<f32, String> {
    if den == 0.0 {
        Err("Indetermined".to_string())
    } else {
        Ok(num / den)
    }
}
