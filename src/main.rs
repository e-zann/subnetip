
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

    // first arg ipv4 without cidr
    let ipstruct = iphandler::IPv4Struct::init_ip(&args[1]);

    // first arg ipv4 without cidr
    let ipscidrtruct = iphandler::IPv4CidrStruct::init_ip_cidr(&args[2]);

    println!("\n\n{:^30} : {:^20} : {:^40}", "First IP",  &args[1], ipstruct.ip_as_binary());
    println!("{:^30} : {:^20} : {:^40}", "Second IP", &args[2], ipscidrtruct.ipv4struct.ip_as_binary());
    println!("{:^30} : {:^20} : {:^40}\n\n", "CIDR",      &ipscidrtruct.cidr, "t.b.c");


}
