# Icons Placeholder

For MVP, you need to generate app icons. You can:

1. Use an online icon generator like [Tauri Icon Generator](https://icongenerator.io/)
2. Or use the Tauri CLI to generate icons from a PNG source:

```bash
# Install @tauri-apps/cli globally if needed
npm install -g @tauri-apps/cli

# Generate icons from a 1024x1024 PNG
tauri icon path/to/your-icon.png
```

3. Or manually create these files in `src-tauri/icons/`:
   - `32x32.png`
   - `128x128.png`
   - `128x128@2x.png`
   - `icon.icns` (macOS)
   - `icon.ico` (Windows)

For quick testing, you can use any PNG images renamed to match these filenames.

## Quick Workaround for Development

The app will run in dev mode without icons, but building for production requires them.

You can use a simple colored square as a placeholder for now.

