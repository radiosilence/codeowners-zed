# CODEOWNERS Extension for Zed

Syntax highlighting, linting, formatting, and ownership info for GitHub CODEOWNERS files.

## Features

- **Syntax Highlighting**: Full support for CODEOWNERS syntax via [tree-sitter-codeowners](https://github.com/lukasmalkmus/tree-sitter-codeowners)
- **Ownership Display**: See who owns each file you're editing via inlay hints and hover (with clickable GitHub links)
- **Go-to-Definition**: Jump from any file to the matching CODEOWNERS rule
- **Linting**: Detects shadowed rules, invalid patterns, duplicate owners, unowned files, and more
- **Quick Fixes**: Code actions to fix detected problems (remove dead rules, add owners, fix all safe issues)
- **Formatting**: Format and normalize CODEOWNERS files
- **Completions**: Path completions (triggered by `/`) and owner completions (triggered by `@`)
- **Auto-detection**: Finds `CODEOWNERS` in standard locations (`.github/`, root, `docs/`)

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

Configure the LSP in your Zed settings:

```json
{
  "lsp": {
    "codeowners-lsp": {
      "initialization_options": {
        "path": ".github/CODEOWNERS",
        "individual": "@your-username",
        "team": "@your-org/your-team",
        "github_token": "env:GITHUB_TOKEN",
        "validate_owners": false
      }
    }
  }
}
```

| Option            | Description                                                        |
| ----------------- | ------------------------------------------------------------------ |
| `path`            | Custom CODEOWNERS location (auto-detected if not set)              |
| `individual`      | Your GitHub handle for "take ownership" actions                    |
| `team`            | Your team's handle for "take ownership" actions                    |
| `github_token`    | GitHub token for owner validation (use `env:VAR` to read from env) |
| `validate_owners` | Enable GitHub API validation of owners                             |
| `diagnostics`     | Severity overrides per diagnostic (see below)                      |

Auto-detection searches: `CODEOWNERS` → `.github/CODEOWNERS` → `docs/CODEOWNERS`

### Diagnostic Severity Overrides

Override severity for specific diagnostics. Values: `"off"`, `"hint"`, `"info"`, `"warning"`, `"error"`

```json
{
  "lsp": {
    "codeowners-lsp": {
      "initialization_options": {
        "diagnostics": {
          "no-owners": "off",
          "pattern-no-match": "hint"
        }
      }
    }
  }
}
```

| Diagnostic Code          | Default | Description                         |
| ------------------------ | ------- | ----------------------------------- |
| `invalid-pattern`        | error   | Invalid glob pattern syntax         |
| `invalid-owner`          | error   | Invalid owner format                |
| `pattern-no-match`       | warning | Pattern matches no files            |
| `duplicate-owner`        | warning | Same owner appears multiple times   |
| `shadowed-rule`          | warning | Rule is shadowed by an earlier rule |
| `no-owners`              | hint    | Rule has no owners                  |
| `unowned-files`          | info    | Files without ownership             |
| `github-owner-not-found` | warning | Owner doesn't exist on GitHub       |

### Project Config Files

You can also configure via `.codeowners-lsp.toml` in your workspace root (and `.codeowners-lsp.local.toml` for user-specific overrides):

```toml
individual = "@username"
team = "@org/team-name"
github_token = "env:GITHUB_TOKEN"
validate_owners = true

[diagnostics]
no-owners = "off"
```

## Development

### Installing as Dev Extension

To test local changes:

1. Clone this repo
2. In Zed: `Cmd+Shift+P` → **zed: install dev extension**
3. Select the `codeowners-zed` directory
4. Rebuild after changes with **zed: rebuild dev extension**

### Building from Source

```bash
# Build the extension (WASM)
cargo build --target wasm32-wasip1 --release
```

The LSP is in a [separate repo](https://github.com/radiosilence/codeowners-lsp) and downloaded automatically from releases.

## Resources

- [GitHub CODEOWNERS Documentation](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-code-owners)

## License

MIT
