use std::error::Error;

use etherparse::PacketHeaders;
use pcap::{Capture, Device};

fn main() -> Result<(), Box<dyn Error>> {
    let devices = Device::list()?;

    let device = devices
        .into_iter()
        .find(|d| d.name.starts_with('e') || d.name.starts_with('w'))
        .ok_or("Подходящее сетевое устр-в не найдено")?;

    println!("Слушаю утср-в {}", device.name);

    run_snifer(device)?;

    Ok(())
}

fn run_snifer(device: Device) -> Result<(), Box<dyn Error>> {
    let mut capture = Capture::from_device(device)
        .unwrap()
        .promisc(true) //вкл режим прослушивания
        .snaplen(5000) // макс размер пакета
        .timeout(1000)
        .open()?;

    println!("Начинаю перехват ... (ctrl+c для остановки)");

    while let Ok(packet) = capture.next_packet() {
        match PacketHeaders::from_ethernet_slice(packet.data) {
            Ok(headers) => {
                if let Some(ip) = headers.ip {
                    match ip {
                        etherparse::IpHeader::Version4(ipv4, _) => {
                            println!(
                                "IPv4: {} -> {}",
                                std::net::Ipv4Addr::from(ipv4.source),
                                std::net::Ipv4Addr::from(ipv4.destination)
                            );
                        }
                        _ => {}
                    }
                }

                if let Some(transport) = headers.transport {
                    match transport {
                        etherparse::TransportHeader::Tcp(tcp) => {
                            // if tcp.destination_port == 443
                            //     || tcp.source_port == 443
                            //     || tcp.destination_port == 80
                            //     || tcp.source_port == 80
                            // {
                                println!(
                                    "\n[TCP] {}:{} -> {}:{}",
                                    "IP_SRC", tcp.source_port, "IP_DST", tcp.destination_port
                                );

                                let playload_slice = headers.payload;

                                if !playload_slice.is_empty() {
                                    let text: String = playload_slice
                                        .iter()
                                        .map(|&b| if b >= 32 && b <= 126 { b as char } else { '.' })
                                        .collect();

                                    println!("   Данные (ASCII): {}", text);
                                }
                            }
                        
                        etherparse::TransportHeader::Udp(udp) => {
                            println!(
                                "   [UDP] Порт: {} -> {}",
                                udp.source_port, udp.destination_port
                            );
                        }
                        _ => {}
                    }
                }
            }
            Err(_) => {}
        }
    }

    Ok(())
}
