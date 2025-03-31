use std::process::Command;

fn main() {
    let mut cmd = Command::new("loginctl");
    let fullcmd = cmd.arg("unlock-session");
    let val = fullcmd.spawn().expect("loginctl failed to start");
    println!("{:?}", val);
}
