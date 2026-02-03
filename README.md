# CODEOWNERS LSP Extension for Zed

LSP-powered linting, formatting, ownership info, and syntax highlighting for GitHub CODEOWNERS files.

This extension uses [codeowners-lsp](https://github.com/radiosilence/codeowners-lsp) for advanced features. If you just want syntax highlighting without the LSP, use the base [CODEOWNERS](https://github.com/lukasmalkmus/codeowners-zed) extension instead.

## Features

- **Syntax Highlighting**: Full support via [tree-sitter-codeowners](https://github.com/lukasmalkmus/tree-sitter-codeowners)
- **Ownership Display**: See who owns each file via inlay hints and hover (with clickable GitHub links)
- **Go-to-Definition**: Jump from any file to the matching CODEOWNERS rule
- **Linting**: Detects shadowed rules, invalid patterns, duplicate owners, unowned files, and more
- **Quick Fixes**: Code actions to fix detected problems (remove dead rules, add owners, fix all safe issues)
- **Formatting**: Format and normalize CODEOWNERS files
- **Completions**: Path completions (`/`) and owner completions (`@`)
- **Document Symbols**: Outline view of sections and rules
- **Find References**: Find all rules for a given owner
- **Rename**: Rename owners across all rules
- **Workspace Symbols**: Search patterns and owners across the file
- **Code Lens**: Inline file counts per rule
- **Auto-detection**: Finds `CODEOWNERS` in standard locations (`.github/`, root, `docs/`)

## Viewing File Ownership

Once installed, ownership info is shown in two ways:

1. **Inlay Hint**: An inline hint appears at line 1 showing `Owned by: @team`
2. **Hover**: Hover anywhere in the file to see ownership in a tooltip

## Installation

1. Open Zed
2. `Cmd+Shift+P` → **zed: extensions**
3. Search for "CODEOWNERS LSP"
4. Click **Install**

The LSP binary is automatically downloaded from [GitHub releases](https://github.com/radiosilence/codeowners-lsp/releases) on first use.

### Using a Custom Binary

If you prefer to manage the binary yourself, install `codeowners-lsp` and either put it in your `PATH` or specify the path:

```json
{
  "lsp": {
    "codeowners-lsp": {
      "binary": {
        "path": "/path/to/codeowners-lsp"
      }
    }
  }
}
```

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

**CODEOWNERS file diagnostics:**

| Diagnostic Code          | Default | Description                                           |
| ------------------------ | ------- | ----------------------------------------------------- |
| `invalid-pattern`        | error   | Invalid glob pattern syntax                           |
| `invalid-owner`          | error   | Invalid owner format (@user, @org/team, or email)     |
| `pattern-no-match`       | warning | Pattern matches no files in the repo                  |
| `duplicate-owner`        | warning | Same owner listed multiple times on one rule          |
| `shadowed-rule`          | warning | Rule is completely shadowed by an earlier rule        |
| `no-owners`              | hint    | Rule line has no owners (e.g., `src/` with no @users) |
| `unowned-files`          | info    | Summary: "X files have no code owners"                |
| `github-owner-not-found` | warning | Owner doesn't exist on GitHub (requires token)        |

**Source file diagnostics:**

| Diagnostic Code   | Default | Description                                      |
| ----------------- | ------- | ------------------------------------------------ |
| `file-not-owned`  | hint    | File has no matching CODEOWNERS rule (full-file) |

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

- [codeowners-lsp](https://github.com/radiosilence/codeowners-lsp) - The language server powering this extension
- [CODEOWNERS extension](https://github.com/lukasmalkmus/codeowners-zed) - Base extension (syntax highlighting only)
- [tree-sitter-codeowners](https://github.com/lukasmalkmus/tree-sitter-codeowners) - The grammar
- [GitHub CODEOWNERS Documentation](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-code-owners)

## License

MIT
