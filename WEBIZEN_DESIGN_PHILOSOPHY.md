# Webizen — Design Philosophy for a 3D Human-Centric Internet

**Author:** Claude (Opus 4.8) · **Date:** 2026-06-15 · **Status:** Philosophy → principles → architecture mapping. Informs `WEBIZEN_3D_NATIVE_VISION.md`, `WEBIZEN_REALMS.md`.

> **Method note:** these five videos were supplied as design references. YouTube transcripts are JS-rendered and weren't retrievable, so this reviews each against its **established thesis** (identified via oEmbed title/author). Where a specific video diverges from the reading below, correct it and I'll revise. The point is not to summarise the videos but to distil **design principles** for the human-centric internet and tie them to concrete architecture already in the plan.

---

## 1. The five sources and their core idea

| # | Source | Core thesis (as established) |
|---|---|---|
| 1 | **Henry Stapp — Quantum Mechanics & Human Consciousness** (NourFoundation) | The von Neumann–Stapp reading: consciousness is **causally participatory** in reality. The observer's *choice of question* ("Process 1") is not fixed by physical law — **attention and intention are efficacious**. Mind is a participant, not a spectator. |
| 2 | **Captains of Spaceship Earth** (Jason Silva) | Buckminster Fuller: we are **crew, not passengers**, on a shared planet — responsible co-stewards. A "design-science revolution"; whole-systems responsibility for a commons. |
| 3 | **Architecting the Mind** (Jason Silva) | We **build cognitive architecture**; tools extend and externalise mind (McLuhan: media as extensions of man). Cognition can be augmented and designed. |
| 4 | **What is Ontological Design?** (Jason Silva) | Anne-Marie Willis: **"we design our world, and our world designs us back."** Designed things recursively shape the designers. Design is world-making, and therefore an ethical act. |
| 5 | **Language Rewires Our Brain** (Jason Silva) | Whorf/neuroplasticity: **language and symbolic systems shape what is thinkable.** New vocabularies create new thoughts; naming is generative. |

Together they describe the human-centric internet as a **participatory, ontological, cognitive, semantic medium** — the opposite of the extractive attention economy.

---

## 2. Seven principles (and what each forbids)

### P1 — Attention is sacred (Stapp)
If the observer's attention is causally meaningful, then **capturing it is a violation, not a business model.** Motion, 3D, and notifications must serve *comprehension and intention*, never capture.
- **Forbids:** engagement-bait, infinite feeds, dark patterns, anxiety-inducing motion.
- **Already in plan:** the 3D vision's "motion encodes meaning, not engagement"; calm-by-default; `prefers-reduced-motion`.

### P2 — The human is a participant-author, not a user (Stapp + Architecting the Mind)
The fundamental act is **asking a question and making something** — not consuming. The omnibox/query is "Process 1": the user's choice shapes what the graph reveals.
- **Forbids:** read-only, take-it-or-leave-it apps; locked content.
- **Already in plan:** every QApp **editable/forkable** (`WEBIZEN_APP_LIFECYCLE.md`); QApp Studio; the omnibox as the primary act; AS-OF queries where *observing* is a first-class, provenance-stamped act.

### P3 — Truth has provenance (Stapp's observer + epistemics)
Every claim shows **where it came from**. In a participatory medium, the integrity of observation is everything; the antidote to misinformation is visible lineage.
- **Forbids:** unsourced assertions, opaque "magic," AI output without citation.
- **Already in plan:** NQuin provenance everywhere; `super_quin_provenance_chip`/citation chips; `qapp_engine` returns a provenance hash; the Mind realm's epistemic/possible-worlds modality.

### P4 — Externalise and spatialise mind (Architecting the Mind)
Use **space as a thinking medium** — the method of loci / memory palace, made literal. Knowledge graphs, the chat-graph DAG, provenance chains become navigable structures you *think in*.
- **Already in plan:** the 3D engine's spatial graph views (Layer C); the **Mind realm**; the chat-graph DAG rendered as a tree you fly through.

### P5 — Stewardship and commons (Spaceship Earth)
The net is a **shared craft we crew together** — cooperative work, civic structures, real-world stewardship; design for the whole system, not the individual silo.
- **Already in plan:** the **Social realm**; cooperative projects + obligation/attribution; **swarm** collaborative compute; `web_civics`; "Bilateral Micro-Commons."

### P6 — Ontological responsibility (Ontological Design)
Because **the tool remakes its user**, every default, realm, animation, and word is an ethical choice with lasting consequence. Design for who people *become* — dignity, agency, truth — not for metrics.
- **Forbids:** "we just build the platform, usage is neutral." It isn't.
- **Already in plan:** the human-centric constraints (consent-first, dignity language, accessibility, the WellFare threat model); realms making **privacy spatial and legible**; HCAI "signature is not authorization."

### P7 — Language/semantics as expansion (Language Rewires)
The semantic substrate (RDF / RDF-star / q42 / ontologies) is **a new language that enlarges what people can think and do** — and letting people **author their own vocabularies** is letting them author their own cognition. Naming is design.
- **Already in plan:** the Ontology Workbench (author your own ontologies); q42/SPARQL-star as a thinking medium; the deliberate **dignity vocabulary** ("care credits" not "crypto"; "inalienable/self-determined"; "Front Door DID"; "realms").

---

## 3. What this changes / sharpens in the architecture

Mostly it **confirms and gives a *why*** to choices already made — which is the right outcome (the architecture was already reaching for this). It sharpens a few things:

1. **The omnibox is the soul, not a search bar.** As "Process 1" (the chosen question), it deserves to be the most considered surface — protocol router (`qdp://`, `did:q42:`), query, intention. Elevate it.
2. **Make provenance *felt*, not just available.** P3+P1: provenance should be a calm, ever-present ambient layer (a depth, a glow on hover), so truth-of-origin is the resting state of the UI — the antithesis of the feed.
3. **The Mind realm is the flagship 3D experience**, not anatomy. P4 says the highest expression is *thinking in space* — the knowledge/epistemic graph you navigate. Prioritise it alongside anatomy.
4. **Author-your-own-language is a headline feature, not a dev tool.** P7: surface the Ontology Workbench prominently; "name your world" is a first-class act, framed for ordinary people, not just engineers.
5. **A stewardship/commons surface.** P5: the cooperative + swarm + civic capabilities deserve a visible "we" — a place that shows shared work, not just personal apps.
6. **Onboarding should teach participation, not features.** P2/P6: the setup wizard (a known gap from the Flutter review) should induct people into *authoring, questioning, stewarding* — the posture, not the buttons.

---

## 4. A one-line design north star

> **A web that moves to show you where things come from, that you think and build inside of, and that you and your people steward together — designed in the knowledge that it is, in turn, designing you.**

This is the felt difference from the attention economy: **participatory** (Stapp), **cognitive/spatial** (Architecting the Mind), **provenant** (epistemics), **stewarded** (Spaceship Earth), **self-authored in language** (Language Rewires), and **ethically designed because design is recursive** (Ontological Design).

---

## 5. Caveats & next step

- This rests on the *established theses* of these pieces, not their transcripts. If you can paste the transcripts (or correct any reading), I'll tighten the mapping — especially the Stapp talk, where the specific argument (Process 1 / Process 2, the Quantum Zeno effect and attention) could add precise language for P1/P2.
- These are **philosophy → principles**, not feature specs. They belong as the preamble/why for `WEBIZEN_3D_NATIVE_VISION.md` and `WEBIZEN_REALMS.md`, and as the rubric every design decision is checked against ("does this treat the person as participant-steward, or as a resource to extract?").
