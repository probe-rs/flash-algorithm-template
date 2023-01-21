use anyhow::Result;
use base64::{engine::general_purpose as base64_engine, Engine as _};
use cargo_metadata::Message;
use clap::Parser;
use xshell::{cmd, Shell};

type DynError = Box<dyn std::error::Error>;

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
        std::process::exit(-1);
    }
}

#[derive(clap::Parser)]
#[clap(
    about = "Utilities to generate flash algorithms for probe-rs",
    author = "Noah Hüsser <yatekii@yatekii.ch> / Dominik Böhi <dominik.boehi@gmail.ch> / Dario Nieuwenhuis <dirbaio@dirbaio.net>"
)]
enum Cli {
    /// Export the flash algorithm to the probe-rs format.
    Export,
}

fn try_main() -> Result<(), DynError> {
    match Cli::parse() {
        Cli::Export => export(),
    }
}

fn export() -> Result<(), DynError> {
    let sh = Shell::new()?;

    // We build the actual flash algorithm.
    // We relay all the output of the build process to the open shell.
    let mut cmd = cmd!(
        sh,
        "cargo build --release --message-format=json-diagnostic-rendered-ansi"
    );
    cmd.set_ignore_status(true);
    let output = cmd.output()?;
    print!("{}", String::from_utf8_lossy(&output.stderr));

    // Parse build information to extract the artifcat.
    let messages = Message::parse_stream(output.stdout.as_ref());

    // Find artifacts.
    let mut target_artifact = None;
    for message in messages {
        match message? {
            Message::CompilerArtifact(artifact) => {
                if let Some(executable) = artifact.executable {
                    if target_artifact.is_some() {
                        // We found multiple binary artifacts,
                        // so we don't know which one to use.
                        // This should never happen!
                        unreachable!()
                    } else {
                        target_artifact = Some(executable);
                    }
                }
            }
            Message::CompilerMessage(message) => {
                if let Some(rendered) = message.message.rendered {
                    print!("{}", rendered);
                }
            }
            // Ignore other messages.
            _ => (),
        }
    }
    let target_artifact = target_artifact.expect("a flash algorithm artifact");
    let target_artifact = target_artifact.as_str();

    let binary = binary(&sh, target_artifact)?;
    let Addresses {
        init,
        uninit,
        program_page,
        erase_sector,
        erase_chip,
    } = addresses(&sh, target_artifact)?;

    // Generate the actual definition yaml.
    std::fs::write(
        "target/definition.yaml",
        format!(
            r#"flash_algorithm:
  - name: {{project-name}}
    instructions: {binary}
    pc_init: 0x{init:x}
    pc_uninit: 0x{uninit:x}
    pc_program_page: 0x{program_page:x}
    pc_erase_sector: 0x{erase_sector:x}
    pc_erase_all: 0x{erase_chip:x}
    "#,
        ),
    )?;

    generate_debug_info(&sh, target_artifact)?;

    Ok(())
}

/// Extracts the binary data as base64 from the ELF.
fn binary(sh: &Shell, target_artifact: &str) -> Result<String> {
    let mut cmd = cmd!(sh, "rust-objcopy {target_artifact} -O binary -");
    cmd.set_ignore_status(true);
    let output = cmd.output()?;
    print!("{}", String::from_utf8_lossy(&output.stderr));

    Ok(base64_engine::STANDARD.encode(output.stdout))
}

/// Extracts the addresses for the different flash algorithm functions from the ELF.
fn addresses(sh: &Shell, target_artifact: &str) -> Result<Addresses> {
    let mut cmd = cmd!(sh, "rust-nm {target_artifact}");
    cmd.set_ignore_status(true);
    let output = cmd.output()?;
    print!("{}", String::from_utf8_lossy(&output.stderr));

    let mut addresses = Addresses {
        init: 0,
        uninit: 0,
        program_page: 0,
        erase_sector: 0,
        erase_chip: 0,
    };

    for line in String::from_utf8_lossy(&output.stdout).lines() {
        let s = line.split(" T ").collect::<Vec<_>>();
        if s.len() == 2 {
            match s[1] {
                "Init" => addresses.init = u64::from_str_radix(s[0], 16).unwrap() + 1,
                "UnInit" => addresses.uninit = u64::from_str_radix(s[0], 16).unwrap() + 1,
                "ProgramPage" => {
                    addresses.program_page = u64::from_str_radix(s[0], 16).unwrap() + 1
                }
                "EraseSector" => {
                    addresses.erase_sector = u64::from_str_radix(s[0], 16).unwrap() + 1
                }
                "EraseChip" => addresses.erase_chip = u64::from_str_radix(s[0], 16).unwrap() + 1,
                _ => {}
            }
        }
    }

    Ok(addresses)
}

/// Generates information about the ELF binary.
fn generate_debug_info(sh: &Shell, target_artifact: &str) -> Result<()> {
    std::fs::write(
        "target/disassembly.s",
        cmd!(sh, "rust-objdump --disassemble {target_artifact}")
            .output()?
            .stdout,
    )?;
    std::fs::write(
        "target/dump.txt",
        cmd!(sh, "rust-objdump -x {target_artifact}")
            .output()?
            .stdout,
    )?;
    std::fs::write(
        "target/nm.txt",
        cmd!(sh, "rust-nm {target_artifact} -n").output()?.stdout,
    )?;

    Ok(())
}

#[derive(Debug)]
struct Addresses {
    init: u64,
    uninit: u64,
    program_page: u64,
    erase_sector: u64,
    erase_chip: u64,
}
