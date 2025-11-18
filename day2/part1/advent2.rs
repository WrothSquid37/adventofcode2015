use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    
    let inputstring : String;
    let mut dimen: [i64; 3] = [0, 0, 0];
    let mut index: usize = 0;
    let mut surface_area : i64 = 0;
    let mut total_area_needed : i64 = 0;

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
  
    let split_string = inputstring.split("\n");

    for str in split_string {
        index = 0;

        let unparsed_dimen = str.split("x");

        for unparsednum in unparsed_dimen {
            dimen[index] = unparsednum.parse().unwrap();
            index += 1;
        }

        surface_area = calc_surface_area_with_slack(dimen[0], dimen[1], dimen[2]);

        println!("Side 1: {} Side 2: {} Side 3: {} | Surface Area: {}", dimen[0], dimen[1], dimen[2], surface_area);

        total_area_needed += surface_area;

    }

    println!("Total Needed to be ordered: {}", total_area_needed);

}

// Surface Area Formula for reference: 2*l*w + 2*w*h + 2*h*l

fn calc_surface_area_with_slack(l: i64, w: i64, h: i64) -> i64 {
    
    let mut smallest_area = 0; 

    let mut a_calc = l * w;
    let mut b_calc = w * h;
    let mut c_calc = h * l;

    smallest_area = a_calc;
    if smallest_area > b_calc {
        smallest_area = b_calc;
    }
    if smallest_area > c_calc {
        smallest_area = c_calc;
    }



    a_calc *= 2;
    b_calc *= 2;
    c_calc *= 2;

    
    

    return ( a_calc + b_calc + c_calc ) + smallest_area;
}