# SD Copy - Photo Import App

![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Windows-blue)
![Tauri](https://img.shields.io/badge/Tauri-v2.0-24C8DB?logo=tauri)
![Rust](https://img.shields.io/badge/Rust-stable-orange?logo=rust)
![React](https://img.shields.io/badge/React-18.3-61DAFB?logo=react)
![TypeScript](https://img.shields.io/badge/TypeScript-5.6-3178C6?logo=typescript)
![License](https://img.shields.io/badge/license-MIT-green)

En moderne, automatisk foto-import applikasjon for SD-kort og minnekort. SD Copy gjør det enkelt og trygt å importere bilder fra kamera eller minnekort til din datamaskin med intelligent overvåking, duplikathåndtering og full sporbarhet.

Bygget med Tauri v2, Rust og React for høy ytelse, sikkerhet og en nativ brukeropplevelse på macOS og Windows.

## Hva gjør SD Copy?

SD Copy automatiserer den vanlige arbeidsflyten for fotografer og hobbyister:

1. **Automatisk oppdagelse** - Setter du inn et SD-kort eller minnekort, oppdager appen det automatisk og skanner for bilder
2. **Intelligent import** - Velg destinasjonsmappe, gi importen et navn (standardformat: 270226 - Import), og start kopieringen
3. **Trygg kopiering** - Filer kopieres (ikke flyttes), med bevaring av mappestruktur og automatisk håndtering av duplikater
4. **Full sporing** - Hvert import genererer en manifest-fil som dokumenterer alle kopierte filer med detaljer

Appen støtter alle vanlige RAW- og bildeformater inkludert JPG, PNG, HEIC, RAF (Fujifilm), CR2/CR3 (Canon), NEF (Nikon), ARW (Sony), DNG og flere.

## Funksjonalitet (MVP)

- ✅ Automatisk overvåking av monterte volumer (polling hver 1500ms)
- ✅ Deteksjon av bilder (DCIM-mappe eller støttede filtyper)
- ✅ Støttede formater: JPG, JPEG, PNG, HEIC, TIF, TIFF, RAF, ARW, CR2, CR3, NEF, DNG
- ✅ Import wizard med tilpassbart mappenavn
- ✅ Foreslått mappenavn: DDMMÅÅ - Import
- ✅ Native mappe-velger for destinasjon
- ✅ Kopierer filer (ikke flytter) med bevarelse av undermappestruktur
- ✅ Progress tracking med event-basert oppdatering
- ✅ Håndtering av duplikater med suffix (1), (2), etc.
- ✅ Import-manifest (JSON) med detaljer om alle kopierte filer
- ✅ Feilhåndtering og mulighet til å avbryte import

## Krav

- Node.js (v18 eller nyere)
- Rust (stable)
- npm eller yarn

### Installere Rust (PÅKREVD)

Rust må være installert for å bygge appen:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Etter installasjon, restart terminalen eller kjør:

```bash
source $HOME/.cargo/env
```

Verifiser at Rust er installert:

```bash
rustc --version
cargo --version
```

## Installasjon

1. Klon eller naviger til prosjektet:

```bash
cd /Users/olefredrikschreuder/dev/SD-Copy
```

2. Installer npm-avhengigheter:

```bash
npm install
```

## Kjøring

### Development mode

```bash
npm run tauri:dev
```

Dette vil:
- Starte Vite dev server på port 1420
- Bygge Rust-backend
- Åpne appen i development mode med hot reload

### Build for produksjon

```bash
npm run tauri:build
```

Dette lager en distribuerbar app for din plattform i `src-tauri/target/release/bundle/`

## Prosjektstruktur

```
SD-Copy/
├── src/                          # React frontend
│   ├── components/
│   │   ├── WizardModal.tsx      # Import wizard dialog
│   │   └── ProgressView.tsx     # Progress tracking UI
│   ├── types.ts                  # TypeScript type definitions
│   ├── utils.ts                  # Frontend utilities
│   ├── App.tsx                   # Main app component
│   ├── App.css                   # Styling
│   └── main.tsx                  # React entry point
├── src-tauri/                    # Rust backend
│   ├── src/
│   │   ├── main.rs              # Tauri app entry point
│   │   ├── volume.rs            # Volume detection and scanning
│   │   ├── import.rs            # Import manager and logic
│   │   ├── types.rs             # Rust type definitions
│   │   └── utils.rs             # Rust utilities
│   ├── Cargo.toml               # Rust dependencies
│   ├── tauri.conf.json          # Tauri configuration
│   └── build.rs                 # Build script
├── package.json                  # Node dependencies and scripts
├── vite.config.ts               # Vite configuration
├── tsconfig.json                # TypeScript configuration
└── index.html                   # HTML entry point
```

## Hvordan det fungerer

### 1. Volume Monitoring

Appen poller `/Volumes/` (macOS) eller drive letters (Windows) hver 1500ms for å oppdage nye monterte enheter.

### 2. Image Detection

Når et nytt volum oppdages:
- Sjekker om det finnes en `DCIM/` mappe
- Skanner rekursivt etter filer med bilde-endelser
- Returnerer antall bilder og eksempel-filer

### 3. Import Process

Når brukeren starter import:
1. Velg destinasjonsmappe via native dialog
2. Rediger mappenavn (standard: DDMMÅÅ - Import)
3. Filer kopieres med bevarelse av mappestruktur
4. Duplikater håndteres automatisk
5. Progress sendes via Tauri events
6. Manifest genereres med alle detaljer

### 4. Manifest Format

`import-manifest.json` inneholder:

```json
[
  {
    "sourcePath": "/Volumes/SD-CARD/DCIM/100MSDCF/IMG_0001.JPG",
    "destPath": "/Users/navn/Pictures/270226 - Import/100MSDCF/IMG_0001.JPG",
    "sizeBytes": 2547896,
    "timestamp": "2026-02-27T10:30:45.123Z"
  }
]
```

## Tauri v2 Sikkerhet

Appen bruker Tauri v2 security capabilities:

- **Dialog Plugin**: For native filvelger
- **FS Plugin**: For filsystem-tilgang med scope til:
  - `/Volumes/**` (macOS volumes)
  - `$HOME/**` (brukerens hjemmemappe)
  - `$APPDATA/**` (app data)

Disse er konfigurert i `tauri.conf.json` under `plugins`.

## Plattformspesifikk Informasjon

### macOS

- Volumer monteres i `/Volumes/`
- Krever ingen ekstra tillatelser for lesing av volumer
- App signering kan være nødvendig for distribusjon

### Windows

- Sjekker drive letters A-Z
- Samme funksjonalitet som macOS

## Feilsøking

### "Failed to read volumes"

Sjekk at `/Volumes/` eksisterer (macOS) eller at drives er tilgjengelige (Windows).

### "Import feilet"

Sjekk at:
- Destinasjonsmappen er skrivbar
- SD-kortet er fortsatt montert
- Det er nok diskplass

### Appen kompilerer ikke

Sjekk at du har:
- Nyeste versjon av Rust: `rustup update`
- Node.js v18+: `node --version`
- Alle npm dependencies: `npm install`

## Lisens

Dette prosjektet er lisensiert under MIT License - se [LICENSE](LICENSE) filen for detaljer.

**TL;DR:** Du kan fritt bruke, modifisere og distribuere denne koden, også i kommersielle prosjekter.

## Fremtidige Forbedringer (utenfor MVP)

- [ ] Native OS events for volume mounting (istedenfor polling)
- [ ] Bilde-forhåndsvisning i wizard
- [ ] Filtrering etter dato/kamera
- [ ] Automatisk import uten wizard (power user mode)
- [ ] Import-historikk og logging
- [ ] Støtte for video-filer
- [ ] Batch rename options
- [ ] Auto-eject etter import


