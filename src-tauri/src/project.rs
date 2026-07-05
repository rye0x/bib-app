// Filesystem access for the editor. These run in-process with the app's own
// privileges, so the frontend can read a project tree and open/save files
// without granting broad `fs:` scope permissions. The source-file filter
// lives here so there is a single source of truth for what the explorer shows.
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};

// Extensions we surface in the explorer — an allowlist, so build/aux artifacts
// (.aux, .log, .out, .toc, .synctex.gz, .fls, .fdb_latexmk, …) fall through and
// stay hidden without needing to enumerate them.
const SOURCE_EXTS: &[&str] = &[
    "tex", "bib", "cls", "sty", "png", "jpg", "jpeg", "gif", "svg", "pdf", "eps",
];

// The subset of sources that hold searchable text (project-wide search).
const TEXT_EXTS: &[&str] = &["tex", "bib", "cls", "sty"];

// Hard caps so a search over a huge project can't hang the UI or flood the IPC.
const MAX_MATCHES: usize = 1000;
const MAX_LINE_LEN: usize = 400;

// Directories we never descend into.
const SKIP_DIRS: &[&str] = &[".git", "node_modules", ".texpadtmp", "auxil"];

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TreeNode {
    pub name: String,
    pub path: String, // absolute
    pub is_dir: bool,
    pub children: Option<Vec<TreeNode>>, // Some for dirs, None for files
}

fn has_ext_in(path: &Path, exts: &[&str]) -> bool {
    match path.extension().and_then(|e| e.to_str()) {
        Some(ext) => exts.contains(&ext.to_ascii_lowercase().as_str()),
        None => false,
    }
}

fn is_source_file(path: &Path) -> bool {
    has_ext_in(path, SOURCE_EXTS)
}

fn build_node(path: &Path) -> Option<TreeNode> {
    let name = path.file_name()?.to_string_lossy().into_owned();

    if path.is_dir() {
        if SKIP_DIRS.contains(&name.as_str()) {
            return None;
        }
        let mut children: Vec<TreeNode> = fs::read_dir(path)
            .ok()?
            .filter_map(|e| e.ok())
            .filter_map(|e| build_node(&e.path()))
            .collect();
        // Prune directories that hold no source files after filtering.
        if children.is_empty() {
            return None;
        }
        children.sort_by(|a, b| {
            b.is_dir
                .cmp(&a.is_dir) // directories first
                .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
        });
        Some(TreeNode {
            name,
            path: path.to_string_lossy().into_owned(),
            is_dir: true,
            children: Some(children),
        })
    } else if is_source_file(path) {
        Some(TreeNode {
            name,
            path: path.to_string_lossy().into_owned(),
            is_dir: false,
            children: None,
        })
    } else {
        None
    }
}

#[tauri::command]
pub fn read_project_tree(root: String) -> Result<TreeNode, String> {
    let root_path = PathBuf::from(&root);
    if !root_path.is_dir() {
        return Err(format!("Not a directory: {root}"));
    }
    build_node(&root_path).ok_or_else(|| "No source files found in this folder".to_string())
}

#[tauri::command]
pub fn read_text_file(path: String) -> Result<String, String> {
    fs::read_to_string(&path).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn write_text_file(path: String, contents: String) -> Result<(), String> {
    fs::write(&path, contents).map_err(|e| e.to_string())
}

// One hit from a project-wide search: enough for the UI to show a preview line
// and jump to the exact spot when clicked.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchMatch {
    pub path: String,     // absolute
    pub rel_path: String, // relative to the search root
    pub line: usize,      // 1-based line number
    pub line_text: String,
    pub match_start: usize, // byte offset of the match within line_text
    pub match_len: usize,
}

// Collect searchable text files under `dir`, honouring the same skip list as
// the explorer so we never descend into .git / node_modules / build dirs.
fn collect_text_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_dir() {
            let name = path.file_name().map(|n| n.to_string_lossy().into_owned());
            if name
                .map(|n| SKIP_DIRS.contains(&n.as_str()))
                .unwrap_or(false)
            {
                continue;
            }
            collect_text_files(&path, out);
        } else if has_ext_in(&path, TEXT_EXTS) {
            out.push(path);
        }
    }
}

/// Case-insensitive plain-text search across the project's source files.
/// Returns up to `MAX_MATCHES` hits; an empty query yields nothing.
#[tauri::command]
pub fn search_project(root: String, query: String) -> Result<Vec<SearchMatch>, String> {
    let root_path = PathBuf::from(&root);
    if !root_path.is_dir() {
        return Err(format!("Not a directory: {root}"));
    }
    if query.is_empty() {
        return Ok(Vec::new());
    }

    let needle = query.to_lowercase();
    let mut files = Vec::new();
    collect_text_files(&root_path, &mut files);
    files.sort();

    let mut matches = Vec::new();
    'outer: for path in files {
        let contents = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(_) => continue, // skip binary/unreadable files
        };
        let rel_path = path
            .strip_prefix(&root_path)
            .unwrap_or(&path)
            .to_string_lossy()
            .into_owned();

        for (i, line) in contents.lines().enumerate() {
            if let Some(pos) = line.to_lowercase().find(&needle) {
                let line_text: String = line.chars().take(MAX_LINE_LEN).collect();
                // `pos` indexes the lowercased copy, but ASCII-case folding
                // preserves byte offsets, so it maps back to `line` directly.
                matches.push(SearchMatch {
                    path: path.to_string_lossy().into_owned(),
                    rel_path: rel_path.clone(),
                    line: i + 1,
                    match_start: pos.min(line_text.len()),
                    match_len: query.len(),
                    line_text,
                });
                if matches.len() >= MAX_MATCHES {
                    break 'outer;
                }
            }
        }
    }

    Ok(matches)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    /// Collect every file name in a tree (dirs excluded) for easy assertions.
    fn file_names(node: &TreeNode) -> Vec<String> {
        match &node.children {
            Some(children) => children.iter().flat_map(file_names).collect(),
            None => vec![node.name.clone()],
        }
    }

    fn dir_names(node: &TreeNode) -> Vec<String> {
        let mut names = Vec::new();
        if let Some(children) = &node.children {
            for child in children {
                if child.is_dir {
                    names.push(child.name.clone());
                    names.extend(dir_names(child));
                }
            }
        }
        names
    }

    #[test]
    fn tree_shows_sources_and_hides_artifacts() {
        // Unique temp project dir (no rand needed — pid is stable per process).
        let base = env::temp_dir().join(format!("bib-test-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        let sections = base.join("sections");
        let aux = base.join("aux-only");
        fs::create_dir_all(&sections).unwrap();
        fs::create_dir_all(&aux).unwrap();
        fs::create_dir_all(base.join(".git")).unwrap();

        // Source files (kept) + build artifacts (hidden).
        fs::write(base.join("main.tex"), "\\documentclass{article}").unwrap();
        fs::write(base.join("refs.bib"), "").unwrap();
        fs::write(base.join("figure.png"), "").unwrap();
        fs::write(base.join("main.aux"), "").unwrap();
        fs::write(base.join("main.log"), "").unwrap();
        fs::write(base.join("main.synctex.gz"), "").unwrap();
        fs::write(sections.join("intro.tex"), "").unwrap();
        fs::write(base.join(".git").join("config"), "").unwrap();
        // A directory containing only artifacts should be pruned entirely.
        fs::write(aux.join("main.fls"), "").unwrap();

        let tree = read_project_tree(base.to_string_lossy().into_owned()).unwrap();

        let files = file_names(&tree);
        assert!(files.contains(&"main.tex".to_string()));
        assert!(files.contains(&"refs.bib".to_string()));
        assert!(files.contains(&"figure.png".to_string()));
        assert!(files.contains(&"intro.tex".to_string()));
        assert!(!files.iter().any(|f| f.ends_with(".aux")));
        assert!(!files.iter().any(|f| f.ends_with(".log")));
        assert!(!files.iter().any(|f| f.ends_with(".synctex.gz")));
        assert!(!files.iter().any(|f| f.ends_with(".fls")));

        let dirs = dir_names(&tree);
        assert!(dirs.contains(&"sections".to_string()));
        assert!(
            !dirs.contains(&"aux-only".to_string()),
            "artifact-only dir should be pruned"
        );
        assert!(
            !dirs.contains(&".git".to_string()),
            ".git should be skipped"
        );

        let _ = fs::remove_dir_all(&base);
    }

    #[test]
    fn rejects_non_directory() {
        assert!(read_project_tree("/no/such/path/here".to_string()).is_err());
    }

    #[test]
    fn search_finds_matches_case_insensitively_and_skips_binaries() {
        let base = env::temp_dir().join(format!("bib-search-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        let sections = base.join("sections");
        fs::create_dir_all(&sections).unwrap();
        fs::create_dir_all(base.join(".git")).unwrap();

        fs::write(
            base.join("main.tex"),
            "\\section{Intro}\nThe THEOREM holds.",
        )
        .unwrap();
        fs::write(sections.join("body.tex"), "A theorem about theorems.").unwrap();
        // A non-text source and a skipped dir must be ignored.
        fs::write(base.join("figure.png"), "theorem").unwrap();
        fs::write(base.join(".git").join("config"), "theorem").unwrap();

        let hits =
            search_project(base.to_string_lossy().into_owned(), "theorem".to_string()).unwrap();

        // main.tex line 2 (one hit) + body.tex line 1 (find returns first only).
        assert_eq!(hits.len(), 2, "expected one hit per matching line");
        assert!(hits.iter().all(|h| h.rel_path.ends_with(".tex")));
        assert!(hits.iter().any(|h| h.rel_path == "main.tex" && h.line == 2));
        assert!(hits
            .iter()
            .any(|h| h.rel_path == format!("sections{}body.tex", std::path::MAIN_SEPARATOR)));

        // Empty query is a no-op.
        assert!(
            search_project(base.to_string_lossy().into_owned(), "".to_string())
                .unwrap()
                .is_empty()
        );

        let _ = fs::remove_dir_all(&base);
    }
}
