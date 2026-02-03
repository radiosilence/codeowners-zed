# CODEOWNERS LSP Extension for Zed

LSP-powered linting, formatting, ownership info, and syntax highlighting for GitHub CODEOWNERS files.

This extension uses [codeowners-lsp](https://github.com/radiosilence/codeowners-lsp) for advanced features. If you just want syntax highlighting without the LSP, use the base [CODEOWNERS](https://github.com/lukasmalkmus/codeowners-zed) extension instead.

## Features

### In Any File

- **Hover**: File ownership with clickable GitHub links and rich metadata (team descriptions, member counts, user bios)
- **Inlay Hints**: Ownership displayed at top of file
- **Go-to-Definition**: Jump to the CODEOWNERS rule matching the current file
- **Code Actions**: Take ownership of files directly from your editor

### In CODEOWNERS Files

- **Diagnostics**: Invalid patterns/owners, no-match patterns, shadowed rules, duplicate owners, coverage stats
- **Completions**: fzf-style fuzzy path completions, owner completions from GitHub API
- **Code Actions**: Remove shadowed rules, remove duplicates, add owners, add catch-all rule, fix all safe issues
- **Formatting**: Format and normalize CODEOWNERS files
- **Document Symbols**: Outline view with sections and rules (Cmd+Shift+O)
- **Workspace Symbols**: Search patterns and owners (Cmd+T)
- **Find References**: Find all rules containing an owner
- **Rename**: Rename an owner across all rules
- **Code Lens**: Inline file count and owners above each rule
- **Folding**: Collapse comment blocks and sections
- **Semantic Highlighting**: Syntax colors for patterns, owners, globs, comments
- **Signature Help**: Pattern syntax documentation while typing (`*`, `**`, `?`, `[...]`)
- **Selection Range**: Smart expand selection (word → owner → all owners → rule → section)
- **Linked Editing**: Edit an owner and all occurrences update simultaneously
- **Pattern Hover**: Hover over patterns to see matching files

## Installation

1. Open Zed
2. `Cmd+Shift+P` → **zed: extensions**
3. Search for "CODEOWNERS LSP"
4. Click **Install**
5. **Important!**: Check you have either `...` or `codeowners-lsp` in your list of language servers for any languages you have overridden the configuration for!

The LSP binary is automatically downloaded from [GitHub releases](https://github.com/radiosilence/codeowners-lsp/releases) on first use.

### Using a Custom Binary

Install `codeowners-lsp` yourself (via [mise](https://mise.jdx.dev/), Homebrew, or direct download) and put it in your `PATH`, or specify the path in Zed settings:

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

**Recommended**: Use `.codeowners-lsp.toml` in your workspace root. This keeps config with your project and works across all editors. For user-specific overrides (gitignore this), use `.codeowners-lsp.local.toml`.

```toml
# CODEOWNERS location (relative to workspace root)
path = "custom/CODEOWNERS"

# Your identifiers for "take ownership" actions
individual = "@username"
team = "@org/team-name"

# GitHub validation (optional)
github_token = "env:GITHUB_TOKEN"
validate_owners = false

# Diagnostic severity overrides
# Values: "off", "hint", "info", "warning", "error"
[diagnostics]
no-owners = "off"
pattern-no-match = "hint"
```

| Option            | Description                                                                    |
| ----------------- | ------------------------------------------------------------------------------ |
| `path`            | Custom CODEOWNERS location (relative to workspace root)                        |
| `individual`      | Your GitHub handle for "take ownership" actions                                |
| `team`            | Your team's handle for "take ownership" actions                                |
| `github_token`    | GitHub token for owner validation. Use `env:VAR_NAME` to read from environment |
| `validate_owners` | Enable GitHub API validation of @user and @org/team (default: false)           |
| `diagnostics`     | Map of diagnostic code to severity override                                    |

Auto-detection searches: `CODEOWNERS` → `.github/CODEOWNERS` → `docs/CODEOWNERS`

### Zed Settings (Alternative)

You can also configure via Zed's `initialization_options` (these override the TOML config):

```json
{
  "lsp": {
    "codeowners-lsp": {
      "initialization_options": {
        "individual": "@your-username",
        "team": "@your-org/your-team",
        "github_token": "env:GITHUB_TOKEN",
        "validate_owners": false,
        "diagnostics": {
          "no-owners": "off"
        }
      }
    }
  }
}
```

### Diagnostic Codes

**CODEOWNERS file diagnostics:**

| Diagnostic Code          | Default | Description                                       |
| ------------------------ | ------- | ------------------------------------------------- |
| `invalid-pattern`        | error   | Invalid glob pattern syntax                       |
| `invalid-owner`          | error   | Invalid owner format (@user, @org/team, or email) |
| `pattern-no-match`       | warning | Pattern matches no files in the repo              |
| `duplicate-owner`        | warning | Same owner listed multiple times on one rule      |
| `shadowed-rule`          | warning | Rule is completely shadowed by a later rule       |
| `no-owners`              | hint    | Rule has no owners                                |
| `unowned-files`          | info    | Summary of files without code owners              |
| `github-owner-not-found` | warning | Owner doesn't exist on GitHub (requires token)    |

**Source file diagnostics:**

| Diagnostic Code  | Default | Description                          |
| ---------------- | ------- | ------------------------------------ |
| `file-not-owned` | hint    | File has no matching CODEOWNERS rule |

## Development

### Installing as Dev Extension

1. Clone this repo
2. In Zed: `Cmd+Shift+P` → **zed: install dev extension**
3. Select the `codeowners-zed` directory
4. Rebuild after changes with **zed: rebuild dev extension**

### Building from Source

```bash
cargo build --target wasm32-wasip1 --release
```

## Credits

Based on [codeowners-zed](https://github.com/lukasmalkmus/codeowners-zed) by [Lukas Malkmus](https://github.com/lukasmalkmus), which provides the tree-sitter grammar integration and base extension structure.

## Resources

- [codeowners-lsp](https://github.com/radiosilence/codeowners-lsp) - The language server (includes CLI for linting, formatting, coverage)
- [CODEOWNERS extension](https://github.com/lukasmalkmus/codeowners-zed) - Base extension (syntax highlighting only)
- [tree-sitter-codeowners](https://github.com/lukasmalkmus/tree-sitter-codeowners) - The grammar
- [GitHub CODEOWNERS Documentation](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-code-owners)

## License

MIT
