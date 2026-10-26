//! USAGE: `cargo boilerplate <s03|e2026> <q01>`
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::process::ExitCode;

const LIB_PATH: &str = "src/lib.rs";

/// Story (`s03`) or year (`e2026`) module name.
fn valid_module(name: &str) -> bool {
    let mut chars = name.chars();
    matches!(chars.next(), Some('s' | 'e')) && name.len() > 1 && chars.all(|c| c.is_ascii_digit())
}

/// Quest module name, e.g. `q01`.
fn valid_quest(name: &str) -> bool {
    name.starts_with('q') && name.len() > 1 && name[1..].chars().all(|c| c.is_ascii_digit())
}

/// Registers the quest in the `library!` invocation, adding the event line if needed.
fn update_lib(lib: &str, module: &str, quest: &str) -> Result<String, String> {
    let mut lines: Vec<String> = lib.lines().map(str::to_string).collect();
    let start = lines
        .iter()
        .position(|l| l.trim_start().starts_with("library!("))
        .ok_or("library!( not found in lib.rs")?;
    let end = lines
        .iter()
        .skip(start)
        .position(|l| l.trim() == ");")
        .map(|i| i + start)
        .ok_or("end of library! not found")?;

    let prefix = format!("{module} {{");
    let existing = (start + 1..end).find(|&i| lines[i].trim_start().starts_with(&prefix));
    if let Some(i) = existing {
        let line = &lines[i];
        let open = line.find('{').unwrap();
        let close = line.rfind('}').ok_or("malformed event line in lib.rs")?;
        if line[open + 1..close].split(',').any(|q| q.trim() == quest) {
            return Err(format!("{module}::{quest} is already registered in lib.rs"));
        }
        let updated = format!("{}, {quest} {}", line[..close].trim_end(), &line[close..]);
        lines[i] = updated;
    } else {
        let last = end - 1;
        if last > start && !lines[last].trim_end().ends_with(',') {
            lines[last].push(',');
        }
        lines.insert(end, format!("    {module} {{ {quest} }}"));
    }

    let mut result = lines.join("\n");
    if lib.ends_with('\n') {
        result.push('\n');
    }
    Ok(result)
}

fn prompt(message: &str) {
    print!("{message}");
    io::stdout().flush().unwrap();
}

/// Reads lines until a line containing only `END` or EOF.
fn read_block() -> String {
    let mut lines = Vec::new();
    for line in io::stdin().lock().lines() {
        let line = line.unwrap_or_default();
        if line.trim() == "END" {
            break;
        }
        lines.push(line.trim_end_matches('\r').to_string());
    }
    lines.join("\n").trim_end_matches('\n').to_string()
}

fn read_line() -> String {
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line).unwrap_or_default();
    line.trim().to_string()
}

/// Renders a sample as a Rust string literal, using the `"\` multi-line style when possible.
fn literal(sample: &str) -> String {
    if sample.contains('\n') && !sample.starts_with(char::is_whitespace) {
        let escaped = sample.replace('\\', "\\\\").replace('"', "\\\"");
        format!("\"\\\n{escaped}\"")
    } else {
        format!("{sample:?}")
    }
}

fn skeleton(sample: &str, expected: &str) -> String {
    let input = literal(sample);
    format!(
        r#"pub fn part1(input: &str) -> String {{
    let _lines: Vec<&str> = input.lines().collect();
    String::new()
}}

pub fn part2(_input: &str) -> String {{
    String::new()
}}

pub fn part3(_input: &str) -> String {{
    String::new()
}}

#[cfg(test)]
mod tests {{
    use super::*;

    #[test]
    fn test_part1() {{
        let input = {input};
        assert_eq!(part1(input), {expected:?});
    }}
}}
"#
    )
}

fn run(module: &str, quest: &str) -> Result<(), String> {
    if !valid_module(module) {
        return Err(format!("Invalid event \"{module}\", expected e.g. s03 or e2026"));
    }
    if !valid_quest(quest) {
        return Err(format!("Invalid quest \"{quest}\", expected e.g. q01"));
    }

    let source = Path::new("src").join(module).join(format!("{quest}.rs"));
    if source.exists() {
        return Err(format!("{} already exists", source.display()));
    }
    let lib = std::fs::read_to_string(LIB_PATH).map_err(|e| format!("Unable to read {LIB_PATH}: {e}"))?;
    let new_lib = update_lib(&lib, module, quest)?;

    println!("Paste the sample input for part 1, then finish with a line containing only END:");
    let sample = read_block();
    prompt("Expected output for part 1: ");
    let expected = read_line();

    std::fs::create_dir_all(source.parent().unwrap()).map_err(|e| format!("Unable to create directory: {e}"))?;
    std::fs::write(&source, skeleton(&sample, &expected))
        .map_err(|e| format!("Unable to write {}: {e}", source.display()))?;
    std::fs::write(LIB_PATH, new_lib).map_err(|e| format!("Unable to write {LIB_PATH}: {e}"))?;

    println!("Created {} and registered it in {LIB_PATH}", source.display());
    Ok(())
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("USAGE: cargo boilerplate <s03|e2026> <q01>");
        return ExitCode::FAILURE;
    }
    match run(&args[1], &args[2]) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("ERROR {err}");
            ExitCode::FAILURE
        }
    }
}
