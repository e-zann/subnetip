use std::fmt;


#[derive(Debug,Clone,Copy)]
pub struct IPv4Struct {
    // Struct for IPv4 without CIDR notation.
    pub octed_0 : u8,
    pub octed_1 : u8,
    pub octed_2 : u8,
    pub octed_3 : u8,
}



#[derive(Debug,Clone,Copy)]
pub struct IPv4CidrStruct {
    // Struct for IPv4 with CIDR notation.
    pub ipv4struct : IPv4Struct,
    pub cidr       : u8,
}



// Handle IPv4 without CIDR notation.
impl IPv4Struct {

    pub fn init_ip(ip: &str) -> Self {
        
        let octeds : Vec<&str> = ip.split('.').collect();
        
        IPv4Struct {
            octed_0 : octeds[0].clone().parse::<u8>().unwrap(),
            octed_1 : octeds[1].clone().parse::<u8>().unwrap(),
            octed_2 : octeds[2].clone().parse::<u8>().unwrap(),
            octed_3 : octeds[3].clone().parse::<u8>().unwrap(),
        }
    }
}



// Handle IP/CIDR Types.
impl IPv4CidrStruct {

    pub fn init_ip_cidr(ip: &str) -> Self {
        
        let ip_cidr : Vec<&str> = ip.split('/').collect();
        
        IPv4CidrStruct {
            ipv4struct : IPv4Struct::init_ip(&ip_cidr[0]),
            cidr       : ip_cidr[1].clone().parse::<u8>().unwrap(),
        }
    }
}



// trait definitions for type IPv4_as_binary.
pub trait IPv4_as_binary {
    fn ip_as_binary (&self) -> String;
}

// trait definitions for type IPv4Cidr.
pub trait IPv4Cidr {
    fn cidr_block   (&self) -> String;
    fn net_host_bits(&self, ip_bin: &str) -> (String, String);
}



// the trait to convert IPs from dec to bin. 
impl IPv4_as_binary for IPv4Struct {

    // here we get the ip in binary format as a string string.
    fn ip_as_binary(&self) -> String {
        let ip_bin = format!("{:08b}{:08b}{:08b}{:08b}", self.octed_0, self.octed_1, self.octed_2, self.octed_3);
        ip_bin.to_string()
    }
}



impl IPv4Cidr for IPv4CidrStruct {

    fn cidr_block(&self) -> String {

        let mut cidr_bin_string : String = String::new();

        for n in 0 .. 32 {
            if n < self.cidr {
                cidr_bin_string.push_str("1");
            }
            else {
                cidr_bin_string.push_str("0");
            }
        } 
        cidr_bin_string
    }

    // function to get the host bits and the network bits
    // todo: 2 for loops... this is redundant code...
    fn net_host_bits(&self, bin_ip: &str) -> (String, String) {

        let mut net_bits  : String = String::new();
        let mut host_bits : String = String::new();

        for n in 0 .. 32 {
            if n < self.cidr {
                net_bits.push_str(&bin_ip.chars().nth(n.into()).unwrap().to_string());
            }
            else if n == self.cidr {
                net_bits.push_str(".0");
                
            }
            else {
                net_bits.push_str("0");
            }
        }
        for x in 0 .. 32 {
            if x < self.cidr {
                host_bits.push_str("0");
            }
            else if x == self.cidr {
                host_bits.push_str(".1");
            }
            else {
                host_bits.push_str("1");
            }
        }
        (net_bits.to_string(), host_bits.to_string())
    }
}


impl fmt::Display for IPv4Struct {
    // the fmt trait to print an ip.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}.{}", self.octed_0, self.octed_1, self.octed_2, self.octed_3)
    }

}   