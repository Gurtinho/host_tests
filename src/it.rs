use std::process::{Command, Output};

pub struct Router {
  pub ip: String,
}

pub trait Network {
  fn ping(&self, host: &str) -> bool;
  fn traceroute(&self, host: &str) -> Vec<String>;
  fn nslookup(&self, host: &str) -> String;
}

impl Network for Router {
  fn ping(&self, host: &str) -> bool {
    // Esse eu consegui fazer a lógica do ping
    println!("Disparando pacotes para: {} com o ip: {}", host, self.ip);
    let ping_test = Command::new("ping")
        .args(["-n", "4"])
        .arg(host)
        .output()
        .expect("falha ao executar o processo");
    match ping_test {
      Output { status, .. } => {
        if status.success() {
          return true;
        } else {
          return false;
        }
      }
    }
  }

  fn traceroute(&self, host: &str) -> Vec<String> {
    println!("Rastreando a rota para: {} com o ip: {}", host, self.ip);
    let trace_test = Command::new("tracert")
    .args(["-h", "3"])
    .arg(host)
    .output()
    .expect("Erro ao rodar o rastreamento");
    let trace_data = String::from_utf8_lossy(&trace_test.stdout);
    let mut address_array = Vec::new();
    for address in trace_data.lines() {
      if let Some(_) = address.split_whitespace().last() {
        address_array.push(address);
      }
    }
    let mut ips: Vec<String> = Vec::new();
    for address_line in &mut address_array[2..=4] {
      if let Some(ip) = address_line.split_whitespace().last() {
        ips.push(ip.to_string());
      }
    }
    ips
  }

  fn nslookup(&self, host: &str) -> String {
    println!("Consultando DNS: {}", host);
    let dns_test = Command::new("nslookup")
      .arg(host)
      .output()
      .expect("falha ao executar o processo");
    let addresses = String::from_utf8_lossy(&dns_test.stdout);
    let mut ip_address = String::new();
    for address in addresses.lines() {
      if address.contains("Address:") {
        ip_address = address.split_whitespace().nth(1).unwrap().to_string();
      }
    }
    ip_address
  }
}