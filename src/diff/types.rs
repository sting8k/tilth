//! Diff value types — pure data, no logic.
//!
//! Extracted from `diff/mod.rs` so types live separate from
//! resolution + computation code.

use std::path::PathBuf;

use crate::types::OutlineKind;


#[derive(Debug)]
pub enum DiffSource {
    GitUncommitted,
    GitStaged,
    GitRef(String),
    Files(PathBuf, PathBuf),
    Patch(PathBuf),
    Log(String),
}

#[derive(Debug)]
pub struct FileDiff {
    pub path: PathBuf,
    pub old_path: Option<PathBuf>,
    pub status: FileStatus,
    pub hunks: Vec<Hunk>,
    pub is_generated: bool,
    pub is_binary: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileStatus {
    Added,
    Modified,
    Deleted,
    Renamed,
}

#[derive(Debug)]
pub struct Hunk {
    pub old_start: u32,
    pub old_count: u32,
    pub new_start: u32,
    pub new_count: u32,
    pub lines: Vec<DiffLine>,
}

#[derive(Debug)]
pub struct DiffLine {
    pub kind: DiffLineKind,
    pub content: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffLineKind {
    Context,
    Added,
    Removed,
}

#[derive(Debug)]
pub struct DiffSymbol {
    pub entry: crate::types::OutlineEntry,
    pub identity: SymbolIdentity,
    pub content_hash: u64,
    pub structural_hash: u64,
    pub source_text: String,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct SymbolIdentity {
    pub kind: OutlineKind,
    pub parent_path: String,
    pub name: String,
}

#[derive(Debug)]
pub struct SymbolChange {
    pub name: String,
    pub kind: OutlineKind,
    pub change: ChangeType,
    pub match_confidence: MatchConfidence,
    pub line: u32,
    pub old_sig: Option<String>,
    pub new_sig: Option<String>,
    pub size_delta: Option<(u32, u32)>,
}

#[derive(Debug, Clone)]
pub enum ChangeType {
    Added,
    Deleted,
    BodyChanged,
    SignatureChanged,
    Renamed { old_name: String },
    Moved { old_path: PathBuf },
    RenamedAndMoved { old_name: String, old_path: PathBuf },
    Unchanged,
}

#[derive(Debug, Clone)]
pub enum MatchConfidence {
    Exact,
    Structural,
    Fuzzy(f32),
    Ambiguous(u32),
}

#[derive(Debug)]
pub struct FileOverlay {
    pub path: PathBuf,
    pub symbol_changes: Vec<SymbolChange>,
    pub attributed_hunks: Vec<(String, Vec<DiffLine>)>,
    pub conflicts: Vec<Conflict>,
    pub new_content: Option<String>,
}

#[derive(Debug)]
pub struct Conflict {
    pub line: u32,
    pub ours: String,
    pub theirs: String,
    pub enclosing_fn: Option<String>,
}

#[derive(Debug)]
pub struct CommitSummary {
    pub hash: String,
    pub timestamp: i64,
    pub message: String,
    pub author: String,
    pub overlays: Vec<FileOverlay>,
}

