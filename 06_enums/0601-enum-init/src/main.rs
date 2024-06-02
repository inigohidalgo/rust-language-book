fn enum_attached_data() {
    fn process_ip(ip: IpAddr) {
        match ip {
            IpAddr::V4(a) => {
                println!("First part of IPv4: {}", a[0])
            }
            IpAddr::V6(address) => {
                println!("IPv6 addr: {}", address)
            }
        }
    }
    enum IpAddr {
        V4([u8; 4]),
        V6(String),
    }
    let home = IpAddr::V4([127, 0, 0, 1]);
    let loopback = IpAddr::V6(String::from("::1"));
    
    fn ipv4_to_string(ip: IpAddr) -> String {
        if let IpAddr::V4(address_array) = ip {
            let address_array: [String; 4]  = address_array.map(|x| x.to_string());
            address_array.join(":")
        }
        else {String::from("")}
    }
    println!("ip4 as string: {}", ipv4_to_string(home))

}


fn catch_all_patterns(){
    fn add_fancy_hat() {println!("Adding hat")}
    fn remove_fancy_hat() {println!("Removing hat")}
    fn move_player(num_spaces: u8) {println!("Moving player {} spaces", num_spaces)}
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other), // this is a catch-all pattern which matches any other possible value dice_roll can take
                                     // it binds the value to `other` which is then passed into `move_player`
    }

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (), // _ is a special kwarg which does not bind to the matched value,
                 //it is a way of expressing we want to ignore the value,
                 // so rust will not raise an unused variable warning
    }

}

fn main() {
    enum_attached_data();
    catch_all_patterns();
}