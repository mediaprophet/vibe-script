/* What it can do — try a script; family map is in family-roster.js. */

import init, {
  eval_program_src,
  diagnose_src,
} from "./pkg/vibe/vibe_wasm.js";

const reduceMotion = window.matchMedia("(prefers-reduced-motion: reduce)");

function markArrived(node) {
  node.classList.add("is-in");
  if (reduceMotion.matches) node.classList.add("still-arrive");
}

function observeSoftRise() {
  const nodes = [...document.querySelectorAll(".soft-rise")];
  if (!nodes.length) return;
  if (reduceMotion.matches || !("IntersectionObserver" in window)) {
    nodes.forEach(markArrived);
    return;
  }
  const io = new IntersectionObserver(
    (entries) => {
      for (const entry of entries) {
        if (entry.isIntersecting) {
          markArrived(entry.target);
          io.unobserve(entry.target);
        }
      }
    },
    { rootMargin: "0px 0px -8% 0px", threshold: 0.12 }
  );
  nodes.forEach((node) => io.observe(node));
}

function setExplainer(root, { title, status, tone, plain, tech }) {
  const box = root.querySelector("[data-explainer]");
  if (!box) return;
  const statusEl = box.querySelector("[data-explainer-status]");
  const titleEl = box.querySelector("[data-explainer-title]");
  const plainEl = box.querySelector("[data-explainer-plain]");
  const techEl = box.querySelector("[data-explainer-tech]");
  if (titleEl) titleEl.textContent = title;
  if (plainEl) plainEl.textContent = plain;
  if (techEl) techEl.textContent = tech;
  if (statusEl) {
    statusEl.textContent = status;
    statusEl.className = `status-label status-${tone || "catalog"}`;
  }
  box.removeAttribute("hidden");
}

function wireSelectable(root, selector) {
  const items = [...root.querySelectorAll(selector)];
  if (!items.length) return;
  const activate = (item) => {
    items.forEach((el) => {
      el.classList.toggle("is-active", el === item);
      if (el.tagName === "BUTTON") el.setAttribute("aria-pressed", el === item ? "true" : "false");
    });
    setExplainer(root, {
      title: item.dataset.title || item.textContent.trim(),
      status: item.dataset.status || "Works here",
      tone: item.dataset.tone || "live",
      plain: item.dataset.plain || "",
      tech: item.dataset.tech || "",
    });
  };
  items.forEach((item) => {
    item.addEventListener("click", () => activate(item));
    item.addEventListener("keydown", (event) => {
      if (event.key === "Enter" || event.key === " ") {
        event.preventDefault();
        activate(item);
      }
    });
  });
  const hashed = items.find((item) => item.id && `#${item.id}` === location.hash);
  activate(hashed || items[0]);
}

function wireLayerTabs() {
  const tabs = [...document.querySelectorAll("[data-layer-tab]")];
  const panels = [...document.querySelectorAll("[data-layer-panel]")];
  if (!tabs.length) return;
  const show = (id) => {
    tabs.forEach((tab) => {
      const on = tab.dataset.layerTab === id;
      tab.setAttribute("aria-selected", on ? "true" : "false");
    });
    panels.forEach((panel) => {
      const on = panel.dataset.layerPanel === id;
      panel.hidden = !on;
    });
    const visible = panels.find((panel) => panel.dataset.layerPanel === id);
    const firstHex = visible?.querySelector(".hex");
    firstHex?.click();
  };
  tabs.forEach((tab) => tab.addEventListener("click", () => show(tab.dataset.layerTab)));
  show(tabs[0].dataset.layerTab);
}

const LIVE_FIXTURES = {
  math: {
    label: "Math — a dot product",
    src: `using LinearAlgebra;

effect fn main() {
    return LinearAlgebra.dot({ a: [1.0, 2.0, 3.0], b: [4.0, 5.0, 6.0] });
}`,
  },
  should: {
    label: "Logic — should",
    src: `using DeonticLogic;

pure fn main() {
    return obligate { "the signer must confirm" };
}`,
  },
  graph: {
    label: "Graph — ask",
    src: `using GraphDatabase;

effect fn main() {
    return GraphDatabase.sparql({ query: "SELECT * WHERE { ?s ?p ?o }" });
}`,
  },
  save: {
    label: "Save — open again",
    src: `using GraphDatabase;

effect fn main() {
    return GraphDatabase.volume_open({ name: "notes" });
}`,
  },
  motion: {
    label: "Motion — an orbit",
    src: `using Animation;

effect fn main() {
    return Animation.orbit_spin(1.0);
}`,
  },
};

function pretty(value) {
  return JSON.stringify(value, null, 2);
}

function wireLiveEval() {
  const picker = document.getElementById("eval-picker");
  const srcEl = document.getElementById("eval-src");
  const outEl = document.getElementById("eval-out");
  const badge = document.getElementById("eval-badge");
  const btnEval = document.getElementById("btn-live-eval");
  const btnDiag = document.getElementById("btn-live-diagnose");
  if (!picker || !srcEl || !outEl || !badge || !btnEval || !btnDiag) return;

  Object.entries(LIVE_FIXTURES).forEach(([key, item]) => {
    const option = document.createElement("option");
    option.value = key;
    option.textContent = item.label;
    picker.appendChild(option);
  });

  const showSrc = () => {
    srcEl.textContent = LIVE_FIXTURES[picker.value].src;
  };
  picker.addEventListener("change", showSrc);
  showSrc();

  let ready = false;
  const run = (mode) => {
    if (!ready) return;
    const src = LIVE_FIXTURES[picker.value].src;
    const start = performance.now();
    try {
      const result = mode === "diagnose" ? diagnose_src(src) : eval_program_src(src);
      const ok = mode === "diagnose" ? result.valid : result.ok;
      outEl.textContent = pretty(result);
      outEl.className = `output eval-out ${ok ? "ok" : "err"}`;
      badge.textContent = `${(performance.now() - start).toFixed(2)} ms`;
    } catch (error) {
      outEl.textContent = `Couldn’t run that: ${error}`;
      outEl.className = "output eval-out err";
      badge.textContent = "try again";
    }
  };

  btnEval.addEventListener("click", () => run("eval"));
  btnDiag.addEventListener("click", () => run("diagnose"));

  init()
    .then(() => {
      ready = true;
      badge.textContent = "ready";
      outEl.textContent = `Ready. Pick a script and Run.`;
      outEl.className = "output eval-out";
    })
    .catch((error) => {
      badge.textContent = "try again";
      outEl.textContent = `Couldn’t start the engine.\n${error}`;
      outEl.className = "output eval-out err";
    });
}

observeSoftRise();
reduceMotion.addEventListener?.("change", () => {
  if (reduceMotion.matches) document.querySelectorAll(".soft-rise").forEach(markArrived);
});
document.querySelectorAll("[data-honey]").forEach((root) => wireSelectable(root, ".hex"));
document.querySelectorAll("[data-family-map]").forEach((root) => wireSelectable(root, ".family-card"));
document.querySelectorAll("[data-flow]").forEach((root) => wireSelectable(root, ".boundary-node"));
document.querySelectorAll("[data-logic-tiles]").forEach((root) => wireSelectable(root, ".reality-card"));
wireLayerTabs();
wireLiveEval();
