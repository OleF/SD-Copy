# 🔧 Feilsøking: Minnekort Oppdages Ikke

## ✅ LØSNING FUNNET OG FIKSET!

### Problemet
Appen oppdaget ikke minnekortet selv om backend fungerte.

### Årsak
**Serde serialisering mismatch**: 
- Rust brukte `snake_case` (has_images, image_count)
- TypeScript forventet `camelCase` (hasImages, imageCount)

Dette førte til at frontend mottok data som `{has_images: true}` men sjekket for `hasImages`, som ga `undefined`.

### Løsning
Lagt til `#[serde(rename_all = "camelCase")]` på alle Rust structs i `src-tauri/src/types.rs`.

---

## 🔍 Hva Ble Endret?

### 1. **Frontend Polling Forbedret**
**Fil:** `src/App.tsx`
- ✅ Bruker nå `useRef` for å unngå stale closure
- ✅ Proper cleanup av interval
- ✅ Skipper system-volumer (Macintosh HD, /)
- ✅ Console logging for debugging

### 2. **Backend Logging Lagt Til**
**Fil:** `src-tauri/src/volume.rs`
- ✅ Logger alle fundne volumer
- ✅ Logger DCIM-deteksjon
- ✅ Logger antall bilder funnet
- ✅ Bedre feilhåndtering

### 3. **Serde Serialisering Fikset** (VIKTIGST!)
**Fil:** `src-tauri/src/types.rs`
- ✅ Alle structs bruker nå `camelCase` når serialisert til JSON
- ✅ `ScanResult`, `ImportProgress`, `ImportManifestEntry`

### 4. **Capabilities Oppdatert**
**Fil:** `src-tauri/capabilities/default.json`
- ✅ Lagt til flere fs-permissions
- ✅ `fs:allow-stat`, `fs:allow-read-file`

---

## 📊 Verifisering

Backend logger nå:
```
Found volume: /Volumes/CAMERA
Found volume: /
Total volumes found: 2
Found DCIM folder in: /Volumes/CAMERA
Scan complete: found 36 images
```

Frontend skal nå motta:
```json
{
  "hasImages": true,
  "imageCount": 36,
  "samplePaths": ["101D3200/DSC_0001.NEF", ...]
}
```

Istedenfor (før):
```json
{
  "has_images": true,
  "image_count": 36,
  "sample_paths": [...]
}
```

---

## 🧪 Test Nå

1. **Start appen:**
   ```bash
   npm run tauri:dev
   ```

2. **Sjekk backend output:**
   ```bash
   tail -f /tmp/tauri-output.log | grep "Scan complete"
   ```

3. **Forvent å se:**
   - App vindu åpner
   - Innen 1.5 sekunder: **modal popper opp**
   - Modal viser: "Fant 36 bilder på CAMERA"

---

## 🐛 Hvis Modal Fortsatt Ikke Vises

### Debug i Browser Console

1. Åpne DevTools: **Cmd+Option+I** (macOS) eller **Høyreklikk → Inspect**
2. Gå til Console-fanen
3. Sjekk for:
   ```javascript
   Polling volumes: ["/Volumes/CAMERA", "/"]
   Scanning volume: /Volumes/CAMERA
   Scan result: {hasImages: true, imageCount: 36, samplePaths: [...]}
   ```

### Hvis du ser `has_images` istedenfor `hasImages`
→ Rust har ikke rekompilert. Kjør:
```bash
cd src-tauri
cargo clean
cd ..
npm run tauri:dev
```

### Hvis ingen console logs
→ Frontend kjører ikke. Sjekk at Vite startet på port 1420.

---

## ✅ Status

| Komponent | Status |
|-----------|--------|
| Backend scanning | ✅ Fungerer (36 bilder funnet) |
| Polling interval | ✅ Kjører hver 1.5s |
| Serde serialisering | ✅ Fikset til camelCase |
| Frontend refs | ✅ Bruker useRef |
| Logging | ✅ Komplett |

---

## 📝 Neste Steg

**Appen skal nå oppdage minnekortet automatisk!**

Hvis det fortsatt ikke fungerer, åpne DevTools Console og del output her.

---

*Oppdatert: 2026-02-27 11:33*  
*Fix: Serde camelCase serialisering*

