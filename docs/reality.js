/* What it can do — tap honeycomb / family cards for a plain + technical one-liner.
   Soft-rise still arrives when the human prefers reduced motion. */

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
      status: item.dataset.status || "held / not yet",
      tone: item.dataset.tone || "notyet",
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

observeSoftRise();
reduceMotion.addEventListener?.("change", () => {
  if (reduceMotion.matches) document.querySelectorAll(".soft-rise").forEach(markArrived);
});
document.querySelectorAll("[data-honey]").forEach((root) => wireSelectable(root, ".hex"));
document.querySelectorAll("[data-family-map]").forEach((root) => wireSelectable(root, ".family-card"));
document.querySelectorAll("[data-flow]").forEach((root) => wireSelectable(root, ".boundary-node"));
document.querySelectorAll("[data-logic-tiles]").forEach((root) => wireSelectable(root, ".reality-card"));
wireLayerTabs();
