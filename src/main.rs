
// std lib
use std::env;

//cidrchk lib
use cidrchk::cidrchklib::iphandler;

use cidrchk::cidrchklib::iphandler::IPv4_as_binary;
use cidrchk::cidrchklib::iphandler::IPv4Cidr;

fn main() {
    
    let args : Vec<String> = env::args().collect();

    if args.len() != 3 {
        panic!("File_1 and File_2 please....");
    }

    // first arg ipv4 without cidr
    let ipstruct = iphandler::IPv4Struct::init_ip(&args[1]);

    // first arg ipv4 without cidr
    let ipscidrtruct = iphandler::IPv4CidrStruct::init_ip_cidr(&args[2]);

    println!("\n\n{:^25} : {:^35} : {:^35}" , "IP to Check", &args[1],           &ipstruct.ip_as_binary());
    println!("{:^25} : {:^35} : {:^35}"     , "Subnet",      &args[2],           &ipscidrtruct.ipv4struct.ip_as_binary());
    println!("{:^25} : {:^35} : {:^35}\n"   , "CIDR",        &ipscidrtruct.cidr, &ipscidrtruct.cidr_block());

    println!("{:^25} : {:^35}"     , "Network Bits", &ipscidrtruct.net_host_bits(&ipscidrtruct.ipv4struct.ip_as_binary()).0);
    println!("{:^25} : {:^35}\n\n" , "Host Bits",    &ipscidrtruct.net_host_bits(&ipscidrtruct.ipv4struct.ip_as_binary()).1);

}
