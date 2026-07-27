//! `wb erfasse` — convenience capture of system output into `abgabe/`.
//!
//! Binding constraint (SPEC §2, CLAUDE.md rule 2): every command below is
//! **compiled into the binary**. Nothing here is ever read from exercise
//! content, and no learner input is ever interpolated into a shell string —
//! `ordnerliste` is implemented in Rust for exactly that reason.

use std::path::{Path, PathBuf};
use std::process::Command;

use crate::checks::text;
use crate::error::{AppError, Result};
use crate::strings_de as de;

pub struct Preset {
    pub name: &'static str,
    /// File written into `abgabe/`.
    pub file: &'static str,
    pub description_de: &'static str,
    pub kind: PresetKind,
}

pub enum PresetKind {
    /// A fixed command per platform.
    Command {
        windows: &'static str,
        unix: Option<&'static str>,
    },
    /// Listing a folder inside the exercise — done natively, no subprocess.
    FolderList,
}

/// The whitelist. Nine entries, exactly as agreed in SPEC §2.
pub const PRESETS: &[Preset] = &[
    Preset {
        name: "systeminfo",
        file: "systeminfo.txt",
        description_de: "Überblick über das ganze System",
        kind: PresetKind::Command {
            windows: "systeminfo",
            unix: Some("uname -a; echo; cat /etc/os-release 2>/dev/null; echo; uptime"),
        },
    },
    Preset {
        name: "ipconfig",
        file: "ipconfig.txt",
        description_de: "Netzwerk-Einstellungen",
        kind: PresetKind::Command {
            windows: "ipconfig /all",
            unix: Some("ip -details address show 2>/dev/null || ifconfig -a"),
        },
    },
    Preset {
        name: "hardware",
        file: "hardware.txt",
        description_de: "Board, Prozessor und Arbeitsspeicher",
        kind: PresetKind::Command {
            windows: "Get-CimInstance Win32_ComputerSystem | \
                      Format-List Manufacturer,Model,SystemFamily,TotalPhysicalMemory,NumberOfProcessors; \
                      Get-CimInstance Win32_Processor | \
                      Format-List Name,NumberOfCores,NumberOfLogicalProcessors,MaxClockSpeed; \
                      Get-CimInstance Win32_PhysicalMemory | \
                      Format-List Manufacturer,Capacity,Speed,MemoryType,DeviceLocator",
            unix: Some("lscpu 2>/dev/null; echo; free -h 2>/dev/null"),
        },
    },
    Preset {
        name: "firmware",
        file: "firmware.txt",
        description_de: "UEFI oder BIOS, GPT oder MBR",
        kind: PresetKind::Command {
            windows: "Get-ComputerInfo -Property BiosFirmwareType,BiosManufacturer,BiosVersion | \
                      Format-List; \
                      Get-Disk | Format-List Number,FriendlyName,PartitionStyle,Size",
            unix: Some(
                "if [ -d /sys/firmware/efi ]; then echo 'Firmware: UEFI'; \
                 else echo 'Firmware: BIOS/Legacy'; fi; echo; \
                 lsblk -o NAME,SIZE,TYPE,PTTYPE 2>/dev/null",
            ),
        },
    },
    Preset {
        name: "datentraeger",
        file: "datentraeger.txt",
        description_de: "Festplatten und SSDs",
        kind: PresetKind::Command {
            windows: "Get-Disk | \
                      Format-List Number,FriendlyName,SerialNumber,Size,PartitionStyle,HealthStatus,OperationalStatus,BusType; \
                      Get-PhysicalDisk | \
                      Format-List DeviceId,FriendlyName,MediaType,Size,HealthStatus,CanPool",
            unix: Some("lsblk -o NAME,SIZE,TYPE,MOUNTPOINT,ROTA,PTTYPE,MODEL 2>/dev/null"),
        },
    },
    Preset {
        name: "spiegel",
        file: "spiegel.txt",
        description_de: "Speicherpool und gespiegelte Laufwerke (RAID 1)",
        kind: PresetKind::Command {
            windows: "Get-StoragePool | \
                      Format-List FriendlyName,OperationalStatus,HealthStatus,Size,AllocatedSize; \
                      Get-VirtualDisk | \
                      Format-List FriendlyName,ResiliencySettingName,NumberOfDataCopies,Size,HealthStatus,OperationalStatus",
            unix: Some("cat /proc/mdstat 2>/dev/null; echo; lsblk 2>/dev/null"),
        },
    },
    Preset {
        name: "bitlocker",
        file: "bitlocker.txt",
        description_de: "Verschlüsselung der Laufwerke",
        kind: PresetKind::Command {
            windows: "manage-bde -status",
            unix: None,
        },
    },
    Preset {
        name: "schutz",
        file: "schutz.txt",
        description_de: "Virenschutz und Firewall",
        kind: PresetKind::Command {
            windows: "Get-MpComputerStatus | \
                      Format-List AMServiceEnabled,AntivirusEnabled,RealTimeProtectionEnabled,AntivirusSignatureLastUpdated; \
                      Get-NetFirewallProfile | Format-List Name,Enabled,DefaultInboundAction",
            unix: None,
        },
    },
    Preset {
        name: "ordnerliste",
        file: "ordnerliste.txt",
        description_de: "Liste eines Ordners in deiner Übung",
        kind: PresetKind::FolderList,
    },
];

pub fn find(name: &str) -> Option<&'static Preset> {
    let name = name.trim().to_lowercase();
    PRESETS.iter().find(|preset| preset.name == name)
}

pub fn names() -> Vec<String> {
    PRESETS.iter().map(|p| p.name.to_string()).collect()
}

impl Preset {
    /// Is this preset usable on the platform we are running on?
    pub fn available_here(&self) -> bool {
        match &self.kind {
            PresetKind::FolderList => true,
            PresetKind::Command { unix, .. } => cfg!(windows) || unix.is_some(),
        }
    }
}

pub struct Capture {
    pub text: String,
    /// The command ran but reported a problem (exit code != 0).
    pub command_failed: bool,
}

/// Produce the capture text. Never touches the filesystem outside `folder`.
pub fn run(preset: &Preset, exercise_dir: &Path, folder: Option<&str>) -> Result<Capture> {
    match &preset.kind {
        PresetKind::FolderList => folder_list(exercise_dir, folder),
        PresetKind::Command { windows, unix } => {
            let (program, args) = platform_command(windows, *unix, preset.name)?;
            run_command(program, &args)
        }
    }
}

#[cfg(windows)]
fn platform_command(
    windows: &'static str,
    _unix: Option<&'static str>,
    _name: &str,
) -> Result<(&'static str, Vec<String>)> {
    Ok((
        "powershell",
        vec![
            "-NoProfile".to_string(),
            "-NonInteractive".to_string(),
            "-Command".to_string(),
            windows.to_string(),
        ],
    ))
}

#[cfg(not(windows))]
fn platform_command(
    _windows: &'static str,
    unix: Option<&'static str>,
    name: &str,
) -> Result<(&'static str, Vec<String>)> {
    match unix {
        Some(script) => Ok(("sh", vec!["-c".to_string(), script.to_string()])),
        None => Err(AppError::new(de::erfasse_nur_windows(name))),
    }
}

fn run_command(program: &str, args: &[String]) -> Result<Capture> {
    let output = Command::new(program)
        .args(args)
        .output()
        .map_err(|e| AppError::new(de::erfasse_fehlgeschlagen(&e.to_string())))?;

    // Decode tolerantly: a German console hands us CP850, PowerShell 5.1 may
    // hand us UTF-16LE. We always store UTF-8 so the file opens cleanly later.
    let (mut text, _) = text::decode(&output.stdout);
    if text.trim().is_empty() && !output.stderr.is_empty() {
        let (stderr_text, _) = text::decode(&output.stderr);
        text = stderr_text;
    }
    Ok(Capture {
        text,
        command_failed: !output.status.success(),
    })
}

/// List a folder inside the exercise — implemented natively so that no learner
/// input is ever passed to a shell.
fn folder_list(exercise_dir: &Path, folder: Option<&str>) -> Result<Capture> {
    let relative = folder.unwrap_or(crate::exercise::SUBMISSION_DIR);
    let target = exercise_dir.join(relative);
    let (Ok(real), Ok(base)) = (target.canonicalize(), exercise_dir.canonicalize()) else {
        return Err(AppError::new(de::erfasse_ordner_fehlt(relative)));
    };
    if !real.starts_with(&base) {
        return Err(AppError::new(de::erfasse_ordner_ausserhalb()));
    }
    if !real.is_dir() {
        return Err(AppError::new(de::erfasse_ordner_fehlt(relative)));
    }

    let mut lines = vec![format!("Ordner: {relative}"), String::new()];
    let mut entries = Vec::new();
    walk(&real, &real, 0, &mut entries);
    entries.sort();
    for entry in &entries {
        lines.push(entry.clone());
    }
    lines.push(String::new());
    lines.push(format!("{} Einträge", entries.len()));
    Ok(Capture {
        text: lines.join("\n") + "\n",
        command_failed: false,
    })
}

fn walk(base: &Path, dir: &Path, depth: usize, out: &mut Vec<String>) {
    if depth > 8 {
        return;
    }
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let Ok(meta) = entry.metadata() else { continue };
        let shown = path
            .strip_prefix(base)
            .unwrap_or(&path)
            .to_string_lossy()
            .replace('\\', "/");
        if meta.is_dir() {
            out.push(format!("[Ordner] {shown}"));
            walk(base, &path, depth + 1, out);
        } else {
            out.push(format!("[Datei ] {shown}  ({} Bytes)", meta.len()));
        }
    }
}

/// Where the capture is written.
pub fn target_path(preset: &Preset, exercise_dir: &Path) -> PathBuf {
    exercise_dir
        .join(crate::exercise::SUBMISSION_DIR)
        .join(preset.file)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preset_list_matches_the_spec() {
        let names = names();
        assert_eq!(
            names,
            vec![
                "systeminfo",
                "ipconfig",
                "hardware",
                "firmware",
                "datentraeger",
                "spiegel",
                "bitlocker",
                "schutz",
                "ordnerliste",
            ]
        );
    }

    #[test]
    fn lookup_is_forgiving_about_case_and_spaces() {
        assert!(find(" SystemInfo ").is_some());
        assert!(find("gibtsnicht").is_none());
    }

    #[test]
    fn windows_only_presets_are_marked_on_unix() {
        let bitlocker = find("bitlocker").unwrap();
        assert_eq!(bitlocker.available_here(), cfg!(windows));
        assert!(find("ordnerliste").unwrap().available_here());
    }

    #[test]
    fn folder_list_stays_inside_the_exercise() {
        let dir = tempfile::tempdir().unwrap();
        let exercise = dir.path().join("01-test");
        std::fs::create_dir_all(exercise.join("abgabe/unterordner")).unwrap();
        std::fs::write(exercise.join("abgabe/notiz.txt"), "Hallo").unwrap();

        let capture = folder_list(&exercise, None).unwrap();
        assert!(capture.text.contains("notiz.txt"), "{}", capture.text);
        assert!(capture.text.contains("unterordner"));

        assert!(folder_list(&exercise, Some("../..")).is_err());
        assert!(folder_list(&exercise, Some("gibtsnicht")).is_err());
    }

    /// The entry point is canonicalised, but the recursive walk must not follow
    /// a symlink out of the exercise either — otherwise `ordnerliste` writes
    /// chunks of the host filesystem into the learner's abgabe.
    #[cfg(unix)]
    #[test]
    fn folder_list_does_not_follow_symlinks_out_of_the_exercise() {
        let dir = tempfile::tempdir().unwrap();
        let exercise = dir.path().join("01-test");
        std::fs::create_dir_all(exercise.join("abgabe")).unwrap();
        std::fs::write(exercise.join("abgabe/notiz.txt"), "Hallo").unwrap();

        let outside = dir.path().join("geheim");
        std::fs::create_dir_all(&outside).unwrap();
        std::fs::write(outside.join("passwoerter.txt"), "streng geheim").unwrap();
        std::os::unix::fs::symlink(&outside, exercise.join("abgabe/link")).unwrap();

        let capture = folder_list(&exercise, None).unwrap();
        assert!(capture.text.contains("notiz.txt"), "{}", capture.text);
        assert!(
            !capture.text.contains("passwoerter.txt"),
            "the walk followed a symlink out of the exercise:\n{}",
            capture.text
        );
    }
}
