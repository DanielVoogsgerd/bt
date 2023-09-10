use std::process::Command;

const DEVICE_COMMANDS: [&str; 7] = [
    "pair",
    "trust",
    "untrust",
    "block",
    "unblock",
    "connect",
    "disconnect",
];
// const CONTROLLER_COMMANDS: [&str; 2] = ["show", "select"];
// const BINARY_COMMANDS: [&str; 5] = ["power", "pairable", "discoverable", "agent", "advertise"];

fn main() {
    let mut args = std::env::args();

    args.next().expect("Could not get arguments");

    match args.next() {
        Some(command) if DEVICE_COMMANDS.contains(&command.as_str()) => {
            let devices = get_devices().expect("Could not get devices");
            let args = std::iter::once(command).chain(args.map(|arg| {
                for (mac, alias) in &devices {
                    if *alias == arg {
                        return mac.clone();
                    }
                }

                return arg;
            }));

            Command::new("bluetoothctl")
                .args(args)
                .spawn()
                .expect("Oops");
        }
        Some(command) => {
            let args = std::iter::once(command).chain(args);
            Command::new("bluetoothctl")
                .args(args)
                .spawn()
                .expect("Oops");
        }
        None => {
            unimplemented!("bluetooth interactive mode is not supported. Call bluetoothctl itself");
        }
    };
}

fn get_devices() -> anyhow::Result<Vec<(String, String)>> {
    let mut command = Command::new("bluetoothctl");
    command.arg("devices");

    let data = String::from_utf8(command.output()?.stdout)?;

    Ok(data
        .split("\n")
        .filter_map(|line| {
            let mut segments = line.split_ascii_whitespace();

            segments.next()?;

            return Some((segments.next()?.to_owned(), segments.next()?.to_owned()));
        })
        .collect())
}
