use crate::config::BackupJob;
use std::process::Command;
use tracing::{debug, error, info, warn};

#[derive(serde::Deserialize)]
pub struct Summary {
    pub message_type: String, // "summary"
    pub dry_run: Option<bool>,

    pub files_new: u64,
    pub files_changed: u64,
    pub files_unmodified: u64,

    pub dirs_new: u64,
    pub dirs_changed: u64,
    pub dirs_unmodified: u64,

    pub data_blobs: i64,
    pub tree_blobs: i64,

    pub data_added: u64,
    pub data_added_packed: u64,

    pub total_files_processed: u64,
    pub total_bytes_processed: u64,

    pub backup_start: chrono::DateTime<chrono::Utc>,
    pub backup_end: chrono::DateTime<chrono::Utc>,

    pub total_duration: f64,

    pub snapshot_id: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct Error {
    pub message_type: String, // "error"
    pub error: ErrorMessage,
    pub during: Option<String>,
    pub item: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct ErrorMessage {
    pub message: Option<String>,
}

pub fn run_job(job: &BackupJob) {
    info!(job = job.name, "Starting backup job");

    if let Some(hooks) = &job.before {
        for cmd in hooks {
            run_hook(cmd, "before");
        }
    }

    let mut command = Command::new("restic");
    command.args(["-r", &job.repository, "backup", &job.source, "--json"]);

    if let Some(args) = &job.extra_args {
        command.args(args);
    }

    if let Some(env_vars) = &job.env {
        command.envs(env_vars);
    }

    match command.output() {
        Ok(result) => {
            if result.status.success() {
                let stdout = String::from_utf8_lossy(&result.stdout);
                for line in stdout.lines() {
                    debug!("{}", line);
                }

                let summary_str = stdout.lines().last().unwrap_or("No output");

                let summary: Summary = match serde_json::from_str(summary_str) {
                    Ok(s) => s,
                    Err(err) => {
                        error!(job = job.name, error = %err, "Failed to parse backup summary");
                        Summary {
                            message_type: "invalid".to_string(),
                            dry_run: None,
                            files_new: 0,
                            files_changed: 0,
                            files_unmodified: 0,
                            dirs_new: 0,
                            dirs_changed: 0,
                            dirs_unmodified: 0,
                            data_blobs: 0,
                            tree_blobs: 0,
                            data_added: 0,
                            data_added_packed: 0,
                            total_files_processed: 0,
                            total_bytes_processed: 0,
                            backup_start: chrono::Utc::now(),
                            backup_end: chrono::Utc::now(),
                            total_duration: 0.0,
                            snapshot_id: None,
                        }
                    }
                };

                if summary.message_type == "summary" {
                    info!(
                        job = job.name,
                        summary_message_type = %summary.message_type,
                        summary_dry_run = summary.dry_run.unwrap_or(false),
                        summary_files_new = summary.files_new,
                        summary_files_changed = summary.files_changed,
                        summary_files_unmodified = summary.files_unmodified,
                        summary_dirs_new = summary.dirs_new,
                        summary_dirs_changed = summary.dirs_changed,
                        summary_dirs_unmodified = summary.dirs_unmodified,
                        summary_data_blobs = summary.data_blobs,
                        summary_tree_blobs = summary.tree_blobs,
                        summary_data_added = summary.data_added,
                        summary_data_added_packed = summary.data_added_packed,
                        summary_total_files_processed = summary.total_files_processed,
                        summary_total_bytes_processed = summary.total_bytes_processed,
                        summary_backup_start = summary.backup_start.to_rfc3339(),
                        summary_backup_end = summary.backup_end.to_rfc3339(),
                        summary_total_duration = summary.total_duration,
                        summary_snapshot_id = ?summary.snapshot_id.unwrap_or("N/A".to_string()),
                        "Backup successful"
                    );
                } else {
                    warn!(
                        job = job.name,
                        stdout = summary_str,
                        "Unexpected message type in summary"
                    );
                }
                run_hook_group(&job.after, "after");
                run_hook_group(&job.success, "success");
            } else {
                let stderr = String::from_utf8_lossy(&result.stderr);
                let err_str = stderr.lines().last().unwrap_or("No output");
                match serde_json::from_str::<Error>(err_str) {
                    Ok(error) => {
                        error!(
                            job = job.name,
                            status = ?result.status,
                            error_message = %error.error.message.unwrap_or("N/A".to_string()),
                            error_during= error.during.unwrap_or("N/A".to_string()),
                            error_item= error.item.unwrap_or("N/A".to_string()),
                            error_message_type = %error.message_type,
                            "Backup failed"
                        );
                    }
                    Err(err) => {
                        error!(job = job.name, error = %err, stderr=err_str, "Failed to parse error message");
                    }
                };

                run_hook_group(&job.after, "after");
                run_hook_group(&job.failure, "failure");
            }
        }
        Err(e) => {
            error!(job = job.name, error = %e, "Failed to spawn restic command");
            run_hook_group(&job.after, "after");
            run_hook_group(&job.failure, "failure");
        }
    }
}

fn run_hook_group(hooks: &Option<Vec<String>>, stage: &str) {
    if let Some(cmds) = hooks {
        for cmd in cmds {
            run_hook(cmd, stage);
        }
    }
}

fn run_hook(cmd: &str, stage: &str) {
    info!(stage = stage, cmd = cmd, "Executing hook");
    let output = Command::new("sh").arg("-c").arg(cmd).output();

    match output {
        Ok(out) => {
            if !out.status.success() {
                warn!(stage = stage, cmd = cmd, stderr = %String::from_utf8_lossy(&out.stderr), "Hook failed");
            }
        }
        Err(e) => error!(stage = stage, cmd = cmd, error = %e, "Failed to execute hook"),
    }
}
