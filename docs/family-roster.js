/* Family map — search / filter. Names stay in the HTML. */

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
    const toneClass = tone === "poet" ? "poet" : tone || "catalog";
    statusEl.textContent = status;
    statusEl.className = `status-label status-${toneClass}`;
  }
  box.removeAttribute("hidden");
}

function wireFamilyRoster(root) {
  const tiles = [...root.querySelectorAll(".roster-tile")];
  const clusters = [...root.querySelectorAll("[data-roster-cluster]")];
  const search = root.querySelector("#roster-search") || document.getElementById("roster-search");
  const countEl = root.querySelector("#roster-count") || document.getElementById("roster-count");
  const filters = [...root.querySelectorAll("[data-roster-filter]")];
  if (!tiles.length) return;

  let cluster = "all";

  const activate = (item) => {
    tiles.forEach((el) => {
      el.classList.toggle("is-active", el === item);
      el.setAttribute("aria-pressed", el === item ? "true" : "false");
    });
    setExplainer(root, {
      title: item.dataset.title || item.textContent.trim(),
      status: item.dataset.status || "Works here",
      tone: item.dataset.tone || "live",
      plain: item.dataset.plain || "",
      tech: item.dataset.tech || "",
    });
  };

  tiles.forEach((item) => {
    item.addEventListener("click", () => activate(item));
  });

  const apply = () => {
    const q = (search?.value || "").trim().toLowerCase();
    let visible = 0;
    tiles.forEach((tile) => {
      const name = tile.dataset.name || "";
      const inCluster = cluster === "all" || tile.dataset.cluster === cluster;
      const inSearch =
        !q ||
        name.toLowerCase().includes(q) ||
        (tile.dataset.title || "").toLowerCase().includes(q) ||
        (tile.dataset.plain || "").toLowerCase().includes(q) ||
        (tile.textContent || "").toLowerCase().includes(q);
      const show = inCluster && inSearch;
      tile.hidden = !show;
      if (show) visible += 1;
    });
    clusters.forEach((group) => {
      const any = [...group.querySelectorAll(".roster-tile")].some((tile) => !tile.hidden);
      group.classList.toggle("is-empty", !any);
    });
    if (countEl) {
      countEl.textContent = `${visible} of 105 families`;
    }
  };

  filters.forEach((btn) => {
    btn.addEventListener("click", () => {
      cluster = btn.dataset.rosterFilter || "all";
      filters.forEach((el) => el.classList.toggle("is-on", el === btn));
      apply();
    });
  });
  search?.addEventListener("input", apply);

  const hashed = tiles.find((item) => item.id && `#${item.id}` === location.hash);
  const named = tiles.find((item) => location.hash === `#family-${item.dataset.name}`);
  activate(hashed || named || tiles[0]);
  apply();
}

document.querySelectorAll("[data-family-roster]").forEach(wireFamilyRoster);
