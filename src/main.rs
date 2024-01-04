use std::io;

fn main() {
    

    loop {
        println!("Enter you temprature in fahrenheit, for quit type q");
        let mut temp_fah = String::new();
    
        io::stdin()
            .read_line(&mut temp_fah)
            .expect("Failed to read line!");

        if temp_fah.trim() == "q" {
            break;
        } else {
            let temp_fah: i32 = match temp_fah.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            let temp_cel = (temp_fah - 32) * 5 / 9;
            println!("temp is {temp_cel}");
        }

        
    }
}
