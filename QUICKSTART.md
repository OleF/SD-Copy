# 🚀 SD Copy - Hurtigstart Guide

## Prosjektstruktur

✅ **Komplett Tauri v2 + React + Rust MVP er nå satt opp!**

```
SD-Copy/
├── src/                          # React TypeScript Frontend
│   ├── components/
│   │   ├── WizardModal.tsx      # Import wizard med folder name input
│   │   └── ProgressView.tsx     # Live progress tracking
│   ├── types.ts                  # TypeScript types
│   ├── utils.ts                  # Date formatting, etc.
│   ├── App.tsx                   # Main app med polling logic
│   ├── App.css                   # Full styling
│   └── main.tsx                  # React entry
├── src-tauri/                    # Rust Backend
│   ├── src/
│   │   ├── main.rs              # Tauri commands
│   │   ├── volume.rs            # Volume scanning (macOS/Windows)
│   │   ├── import.rs            # Import manager med events
│   │   ├── types.rs             # Rust structs
│   │   └── utils.rs             # File utilities
│   ├── Cargo.toml               # Rust dependencies
│   ├── tauri.conf.json          # Tauri v2 config med permissions
│   └── build.rs
├── package.json
├── tsconfig.json
├── vite.config.ts
└── README.md                     # Full dokumentasjon
```

## 📋 Steg-for-steg Setup

### 1. Installer Rust (hvis ikke allerede installert)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustc --version  # Verifiser
```

### 2. Installer Node.js Dependencies

```bash
cd /Users/olefredrikschreuder/dev/SD-Copy
npm install
```

### 3. Kjør Appen i Development Mode

```bash
npm run tauri:dev
```

Dette vil:
- Bygge Rust backend (første gang tar 2-5 min)
- Starte Vite dev server
- Åpne appen

### 4. Test Funksjonalitet

1. **Uten SD-kort:**
   - Appen åpner og viser "Overvåker volumer..."
   - Ingenting skjer (som forventet)

2. **Med SD-kort/USB med bilder:**
   - Sett inn SD-kortet
   - Innen 1.5 sekunder: modal popper opp
   - Se antall bilder og eksempel-filer
   - Rediger mappenavn (default: DDMMÅÅ - Import)
   - Klikk "Importer bilder"
   - Velg destinasjonsmappe
   - Se live progress
   - Vent på "Import fullført!"

3. **Sjekk resultat:**
   - Gå til destinasjonsmappen
   - Se mappen med ditt valgte navn
   - Sjekk at alle bilder er kopiert
   - Åpne `import-manifest.json` for detaljer

## 🎯 Funksjonalitet (Implementert)

### Volume Detection
- ✅ Polling hver 1500ms
- ✅ macOS: `/Volumes/` scanning
- ✅ Windows: Drive letters A-Z
- ✅ Detekterer både DCIM/ og løse bildefiler

### Image Scanning
- ✅ Rekursiv scanning (maks 10 nivåer)
- ✅ Støttede formater: JPG, JPEG, PNG, HEIC, TIF, TIFF, RAF, ARW, CR2, CR3, NEF, DNG
- ✅ Case-insensitive
- ✅ Rask preview (første 10 filer vises)

### Import Wizard
- ✅ Modal dialog med voluminfo
- ✅ Redigerbar folder name
- ✅ Default: DDMMÅÅ - Import (Norge-tid)
- ✅ Native folder picker

### Import Process
- ✅ Kopierer (ikke flytter) filer
- ✅ Bevarer undermappestruktur fra DCIM/
- ✅ Duplikat-håndtering: " (1)", " (2)", etc.
- ✅ Live progress via Tauri events
- ✅ Avbryt-funksjon
- ✅ Robust feilhåndtering

### Manifest
- ✅ JSON-fil: `import-manifest.json`
- ✅ Per fil: source path, dest path, size, timestamp
- ✅ ISO 8601 timestamps

### Security (Tauri v2)
- ✅ Dialog plugin aktivert
- ✅ FS plugin med scoped access:
  - `/Volumes/**` (macOS)
  - `$HOME/**` (bruker hjemme)
- ✅ Ingen "allow all" - kun nødvendige tillatelser

## 🔧 Tauri Commands (Backend API)

| Command | Beskrivelse |
|---------|-------------|
| `list_volumes()` | Returnerer liste over alle monterte volumer |
| `scan_volume_for_images(volumePath)` | Skanner volum for bilder, returnerer count + samples |
| `start_import(volumePath, destinationRoot, folderName)` | Starter import i bakgrunnen, returnerer importId |
| `cancel_import(importId)` | Avbryter pågående import |

## 📡 Events (Frontend Updates)

| Event | Payload | Beskrivelse |
|-------|---------|-------------|
| `import-progress` | `ImportProgress` | Live oppdateringer: copied/total, current file, status |

## 🧪 Testing Checklist

- [ ] Appen starter uten feil
- [ ] Kan se "Overvåker volumer..." status
- [ ] Setter inn SD-kort → modal vises
- [ ] Modal viser korrekt antall bilder
- [ ] Kan endre mappenavn
- [ ] Folder picker åpner
- [ ] Import starter og viser progress
- [ ] Progress oppdateres live
- [ ] Import fullføres uten feil
- [ ] Alle filer kopiert korrekt
- [ ] Undermapper bevart
- [ ] Manifest generert
- [ ] Duplikater håndtert (test ved å importere samme kort to ganger)
- [ ] Avbryt-knappen fungerer

## 🐛 Feilsøking

### "zsh: command not found: cargo"
→ Rust er ikke installert eller ikke i PATH
→ Se steg 1

### "Failed to read volumes"
→ Normalt på systemer uten /Volumes/ (Windows bruker annen metode)
→ På Windows: automatisk fallback til drive letters

### Import henger eller feiler
→ Sjekk at SD-kort er fortsatt montert
→ Sjekk destinasjonsmappe permissions
→ Se console logs (Cmd+Shift+I for DevTools)

### TypeScript errors
→ Kjør `npm install` på nytt
→ Restart IDE

### Rust compile errors
→ Kjør `rustup update`
→ Sjekk at Cargo.toml dependencies er korrekte

## 📦 Build for Produksjon

```bash
npm run tauri:build
```

Output: `src-tauri/target/release/bundle/`

- **macOS:** `.dmg` fil
- **Windows:** `.exe` installer

## 🎨 Ikoner (TODO)

For production build trengs ikoner. Se `src-tauri/icons/README.md`.

For development: ikke nødvendig.

## 📚 Neste Steg (utenfor MVP)

- [ ] Native OS events (istedenfor polling)
- [ ] Bilde thumbnails i wizard
- [ ] Batch rename/organize
- [ ] Video support
- [ ] Auto-eject
- [ ] Settings panel

---

## ✅ Status: KLAR TIL BRUK

Prosjektet er komplett og klart til test!

**Kjør:**
```bash
npm run tauri:dev
```

