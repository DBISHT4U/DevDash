use std::process::Command;

pub struct GitInfo {
    pub is_repo: bool,
    pub branch: String,
    pub remote_url: String,
    pub last_commit: String,
    pub last_author: String,
    pub last_date: String,
    pub commit_count: String,
    pub staged: usize,
    pub modified: usize,
    pub untracked: usize,
    pub ahead: usize,
    pub behind: usize,
    pub recent_commits: Vec<CommitEntry>,
}

pub struct CommitEntry {
    pub hash: String,
    pub message: String,
    pub author: String,
    pub date: String,
}

impl GitInfo {
    pub fn collect() -> Self {
        let is_repo = run_git(&["rev-parse", "--is-inside-work-tree"])
            .map(|s| s.trim() == "true")
            .unwrap_or(false);

        if !is_repo {
            return Self::empty();
        }

        let branch = run_git(&["rev-parse", "--abbrev-ref", "HEAD"])
            .unwrap_or_else(|| "unknown".to_string())
            .trim()
            .to_string();

        let remote_url = run_git(&["config", "--get", "remote.origin.url"])
            .unwrap_or_else(|| "—".to_string())
            .trim()
            .to_string();

        let last_commit = run_git(&["log", "-1", "--pretty=%s"])
            .unwrap_or_else(|| "—".to_string())
            .trim()
            .to_string();

        let last_author = run_git(&["log", "-1", "--pretty=%an"])
            .unwrap_or_else(|| "—".to_string())
            .trim()
            .to_string();

        let last_date = run_git(&["log", "-1", "--pretty=%ar"])
            .unwrap_or_else(|| "—".to_string())
            .trim()
            .to_string();

        let commit_count = run_git(&["rev-list", "--count", "HEAD"])
            .unwrap_or_else(|| "0".to_string())
            .trim()
            .to_string();

        // Git status
        let status_output = run_git(&["status", "--porcelain"]).unwrap_or_default();
        let mut staged = 0;
        let mut modified = 0;
        let mut untracked = 0;
        for line in status_output.lines() {
            if line.len() < 2 {
                continue;
            }
            let index = line.chars().next().unwrap_or(' ');
            let worktree = line.chars().nth(1).unwrap_or(' ');
            if index != ' ' && index != '?' {
                staged += 1;
            }
            if worktree == 'M' || worktree == 'D' {
                modified += 1;
            }
            if index == '?' {
                untracked += 1;
            }
        }

        // Ahead/behind
        let (ahead, behind) = get_ahead_behind();

        // Recent commits
        let log_output = run_git(&[
            "log",
            "--oneline",
            "--pretty=format:%h|%s|%an|%ar",
            "-10",
        ])
        .unwrap_or_default();

        let recent_commits: Vec<CommitEntry> = log_output
            .lines()
            .filter_map(|line| {
                let parts: Vec<&str> = line.splitn(4, '|').collect();
                if parts.len() == 4 {
                    Some(CommitEntry {
                        hash: parts[0].to_string(),
                        message: parts[1].to_string(),
                        author: parts[2].to_string(),
                        date: parts[3].to_string(),
                    })
                } else {
                    None
                }
            })
            .collect();

        GitInfo {
            is_repo,
            branch,
            remote_url,
            last_commit,
            last_author,
            last_date,
            commit_count,
            staged,
            modified,
            untracked,
            ahead,
            behind,
            recent_commits,
        }
    }

    fn empty() -> Self {
        GitInfo {
            is_repo: false,
            branch: "—".to_string(),
            remote_url: "—".to_string(),
            last_commit: "Not a git repository".to_string(),
            last_author: "—".to_string(),
            last_date: "—".to_string(),
            commit_count: "0".to_string(),
            staged: 0,
            modified: 0,
            untracked: 0,
            ahead: 0,
            behind: 0,
            recent_commits: vec![],
        }
    }

    pub fn status_summary(&self) -> String {
        let mut parts = Vec::new();
        if self.staged > 0 {
            parts.push(format!("{}+ staged", self.staged));
        }
        if self.modified > 0 {
            parts.push(format!("{}~ modified", self.modified));
        }
        if self.untracked > 0 {
            parts.push(format!("{}? untracked", self.untracked));
        }
        if parts.is_empty() {
            "✓ clean".to_string()
        } else {
            parts.join("  ")
        }
    }
}

fn run_git(args: &[&str]) -> Option<String> {
    Command::new("git")
        .args(args)
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() {
                String::from_utf8(o.stdout).ok()
            } else {
                None
            }
        })
}

fn get_ahead_behind() -> (usize, usize) {
    let output = run_git(&["rev-list", "--left-right", "--count", "HEAD...@{upstream}"]);
    match output {
        Some(s) => {
            let parts: Vec<&str> = s.trim().split('\t').collect();
            if parts.len() == 2 {
                let ahead = parts[0].parse().unwrap_or(0);
                let behind = parts[1].parse().unwrap_or(0);
                (ahead, behind)
            } else {
                (0, 0)
            }
        }
        None => (0, 0),
    }
}
