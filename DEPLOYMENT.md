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

## Automatic Deployment (Recommended)

The project includes a GitHub Actions workflow (`.github/workflows/github-pages-deploy.yml`) that will automatically:
- Build the project on every push to `main`
- Optimize WASM bundle with wasm-opt
- Deploy to `gh-pages` branch for GitHub Pages
- Create/update `release` branch for backup and reference

### GitHub Pages Settings for Automatic Deployment

After the first automatic deployment, configure GitHub Pages:

1. Go to **Repository Settings → Pages**
2. Change **Source** from "GitHub Actions" to **"Deploy from a branch"**
3. Select:
   - **Branch**: `gh-pages`
   - **Folder**: `/ (root)`
4. Click **Save**

Your site will be available at `https://yourusername.github.io/remind-me-pwa/`

Just push your code and the workflow will handle the rest!

## Important Notes

- Make sure `base_path` in `Dioxus.toml` matches your repository name
- The app will be available at: `https://yourusername.github.io/remind-me-pwa/`
- First deployment might take a few minutes to become available


