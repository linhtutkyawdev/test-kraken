# use shuttle.rs

```
cargo install cargo-shuttle -f
cargo shuttle init
cargo watch -x "shuttle run"
```

# askama templating with axum for htmx

```
cargo add askama -F askama/with-axum askama-axum
```

# add tailwindcss

<!-- https://github.com/tailwindlabs/tailwindcss/releases/tag/v3.3.5 -->

```
curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/download/v3.3.5/tailwindcss-linux-arm64
chmod +x tailwindcss-linux-arm64
mv tailwindcss-linux-arm64 /bin/tailwindcss
tailwindcss -i ./styles/styles.css -o ./styles/tailwind.css --watch
```

# turso

```
cargo add libsql-client
```
