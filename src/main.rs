use std::{process::Command, thread, time::Duration};
use clap::Parser;

const TRIGGER_ADDR: u32 = 0x767;
const R_ADDR: u32 = 0x769;
const G_ADDR: u32 = 0x76A;
const B_ADDR: u32 = 0x76B;

#[derive(Parser, Debug)]
#[command()]
struct Args {
    /// The amount of Red. 
    #[arg(short, long)]
    r: u8,
    
    /// The amount of Green. 
    #[arg(short, long)]
    g: u8,

    /// The amount of Blue. 
    #[arg(short, long)]
    b: u8,
}


fn write_acpi(addr: u32, value: u8) {
    let echo_string = format!("echo '\\_SB.INOU.ECRW {:#01x} {}' > /proc/acpi/call", addr, value);
    Command::new("/bin/bash")
        .args(&["-c", &echo_string])
        .spawn()
        .expect("Failed to execute process.");

    thread::sleep(Duration::from_millis(200));
}

/// This is distributing the RGB values from 0 to 50, since the RGB in the
/// keyboard treats 255 as 50 (or something close to it)
fn to_level(value: u8, base_rgb_value: u8) -> u8 {
	return (value / base_rgb_value).clamp(1, 50)
}

fn set_rgb(r: u8, g: u8, b: u8) {
    let r_level = to_level(r, 5);
    let b_level = to_level(b, 6);
    let g_level = to_level(g, 6);

    write_acpi(R_ADDR, r_level);
    write_acpi(G_ADDR, g_level);
    write_acpi(B_ADDR, b_level);
    write_acpi(TRIGGER_ADDR, 0x20);
    println!("Changed keyboard color.");

}

fn main() {
    let args = Args::parse();
    sudo::escalate_if_needed()
        .expect("This application requires sudo to write to /proc/acpi");

    set_rgb(args.r, args.g, args.b);

}
