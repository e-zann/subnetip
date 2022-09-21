#[allow(unused_imports)]
#[allow(non_camel_case_types)]

use std::fs::File;
use std::io::{self, prelude::*, BufRead};
use std::io::BufReader;
use std::io::SeekFrom;

use crate::cidrchklib::iphandler::{self, IPv4_as_binary, IPv4Cidr};

pub fn ip_from_files(ip_file: &str, subnet_file: &str) {

	let ip_list = File::open(&ip_file).unwrap();
	let subnet_list = File::open(&subnet_file).unwrap();

	let mut ip_array : Vec<String> = BufReader::new(&ip_list).lines().map(|x| x.unwrap()).collect();
	let start_count_ips = ip_array.len();

	for subnet_line in BufReader::new(&subnet_list).by_ref().lines() {
		
		let mut found  = false;
		let mut tmp_ip : String = String::new();
		let tmp_subnet = String::from(subnet_line.unwrap());
		let mut element = 0;

		for i in 0..(ip_array.len()) {

			let tmp_ip : String = String::from(&ip_array[i].clone().to_string());

			if ! &tmp_ip.starts_with(&tmp_subnet[0 as usize .. 2]) {
					continue;
			}

			let ipstruct   = iphandler::IPv4Struct::init_ip(&tmp_ip.trim());
			let cidrstruct = iphandler::IPv4CidrStruct::init_ip_cidr(&tmp_subnet.trim());

			if crate::cidrchklib::compute::check_a_pair(&ipstruct.ip_as_binary(), &cidrstruct.ipv4struct.ip_as_binary(), &cidrstruct.cidr) == true {
        		println!("IP : {:^50}\tis in Subnet :\t{:^50}/{}", &ipstruct, &cidrstruct.ipv4struct, &cidrstruct.cidr);
        		found = true;
        		element = i;
    		}
    	}
    	if found == true {
    		ip_array.remove(element);
    	}
	}

	let end_count_ips = ip_array.len();

	println!("\n\nStarted with an array of : {} IPs", &start_count_ips);
	println!("Cleaned list has now     : {} IPs\n\n", &end_count_ips);
}