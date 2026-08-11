import init, { summarize } from "./pkg/{{crate_name}}_web.js";

const el = (id) => document.getElementById(id);
const dropZone = el("drop-zone");
const fileInput = el("file-input");

async function loadFile(file) {
  const bytes = new Uint8Array(await file.arrayBuffer());
  const summary = JSON.parse(summarize(bytes));
  el("file-name").textContent = file.name;
  el("output").textContent = JSON.stringify(summary, null, 2);
  el("result").hidden = false;
}

dropZone.addEventListener("click", () => fileInput.click());
dropZone.addEventListener("keydown", (event) => {
  if (event.key === "Enter" || event.key === " ") fileInput.click();
});
fileInput.addEventListener("change", () => {
  if (fileInput.files.length > 0) void loadFile(fileInput.files[0]);
});
dropZone.addEventListener("dragover", (event) => {
  event.preventDefault();
  dropZone.classList.add("drag");
});
dropZone.addEventListener("dragleave", () => dropZone.classList.remove("drag"));
dropZone.addEventListener("drop", (event) => {
  event.preventDefault();
  dropZone.classList.remove("drag");
  if (event.dataTransfer.files.length > 0) void loadFile(event.dataTransfer.files[0]);
});

// Offline support. Skipped on localhost so dev never fights a stale cache;
// persistent storage keeps the browser from evicting the cache under pressure.
function registerServiceWorker() {
  if (!("serviceWorker" in navigator)) return;
  if (location.hostname === "localhost" || location.hostname === "127.0.0.1") return;
  navigator.serviceWorker.register("./sw.js").catch((err) => {
    console.warn("Service worker registration failed:", err);
  });
  if (navigator.storage?.persist) {
    navigator.storage.persist().catch(() => {});
  }
}

await init();
registerServiceWorker();
