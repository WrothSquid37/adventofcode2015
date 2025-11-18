use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    
    let inputstring : String;
    let mut dimen: [u32; 3] = [0, 0, 0];
    let mut smallest_index: usize = 0;
    let mut total: u32 = 0;
    let mut index: usize = 0;
    let mut index_already_used = 10;
    let mut countindex = 0;

    // Create a path to the desired file
    let path = Path::new("input.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    // This checks when opening the file and throws an error if not succeded 
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => inputstring = s,
    }

    // Split all of the strings into a line by basis and iterate through them
    for str in inputstring.split("\n") {

        dimen[0] = 0;
        dimen[1] = 1;
        dimen[2] = 2;

        let mut volume: u32 = 0;
        let mut perrim: u32 = 0;
        let mut first_lowest: u32 = 0;
        let mut second_lowest: u32 = 0;
        let mut current_num: u32 = 0;

        // Count the index for each dimension. reset it here
        index = 0;

        // Iterate through the dimensions spliting on the 'x' and parse the numbers
        for unparsednum in str.split("x") {
            dimen[index] = unparsednum.parse().unwrap();
            index += 1;
        }

        current_num = 9999;
        for d in 0..3 {
            if current_num > dimen[d] {
                current_num = dimen[d];
                index_already_used = d;
            }            
        }

        first_lowest = current_num;
        current_num = 9999;

        for f in 0..3 {
            if current_num >= dimen[f] && f != index_already_used  {
                current_num = dimen[f];
            } 
        }

        second_lowest = current_num;

        if dimen[0] <= 5 && dimen[1] <= 5 {
            println!("First Found: {} | Second Found: {}", first_lowest, second_lowest);
        }

        perrim = first_lowest + first_lowest + second_lowest + second_lowest;

        // Calcuate the amount of extra needed via the volume
        volume = dimen[0] * dimen[1] * dimen[2];
        
        total += volume + perrim;

        println!("Volume: {} Perrim: {} | Running Total: {}", volume, perrim, total);

    }

}
