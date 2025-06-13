const HOUR_IN_SECONDS: u32 = 60 * 60;

fn main() {
    let mut hours = 1;

    loop {        
        if hours == 11 {
            break;
        }

        let seconds = hours * HOUR_IN_SECONDS;
        println!("It is {seconds} seconds in {hours} hour(s)!");
        hours += 1;
    }

}
