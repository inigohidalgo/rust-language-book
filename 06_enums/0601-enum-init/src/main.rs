fn enum_attached_data() {
    fn process_ip(ip: IpAddr) {
        match ip {
            IpAddr::V4(a) => {
                println!("First part of IPv4: {}", a.1)
            }
            IpAddr::V6(address) => {
                println!("IPv6 addr: {}", address)
            }
        }
    }
    enum IpAddr {
        V4((u8, u8, u8, u8)),
        V6(String),
    }
    let home = IpAddr::V4((127, 0, 0, 1));
    let loopback = IpAddr::V6(String::from("::1"));
    process_ip(home);
    process_ip(loopback);

}



fn main() {
    enum_attached_data();
}