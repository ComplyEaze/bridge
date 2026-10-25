use std::{
    fs,
    path::{Path, PathBuf},
    process::ExitCode,
};

use bridge_tally_compatibility::{
    enforce_support_gate, format_gate_success, now_unix_ms, parse_artifact, render_claim_matrix,
    safe_error_code, verify_claim_matrix_markdown, LiveCompatibilityReceipt,
    ReviewedEvidenceAttestation, SupportClaimsManifest, SurfacePins, TrustedEvidenceKeys,
    MAX_ARTIFACT_BYTES,
};

fn main() -> ExitCode {
    match run() {
        Ok(output) => match emit_output(output) {
            Ok(()) => ExitCode::SUCCESS,
            Err(code) => {
                eprintln!("bridge_tally_compatibility_failed:{code}");
                ExitCode::FAILURE
            }
        },
        Err(code) => {
            eprintln!("bridge_tally_compatibility_failed:{code}");
            ExitCode::FAILURE
        }
    }
}

fn emit_output(message: String) -> Result<(), &'static str> {
    println!("{message}");
    Ok(())
}

fn run() -> Result<String, &'static str> {
    run_from_args(std::env::args().skip(1))
}

fn run_from_args(mut args: impl Iterator<Item = String>) -> Result<String, &'static str> {
    match args.next().as_deref() {
        Some("validate-receipt") => {
            let path = one_path(&mut args)?;
            let bytes = read_bounded(&path)?;
            LiveCompatibilityReceipt::from_json(&bytes).map_err(|error| safe_error_code(&error))?;
            Ok("compatibility_receipt_valid".to_string())
        }
        Some("gate") => {
            let support = next_path(&mut args, "missing_support_manifest")?;
            let surface = next_path(&mut args, "missing_surface_manifest")?;
            let trust = next_path(&mut args, "missing_trust_manifest")?;
            let evidence = next_path(&mut args, "missing_evidence_directory")?;
            let root = next_path(&mut args, "missing_repository_root")?;
            if args.next().is_some() {
                return Err("unexpected_argument");
            }
            gate_command(&support, &surface, &trust, &evidence, &root)
        }
        Some("surface-digest") => {
            let surface = next_path(&mut args, "missing_surface_manifest")?;
            let repository_root = next_path(&mut args, "missing_repository_root")?;
            if args.next().is_some() {
                return Err("unexpected_argument");
            }
            surface_digest_command(&surface, &repository_root)
        }
        Some("check-matrix-markdown") => {
            let manifest_path = next_path(&mut args, "missing_support_manifest")?;
            let markdown_path = next_path(&mut args, "missing_matrix_markdown")?;
            if args.next().is_some() {
                return Err("unexpected_argument");
            }
            let manifest = SupportClaimsManifest::from_json(&read_bounded(&manifest_path)?)
                .map_err(|error| safe_error_code(&error))?;
            let markdown = fs::read(&markdown_path).map_err(|_| "matrix_markdown_unavailable")?;
            verify_claim_matrix_markdown(&manifest, &markdown)
                .map_err(|error| safe_error_code(&error))?;
            Ok("compatibility_matrix_markdown_current".to_string())
        }
        Some("render-matrix") => {
            let path = one_path(&mut args)?;
            let manifest = SupportClaimsManifest::from_json(&read_bounded(&path)?)
                .map_err(|error| safe_error_code(&error))?;
            render_claim_matrix(&manifest).map_err(|error| safe_error_code(&error))
        }
        _ => Err("usage_validate_receipt_surface_digest_render_or_check_matrix_markdown_or_gate"),
    }
}

/// Prints the surface digest that receipts and attestations bind, computed from the live bytes of
/// every pinned file. Used to compare the digest across a change to the surface layout.
fn surface_digest_command(
    surface_path: &Path,
    repository_root: &Path,
) -> Result<String, &'static str> {
    let pins = SurfacePins::from_json(&read_bounded(surface_path)?)
        .map_err(|error| safe_error_code(&error))?;
    pins.resolve(repository_root)
        .and_then(|surface| surface.digest())
        .map_err(|error| safe_error_code(&error))
}

fn gate_command(
    support_path: &Path,
    surface_path: &Path,
    trust_path: &Path,
    evidence_dir: &Path,
    repository_root: &Path,
) -> Result<String, &'static str> {
    let support = SupportClaimsManifest::from_json(&read_bounded(support_path)?)
        .map_err(|error| safe_error_code(&error))?;
    let surface = SurfacePins::from_json(&read_bounded(surface_path)?)
        .and_then(|pins| pins.resolve(repository_root))
        .map_err(|error| safe_error_code(&error))?;
    let trust = TrustedEvidenceKeys::from_json(&read_bounded(trust_path)?)
        .map_err(|error| safe_error_code(&error))?;
    let mut receipts = Vec::new();
    let mut attestations = Vec::new();
    if evidence_dir.exists() {
        let entries = fs::read_dir(evidence_dir).map_err(|_| "evidence_directory_unavailable")?;
        for (index, entry) in entries.enumerate() {
            if index >= 128 {
                return Err("evidence_file_limit");
            }
            let path = entry.map_err(|_| "evidence_directory_unavailable")?.path();
            if !path.is_file() {
                continue;
            }
            let name = path
                .file_name()
                .and_then(|value| value.to_str())
                .ok_or("evidence_filename_invalid")?;
            if name.ends_with(".receipt.json") {
                receipts.push(
                    LiveCompatibilityReceipt::from_json(&read_bounded(&path)?)
                        .map_err(|error| safe_error_code(&error))?,
                );
            } else if name.ends_with(".attestation.json") {
                attestations.push(
                    parse_artifact::<ReviewedEvidenceAttestation>(&read_bounded(&path)?)
                        .map_err(|error| safe_error_code(&error))?,
                );
            } else if name != "README.md" && name != ".gitkeep" {
                return Err("evidence_filename_invalid");
            }
        }
    }
    let report = enforce_support_gate(
        &support,
        &surface,
        &trust,
        &receipts,
        &attestations,
        repository_root,
        now_unix_ms().map_err(|error| safe_error_code(&error))?,
    )
    .map_err(|error| safe_error_code(&error))?;
    Ok(format_gate_success(&report))
}

fn one_path(args: &mut impl Iterator<Item = String>) -> Result<PathBuf, &'static str> {
    let path = next_path(args, "missing_path")?;
    if args.next().is_some() {
        return Err("unexpected_argument");
    }
    Ok(path)
}

fn next_path(
    args: &mut impl Iterator<Item = String>,
    missing: &'static str,
) -> Result<PathBuf, &'static str> {
    args.next().map(PathBuf::from).ok_or(missing)
}

fn read_bounded(path: &Path) -> Result<Vec<u8>, &'static str> {
    let metadata = fs::metadata(path).map_err(|_| "artifact_unavailable")?;
    if metadata.len() == 0 || metadata.len() > MAX_ARTIFACT_BYTES as u64 {
        return Err("artifact_size_invalid");
    }
    fs::read(path).map_err(|_| "artifact_unavailable")
}
