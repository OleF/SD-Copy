# GitHub Repository Setup - Quick Reference

## 📦 Repository Created Locally

```
✅ Git initialized
✅ All files committed
✅ Ready to push
```

---

## 🚀 RASKESTE METODE: Kjør Script

```bash
./push-to-github.sh
```

Dette scriptet:
1. Ber deg om GitHub username
2. Setter opp remote
3. Pusher til GitHub
4. Håndterer feil

---

## 🔧 MANUELL METODE

### 1. Opprett Repository på GitHub

Gå til: https://github.com/new

```
Repository name: SD-Copy
Description: Photo import app for SD cards - Tauri v2 + Rust + React
Visibility: Public (anbefalt) eller Private
❌ Ikke hak av "Initialize with README"
```

### 2. Legg Til Remote og Push

**Med HTTPS:**
```bash
cd /Users/olefredrikschreuder/dev/SD-Copy
git remote add origin https://github.com/USERNAME/SD-Copy.git
git branch -M main
git push -u origin main
```

**Med SSH:**
```bash
cd /Users/olefredrikschreuder/dev/SD-Copy
git remote add origin git@github.com:USERNAME/SD-Copy.git
git branch -M main
git push -u origin main
```

*(Erstatt `USERNAME` med ditt GitHub-brukernavn)*

---

## 🎨 Etter Push: Pynt Repository

### Legg til Topics
Klikk "Add topics" på GitHub:
```
tauri, rust, react, typescript, desktop-app, 
photo-management, sd-card, macos, windows, 
file-management, import-tool
```

### Legg til About Section
```
Description: 
Automatic photo import app for SD cards. Built with Tauri v2, 
Rust, and React. Detects mounted volumes, scans for images, 
and imports with progress tracking.

Website: (optional)
```

### Enable Features
- ✅ Releases
- ✅ Issues (hvis du vil ha feedback)
- ✅ Projects (optional)

---

## 📊 Repository Stats (etter push)

- **Languages:** Rust (45%), TypeScript (35%), CSS (10%), Other (10%)
- **Files:** 42
- **Lines of Code:** ~2500
- **Documentation:** 6 comprehensive markdown files

---

## 🏷️ Suggested Badges (legg til i README.md)

```markdown
![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Windows-blue)
![Tauri](https://img.shields.io/badge/Tauri-v2.0-brightgreen)
![Rust](https://img.shields.io/badge/Rust-stable-orange)
![React](https://img.shields.io/badge/React-18.3-blue)
![TypeScript](https://img.shields.io/badge/TypeScript-5.6-blue)
![License](https://img.shields.io/badge/license-MIT-green)
```

---

## 🔐 Autentisering

### HTTPS (anbefalt for begynnere)

Du trenger Personal Access Token:

1. https://github.com/settings/tokens
2. "Generate new token (classic)"
3. Velg scope: `repo` (all)
4. Kopier token
5. Bruk som passord når git spør

### SSH (for avanserte)

Sjekk om du har nøkkel:
```bash
ls ~/.ssh/id_*.pub
```

Hvis ikke:
```bash
ssh-keygen -t ed25519 -C "your_email@example.com"
cat ~/.ssh/id_ed25519.pub
# Kopier output og legg til på: https://github.com/settings/keys
```

---

## 🎯 URL-er

- **Repository:** `https://github.com/USERNAME/SD-Copy`
- **Clone URL (HTTPS):** `https://github.com/USERNAME/SD-Copy.git`
- **Clone URL (SSH):** `git@github.com:USERNAME/SD-Copy.git`

---

## ✅ Verifiser at Alt Er Pushed

```bash
git log --oneline
git remote -v
git status
```

Skal vise:
```
✅ Initial commit synlig
✅ Remote origin konfigurert
✅ Branch main oppdatert
✅ Working tree clean
```

---

## 📝 Fremtidige Oppdateringer

```bash
# Gjør endringer i koden
# ...

# Commit og push
git add .
git commit -m "Beskrivelse av endring"
git push
```

---

**Alt er klart! Kjør enten `./push-to-github.sh` eller følg manual-stegene over.** 🚀

