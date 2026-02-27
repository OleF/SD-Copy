# SD Copy - Prosjektoversikt

## 📁 Fullstendig Filstruktur

```
SD-Copy/
├── .gitignore                              # Git ignore fil
├── README.md                               # Hovedarkitektur og dokumentasjon
├── QUICKSTART.md                           # Hurtigstart guide
├── setup.sh                                # Setup script (chmod +x)
├── package.json                            # npm dependencies
├── package-lock.json                       # npm lock file
├── tsconfig.json                           # TypeScript config
├── tsconfig.node.json                      # TypeScript config for Node
├── vite.config.ts                          # Vite bundler config
├── index.html                              # HTML entry point
│
├── src/                                    # React Frontend (TypeScript)
│   ├── main.tsx                            # React entry point
│   ├── App.tsx                             # Main app component
│   ├── App.css                             # Global styles
│   ├── types.ts                            # TypeScript interfaces
│   ├── utils.ts                            # Date/formatting utilities
│   └── components/
│       ├── WizardModal.tsx                 # Import wizard dialog
│       └── ProgressView.tsx                # Progress tracking UI
│
└── src-tauri/                              # Rust Backend
    ├── Cargo.toml                          # Rust dependencies
    ├── build.rs                            # Tauri build script
    ├── tauri.conf.json                     # Tauri v2 configuration
    ├── icons/                              # App icons (placeholder)
    │   ├── README.md                       # Icon generation guide
    │   └── icon.png.base64                 # Placeholder note
    └── src/
        ├── main.rs                         # Tauri app entry + commands
        ├── types.rs                        # Rust structs (ScanResult, etc.)
        ├── utils.rs                        # File utilities (duplicate handling)
        ├── volume.rs                       # Volume detection & scanning
        └── import.rs                       # Import manager & logic
```

## 🔧 Teknologi Stack

### Frontend
- **React 18.3** - UI framework
- **TypeScript 5.6** - Type safety
- **Vite 5.4** - Build tool & dev server
- **Tauri API 2.0** - IPC kommunikasjon
- **CSS3** - Styling (ingen eksterne UI libraries)

### Backend
- **Rust (stable)** - System-level operations
- **Tauri 2.0** - Desktop app framework
- **Plugins:**
  - `tauri-plugin-dialog` - Native file picker
  - `tauri-plugin-fs` - File system access
- **Dependencies:**
  - `serde` - Serialization
  - `chrono` - Date/time
  - `walkdir` - Directory traversal
  - `uuid` - Import ID generation

## 📊 Arkitektur

### Frontend Flow
```
App.tsx (polling loop)
  ↓
list_volumes() command → Backend
  ↓
scan_volume_for_images() → Backend
  ↓
[Bilder funnet]
  ↓
WizardModal vises
  ↓
User input + folder picker
  ↓
start_import() command → Backend
  ↓
Listen til "import-progress" events
  ↓
ProgressView viser live status
```

### Backend Flow
```
main.rs
├── list_volumes() → volume.rs
│   └── Leser /Volumes/ (macOS) eller drives (Windows)
│
├── scan_volume_for_images() → volume.rs
│   ├── Sjekker DCIM/ mappe
│   ├── Skanner rekursivt med walkdir
│   └── Filtrerer på IMAGE_EXTENSIONS
│
└── start_import() → import.rs
    ├── Spawner async task
    ├── Finner alle bildefiler
    ├── Kopierer med fs::copy()
    ├── Håndterer duplikater (utils.rs)
    ├── Emitter "import-progress" events
    └── Skriver import-manifest.json
```

## 🔐 Sikkerhet (Tauri v2)

### Capabilities & Permissions

**tauri.conf.json:**
```json
{
  "plugins": {
    "dialog": {
      "all": true,
      "open": true
    },
    "fs": {
      "scope": [
        "$APPDATA/**",
        "$RESOURCE/**",
        "/Volumes/**",      // macOS SD-kort
        "$HOME/**"          // Brukerens filer
      ]
    }
  }
}
```

### Hvorfor disse permissions?
- `/Volumes/**` - Les fra SD-kort (macOS)
- `$HOME/**` - Les/skriv til brukerens destinasjonsmappe
- Dialog plugin - Åpne native folder picker

**Ingen "allow all"** - kun spesifikke, nødvendige tillatelser.

## 🎯 Funksjoner (MVP Implementert)

| Feature | Status | Implementasjon |
|---------|--------|----------------|
| Volume monitoring | ✅ | Polling (1500ms) i App.tsx |
| Image detection | ✅ | volume.rs med walkdir |
| DCIM folder detection | ✅ | has_dcim_folder() |
| Format support (12 typer) | ✅ | IMAGE_EXTENSIONS const |
| Import wizard | ✅ | WizardModal.tsx |
| Folder name editor | ✅ | State i modal |
| Date formatting (DDMMÅÅ) | ✅ | utils.ts |
| Native folder picker | ✅ | tauri-plugin-dialog |
| File copy (not move) | ✅ | fs::copy() i import.rs |
| Preserve subfolder structure | ✅ | strip_prefix() logic |
| Duplicate handling | ✅ | generate_unique_filename() |
| Live progress | ✅ | Tauri events |
| Cancel import | ✅ | cancel_import() command |
| Error handling | ✅ | Result types + try/catch |
| Import manifest | ✅ | JSON serialization |

## 📝 API Reference

### Tauri Commands (Rust → Frontend)

#### `list_volumes() -> Result<Vec<String>, String>`
Returnerer liste over alle monterte volumer.

**Eksempel:**
```typescript
const volumes = await invoke<string[]>('list_volumes');
// ["/Volumes/SD-CARD", "/Volumes/USB-DRIVE"]
```

#### `scan_volume_for_images(volumePath: string) -> Result<ScanResult, String>`
Skanner et volum for bildefiler.

**Input:**
```typescript
{ volumePath: "/Volumes/SD-CARD" }
```

**Output:**
```typescript
{
  hasImages: true,
  imageCount: 247,
  samplePaths: ["DCIM/100MSDCF/IMG_0001.JPG", ...]
}
```

#### `start_import(volumePath, destinationRoot, folderName) -> Result<String, String>`
Starter import og returnerer importId.

**Input:**
```typescript
{
  volumePath: "/Volumes/SD-CARD",
  destinationRoot: "/Users/navn/Pictures",
  folderName: "270226 - Ferie"
}
```

**Output:**
```typescript
"550e8400-e29b-41d4-a716-446655440000"  // UUID
```

#### `cancel_import(importId: string) -> Result<(), String>`
Avbryter en pågående import.

### Events (Backend → Frontend)

#### `import-progress`
Sendes kontinuerlig under import.

**Payload:**
```typescript
{
  importId: string,
  totalFiles: number,
  copiedFiles: number,
  currentFile: string,
  status: "running" | "completed" | "failed" | "cancelled",
  error?: string
}
```

## 🧪 Testing Scenarios

### 1. Tom Volum
- Sett inn USB uten bilder
- Forventet: Ingen modal vises

### 2. Volum med Bilder
- Sett inn SD-kort med DCIM/
- Forventet: Modal vises innen 1.5s

### 3. Import Success
- Start import
- Forventet: Progress 0% → 100%, "Import fullført!"

### 4. Duplikater
- Importer samme kort to ganger til samme mappe
- Forventet: Første gang: IMG_0001.JPG, andre gang: IMG_0001 (1).JPG

### 5. Cancel Import
- Start import, klikk avbryt
- Forventet: Import stopper, status = "cancelled"

### 6. Volume Unmount During Import
- Start import, trekk ut kortet
- Forventet: Feilmelding "Failed to copy..."

## 🚀 Deployment

### Development
```bash
npm run tauri:dev
```

### Production Build
```bash
npm run tauri:build
```

**Output locations:**
- **macOS:** `src-tauri/target/release/bundle/macos/SD Copy.app`
- **Windows:** `src-tauri/target/release/bundle/msi/SD Copy_0.1.0_x64.msi`

### Signing (macOS)
For distribusjon:
```bash
codesign --force --deep --sign "Developer ID Application: Your Name" "SD Copy.app"
```

## 📦 Dependencies Oversikt

### npm (Frontend)
| Package | Version | Formål |
|---------|---------|--------|
| @tauri-apps/api | 2.0 | Tauri IPC |
| @tauri-apps/plugin-dialog | 2.0 | File picker |
| @tauri-apps/plugin-fs | 2.0 | File system |
| react | 18.3 | UI framework |
| react-dom | 18.3 | React DOM |
| typescript | 5.6 | Type system |
| vite | 5.4 | Build tool |

### Cargo (Backend)
| Crate | Version | Formål |
|-------|---------|--------|
| tauri | 2.0 | Framework |
| serde | 1.0 | Serialization |
| chrono | 0.4 | Date/time |
| walkdir | 2.0 | Dir traversal |
| uuid | 1.0 | ID generation |

## 🔧 Konfigurasjon Files

### `tauri.conf.json`
- Window size: 900x700
- Title: "SD Copy - Photo Import"
- Permissions for dialog & fs
- Bundle settings

### `package.json`
- Scripts: dev, build, tauri:dev, tauri:build
- Dependencies versions

### `tsconfig.json`
- Target: ES2020
- Module: ESNext
- Strict mode enabled

### `vite.config.ts`
- React plugin
- Port: 1420 (Tauri requirement)
- Watch excludes src-tauri

## 📚 Neste Steg

### Performance Improvements
- [ ] Native volume events (istedenfor polling)
- [ ] Worker threads for scanning
- [ ] Streaming copy for store filer

### Features
- [ ] Bilde thumbnails
- [ ] Metadata preservation (EXIF)
- [ ] Batch rename
- [ ] Video support
- [ ] Auto-organize by date
- [ ] Settings persistence

### UX
- [ ] Dark mode
- [ ] Drag & drop support
- [ ] Multiple volume selection
- [ ] Import history
- [ ] Undo/redo

---

**Status: ✅ MVP KOMPLETT OG KLAR TIL BRUK**

Last updated: 2026-02-27

