
pub fn usage(binary: &str) {

	println!("\n{:_^80}\n", "CIDR Check");
	println!("\n\t{} -s --simple\t[IP]\t\t[subnet]", &binary);
	println!("\t{} -f --file\t[IP list file]\t[subnet list file]\n\n", &binary);

}