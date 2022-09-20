
// std lib
use std::env;

//cidrchk lib
use cidrchk::cidrchklib::iphandler;
use cidrchk::cidrchklib::iphandler::IPv4_as_binary;

fn main() {
    
    let args : Vec<String> = env::args().collect();

    if args.len() != 3 {
        panic!("File_1 and File_2 please....");
    }

    //println!("\nFile_1: {}\nFile_2: {}\n", &args[1], &args[2]);

    // first arg ipv4 without cidr
    let ipstruct = iphandler::IPv4Struct::init_ip(&args[1]);

    // first arg ipv4 without cidr
    let ipscidrtruct = iphandler::IPv4CidrStruct::init_ip_cidr(&args[2]);


    println!("Ip without CIDR\n{:#?}\n\nIP with Cidr:\n{:#?}", &ipstruct, &ipscidrtruct); 

    println!("IP in bin: {}", ipstruct.ip_as_binary());

}
