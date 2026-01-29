use anyhow::{Context, Result};
use minifb::{Key, Window, WindowOptions};
use std::io::Read;
use std::time::Duration;

fn main() -> Result<()> {
    let ports = serialport::available_ports().expect("No ports found!");

    if ports.is_empty() {
        eprintln!("No serial ports found!");
        return Ok(());
    }

    println!("Available ports:");
    for (i, p) in ports.iter().enumerate() {
        println!("{}: {}", i, p.port_name);
    }

    // Simple selection logic or hardcode for now
    let port_name = if ports.len() == 1 {
        ports[0].port_name.clone()
    } else {
        // Just pick the first likely one or ask user?
        // For automation, let's try to find one with likely name or just user first
        // Usually /dev/ttyACM0 on Linux for Pico
        ports
            .iter()
            .find(|p| p.port_name.contains("ACM"))
            .map(|p| p.port_name.clone())
            .unwrap_or(ports[0].port_name.clone())
    };

    println!("Opening port: {}", port_name);

    let mut port = serialport::new(&port_name, 115_200)
        .timeout(Duration::from_millis(100))
        .open()
        .context("Failed to open serial port")?;

    let mut window = Window::new("Arducam Viewer", 320, 240, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    // JPEG Buffer
    let mut jpeg_buf = Vec::new();
    let mut read_buf = [0u8; 1024];

    // Read loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        match port.read(&mut read_buf) {
            Ok(n) if n > 0 => {
                // println!("Received {} bytes", n); // Verbose
                jpeg_buf.extend_from_slice(&read_buf[..n]);

                // Sync Marker: AA 55 AA 55
                // Frame Size: 64 * 64 * 2 = 8192 bytes

                if let Some(start_idx) = find_sequence(&jpeg_buf, &[0xAA, 0x55, 0xAA, 0x55]) {
                    let frame_start = start_idx + 4;
                    let frame_len = 320 * 240 * 2;

                    if jpeg_buf.len() >= frame_start + frame_len {
                        let frame_data = &jpeg_buf[frame_start..frame_start + frame_len];

                        let mut buffer: Vec<u32> = Vec::with_capacity(320 * 240);

                        for chunk in frame_data.chunks_exact(2) {
                            let b2 = chunk[0] as u32;
                            let b1 = chunk[1] as u32;

                            // RGB565: RRRRRGGG GGGBBBBB
                            // b1: RRRRRGGG
                            // b2: GGGBBBBB

                            let r5 = (b1 & 0xF8) >> 3;
                            let g6 = ((b1 & 0x07) << 3) | ((b2 & 0xE0) >> 5);
                            let b5 = b2 & 0x1F;

                            let r8 = (r5 * 527 + 23) >> 6;
                            let g8 = (g6 * 259 + 33) >> 6;
                            let b8 = (b5 * 527 + 23) >> 6;

                            let rgb = (r8 << 16) | (g8 << 8) | b8;
                            buffer.push(rgb);
                        }

                        window.update_with_buffer(&buffer, 320, 240).unwrap();
                        println!("Frame decoded: 320x240 Raw");

                        // Remove processed data
                        jpeg_buf.drain(0..frame_start + frame_len);
                    }
                }

                // Buffer cleanup
                if jpeg_buf.len() > 300_000 {
                    // println!("Buffer too large, clearing");
                    jpeg_buf.clear();
                }
            }
            Ok(_) => {}
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {}
            Err(e) => eprintln!("Serial error: {}", e),
        }

        window.update();
    }

    Ok(())
}

fn find_sequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}
