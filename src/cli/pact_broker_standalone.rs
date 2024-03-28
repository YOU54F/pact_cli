use std::{
    env, fs,
    io::{Read, Write},
};

use clap::{Arg, ArgMatches, Command};
use std::process::{Command as Cmd, ExitStatus};
pub fn add_standalone_broker_subcommand() -> Command {
    Command::new("standalone")
        .about(
            "Install & Run the Pact Broker with a bundled ruby runtime in $HOME/traveling-broker",
        )
        .subcommand(
            Command::new("start")
                .about("Download and Start the Pact Broker")
                .arg(
                    Arg::new("detach")
                        .short('d')
                        .long("detach")
                        .num_args(0)
                        .action(clap::ArgAction::SetTrue)
                        .help("Run the Pact Broker in the background"),
                ),
        )
        .subcommand(Command::new("stop").about("Stop the Pact Broker"))
        .subcommand(Command::new("remove").about("Remove the Pact Broker"))
        .subcommand(Command::new("info").about("Info about the Pact Broker"))
}

pub fn run(args: &ArgMatches) {
    let traveling_pact_broker_home = home::home_dir()
        .map(|dir| dir.join(".pact/traveling-broker"))
        .unwrap_or_default()
        .display()
        .to_string();
    let traveling_pact_broker_pid_file_path =
        format!("{}/pact_broker-standalone.pid", traveling_pact_broker_home);
    let os = match env::consts::OS {
        "macos" => "osx",
        other => other,
    };

    let arch = match env::consts::ARCH {
        "aarch64" => "arm64",
        other => other,
    };

    // check if os/arch is supported
    // supported are osx, linux, windows and arm64, x86_64
    if os != "osx" && os != "linux" && os != "windows" {
        println!("⚠️  Unsupported OS: {}", os);
        std::process::exit(1);
    }
    if arch != "arm64" && arch != "x86_64" {
        println!("⚠️  Unsupported architecture: {}", arch);
        std::process::exit(1);
    }
    let traveling_pact_broker_app_path = if os == "windows" {
        format!(
            "{}/packed-broker/pact-broker-app.bat",
            traveling_pact_broker_home
        )
    } else {
        format!("{}/pact-broker-app.sh", traveling_pact_broker_home)
    };
    let traveling_pact_broker_ruby_path = if os == "windows" {
        format!("{}/packed-broker/bin/ruby.exe", traveling_pact_broker_home)
    } else {
        format!("{}/bin/ruby", traveling_pact_broker_home)
    };
    match args.subcommand() {
        Some(("start", args)) => {
            tokio::runtime::Runtime::new().unwrap().block_on(async {

        // Store the binary in the user's home .pact/traveling-broker directory
        if !fs::metadata(&traveling_pact_broker_home).is_ok() {
            let _ = fs::create_dir_all(&traveling_pact_broker_home);
        }

        // check is app path exists, if so, do not download the file

        if !fs::metadata(&traveling_pact_broker_app_path).is_ok() {
            // Download the correct version of the traveling ruby binary
            let mut os_variant: String = os.to_string();
            if os == "linux" && cfg!(target_env = "musl") {
                let output = Cmd::new("ldd")
                    .arg("/bin/sh")
                    .output()
                    .ok();

                if let Some(output) = output {
                    let output_str = String::from_utf8_lossy(&output.stdout);
                    if output_str.contains("musl") {
                        println!("🚀 Detected musl libc, downloading musl version");
                        os_variant.push_str("-musl");
                    }
                } else {
                    println!("⚠️  Failed to execute ldd command, downloading glibc version");
                }
            }
            let broker_archive_path = if os == "windows" {
                format!("{}/packed-broker.zip", traveling_pact_broker_home)
            } else {
                format!("{}/traveling-pact-20240317-3.2.3-{}-{}-full.tar.gz", traveling_pact_broker_home, os_variant, arch)
            };
            let url = if os == "windows" {
                format!(
                    "https://github.com/YOU54F/test/releases/download/0.0.0/packed-broker.zip",
                )
            } else {
                let download_url = format!(
                    "https://github.com/YOU54F/traveling-ruby/releases/download/rel-20240317-pact/traveling-pact-20240317-3.2.3-{}-{}-full.tar.gz",
                    os_variant, arch
                );
                download_url

            };
            println!("🚀 Downloading Pact Broker binary from {}", url);
            let response = reqwest::get(&url).await.unwrap();
            let body = response.bytes().await.unwrap();

            let mut file = fs::File::create(&broker_archive_path).unwrap();

            // Use tokio::task::spawn_blocking to wait for the file writing to complete
            tokio::task::spawn_blocking(move || {
                let _ = file.write_all(&body).expect("Unable to write zip to file");
            }).await.unwrap();

            // Unpack the binary
            println!("🚀 Unpacking the binary...");
            let mut status: ExitStatus;
            if os == "windows" {
                if Cmd::new("unzip").output().is_ok() {
                    println!("Unpacking {} to {}, tool: {}", broker_archive_path, traveling_pact_broker_home, "unzip");
                    status = Cmd::new("unzip")
                        .arg(&broker_archive_path)
                        .arg("-d")
                        .arg(&traveling_pact_broker_home)
                        .status()
                        .expect("Failed to unpack the binary");
                } else {
                    println!("Unpacking {} to {}, tool: {}", broker_archive_path, traveling_pact_broker_home, "pwsh Expand-Archive");
                    status = Cmd::new("powershell")
                        .arg("-Command")
                        .arg(format!(
                            "Expand-Archive -Path '{}' -DestinationPath '{}'",
                            &broker_archive_path, &traveling_pact_broker_home
                        ))
                        .status()
                        .expect("Failed to unpack the binary");
                }
            } else {
                println!("Unpacking {} to {}, tool: {}", broker_archive_path, traveling_pact_broker_home, "tar");
                status = Cmd::new("tar")
                    .arg("-xf")
                    .arg(&broker_archive_path)
                    .arg("-C")
                    .arg(&traveling_pact_broker_home)
                    .status()
                    .expect("Failed to unpack the binary");
            }
            if !status.success() {
                println!("⚠️  Failed to unpack the binary");
                std::process::exit(1);
            } else {
                println!("🚀 Removing the archive at {}", broker_archive_path);
                let _ = fs::remove_file(broker_archive_path);
            }
    } else {
        println!("🚀 Pact Broker binary already exists at {}", traveling_pact_broker_app_path);
    }
        // Execute the pact-broker-app.sh file
        println!("🚀 Starting Pact Broker (this may take a few seconds)...");
        println!("🚀 Running: {}", traveling_pact_broker_app_path);
        let mut child_cmd = Cmd::new(&traveling_pact_broker_app_path);

        if let Ok(mut child) = child_cmd
        .arg("--pidfile")
        .arg(&traveling_pact_broker_pid_file_path).spawn() {
            let pid = child.id();
            // let mut pid_file = fs::File::create(&traveling_pact_broker_pid_file_path).unwrap();
            // let _ = pid_file.write_all(pid.to_string().as_bytes());
            println!("🚀 Pact Broker is running on http://localhost:9292");
            println!("🚀 PID: {}", pid);
            println!("🚀 PID file: {}", traveling_pact_broker_pid_file_path);
            let mut pid_file_contents = String::from("unknown");
            while !pid_file_contents.chars().all(char::is_numeric) {
                std::thread::sleep(std::time::Duration::from_secs(1));
                pid_file_contents = fs::read_to_string(&traveling_pact_broker_pid_file_path).unwrap_or_else(|_| String::from("unknown"));
            }
            println!("Traveling Broker PID: {}", pid_file_contents);

            // we should support a detach flag to run the broker in the background
            let detach = args.get_flag("detach");
            if detach {
                println!("🚀 Running in the background");
                std::process::exit(0);
            } else {
            while child.try_wait().unwrap().is_none() {
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
            let _ = child.kill();
            let pid_file = fs::File::open(&traveling_pact_broker_pid_file_path);
            match pid_file {
                Ok(mut file) => {
                    let mut pid = String::new();
                    file.read_to_string(&mut pid).unwrap();
                    let pid = pid.trim().parse::<u32>().unwrap();
                    println!("🚀 Stopping Pact Broker with PID: {}", pid);
                    #[cfg(windows)]
                    Cmd::new("taskkill")
                        .arg("/F")
                        .arg("/PID")
                        .arg(pid.to_string())
                        .output()
                        .expect("Failed to stop the process");
                }
                Err(_) => {
                    println!("PID file not found");
                }
            }
            let _ = fs::remove_file(&traveling_pact_broker_pid_file_path);
            std::process::exit(0);
            }
        } else {
            println!("{} didn't start", traveling_pact_broker_app_path);
            std::process::exit(1);
        }
    });
        }
        Some(("stop", args)) => {
            // Stop the broker
            let pid_file = fs::File::open(&traveling_pact_broker_pid_file_path);
            match pid_file {
                Ok(mut file) => {
                    let mut pid = String::new();
                    file.read_to_string(&mut pid).unwrap();
                    let pid = pid.trim().parse::<u32>().unwrap();
                    println!("🚀 Stopping Pact Broker with PID: {}", pid);
                    #[cfg(windows)]
                    Cmd::new("taskkill")
                        .arg("/F")
                        .arg("/PID")
                        .arg(pid.to_string())
                        .output()
                        .expect("⚠️ Failed to stop the broker");

                    #[cfg(not(windows))]
                    Cmd::new("kill")
                        .arg(pid.to_string())
                        .output()
                        .expect("⚠️ Failed to stop the broker");
                    let _ = fs::remove_file(&traveling_pact_broker_pid_file_path);
                    println!("🛑 Pact Broker stopped");
                    std::process::exit(0);
                }
                Err(_) => {
                    println!("⚠️ Pact Broker is not running");
                    std::process::exit(1);
                }
            }
        }
        Some(("remove", args)) => {
            if let Ok(metadata) = std::fs::metadata(traveling_pact_broker_home.clone()) {
                if metadata.is_dir() {
                    if let Err(err) = std::fs::remove_dir_all(traveling_pact_broker_home) {
                        println!("Failed to remove traveling_pact_broker_home: {}", err);
                    } else {
                        println!("traveling_pact_broker_home removed successfully");
                    }
                }
            } else {
                println!(
                    "traveling_pact_broker_home {} not found",
                    traveling_pact_broker_home
                );
            }
        }
        Some(("info", args)) => {
            fn check_directory_exists(directory: &str) -> bool {
                std::path::Path::new(directory).exists()
            }

            let pact_broker_standalone_exists = check_directory_exists(&traveling_pact_broker_home);

            println!(
                "Pact broker directory exists: {}",
                pact_broker_standalone_exists
            );

            fn get_traveling_ruby_version(app_path: &str) -> std::io::Result<String> {
                let output = Cmd::new(app_path).arg("-v").output()?;
                Ok(String::from_utf8_lossy(&output.stdout).to_string())
            }

            println!(
                "Ruby version: {:?}",
                get_traveling_ruby_version(&traveling_pact_broker_ruby_path)
            );

            fn check_pid_file_exists(pid_file_path: &str) -> bool {
                std::path::Path::new(pid_file_path).exists()
            }

            let pact_broker_pid_file_exists =
                check_pid_file_exists(&traveling_pact_broker_pid_file_path);
            println!("Pact broker pid exists: {}", pact_broker_pid_file_exists);

            fn get_pid_from_file(pid_file_path: &str) -> Option<u32> {
                if let Ok(mut file) = std::fs::File::open(pid_file_path) {
                    let mut pid = String::new();
                    file.read_to_string(&mut pid).unwrap();
                    Some(pid.trim().parse::<u32>().unwrap())
                } else {
                    None
                }
            }

            let pact_broker_pid_exists = get_pid_from_file(&traveling_pact_broker_pid_file_path);
            println!("Pact broker pid: {:?}", pact_broker_pid_exists);
        }
        _ => {
            println!("⚠️  No option provided, try running standalone --help");
        }
    }
}
