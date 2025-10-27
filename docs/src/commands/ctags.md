# ctags

The `ctags` command manages code tags for efficient navigation across multiple repositories.

## Syntax

```bash
vem generate ctags <repository> [options]
vem update ctags <repository> [options]  
vem delete ctags <repository> [options]
vem list ctags [options]
vem clean ctags [options]
```

## Subcommands

### generate

Generate ctags for a specified repository.

```bash
vem generate ctags <repository> [options]
```

**Arguments:**
- `<repository>`: Repository name defined in vem.toml

**Options:**
- `--languages=<langs>`: Comma-separated list of languages (e.g., `python,rust,javascript`)
- `--exclude=<patterns>`: Additional exclude patterns
- `--tag-set=<name>`: Generate specific tag set from configuration
- `--ai-enhance`: Enable AI-powered tag enhancement (AI environments only)
- `--force`: Overwrite existing tags without confirmation

**Examples:**
```bash
# Generate tags for main project
vem generate ctags main_project

# Generate tags for specific languages
vem generate ctags shared_libs --languages=python,rust

# Generate with AI enhancement
vem generate ctags ai_project --ai-enhance

# Generate specific tag set
vem generate ctags . --tag-set=copilot_context
```

### update

Update existing ctags for a repository.

```bash
vem update ctags <repository> [options]
```

**Arguments:**
- `<repository>`: Repository name or `--all` for all repositories

**Options:**
- `--incremental`: Update only changed files
- `--force`: Force full regeneration
- `--ai-context`: Update AI context (AI environments only)
- `--all`: Update all configured repositories

**Examples:**
```bash
# Update main project tags
vem update ctags main_project

# Incremental update
vem update ctags shared_libs --incremental

# Update all repositories
vem update ctags --all

# Force full regeneration
vem update ctags ml_models --force
```

### delete

Delete ctags for a repository or specific tag files.

```bash
vem delete ctags <repository> [options]
```

**Arguments:**
- `<repository>`: Repository name

**Options:**
- `--tag-file=<file>`: Delete specific tag file
- `--confirm`: Require confirmation (default in basic environments)
- `--no-backup`: Skip creating backup
- `--ai-context`: Also clean AI context (AI environments)

**Examples:**
```bash
# Delete all tags for repository
vem delete ctags external_deps

# Delete specific tag file
vem delete ctags . --tag-file=project_tags

# Delete without confirmation
vem delete ctags temp_repo --no-confirm
```

### list

List all ctags with status and metadata.

```bash
vem list ctags [options]
```

**Options:**
- `--format=<fmt>`: Output format (`table`, `json`, `yaml`, `simple`)
- `--repository=<repo>`: Filter by repository
- `--status=<status>`: Filter by status (`active`, `stale`, `error`)
- `--sort=<field>`: Sort by field (`name`, `size`, `updated`)

**Examples:**
```bash
# List all ctags (default format based on environment)
vem list ctags

# List in JSON format
vem list ctags --format=json

# List for specific repository
vem list ctags --repository=main_project

# List only active tags
vem list ctags --status=active
```

**Output Examples:**

**Table format (Developer Vim):**
```
Name          Repository    Tag File        Last Updated    Size    Status
project       main_project  tags           2024-01-01      1.2MB   Active
shared_libs   shared_libs   shared_tags    2024-01-01      856KB   Active
api_schemas   api_defs      api_tags       2024-01-01      234KB   Stale
```

**JSON format (Modern Neovim):**
```json
{
  "workspace": {
    "repository": "current_project",
    "tag_file": "workspace_tags",
    "last_updated": "2024-01-01T12:00:00Z",
    "size": 1048576,
    "lsp_integrated": true,
    "status": "active"
  }
}
```

**Enhanced format (AI Development):**
```
Name            Repository   AI Score   Copilot Ready   Size     Status
ai_comprehensive ml_models   95%        ✓              2.1MB    Active
copilot_enhanced ai_project  88%        ✓              1.8MB    Active
ml_pipeline     data_pipe   76%        ✓              1.2MB    Active
```

### clean

Clean all ctags files and optionally reset AI context.

```bash
vem clean ctags [options]
```

**Options:**
- `--backup`: Create backup before cleaning (default: true)
- `--no-confirm`: Skip confirmation prompt
- `--ai-reset`: Reset AI context (AI environments only)
- `--cache`: Also clean ctags cache directories

**Examples:**
```bash
# Clean all ctags with backup
vem clean ctags

# Clean without confirmation
vem clean ctags --no-confirm

# Clean with AI context reset
vem clean ctags --ai-reset

# Clean everything including cache
vem clean ctags --cache --no-backup
```

## Configuration

Ctags behavior is configured in `vem.toml`:

### Repository Configuration
```toml
[ctags.repositories.main_project]
name = "main_project"
path = "."
enabled = true
auto_sync = true
priority = 1
```

### Tag Configuration  
```toml
[ctags.tags.project]
name = "project"
repositories = ["main_project"]
tag_file = "tags"
languages = ["python", "rust"]
auto_generate = true
```

### Command Configuration
```toml
[ctags.commands]
generate_options = ["--recurse=yes", "--sort=yes"]
list_format = "table"
clean_backup = true
```

## Environment-Specific Behavior

### Basic Vim
- Simple tag management
- Always requires confirmation
- Text-based output
- Manual tag generation

### Developer Vim  
- Multi-repository support
- Gutentags integration
- Table format output
- Automatic tag updates

### Modern Neovim
- LSP integration
- JSON format output
- Telescope integration
- Workspace-focused tagging

### AI Development
- AI-enhanced tagging
- Context optimization for Copilot
- Smart filtering and relevance scoring
- Cross-repository AI context

## Error Handling

Common error scenarios and solutions:

### Repository Not Found
```bash
Error: Repository 'unknown_repo' not found in configuration
```
**Solution:** Check `vem.toml` for correct repository names

### Ctags Executable Missing
```bash
Error: ctags executable not found in PATH
```
**Solution:** Install universal-ctags or update PATH

### Permission Denied
```bash
Error: Permission denied writing to tag file
```
**Solution:** Check file/directory permissions

### Invalid Language
```bash
Error: Language 'unknown_lang' not supported by ctags
```
**Solution:** Use `ctags --list-languages` to see supported languages

## See Also

- [Configuration Guide](../configuration.md) - Detailed vem.toml reference
- [Environment Structure](../environment-structure.md) - Understanding VEM environments
- [Multi-Repository Setup](../multi-repo.md) - Managing multiple codebases