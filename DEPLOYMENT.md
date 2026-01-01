# Deployment Guide for GitHub Pages

## Quick Start

1. **Install Dioxus CLI** (if not already installed):
   ```bash
   cargo install dioxus-cli --locked
   ```

2. **Build the project**:
   ```bash
   dx bundle --release --out-dir docs
   ```

3. **Move files to docs root**:
   ```bash
   # If docs/public exists, move its contents
   if [ -d "docs/public" ]; then
       mv docs/public/* docs/
       rmdir docs/public
   fi
   ```

4. **Create 404.html for client-side routing**:
   ```bash
   cp docs/index.html docs/404.html
   ```

5. **Commit and push**:
   ```bash
   git add docs/
   git commit -m "Deploy to GitHub Pages"
   git push origin main
   ```

6. **Enable GitHub Pages**:
   - Go to your repository on GitHub
   - Settings → Pages
   - Source: Deploy from a branch
   - Branch: `main` / `docs` folder
   - Save

## Automatic Deployment

The project includes a GitHub Actions workflow (`.github/workflows/deploy.yml`) that will automatically:
- Build the project on every push to `main`
- Deploy to GitHub Pages

Just push your code and the workflow will handle the rest!

## Important Notes

- Make sure `base_path` in `Dioxus.toml` matches your repository name
- The app will be available at: `https://yourusername.github.io/remind-me-pwa/`
- First deployment might take a few minutes to become available


