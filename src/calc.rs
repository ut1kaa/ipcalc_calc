#[derive(Debug)]
pub enum NetworkClass{
    A,
    B,
    C,
    D
}

impl NetworkClass {
    fn mask(&self) -> [u8; 4] {
        match self {
            NetworkClass::A => [255, 0, 0, 0],
            NetworkClass::B => [255, 255, 0, 0],
            NetworkClass::C => [255, 255, 255, 0],
            NetworkClass::D => [255, 255, 255, 255],
        }
    }
}


impl ToString for NetworkClass {
    fn to_string(&self) -> String {
        match self {
            NetworkClass::A => "A".to_string(),
            NetworkClass::B => "B".to_string(),
            NetworkClass::C => "C".to_string(),
            NetworkClass::D => "D".to_string(),
        }
    }
}


pub fn ip_class(ip_first_byte: &u8) -> NetworkClass {
    match ip_first_byte {
        1..=126 => NetworkClass::A,
        127..=191 => NetworkClass::B,
        192..=223 => NetworkClass::C,
        224..=239 => NetworkClass::D,
        _ => panic!("Invalid IP address")
    }
}


pub fn network_adress(ip: &[u8; 4]) -> [u8; 4] {
    let mut network_adress: [u8; 4] = [0; 4];
    let mask = ip_class(&ip[0]).mask();

    for i in ip.iter().enumerate(){
        if mask[i.0] != 0 {
            network_adress[i.0] = *i.1;
        } else {
            network_adress[i.0] = 0
        }
    };

    network_adress
}

pub fn undernetwork_adress(full_mask: &[u8; 4], ip: &[u8; 4]) -> [u8; 4] {
    let mut undernetwork_adress: [u8; 4] = [0; 4];
    let mask = ip_class(&ip[0]).mask();
    
    for i in ip.iter().enumerate() {
        match mask[i.0] {
            255 => {
                match full_mask[i.0] {
                    255 => undernetwork_adress[i.0] = *i.1,
                    _ => undernetwork_adress[i.0] = i.1 & full_mask[i.0],
                }
            }
            _ => {
                match full_mask[i.0] {
                    255 => undernetwork_adress[i.0] = i.1 & full_mask[i.0],
                    0 => undernetwork_adress[i.0] = 0,
                    _ => undernetwork_adress[i.0] = i.1 & full_mask[i.0],
                }
            }
        }
    }

    undernetwork_adress
}

pub fn network_node(full_mask: &[u8; 4], ip: &[u8; 4]) -> [u8; 4] {
    let mut network_node: [u8; 4] = [0; 4];
    let mask = ip_class(&ip[0]).mask();

    for i in ip.iter().enumerate() {
        match mask[i.0] {
            255 => {
                match full_mask[i.0] {
                    255 => network_node[i.0] = 0,
                    _ => network_node[i.0] = i.1 & !full_mask[i.0],
                }
            }
            _ => {
                match full_mask[i.0] {
                    255 => network_node[i.0] = i.1 & !full_mask[i.0],
                    0 => network_node[i.0] = *i.1,
                    _ => network_node[i.0] = i.1 & !full_mask[i.0],
                }
            }
        }
    }

    network_node
}

pub fn network_broadcast(ip: &[u8; 4]) -> [u8; 4] {
    let mut network_broadcast: [u8; 4] = *ip;
    let mask = ip_class(&ip[0]).mask();

    for i in mask.iter().enumerate() {
        if mask[i.0] == 255 {
            network_broadcast[4 - i.0 - 1] = *i.1;
        }
    }
    network_broadcast
}
