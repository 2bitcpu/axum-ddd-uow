
```
cargo add tokio --features macros,rt-multi-thread,signal --no-default-features
cargo add serde --features derive --no-default-features
cargo add serde_json --features std --no-default-features
cargo add chrono --features serde,now --no-default-features
cargo add async-trait --no-default-features
cargo add sqlx --features runtime-tokio-rustls,chrono,derive,sqlite --no-default-features
cargo add axum --features macros
cargo add axum-extra --features typed-header --no-default-features
cargo add tower --features timeout --no-default-features
cargo add tower-http --features fs,cors --no-default-features
cargo add derive-new --no-default-features
cargo add libsqlite3-sys@^0.30.1 --optional --no-default-features

cargo add validator --features derive --no-default-features
cargo add axum-valid --features basic,form,json,query,validator --no-default-features
```

```
cat << EOS >> Cargo.toml

simple-jwt = { git = "https://github.com/2bitcpu/simple-jwt" }
async-argon2 = { git = "https://github.com/2bitcpu/async-argon2" }

[profile.release]
opt-level = "z"
debug = false
lto = true
strip = true
codegen-units = 1
panic = "abort"
EOS
```
