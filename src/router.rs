use std::collections::HashMap;

pub enum Packet {
    Data(String),
    Control(u32),
    Ping,
}

#[derive(Debug, PartialEq)]
pub enum SecurityStatus {
    Approved,
    Malicious,
} 

pub trait LogPacket {
    fn log_payload(&self)->Result<String,String>;
}

impl LogPacket for Packet {
    fn log_payload(&self)->Result<String,String> {
        match self {
            Packet::Data(text)=> {
                if text.is_empty() {
                    Err("Data is empty".to_string())
                }
                else {
                    Ok(text.to_string())
                }
                    
            },
            Packet::Control(value)=> {
                Ok(format!("control value is {}",value))
            },
            Packet::Ping => Ok("Ok".to_string()),
        }
    }
}

pub trait Firewall {
    fn inspect_packet(&self, packet: &Packet)->SecurityStatus;
}

pub struct Router {
    pub routing_table:HashMap<String, Vec<Packet>>
}

impl Router {
    pub fn new()->Self {
        Self { 
            routing_table: HashMap::new(),
        }
    }

    pub fn route_packet(&mut self, destrination:String, packet:Packet,firewall: &impl Firewall )->Result<(),String> {
        let eval = firewall.inspect_packet(&packet);

        match eval {
            SecurityStatus::Malicious =>{
                Err("Packet is blocked".to_string())
            },
            SecurityStatus::Approved =>{
                //self.routing_table.insert(destrination, vec![packet]);
                self.routing_table.entry(destrination).or_insert(Vec::new()).push(packet);

                Ok(())
            }
        }
    }
}

struct BasicFirewall;

impl Firewall for BasicFirewall {
    fn inspect_packet(&self, packet: &Packet)->SecurityStatus {
        match packet {
            Packet::Data(text)=>{
                if text.contains("MALWARE") {
                    SecurityStatus::Malicious
                }
                else {
                    SecurityStatus::Approved
                }
            },
            _ => SecurityStatus::Approved
        }
    }
}
fn main() {
    let mut router = Router::new();

    let firewall = BasicFirewall;

    // Test 1: Route a valid Data packet
    let valid_packet = Packet::Data("Secure encrypted payload".to_string());
    println!("\n[Test 1] Routing safe data...");
    match router.route_packet("192.168.1.1".to_string(), valid_packet, &firewall) {
        Ok(_) => println!("Success: Packet successfully routed to destination queue."),
        Err(e) => println!("Error: {}", e),
    }

    // Test 2: Route a Malicious Data packet (Should be blocked by your firewall!)
    let bad_packet = Packet::Data("CRITICAL_ERR: MALWARE_DETECTED".to_string());
    println!("\n[Test 2] Routing malicious data...");
    match router.route_packet("192.168.1.1".to_string(), bad_packet, &firewall) {
        Ok(_) => println!("Success: Packet routed (This is a bug!)"),
        Err(e) => println!("Blocked by Firewall: {}", e),
    }

    // Test 3: Log payloads using your trait methods
    println!("\n--- Testing Trait Extraction Logs ---");
    if let Some(queue) = router.routing_table.get("192.168.1.1") {
        for packet in queue {
            match packet.log_payload() {
                Ok(log) => println!("Audit Log -> {}", log),
                Err(err) => println!("Audit Alert -> Formatting error: {}", err),
            }
        }
    }
}