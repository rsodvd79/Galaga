# Piano di miglioramento grafico — Galaga

## Obiettivo
Ridisegnare i personaggi e gli effetti in stile **retro pixel-art**, rimanendo 100% codice Avalonia (`DrawingContext`) e senza introdurre asset esterni.

---

## Contesto tecnico
- Stack: .NET 8, Avalonia 11.
- Tutti i disegni avvengono tramite `SpriteRenderer` (`Galaga/Views/SpriteRenderer.cs`) e `GameCanvas` (`Galaga/Views/GameCanvas.cs`).
- I nemici sono già basati su sprite map ASCII `8×8` scalate `3×` → `24×24`.
- Il player è disegnato con `StreamGeometry` e rettangoli; il proiettile e le esplosioni con rettangoli.
- `GameEngine` non dipende da Avalonia: le modifiche devono restare confinate nella vista.

Skill utilizzabili: `avalonia`, `avalonia-api`. Nessuna nuova skill da installare.

---

## Fase 1 — Player
**Stato:** completata

### Risultato
La navicella del giocatore è ora uno sprite pixel-art `16×16` renderizzato a `32×32`.

### Modifiche effettuate
- `Player.cs`: aggiunte costanti `SpritePixelScale`, `SpriteCols`, `SpriteRows`, `SpriteWidth`, `SpriteHeight`; `Width`/`Height` aggiornati a 32×32; `DefaultX`/`DefaultY` riallineati.
- `SpriteRenderer.cs`:
  - Nuove sprite map ASCII `PlayerPx0`/`PlayerPx1` con cella `w` per il nucleo bianco della fiamma.
  - Pennelli `Player*` e palette a due frame.
  - `DrawPlayer(DrawingContext, Player, int)` che accetta l’istanza `Player` per la scia.
  - `DrawPlayerAt(..., moveLeft, moveRight)` disegna una copia opacizzata al 25% spostata in direzione opposta al movimento.
  - Luci di navigazione rosse/verdi lampeggianti alternate ogni frame.
  - Nuova mini navicella `MiniPlayerPx` per l’HUD.
- `GameCanvas.cs`: chiamata aggiornata per passare `Player` al renderer.

### Verifica
- `dotnet test`: 20/20 superati.
- Screenshot `--screenshots`: menu + gameplay confermano il rendering corretto.

---

## Fase 2 — Nemici
**Stato:** completata

### Risultato
Bee, Butterfly e Boss Galaga hanno sprite pixel-art più distinti; stati di movimento visibili tramite inclinazione e scia.

### Modifiche effettuate
- `SpriteRenderer.cs`:
  - Palette estesa per ogni nemico (toni scuri, highlight, occhi, antenne).
  - Nuove sprite map `8×8` per Bee, Butterfly e Boss con dettagli più ricchi.
  - `DrawEnemyTrail`: scia arancione opacizzata dietro i nemici in `Diving`/`Returning`.
  - `DrawBee`/`DrawButterfly`/`DrawBossGalaga` ricevono l’intero `Enemy`.
  - L’inclinazione (tilt) è stata rimossa: i pixel shiftavano le righe e apparivano distorti durante la caduta verticale.

### Verifica
- `dotnet test`: 20/20 superati.
- Screenshot `--screenshots`: formazioni e nemici in immersione renderizzati correttamente.

---

## Fase 3 — Proiettili ed esplosioni
**Stato:** completata

### Risultato
Proiettili divenuti raggi laser stratificati; esplosioni più ricche con anello d’onda e fumo.

### Modifiche effettuate
- `SpriteRenderer.cs`:
  - Aggiunti pennelli per testa/core/coda dei proiettili (verdi per il player, rossi per i nemici).
  - Nuovo metodo `DrawBullet(DrawingContext, Bullet)`: disegna tre strati verticali per simulare un raggio incandescente.
  - `DrawExplosion` rinnovato:
    - anello d’onda giallo con opacità decrescente
    - 12 particelle invece di 8, con distribuzione casuale
    - flash bianco iniziale
    - nucleo di fumo grigio nella fase finale
- `GameCanvas.cs`: ciclo proiettili ora chiama `SpriteRenderer.DrawBullet` invece di `FillRectangle`.

### Verifica
- `dotnet test`: 20/20 superati.
- Screenshot `--screenshots`: proiettili e formazioni renderizzati correttamente.

---

## Fase 4 — Sfondo, stelle e HUD
**Stato:** completata

### Risultato
Campo stellare più ricco con parallasse e HUD più leggibile.

### Modifiche effettuate
- `GameCanvas.cs`:
  - Aumentate le stelle a 110 con 4 livelli di profondità.
  - Ogni livello ha velocità propria (`StarSpeeds`), dimensione e opacità diverse per l’effetto parallasse.
  - Aggiunto livello stellare “bluastro” per varietà cromatica.
  - `DrawText` disegna un’ombra nera opacizzata dietro ogni stringa di testo.

### Verifica
- `dotnet test`: 20/20 superati.
- Screenshot `--screenshots`: menu e gameplay con stelle più dinamiche e testo nitido.

---

## Note comuni
- Non aggiungere asset esterni (PNG/SVG).
- Non introdurre dipendenze Avalonia in `GameEngine` o `GameState`.
- Mantenere il formato 800×600 logico gestito da `GameCanvas`.
- Dopo ogni fase eseguire `dotnet test` per validare la logica.
