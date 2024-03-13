Start static development container
```fish
docker run -d --rm -v (pwd):/app -w /app -e CARGO_HOME=/app/.cargo --network host --name rust-arb -e HOME=/tmp -v (pwd)/.fish_history:/tmp/.local/share/fish/fish_history rust:1.66.0 sh -c 'apt update && apt install -y fish && sleep 360000000
```

Connect to development container
```fish
docker exec -it -u 1000:1000 rust-arb fish
```
