


fn main() {

    let mut test_num: i32 = 0;

    let secretcode = "yzbqklnj";

    let mut digest;

    let mut formatted_digest_string: String;

    
    loop {

        test_num += 1;
        
        formatted_digest_string = format!("{}{}", secretcode, test_num);

        digest = md5::compute(formatted_digest_string);         

        //println!("Digest Generated: {:x}", digest);


        if (format!("{:x}", digest)).starts_with("000000") {
            break;
        }

    }

    println!("{}", test_num);

}
