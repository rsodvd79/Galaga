// Run against `dist/` served over HTTP, with Playwright available on NODE_PATH.
const assert = require("node:assert/strict");
const { readFileSync } = require("node:fs");
const { chromium, firefox, webkit } = require("playwright");

async function main() {
  const name = process.env.BROWSER || "chromium";
  const browser = await ({ chromium, firefox, webkit })[name].launch({
    headless: true,
    ...(process.env.BROWSER_EXECUTABLE ? { executablePath: process.env.BROWSER_EXECUTABLE } : {}),
  });
  try {
    const page = await browser.newPage({ viewport: { width: 1200, height: 700 }, deviceScaleFactor: 2 });
    const errors = [];
    page.on("pageerror", error => errors.push(error.message));
    await page.addInitScript(() => {
      window.audioContexts = [];
      const Original = window.AudioContext;
      window.AudioContext = class extends Original {
        constructor(...args) { super(...args); window.audioContexts.push(this); }
      };
    });
    await page.goto(process.env.URL || "http://127.0.0.1:8765");
    await page.waitForFunction(() => window.wasmBindings && !document.getElementById("status"));
    await page.locator("#audio").click();
    await page.waitForFunction(() => window.audioContexts.some(context => context.state === "running"));

    async function capture() {
      const download = page.waitForEvent("download");
      await page.locator("#screenshot").click();
      const file = await download;
      assert.match(file.suggestedFilename(), /^galaga_.*\.png$/);
      const png = readFileSync(await file.path());
      assert.equal(png.readUInt32BE(16), 800);
      assert.equal(png.readUInt32BE(20), 600);
      return { file: file.suggestedFilename(), png };
    }

    async function pixels(png, area, color) {
      return page.evaluate(async ({ base64, area, color }) => {
        const bitmap = await createImageBitmap(await (await fetch(`data:image/png;base64,${base64}`)).blob());
        const canvas = document.createElement("canvas");
        canvas.width = 800; canvas.height = 600;
        const ctx = canvas.getContext("2d");
        ctx.drawImage(bitmap, 0, 0);
        const data = ctx.getImageData(...area).data;
        let count = 0;
        for (let i = 0; i < data.length; i += 4) {
          if (color.every((v, index) => Math.abs(data[i + index] - v) < 30)) count++;
        }
        return count;
      }, { base64: png.toString("base64"), area, color });
    }

    assert.equal((await capture()).file, "galaga_menu.png");
    await page.keyboard.press("Space");
    await page.waitForTimeout(300);
    const playing = await capture();
    assert.equal(playing.file, "galaga_playing.png");
    assert.ok(await pixels(playing.png, [380, 548, 40, 32], [50, 210, 90]) > 50);
    await page.keyboard.down("ArrowLeft");
    await page.waitForTimeout(250);
    await page.keyboard.up("ArrowLeft");
    const moved = await capture();
    assert.equal(await pixels(moved.png, [384, 548, 32, 32], [50, 210, 90]), 0);
    await page.keyboard.press("Space");
    await page.keyboard.press("p");
    const paused = await capture();
    assert.equal(paused.file, "galaga_paused.png");
    assert.ok(await pixels(paused.png, [300, 260, 200, 60], [255, 255, 0]) > 50);
    await page.keyboard.press("p");
    assert.equal((await capture()).file, "galaga_playing.png");
    await page.keyboard.press("Escape");
    assert.equal((await capture()).file, "galaga_menu.png");

    await page.evaluate(() => localStorage.setItem("galaga_rust_high_score_v1", "1234"));
    await page.reload();
    await page.waitForFunction(() => !document.getElementById("status"));
    await page.setViewportSize({ width: 600, height: 900 });
    assert.equal((await capture()).file, "galaga_menu.png");
    assert.equal(await page.evaluate(() => localStorage.getItem("galaga_rust_high_score_v1")), "1234");
    assert.deepEqual(errors, []);

    // Storage denied and audio unavailable must still leave a playable game.
    const unavailable = await browser.newPage();
    const unavailableErrors = [];
    unavailable.on("pageerror", error => unavailableErrors.push(error.message));
    await unavailable.addInitScript(() => {
      Object.defineProperty(window, "localStorage", { get() { throw new Error("storage denied"); } });
      window.AudioContext = class { constructor() { throw new Error("no audio device"); } };
    });
    await unavailable.goto(process.env.URL || "http://127.0.0.1:8765");
    await unavailable.waitForFunction(() => !document.getElementById("status"));
    await unavailable.locator("#audio").click();
    await unavailable.keyboard.press("Space");
    const failedAudioDownload = unavailable.waitForEvent("download");
    await unavailable.locator("#screenshot").click();
    assert.equal((await failedAudioDownload).suggestedFilename(), "galaga_playing.png");
    assert.deepEqual(unavailableErrors, []);
    console.log(`${name}: boot, audio gesture, input, pause, resize/HiDPI, record, PNG and unavailable services passed`);
  } finally { await browser.close(); }
}

main().catch(error => { console.error(error); process.exitCode = 1; });
