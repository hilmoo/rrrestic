# RRRESTIC

An opinionated Restic wrapper written in Rust with OTEL integration

## Configuration

By default, the configuration path is `rrrestic.toml`. Use `RRRESTIC_CONFIG` to override it. You must initialize the repository manually first!

Example configuration:
```toml
[[backup]]
name = "backup"
source = "test-local/target/"
extra_args = ["--exclude-larger-than", "1M"]

env.RESTIC_REPOSITORY = "test-local/repo/"
env.RESTIC_PASSWORD = "${RESTIC_PASSWORD}"

before = ["echo Starting backup job.", "date"]
after = ["echo Backup job finished.", "date"]
success = ["echo Backup job completed successfully.", "date"]
failure = ["echo Backup job failed.", "date"]
```

## License

MIT License