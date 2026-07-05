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

fn is_source_file(path: &Path) -> bool {
    match path.extension().and_then(|e| e.to_str()) {
        Some(ext) => SOURCE_EXTS.contains(&ext.to_ascii_lowercase().as_str()),
        None => false,
    }
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
}
