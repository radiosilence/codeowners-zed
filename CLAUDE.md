# CODEOWNERS Zed Extension

## Project Structure

```
codeowners-zed/
├── src/lib.rs                    # Zed extension (WASM) - downloads/spawns LSP
├── crates/codeowners-lsp/        # LSP binary - hover, inlay hints, ownership lookup
├── extension.toml                # Extension manifest, language server config
├── .github/CODEOWNERS            # This repo's CODEOWNERS
└── languages/codeowners/         # Tree-sitter grammar config for syntax highlighting
```

## Building

```bash
# Build LSP binary
cargo build -p codeowners-lsp --release

# Build extension WASM (not strictly required for local dev)
cargo build --target wasm32-wasip1 --release -p codeowners-zed
```

## Local Testing

**Important:** Don't use a symlink for the extension - Zed will rebuild on every file change, killing the LSP repeatedly.

1. Copy extension to Zed:

```bash
rm -rf ~/Library/Application\ Support/Zed/extensions/installed/codeowners
cp -r . ~/Library/Application\ Support/Zed/extensions/installed/codeowners
```

2. Configure LSP binary path in Zed settings (`~/.config/zed/settings.json`):

```json
{
  "lsp": {
    "codeowners-lsp": {
      "binary": {
        "path": "/path/to/codeowners-zed/target/release/codeowners-lsp"
      }
    }
  }
}
```

3. Restart Zed or close/reopen a file to trigger LSP start

4. Check logs:
   - Zed log: `Cmd+Shift+P` → "zed: open log"
   - LSP debug log: `/tmp/codeowners-lsp.log` (if debug logging enabled)

## How It Works

- Extension declares `codeowners-lsp` language server for ~50 languages in `extension.toml`
- When you open a file in one of those languages, Zed spawns the LSP
- LSP finds CODEOWNERS file (`.github/CODEOWNERS`, `CODEOWNERS`, `docs/CODEOWNERS`)
- LSP provides hover info and inlay hints showing file ownership

## Dependencies

- `codeowners` crate - parses CODEOWNERS files
- `tower-lsp` - LSP protocol implementation
- `zed_extension_api` - Zed extension bindings

## Known Issues

- `tower-lsp` throws "Unexpected params: null" on shutdown - harmless, Zed sends null params which is valid LSP spec but tower-lsp is strict
- Extension rebuilds kill running LSP when using symlink for local dev (use copy instead)

## Current Status

- [x] LSP binary builds and runs
- [x] Extension loads in Zed
- [x] LSP starts when opening supported files
- [ ] Verify hover/inlay hints display correctly
- [ ] GitHub Actions for releases
