# Vibe 0.1 vocab (current dialect)

This is the **customer and authoring word list** for vibe-script pages and helpers. It is not the first-shot workshop sketch. Engine syntax still lives in QualiaDB `0.0.38` [`vibescript-core.md`](https://github.com/mediaprophet/qualiaDB/blob/0.0.38/docs/manuals/standards/vibescript-core.md). Workspace version stays **0.0.1**.

`vibe-0.1` is the **current dialect**. Grammar **may grow** when humans need a better form — versioned, documented. A Host does not invent new words. The **catalog already grows**.

## Who

| Word | Use |
|---|---|
| **human** | The living subject. You. Directory lists humans first. |
| **person** | What the law still says. Keep this word in law text. |
| **handle** | A name, login, DID, DNS name, or session id. A handle is not the person. |
| **chatbot / agent** | A tool. Not the other party. Not a human. |
| **Poet / Desktop** | The full habitat app. Still in the product. |

## Status (pages)

Say only these on customer pages:

| Chip | Meaning |
|---|---|
| **Works here** | Runs on this page. |
| **Works in Poet / Desktop** | In the product. Open the full app. Not a teaser. |
| **Planned** | Not built yet. Today: a public internet relay. |

Do not put `ALL_BOUND`, `held / not yet`, `Capability.method`, or `LocalHost` on customer pages. Tooling diagnose (`docs/diagnose.md`) still uses held / not yet for wait-honest chrome.

## What it can do

- **Write a script. Run it. Change it without rebuilding the app.**
- **It already does:** logic you can check; math that runs (including linear algebra); graphs you can ask; motion you can play; saves you can open again.
- **104 families** in plain words (Rules, Shapes, Math, Graphs, Motion…). The catalog name stays on the tile so nothing is hidden.
- If a listed live family does not run, that is a **bug to finish** — do not hide it.

## Authoring

```vibe
using Animation, LinearAlgebra;
cell score := LinearAlgebra.dot({ a: xs, b: ys });
present lamp { color: #ff8800 }
```

- Lease a live family, then call it. Do not teach `capability.invoke` as the human dialect.
- Unknown names fail closed. Do not invent Host ids or dotted `qualia.*`.
- Domain vocabularies stay canonical: `prefix snomed: <http://snomed.info/id/>;` plus a bounded chunk. No rename layer. Do not ingest all of SNOMED.

## Grammar and catalog

- **vibe-0.1** is current, not frozen-as-law.
- Grammar may grow when humans need a better form. New forms are versioned and documented. This repo does not mint keywords; the Host does not invent them.
- The Host catalog already grows. Count the tip. Do not hardcode a forever id count.

See also: [`language-contract.md`](language-contract.md), [`../skills/SKILL.md`](../skills/SKILL.md), [`reality.html`](reality.html).
