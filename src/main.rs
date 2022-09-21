
// std lib
use std::env;
use std::process;

use ansi_term::Colour::{Red, Green};

use cidrchk::cidrchklib::iphandler::{self, IPv4_as_binary, IPv4Cidr};
use cidrchk::utils::help_funcs;

fn main() {
    
    let args : Vec<String> = env::args().collect();


    if &args[1] == "-f" || &args[1] == "--file" {
        println!("{}", &args[1]);
        cidrchk::cidrchklib::ip_files::ip_from_files(&args[2], &args[3]);
    }

    else {

        // first arg ipv4 without cidr
        let ipstruct = iphandler::IPv4Struct::init_ip(&args[1]);

        // first arg ipv4 without cidr
        let ipscidrtruct = iphandler::IPv4CidrStruct::init_ip_cidr(&args[2]);

        println!("\n\n{:^25} : {:^35} : {:^35}" , "IP to Check", &args[1],           &ipstruct.ip_as_binary());
        println!("{:^25} : {:^35} : {:^35}"     , "Subnet",      &args[2],           &ipscidrtruct.ipv4struct.ip_as_binary());
        println!("{:^25} : {:^35} : {:^35}\n"   , "CIDR",        &ipscidrtruct.cidr, &ipscidrtruct.cidr_block());


        let netbits_raw : Vec<String> = ipscidrtruct.net_host_bits(&ipscidrtruct.ipv4struct.ip_as_binary()).0.split('.').map(|s| s.to_owned()).collect();
        let netbits = format!("{}.{}", Green.paint(&netbits_raw[0]), (Red.paint(&netbits_raw[1]))); 
    
        let hstbits_raw : Vec<String> = ipscidrtruct.net_host_bits(&ipscidrtruct.ipv4struct.ip_as_binary()).1.clone().split('.').map(|s| s.to_owned()).collect();
        let hstbits = format!("{}.{}", Red.paint(&hstbits_raw[0]), (Green.paint(&hstbits_raw[1])));

        println!("{:^25} : {:^35}"     , "Network Bits", &netbits);
        println!("{:^25} : {:^35}\n\n" , "Host Bits",    &hstbits);

        println!("\n[ !! ] - Checking IP...\n");

        // lets check the ip against the subnet

        if cidrchk::cidrchklib::compute::check_a_pair(&ipstruct.ip_as_binary(), &ipscidrtruct.ipv4struct.ip_as_binary(), &ipscidrtruct.cidr) {
            println!("IP: {} is in Subnet: {}\n\n", &args[1], &args[2]);
        }
        else {
            println!("IP: {} is NOT in Subnet: {}\n\n", &args[1], &args[2]);
        }
    }
}
