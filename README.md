```bash
set -a
source .env
set +a
cargo test initialize_devnet -- --nocapture

```

to run devnet test after deployments with .env
