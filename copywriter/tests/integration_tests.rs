use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::io::Write;
use tempfile::NamedTempFile;
use chrono::Datelike;

#[test]
fn test_print_dry_run_replaces_header() -> Result<(), Box<dyn std::error::Error>> {
    // Create a temporary license file and a target source file.
    let mut license = NamedTempFile::new()?;
    writeln!(license, "Custom License Body")?;

    let mut tmp = NamedTempFile::new()?;
    writeln!(tmp, "fn main() {{}}")?;

    // Run the tool to actually write changes.
    let mut cmd = assert_cmd::Command::cargo_bin("copywriter")?;
    cmd.arg("--author").arg("Achilles")
        .arg("--license").arg(license.path())
        .arg(tmp.path());

    cmd.assert().success();

    let contents = std::fs::read_to_string(tmp.path())?;
    assert!(contents.contains("Achilles"));
    assert!(contents.contains("Custom License Body"));

    Ok(())
}

#[test]
fn test_interactive_prompt_changes_header_and_year() -> Result<(), Box<dyn std::error::Error>> {
    // Prepare license and target file.
    let mut license = NamedTempFile::new()?;
    writeln!(license, "Interactive License")?;

    let mut tmp = NamedTempFile::new()?;
    writeln!(tmp, "fn main() {{}}")?;

    // Run the tool in interactive mode and supply name + year on stdin.
    let mut cmd = assert_cmd::Command::cargo_bin("copywriter")?;
    cmd.arg("--interactive")
        .arg("--license").arg(license.path())
        .arg(tmp.path())
        .write_stdin("InteractiveName\n2001\n");

    cmd.assert().success();

    let contents = std::fs::read_to_string(tmp.path())?;
    assert!(contents.contains("InteractiveName"));
    assert!(contents.contains("2001"));

    Ok(())
}

#[test]
fn test_force_and_year_override() -> Result<(), Box<dyn std::error::Error>> {
    // Dry-run should report what would be updated.
    let mut license = NamedTempFile::new()?;
    writeln!(license, "Another License")?;

    let mut tmp = NamedTempFile::new()?;
    writeln!(tmp, "fn main() {{}}")?;

    let mut cmd = assert_cmd::Command::cargo_bin("copywriter")?;
    cmd.arg("--author").arg("DryRunTester")
        .arg("--license").arg(license.path())
        .arg("--dry-run")
        .arg(tmp.path());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Would update"));

    Ok(())
}

#[test]
fn test_no_changes_needed_when_current_year() -> Result<(), Box<dyn std::error::Error>> {
    use chrono::Utc;

    let year = Utc::now().year();

    // Create file that already contains current-year header so no change is needed.
    let mut tmp = NamedTempFile::new()?;
    writeln!(tmp, "fn main() {{}}")?;

    let mut license = NamedTempFile::new()?;
    writeln!(license, "License X")?;

    // First run: actually write the header+license into the file.
    let mut cmd_write = assert_cmd::Command::cargo_bin("copywriter")?;
    cmd_write.arg("--author").arg("Tester")
        .arg("--license").arg(license.path())
        .arg(tmp.path());
    cmd_write.assert().success();

    // Second run: dry-run should report no changes needed.
    let mut cmd = assert_cmd::Command::cargo_bin("copywriter")?;
    cmd.arg("--author").arg("Tester")
        .arg("--license").arg(license.path())
        .arg("--dry-run")
        .arg(tmp.path());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("No changes needed"));

    Ok(())
}

/*
 * License:
 * Copyright (c) 2025 Eric Hernandez
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */