// LaTeX compilation. Runs the system's latexmk (falling back to pdflatex) on
// the project's main document, surfaces the compile log, and parses it into
// structured diagnostics the editor can jump to. Compiling happens in-process
// with the app's own privileges, in the project directory, so build/auxiliary
// artifacts land next to the sources (and stay hidden by the explorer's
// source-file allowlist).
use serde::Serialize;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;
use std::{env, fs};

// Directories we never scan when hunting for the main document — mirrors the
// explorer/search skip list.
const SKIP_DIRS: &[&str] = &[".git", "node_modules", ".texpadtmp", "auxil"];

// TeX distributions install outside the PATH a GUI app inherits on macOS
// (Finder-launched apps don't get the shell's PATH). Prepend the usual homes so
// `latexmk`/`pdflatex` resolve whether launched from a terminal or the dock.
const TEX_BIN_DIRS: &str = "/Library/TeX/texbin:/usr/local/bin:/opt/homebrew/bin:/usr/bin";

// Keep the log payload sane over IPC; LaTeX logs are rarely larger, but a
// runaway package can spew megabytes. We keep the tail, where the error summary
// and `Output written` line live.
const MAX_LOG_LEN: usize = 200_000;
const MAX_DIAGNOSTICS: usize = 200;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Diagnostic {
    pub severity: String,     // "error" | "warning"
    pub file: Option<String>, // relative to the project root when known
    pub line: Option<usize>,  // 1-based
    pub message: String,
    pub hint: Option<String>, // plain-language explanation for common errors
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CompileResult {
    pub success: bool,
    pub pdf_path: Option<String>, // absolute path to the produced PDF, if any
    pub main_file: String,        // relative path of the document we compiled
    pub log: String,              // full compiler log (truncated to MAX_LOG_LEN)
    pub diagnostics: Vec<Diagnostic>,
    pub duration_ms: u64,
}

// A Command with the TeX bin dirs prepended to PATH so the compiler resolves
// regardless of how the app was launched.
fn tex_command(program: &str) -> Command {
    let mut cmd = Command::new(program);
    let path = match env::var("PATH") {
        Ok(existing) => format!("{TEX_BIN_DIRS}:{existing}"),
        Err(_) => TEX_BIN_DIRS.to_string(),
    };
    cmd.env("PATH", path);
    cmd
}

// The first available compiler, preferring latexmk (it runs the right number of
// passes for refs/citations on its own).
fn pick_compiler() -> Option<&'static str> {
    ["latexmk", "pdflatex"].into_iter().find(|c| {
        tex_command(c)
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    })
}

fn has_documentclass(path: &Path) -> bool {
    fs::read_to_string(path)
        .map(|c| c.contains("\\documentclass"))
        .unwrap_or(false)
}

fn collect_tex(dir: &Path, out: &mut Vec<PathBuf>) {
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_dir() {
            let skip = path
                .file_name()
                .map(|n| SKIP_DIRS.contains(&n.to_string_lossy().as_ref()))
                .unwrap_or(false);
            if !skip {
                collect_tex(&path, out);
            }
        } else if path.extension().and_then(|e| e.to_str()) == Some("tex") {
            out.push(path);
        }
    }
}

// Resolve the document to compile: an explicit hint if it exists, otherwise the
// best `\documentclass`-bearing `.tex` (preferring one literally named
// main.tex, then the shallowest).
fn find_main(root: &Path, hint: Option<&str>) -> Option<PathBuf> {
    if let Some(h) = hint {
        let p = root.join(h);
        if p.is_file() {
            return Some(p);
        }
    }

    let mut candidates = Vec::new();
    collect_tex(root, &mut candidates);

    let with_class: Vec<PathBuf> = candidates
        .iter()
        .filter(|p| has_documentclass(p))
        .cloned()
        .collect();
    let mut pool = if with_class.is_empty() {
        candidates
    } else {
        with_class
    };

    if let Some(main) = pool
        .iter()
        .find(|p| p.file_name().map(|n| n == "main.tex").unwrap_or(false))
    {
        return Some(main.clone());
    }
    pool.sort_by_key(|p| p.components().count());
    pool.into_iter().next()
}

// Present a log path relative to the project root, dropping any leading "./".
fn normalize_file(raw: &str, root: &Path) -> String {
    let raw = raw.trim();
    let pb = PathBuf::from(raw);
    let rel = pb.strip_prefix(root).unwrap_or(&pb);
    rel.to_string_lossy().trim_start_matches("./").to_string()
}

// The trailing "... on input line 42." many warnings carry.
fn extract_input_line(s: &str) -> Option<usize> {
    let idx = s.find("input line ")?;
    let digits: String = s[idx + "input line ".len()..]
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect();
    digits.parse().ok()
}

// Plain-language explanations for the errors people actually hit, keyed by a
// distinctive substring of the TeX message.
fn hint_for(message: &str) -> Option<String> {
    let m = message.to_lowercase();
    let hint = if m.contains("undefined control sequence") {
        "A command isn't recognized — check for a typo, or a missing \\usepackage that defines it."
    } else if m.contains("missing $") {
        "Math content appeared outside math mode — wrap it in $…$ or \\(…\\)."
    } else if m.contains("missing \\begin{document}") {
        "Text appeared before \\begin{document} — move it into the document body or the preamble."
    } else if m.contains("environment") && m.contains("undefined") {
        "An environment name is unknown — check the spelling or the package that provides it."
    } else if m.contains("\\begin") && m.contains("ended by") {
        "A \\begin{…} and \\end{…} don't match — check for a mismatched or unclosed environment."
    } else if m.contains("runaway argument") {
        "A brace or bracket is unbalanced — look for an unclosed { or a missing }."
    } else if m.contains("reference") && m.contains("undefined") {
        "A \\ref points at a label that doesn't exist yet — check the key, then recompile (references need a second pass)."
    } else if m.contains("citation") && m.contains("undefined") {
        "A \\cite has no matching .bib entry — check the key and that the bibliography ran."
    } else if m.contains("not found") {
        "A referenced file is missing — check the \\input/\\include/\\includegraphics path, relative to the project root."
    } else if m.contains("too many }") || m.contains("extra }") {
        "There's an extra } — remove it or add the { it was meant to close."
    } else if m.contains("emergency stop") {
        "The compiler gave up — fix the first real error above; the rest are usually knock-on effects."
    } else {
        return None;
    };
    Some(hint.to_string())
}

// A candidate error of the `-file-line-error` form: `path:line: message`.
fn parse_file_line_error(line: &str, root: &Path) -> Option<Diagnostic> {
    let (path_part, rest) = line.split_once(':')?;
    // The path must look like a file (has an extension) to avoid matching prose
    // such as "Package hyperref Warning: ...".
    if !path_part.contains('.') {
        return None;
    }
    let (num, msg) = rest.split_once(':')?;
    let line_no: usize = num.trim().parse().ok()?;
    let message = msg.trim().trim_end_matches('.').trim().to_string();
    if message.is_empty() {
        return None;
    }
    Some(Diagnostic {
        severity: "error".to_string(),
        file: Some(normalize_file(path_part, root)),
        line: Some(line_no),
        hint: hint_for(&message),
        message,
    })
}

// Append `d` unless an identical (message, file, line) was already recorded.
fn push_unique(out: &mut Vec<Diagnostic>, d: Diagnostic) {
    let dup = out
        .iter()
        .any(|e| e.message == d.message && e.file == d.file && e.line == d.line);
    if !dup {
        out.push(d);
    }
}

fn parse_diagnostics(log: &str, root: &Path) -> Vec<Diagnostic> {
    let mut out: Vec<Diagnostic> = Vec::new();

    for line in log.lines() {
        if out.len() >= MAX_DIAGNOSTICS {
            break;
        }

        if let Some(d) = parse_file_line_error(line, root) {
            push_unique(&mut out, d);
            continue;
        }

        // Bare TeX error, e.g. "! Undefined control sequence." — carries no
        // file/line under -file-line-error but the message is worth surfacing.
        if let Some(rest) = line.strip_prefix("! ") {
            let message = rest.trim().trim_end_matches('.').trim().to_string();
            if !message.is_empty() {
                push_unique(
                    &mut out,
                    Diagnostic {
                        severity: "error".to_string(),
                        file: None,
                        line: None,
                        hint: hint_for(&message),
                        message,
                    },
                );
            }
            continue;
        }

        // Warnings from LaTeX core or any package: "… Warning: message … input line N."
        if let Some(idx) = line.find("Warning:") {
            let message = line[idx + "Warning:".len()..]
                .trim()
                .trim_end_matches('.')
                .trim()
                .to_string();
            if !message.is_empty() {
                push_unique(
                    &mut out,
                    Diagnostic {
                        severity: "warning".to_string(),
                        file: None,
                        line: extract_input_line(line),
                        hint: hint_for(&message),
                        message,
                    },
                );
            }
        }
    }

    out
}

fn truncate_tail(s: String) -> String {
    if s.len() <= MAX_LOG_LEN {
        return s;
    }
    let start = s.len() - MAX_LOG_LEN;
    // Snap to a char boundary so we never split a UTF-8 sequence.
    let start = (start..s.len())
        .find(|&i| s.is_char_boundary(i))
        .unwrap_or(s.len());
    format!("… (log truncated) …\n{}", &s[start..])
}

/// Compile the project's main document to PDF and return the log + parsed
/// diagnostics. `main_file` is an optional relative hint; when omitted the main
/// document is auto-detected.
#[tauri::command]
pub fn compile_project(root: String, main_file: Option<String>) -> Result<CompileResult, String> {
    let root_path = PathBuf::from(&root);
    if !root_path.is_dir() {
        return Err(format!("Not a directory: {root}"));
    }

    let main = find_main(&root_path, main_file.as_deref())
        .ok_or_else(|| "No main .tex file found in this project".to_string())?;
    let main_rel = main
        .strip_prefix(&root_path)
        .unwrap_or(&main)
        .to_string_lossy()
        .into_owned();
    let stem = main
        .file_stem()
        .map(|s| s.to_string_lossy().into_owned())
        .ok_or_else(|| "Main file has no name".to_string())?;

    let compiler = pick_compiler().ok_or_else(|| {
        "No LaTeX compiler found. Install a TeX distribution (e.g. MacTeX or TeX Live) that \
         provides latexmk or pdflatex."
            .to_string()
    })?;

    let started = Instant::now();
    let mut cmd = tex_command(compiler);
    cmd.current_dir(&root_path);
    match compiler {
        "latexmk" => {
            cmd.args([
                "-pdf",
                "-interaction=nonstopmode",
                "-file-line-error",
                "-synctex=1",
            ]);
        }
        _ => {
            cmd.args(["-interaction=nonstopmode", "-file-line-error", "-synctex=1"]);
        }
    }
    cmd.arg(&main_rel);

    let output = cmd
        .output()
        .map_err(|e| format!("Failed to run {compiler}: {e}"))?;
    let duration_ms = started.elapsed().as_millis() as u64;

    // The .log file is the canonical, detailed record; fall back to the
    // captured streams if it's missing (e.g. the compiler never started).
    let log_path = root_path.join(format!("{stem}.log"));
    let file_log = fs::read_to_string(&log_path).unwrap_or_default();
    let log = if file_log.trim().is_empty() {
        format!(
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )
    } else {
        file_log
    };

    let diagnostics = parse_diagnostics(&log, &root_path);

    let pdf = root_path.join(format!("{stem}.pdf"));
    let pdf_path = pdf.is_file().then(|| pdf.to_string_lossy().into_owned());

    Ok(CompileResult {
        success: output.status.success(),
        pdf_path,
        main_file: main_rel,
        log: truncate_tail(log),
        diagnostics,
        duration_ms,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn finds_main_tex_by_documentclass() {
        let base = env::temp_dir().join(format!("bib-compile-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        let sections = base.join("sections");
        fs::create_dir_all(&sections).unwrap();

        fs::write(sections.join("intro.tex"), "Just a fragment.").unwrap();
        fs::write(
            base.join("paper.tex"),
            "\\documentclass{article}\n\\begin{document}\\end{document}",
        )
        .unwrap();

        let main = find_main(&base, None).unwrap();
        assert_eq!(main.file_name().unwrap(), "paper.tex");

        let _ = fs::remove_dir_all(&base);
    }

    #[test]
    fn prefers_main_tex_by_name() {
        let base = env::temp_dir().join(format!("bib-compile-main-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(&base).unwrap();
        fs::write(base.join("other.tex"), "\\documentclass{article}").unwrap();
        fs::write(base.join("main.tex"), "\\documentclass{article}").unwrap();

        let main = find_main(&base, None).unwrap();
        assert_eq!(main.file_name().unwrap(), "main.tex");

        let _ = fs::remove_dir_all(&base);
    }

    #[test]
    fn parses_errors_and_warnings_with_hints() {
        let root = PathBuf::from("/proj");
        let log = "\
./main.tex:12: Undefined control sequence.
! Missing $ inserted.
LaTeX Warning: Reference `fig:one' on page 1 undefined on input line 30.
Package hyperref Warning: Token not allowed on input line 5.
This is just noise: 42 not an error.";

        let diags = parse_diagnostics(log, &root);

        // file:line error, mapped relative, with a hint
        let e = &diags[0];
        assert_eq!(e.severity, "error");
        assert_eq!(e.file.as_deref(), Some("main.tex"));
        assert_eq!(e.line, Some(12));
        assert!(e.hint.is_some());

        // bare "! …" error
        assert!(diags
            .iter()
            .any(|d| d.message.contains("Missing $") && d.hint.is_some()));

        // warning with input line captured
        let w = diags
            .iter()
            .find(|d| d.severity == "warning" && d.message.contains("Reference"))
            .unwrap();
        assert_eq!(w.line, Some(30));

        // the "noise" line must not be misparsed as an error
        assert!(!diags.iter().any(|d| d.message.contains("not an error")));
    }

    #[test]
    fn truncate_tail_keeps_end_on_char_boundary() {
        let s = "é".repeat(MAX_LOG_LEN); // 2 bytes each → exceeds the cap
        let out = truncate_tail(s);
        assert!(out.starts_with("… (log truncated) …"));
        // Round-trips as valid UTF-8 (no panic on a split multibyte char).
        assert!(out.len() <= MAX_LOG_LEN + 64);
    }

    #[test]
    fn rejects_non_directory() {
        assert!(compile_project("/no/such/dir".to_string(), None).is_err());
    }

    // Real end-to-end build against the system TeX toolchain. Ignored by default
    // (needs latexmk/pdflatex installed); run with `cargo test -- --ignored`.
    #[test]
    #[ignore]
    fn compiles_a_real_document() {
        let base = env::temp_dir().join(format!("bib-real-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(&base).unwrap();
        fs::write(
            base.join("main.tex"),
            "\\documentclass{article}\n\\begin{document}\nHello, world.\n\\end{document}\n",
        )
        .unwrap();

        let res = compile_project(base.to_string_lossy().into_owned(), None).unwrap();
        assert!(res.success, "expected a clean build, log:\n{}", res.log);
        assert_eq!(res.main_file, "main.tex");
        let pdf = res.pdf_path.expect("a PDF should have been produced");
        assert!(PathBuf::from(&pdf).is_file());
        assert!(res.diagnostics.iter().all(|d| d.severity != "error"));

        // A broken document surfaces an error diagnostic and fails.
        fs::write(
            base.join("bad.tex"),
            "\\documentclass{article}\n\\begin{document}\n\\undefinedcmd\n\\end{document}\n",
        )
        .unwrap();
        let res = compile_project(
            base.to_string_lossy().into_owned(),
            Some("bad.tex".to_string()),
        )
        .unwrap();
        assert!(!res.success);
        assert!(
            res.diagnostics.iter().any(|d| d.severity == "error"),
            "expected an error diagnostic, got: {:?}",
            res.diagnostics
                .iter()
                .map(|d| &d.message)
                .collect::<Vec<_>>()
        );

        let _ = fs::remove_dir_all(&base);
    }
}
