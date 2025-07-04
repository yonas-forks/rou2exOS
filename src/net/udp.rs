#[repr(C, packed)]
pub struct UdpHeader {
    pub source_port: u16,
    pub dest_port: u16,
    pub length: u16,
    pub checksum: u16,
}

pub fn create_packet(
    _source_ip: [u8; 4],
    _dest_ip: [u8; 4],
    source_port: u16,
    dest_port: u16,
    payload: &[u8],
    out_buffer: &mut [u8],
) -> usize {
    let udp_len = 8 + payload.len(); // 8 bytes header + payload

    let header = UdpHeader {
        source_port: source_port.to_be(),
        dest_port: dest_port.to_be(),
        length: (udp_len as u16).to_be(),
        checksum: 0, // Temporary 0, we'll compute later
    };

    // Copy header
    unsafe {
        let header_bytes = core::slice::from_raw_parts(
            &header as *const _ as *const u8,
            core::mem::size_of::<UdpHeader>(),
        );
        if let Some(slice) = out_buffer.get_mut(..header_bytes.len()) {
            slice.copy_from_slice(header_bytes);
        }
    }

    // Copy payload
    if let Some(slice) = out_buffer.get_mut(8..8 + payload.len()) {
        slice.copy_from_slice(payload);
    }

    // Calculate checksum
    // TODO

    udp_len
}

pub fn parse_packet(packet: &[u8]) -> Option<(u16, u16, &[u8])> {
    if packet.len() < 8 {
        return None;
    }

    let (mut source_port, mut dest_port, mut length): (u16, u16, u16) = (0, 0, 0);

    if let Some(w1) = packet.first() {
        if let Some(w2) = packet.get(1) {
            source_port = u16::from_be_bytes([*w1, *w2]);
        }
    }
    if let Some(w1) = packet.get(2) {
        if let Some(w2) = packet.get(3) {
            dest_port = u16::from_be_bytes([*w1, *w2]);
        }
    }
    if let Some(w1) = packet.get(4) {
        if let Some(w2) = packet.get(5) {
            length = u16::from_be_bytes([*w1, *w2]);
        }
    }
    /*if let Some(w1) = packet.get(6) {
        if let Some(w2) = packet.get(7) {
            _checksum = u16::from_be_bytes([*w1, *w2]);
        }
    }*/

    //let source_port = u16::from_be_bytes([packet[0], packet[1]]);
    //let dest_port = u16::from_be_bytes([packet[2], packet[3]]);
    //let length = u16::from_be_bytes([packet[4], packet[5]]);
    //let _checksum = u16::from_be_bytes([packet[6], packet[7]]);

    if packet.len() < length as usize {
        return None;
    }

    let payload_slice = packet.get(8..length as usize).unwrap_or(&[]);
    //let payload = &packet[8..length as usize];
    Some((source_port, dest_port, payload_slice))
}

/// Calculate UDP checksum including IPv4 pseudo-header
pub fn get_checksum(
    src_ip: [u8; 4],
    dst_ip: [u8; 4],
    udp_packet: &[u8], // Whole UDP header + data
) -> u16 {
    let mut sum = 0u32;

    // Pseudo-header
    
    if let Some(w1) = src_ip.first() {
        if let Some(w2) = src_ip.get(1) {
            sum += u16::from_be_bytes([*w1, *w2]) as u32;
        }
    }
    if let Some(w1) = src_ip.get(2) {
        if let Some(w2) = src_ip.get(3) {
            sum += u16::from_be_bytes([*w1, *w2]) as u32;
        }
    }

    if let Some(w1) = dst_ip.first() {
        if let Some(w2) = dst_ip.get(1) {
            sum += u16::from_be_bytes([*w1, *w2]) as u32;
        }
    }
    if let Some(w1) = dst_ip.get(2) {
        if let Some(w2) = dst_ip.get(3) {
            sum += u16::from_be_bytes([*w1, *w2]) as u32;
        }
    }

    sum += 0x11u8 as u32;           // Protocol (UDP = 17 decimal)
    sum += udp_packet.len() as u32; // UDP length

    // UDP header + payload
    let mut i = 0;
    while i + 1 < udp_packet.len() {
        if let Some(w1) = udp_packet.get(i) {
            if let Some(w2) = udp_packet.get(i+1) {
                sum = sum.wrapping_add( u16::from_be_bytes([*w1, *w2]) as u32 );
            }
        }
        i += 2;
    }

    if i < udp_packet.len() {
        if let Some(w) = udp_packet.get(i) {
            sum = sum.wrapping_add(((*w as u16) << 8) as u32);
        }
    }

    // Fold carries
    while (sum >> 16) != 0 {
        sum = (sum & 0xFFFF) + (sum >> 16);
    }

    !(sum as u16)
}

