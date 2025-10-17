# Quick Start

This guide will help you get started with VEM in just a few minutes. We'll create your first environment and show you the basic workflow.

## Step 1: Create Your First Environment

Let's create a development environment:

```bash
vem create development
```

This creates a new Vim environment called "development" with:
- A fresh `.vimrc` file
- An empty `.vim` directory
- Isolated plugin storage

## Step 2: List Your Environments

See all available environments:

```bash
vem list
```

Output:
```
development
```

## Step 3: Switch to Your Environment

Activate the development environment:

```bash
vem switch development
```

This will:
- Set up symbolic links to the development environment
- Make it the active Vim configuration

## Step 4: Check Current Environment

Verify which environment is currently active:

```bash
vem current
```

Output:
```
development
```

## Step 5: Customize Your Environment

Now you can customize this environment:

1. Edit the `.vimrc` file:
   ```bash
   vim ~/.vimrc
   ```

2. Install plugins in the `.vim` directory:
   ```bash
   mkdir -p ~/.vim/bundle
   # Install your favorite plugins
   ```

3. Any changes you make will only affect the "development" environment.

## Step 6: Create Additional Environments

Let's create a writing environment:

```bash
vem create writing
```

Configure it for writing:
```bash
vem switch writing
vim ~/.vimrc
```

Add writing-specific configurations:
```vim
" Writing-focused Vim configuration
set spell
set linebreak
set textwidth=80
colorscheme peachpuff
```

## Step 7: Switch Between Environments

Now you can easily switch between environments:

```bash
# Switch to development
vem switch development

# Switch to writing  
vem switch writing

# Check current environment
vem current
```

## Common Workflow

1. **Create environments** for different use cases
2. **Switch** to the appropriate environment for your task
3. **Customize** each environment independently
4. **Switch back** whenever you need a different setup

## What's Next?

- Learn about all available [Commands](./commands.md)
- Understand the [Environment Structure](./environment-structure.md)
- Explore [Configuration](./configuration.md) options
- Read about [Architecture](./architecture.md) if you're interested in how VEM works

You're now ready to manage multiple Vim environments efficiently with VEM!