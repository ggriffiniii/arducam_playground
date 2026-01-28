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

                // Search for JPEG markers
                // SOI: FF D8
                // EOI: FF D9

                if let Some(start_idx) = find_sequence(&jpeg_buf, &[0xFF, 0xD8]) {
                    if let Some(end_idx) = find_sequence(&jpeg_buf[start_idx..], &[0xFF, 0xD9]) {
                        let total_end = start_idx + end_idx + 2;
                        let frame = &jpeg_buf[start_idx..total_end];

                        // Try decode
                        match image::load_from_memory_with_format(frame, image::ImageFormat::Jpeg) {
                            Ok(img) => {
                                let rgb = img.to_rgb8();
                                let (width, height) = rgb.dimensions();

                                // Convert to u32 buffer for minifb (00RRGGBB)
                                let buffer: Vec<u32> = rgb
                                    .pixels()
                                    .map(|p| {
                                        let r = p[0] as u32;
                                        let g = p[1] as u32;
                                        let b = p[2] as u32;
                                        (r << 16) | (g << 8) | b
                                    })
                                    .collect();

                                window
                                    .update_with_buffer(&buffer, width as usize, height as usize)
                                    .unwrap();
                                println!("Frame decoded: {}x{}", width, height);
                            }
                            Err(e) => {
                                eprintln!("Decode error: {}", e);
                            }
                        }

                        // Remove processed data (and anything before it)
                        jpeg_buf.drain(0..total_end);
                    }
                }

                // Use a sliding window to prevent buffer from growing indefinitely if no valid frame found
                if jpeg_buf.len() > 1024 * 512 {
                    println!("Buffer too large, clearing");
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
