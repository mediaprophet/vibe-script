# Catalog honesty (ecosystem)

**Live list:** QualiaDB `crates/vibe/src/catalog/ids.rs` → `ALL_INVOKE_IDS` (104 families + `agent` on What it can do).  
**Policy:** [`vibescript-core.md` §11.5](https://github.com/mediaprophet/qualiaDB/blob/0.0.38/docs/manuals/standards/vibescript-core.md) · [`vibescript-sprint-deltas.md`](https://github.com/mediaprophet/qualiaDB/blob/0.0.38/docs/manuals/standards/vibescript-sprint-deltas.md) · [`vibe-catalog-honesty.md`](https://github.com/mediaprophet/qualiaDB/blob/0.0.38/docs/manuals/standards/vibe-catalog-honesty.md) (historical 2026-09-05 remap; its freeze line is superseded).

Customer voice: **live** / **planned**. Pages chips: **Works here** · **Works in Poet / Desktop** · **Planned**.

## The catalog grows

| Snapshot | What it is |
|---|---|
| ~885 (2026-09-05 honesty note) | Historical paired-catalog count. **Not** a freeze. |
| ~905 | Earlier 0.0.38-era cite before Wave 40. |
| **1121** at Wave 40 / `7928e3d` | Counted on QualiaDB tree `0.0.38` for this alignment. |
| Tip | **Count the tip. Do not hardcode a forever number in tools or pages.** |

`vibe-host-0.1` is an **outcome**, not a lock. New named work lands via catalog honesty. Grammar **may grow** when humans need a better form (versioned, documented). Do not invent families.

This repo MUST NOT paste APIs that are not live. When in doubt, open `ids.rs` on `0.0.38` (or the current tip).

## Do not invent

Never add the left column to samples, skills, or the live catalog:

| Do not write | Live bind |
|---|---|
| `qualia.graph.query` / `qualia.graph.commit` | `GraphDatabase.sparql` · `GraphDatabase.volume_commit` |
| `qualia.volume.open` | `GraphDatabase.volume_open` |
| `qualia.infer.complete` | `Inference.grounding` / `Inference.verify_turn` / `Inference.run_transformer` (pick the actual method) |
| `qualia.render.preview` | live `Render.*` (still / clip / scene below) |
| `qualia.qisp.*` | `GraphDatabase.sparql` results + `Manifold.*` / `Render.gpu_upload_tensor` |
| invented public relay | **planned** — not built |

## Preview handles (still / clip / scene)

One `Render.*` family. No sibling invent.

| Handle kind | Live methods (cite catalog) |
|---|---|
| still | `Render.gpu_render_frame`, `Render.gpu_read_pixels`, `Render.scene` |
| clip | `Render.animation_eval_curve`, `Render.animation_eval_preset`, `Render.css_animation` |
| scene | `Render.gpu_init_surface`, `Render.gpu_upload_mesh`, `Render.gpu_set_camera`, `Render.scene` |

## Math (live, §11.5)

`LinearAlgebra` — human word **Math**. Matrices are row-major `{ rows, cols, data }`.

Human sayables on the live catalog include:

`LinearAlgebra.gemm` · `matmul` · `dot` · `norm` · `trace` · `identity` · `inverse` · `transpose` · `determinant` · `solve` · `lu_*` · `qr_*` · `cholesky_*` · `svd` · `eigen_*`

`LinearAlgebra.gemm` MUST call the engine solver (CPU floor; GPU only when the machine actually has an accelerator). WASM-ontology without `wasm-scientific` returns E300 — not a fake product.

Do not invent extra LinearAlgebra methods. If a name is not on the live catalog, it is **planned**.

## Ledger vs showcase

| Surface | Honest label |
|---|---|
| Native `GraphDatabase.volume_commit` | durable when sanctuary permits — **Works in Poet / Desktop** |
| wasm volume invoke | local or E300 — **not** a saved `.q42` |
| This repo’s Pages WASM | `vibe-wasm 0.0.38` from QualiaDB tag `v0.0.38`. **Works here** for Math and Rules / Shapes (`honesty: "local"` residual). Not Poet furniture. |
| Inference chrome | **Works in Poet / Desktop** when a model is attached; otherwise planned on this page |

A demo that cannot open a volume says **planned** or **Works in Poet / Desktop**, not “saved”.

## Qualia 0.0.38 pages (do not fake completeness)

Operator record: [`progress-0.0.38.html`](https://github.com/mediaprophet/qualiaDB/blob/0.0.38/docs/progress-0.0.38.html).  
Fabric map: [`qdnf.html`](https://github.com/mediaprophet/qualiaDB/blob/0.0.38/docs/qdnf.html).

- Wait-honest chrome: **live** / **planned**.
- A public relay is **planned**, not built. Honesty flags stay false.
- Ask · Keep · Talk are teachable loops, not locked top nav.
- This ecosystem repo does not tick those flags.
- A name or login is not the person.
