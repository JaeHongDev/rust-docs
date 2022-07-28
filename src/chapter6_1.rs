enum IpAddrKind{
    V4(String),
    V6(String)
}

struct IpAddr{
    kind:IpAddrKind,
    address:String,
}
fn route(ip_types:IpAddrKind){

}
pub fn run(){
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr{
        kind:IpAddrKind::V4,
        address:String::from("127.0.0.1"),
    };
    let lookback = IpAddr{
        kind:IpAddrKind::V6,
        address:String::from("::1")
    };

}