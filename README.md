<p align="center">
  <img src="logo.png" width="116" alt="craft-rpc logo" />
</p>

<h1 align="center">craft-rpc</h1>

<p align="center">
  A tiny background app that slaps goofy Rich Presence status on your Discord whenever you're using Craft apps.
</p>

---

Runs silently in the background on Windows, catches whatever Craft tool you got open, and updates your Discord status with custom dumb taglines in real time.

### Tweak Your Own Taglines

Wanna change what it says? Just edit the `tagline` match block in `src/main.rs`:

```rust
fn tagline(label: &str) -> &'static str {
    match label {
        "PhotoCraft" => "mulusin muka burik",
        "VectorCraft" => "nyiksa pen tool",
        "FilmCraft" => "cut, render, crash",
        // put your own silly lines here
        _ => "cengo doang",
    }
}
```

Then rebuild it:
```bash
cargo build --release
```

### Heads Up & Disclaimer

Made purely for fun (unofficial project). Provided strictly **AS IS** without warranties. Don't be weird and use it for shady stuff like spamming Discord IPC or breaking Discord ToS. Use at your own risk.

### License

[MIT](LICENSE)
