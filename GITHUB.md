# 🚀 Push til GitHub - Steg-for-Steg Guide

## ✅ Git Repository Er Klar!

Jeg har allerede:
- ✅ Initialisert Git repository
- ✅ Lagt til alle filer
- ✅ Opprettet initial commit

---

## 📋 Metode 1: Via GitHub.com (Anbefalt)

### Steg 1: Opprett Repository på GitHub

1. Gå til [github.com](https://github.com) og logg inn
2. Klikk på **"+"** oppe til høyre → **"New repository"**
3. Fyll ut:
   - **Repository name:** `SD-Copy` eller `sd-copy-app`
   - **Description:** `Photo import app for SD cards - Tauri v2 + Rust + React`
   - **Visibility:** Public eller Private (ditt valg)
   - **❌ IKKE** hak av "Initialize with README" (vi har allerede en)
4. Klikk **"Create repository"**

### Steg 2: Push Koden

GitHub vil vise deg kommandoer. Kjør dette i terminalen:

```bash
cd /Users/olefredrikschreuder/dev/SD-Copy

# Legg til GitHub remote (erstatt USERNAME med ditt GitHub-brukernavn)
git remote add origin https://github.com/USERNAME/SD-Copy.git

# Push til GitHub
git branch -M main
git push -u origin main
```

**Eller hvis du bruker SSH:**
```bash
git remote add origin git@github.com:USERNAME/SD-Copy.git
git branch -M main
git push -u origin main
```

---

## 📋 Metode 2: Med GitHub CLI (Raskest)

Hvis du vil installere GitHub CLI:

```bash
# Installer med Homebrew
brew install gh

# Autentiser
gh auth login

# Opprett repository og push automatisk
cd /Users/olefredrikschreuder/dev/SD-Copy
gh repo create SD-Copy --public --source=. --push
```

---

## 🔑 Autentisering

### Hvis du får feil ved push:

#### For HTTPS:
GitHub krever Personal Access Token (ikke passord):

1. Gå til GitHub.com → Settings → Developer settings → Personal access tokens → Tokens (classic)
2. **Generate new token (classic)**
3. Velg scopes: `repo` (alle)
4. Kopier token
5. Bruk token som passord når du pusher

#### For SSH:
Sjekk om du har SSH-nøkkel:
```bash
ls -la ~/.ssh/id_*.pub
```

Hvis ikke, opprett en:
```bash
ssh-keygen -t ed25519 -C "your_email@example.com"
cat ~/.ssh/id_ed25519.pub
```

Legg til på GitHub: Settings → SSH and GPG keys → New SSH key

---

## 📊 Repository Innhold

Når du har pushet, vil GitHub vise:

### Prosjektstruktur
- ✅ 42 filer
- ✅ ~2500 linjer kode
- ✅ Rust backend
- ✅ React frontend
- ✅ Komplett dokumentasjon

### README.md
- Automatisk vises på repository-forsiden
- Komplett feature-liste
- Installasjonsinstruksjoner
- Teknologi stack

### Dokumentasjon
- `START_HERE.md` - Hurtigstart
- `QUICKSTART.md` - Detaljert guide
- `ARCHITECTURE.md` - Teknisk dokumentasjon
- `TROUBLESHOOTING.md` - Feilsøking
- `FIXED.md` - Fiks-historikk

---

## 🏷️ Legg Til Topics (Anbefalt)

På GitHub repository-siden, klikk **"Add topics"** og legg til:
```
tauri
rust
react
typescript
photo-management
sd-card
desktop-app
macos
windows
```

---

## 📝 Fremtidige Endringer

Når du gjør endringer:

```bash
# Se endringer
git status

# Legg til endringer
git add .

# Commit
git commit -m "beskrivelse av endring"

# Push til GitHub
git push
```

---

## 🔒 .gitignore Er Allerede Satt Opp

Følgende ignoreres automatisk:
- `node_modules/`
- `dist/`
- `src-tauri/target/`
- `.DS_Store`
- IDE-filer

---

## ✅ Neste Steg

1. **Opprett repository på GitHub.com**
2. **Kjør git remote add kommandoen** (se over)
3. **Push med git push -u origin main**
4. **Legg til topics og beskrivelse**
5. **Ferdig!** 🎉

---

## 🎯 Repository URL

Når opprettet, vil URLen være:
```
https://github.com/USERNAME/SD-Copy
```

---

## 📞 Hvis Du Får Problemer

### "Permission denied"
→ Sjekk autentisering (SSH-nøkkel eller token)

### "Repository not found"
→ Sjekk at du har erstattet USERNAME med ditt brukernavn

### "Failed to push"
→ Sjekk at remote er riktig lagt til: `git remote -v`

---

**Klar til å pushe! Følg stegene over.** 🚀

---

*Opprettet: 2026-02-27*  
*Git commit: ✅ Klar*  
*Files: 42 tracked*

