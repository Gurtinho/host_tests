mod it;
use it::*;

fn main() {
    let router = Router {
        ip: "192.168.1.1".to_string(),
    };

    let result_ping = router.ping("www.google.com");
    println!("resultado do ping: {}\n\n", result_ping);

    let result_trace = router.traceroute("www.facebook.com");
    println!("Endereços IP encontrados: {:?}\n\n", result_trace);

    let result_dns = router.nslookup("www.facebook.com");
    println!("Endereço DNS: {}\n\n", result_dns);
}
