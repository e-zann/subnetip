
// here we check if the given ip is in the given subnet or not.
// at the first mismatch, return false fin...
pub fn check_a_pair(test_ip: &str, subnet_ip: &str, cidr_length: &u8) -> bool {

	for n in 0 .. *cidr_length {

		if test_ip.chars().nth(n.into()).unwrap() != subnet_ip.chars().nth(n.into()).unwrap() {

			return false;

		}
	}

	return true
}
