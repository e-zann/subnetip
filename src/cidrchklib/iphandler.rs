
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

    /*  end of impl */ 
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

    /*  end of impl */ 
}


pub trait IPv4_as_binary {

    fn ip_as_binary(&self) -> String;

}

// the trait to handle IPs in dec to bin. 
impl IPv4_as_binary for IPv4Struct {
    
    fn ip_as_binary(&self) -> String {
        let ip_bin = format!("{:08b}{:08b}{:08b}{:08b}", self.octed_0, self.octed_1, self.octed_2, self.octed_3);
        ip_bin.to_string()
    }

}
