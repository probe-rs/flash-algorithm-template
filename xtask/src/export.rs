use cargo_metadata::Message;
use color_eyre::eyre::Result;
use xshell::{cmd, Shell};

const DEFINITION_PATH: &str = "target/definition.yaml";

pub fn export() -> Result<&'static str> {
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

    cmd!(sh, "cp template.yaml {DEFINITION_PATH}").run()?;
    cmd!(
        sh,
        "target-gen elf -n algorithm-test -u {target_artifact} {DEFINITION_PATH}"
    )
    .run()?;

    generate_debug_info(&sh, target_artifact)?;

    Ok(DEFINITION_PATH)
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
