# CODEOWNERS Extension for Zed

Syntax highlighting and ownership info for GitHub CODEOWNERS files.

## Features

- **Syntax Highlighting**: Full support for CODEOWNERS syntax via [tree-sitter-codeowners](https://github.com/lukasmalkmus/tree-sitter-codeowners)
- **Ownership Display**: See who owns each file you're editing
- **Auto-detection**: Finds `CODEOWNERS` in standard locations (`.github/`, root, `.gitlab/`, `docs/`)

## Viewing File Ownership

Once installed, ownership info is shown in two ways:

1. **Inlay Hint**: An inline hint appears at line 1 showing `Owned by: @team`
2. **Hover**: Hover anywhere in the file to see ownership in a tooltip

## Installation

1. Open Zed
2. `Cmd+Shift+P` → **zed: extensions**
3. Search for "CODEOWNERS"
4. Click **Install**

The LSP binary is automatically downloaded from GitHub releases on first use.

## Configuration

You can customize the CODEOWNERS file path in your Zed settings:

```json
{
  "lsp": {
    "codeowners-lsp": {
      "initialization_options": {
        "path": ".github/CODEOWNERS"
      }
    }
  }
}
```

If not specified, it searches these locations in order:

- `CODEOWNERS`
- `.github/CODEOWNERS`
- `docs/CODEOWNERS`

## Building from Source

```bash
# Build the extension (WASM)
cargo build --target wasm32-wasip1 --release

# Build the LSP binary
cargo build -p codeowners-lsp --release
```

## Resources

- [GitHub CODEOWNERS Documentation](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-code-owners)

## License

MIT
