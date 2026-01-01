# RRRESTIC

An opinionated Restic wrapper written in Rust with OTEL integration

## Configuration

By default, the configuration path is `rrrestic.toml`. Use `RRRESTIC_CONFIG` to override it. You must initialize the repository manually first!

Example configuration:
```toml
[[backup]]
name = "backup"
repository = "test-local/repo/"
source = "test-local/target/"

env.RESTIC_PASSWORD = "${RESTIC_PASSWORD}"

before = ["echo Starting backup job.", "date"]
after = ["echo Backup job finished.", "date"]
success = ["echo Backup job completed successfully.", "date"]
failure = ["echo Backup job failed.", "date"]

extra_args = ["--exclude-larger-than", "1M"]
```

## License

MIT License