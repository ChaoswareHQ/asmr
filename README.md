# Algebraic Security for Monitoring and Response

### A Formal Framework for Security Operations (ASMR)

*Serving as the manuscript of a monograph on the axiomatics, theorems, and
computations of security monitoring and response.*

---

**Status.** Draft edition, version 0.1. The text is complete in structure and
self-contained; several chapters remain open to further expansion, and the
empirical validation results are presented as a program to be reproduced
rather than as a claim of finished measurement. See the Preface for a full
statement of scope.

---

## Abstract

Security monitoring and response is customarily treated as a collection of
engineering practices: sensor placement, alert tuning, playbook authoring, and
analyst workflow. This book takes a different position. It argues that the
entire pipeline — from the hidden state of a system, through observation,
inference, decision, and governed action, and back to the state — admits a
single mathematical structure, and that this structure can be made explicit
as a small set of axioms.

The framework developed here is called *Algebraic Security for Monitoring and Response*
(ASMR). It defines what exists (state, event, observation, time, probability,
action, agent, cost, information, trust, resource, and governance), how these
entities relate (projection, transition, composition, decay, adaptation), and
what can be proved about them (gap theorems, threshold theorems, channel-capacity
bounds, and a master equation for the interaction between defender and
attacker). Every example in the book is computed exactly; the computation source
is committed alongside the text so that no figure is asserted without being
reproducible.

## How to read this book

The book is a sequence of numbered chapters collected into eight parts and
three appendices. Chapters are intended to be read in order, but the
*Reference of Axioms* (Appendix A) and the *Notation and Symbols* (Appendix B)
are designed to be consulted independently.

The manuscript is written in Markdown with embedded mathematical notation in
LaTeX delimited by `$ ... $` (inline) and `$$ ... $$` (display). It renders
correctly on GitHub, in any Markdown viewer with MathJax or KaTeX support, or
after conversion to LaTeX or PDF.

## The linear build order

The thirty axioms are numbered in the order they are constructed, so the
numbering itself is the reading order. Reading front to back follows the
build-up from a single definition to a complete system.

**Foundations — the core axioms (A1–A12).**

A1 State → A2 Events → A3 Observation → A4 Time and Causality → A5 Uncertainty
→ A6 Actions → A7 Agents and Games → A8 Cost and Utility → A9 Information Sets
→ A10 Trust and Integrity → A11 Resource Constraints → A12 Governance and Policy.

**Extensions (A13–A20).**

A13 Composition → A14 Adversarial Adaptation → A15 Channel Capacity → A16
Temporal Decay → A17 Deception → A18 Multi-Scale Hierarchy → A19 Privacy →
A20 Recovery and Resilience.

**Zero-day (A21–A30).**

A21 Open State Space → A22 Novelty → A23 Structural Anomaly → A24 Epistemic
Uncertainty → A25 Minimax Response → A26 Patch Latency → A27 Compensating
Controls → A28 Zero-Day Hunting → A29 Transfer Learning → A30 Freeze and
Isolate.

Each axiom presupposes the ones above it and extends them. A later chapter may
refer to an earlier one, never the reverse. The Preface (Conventions) states
this rule formally; Appendix A lists the axioms in the same order.

## Table of contents

**Front matter**

- [Preface](book/00-preface.md)
- [Chapter 1 — Introduction](book/01-introduction.md)

**Part I — Foundations (Axioms A1–A12)**

- [Chapter 2 — The State Space (A1)](book/02-state-space.md)
- [Chapter 3 — Events and Traces (A2)](book/03-events-and-traces.md)
- [Chapter 4 — Observation and Partial Observability (A3)](book/04-observation.md)
- [Chapter 5 — Time and Causality (A4)](book/05-time-and-causality.md)
- [Chapter 6 — Uncertainty (A5)](book/06-uncertainty.md)
- [Chapter 7 — Actions (A6)](book/07-actions.md)
- [Chapter 8 — Agents and Games (A7)](book/08-agents-and-games.md)
- [Chapter 9 — Cost and Utility (A8)](book/09-cost-and-utility.md)
- [Chapter 10 — Information Sets (A9)](book/10-information-sets.md)
- [Chapter 11 — Trust and Integrity (A10)](book/11-trust-and-integrity.md)
- [Chapter 12 — Resource Constraints (A11)](book/12-resource-constraints.md)
- [Chapter 13 — Governance and Policy (A12)](book/13-governance.md)

**Part II — Extended Axioms (A13–A20)**

- [Chapter 14 — Composition Algebra (A13)](book/14-composition-algebra.md)
- [Chapter 15 — Adversarial Adaptation (A14)](book/15-adversarial-adaptation.md)
- [Chapter 16 — Channel Capacity (A15)](book/16-channel-capacity.md)
- [Chapter 17 — Temporal Decay (A16)](book/17-temporal-decay.md)
- [Chapter 18 — Deception and Active Defense (A17)](book/18-deception.md)
- [Chapter 19 — Multi-Scale Hierarchy (A18)](book/19-multiscale.md)
- [Chapter 20 — Privacy and Data Minimization (A19)](book/20-privacy.md)
- [Chapter 21 — Recovery and Resilience (A20)](book/21-recovery-and-resilience.md)

**Part III — Zero-Day Axioms (A21–A30)**

- [Chapter 22 — Open State Space (A21)](book/22-open-state-space.md)
- [Chapter 23 — Novelty Measurement (A22)](book/23-novelty.md)
- [Chapter 24 — Structural Anomaly (A23)](book/24-structural-anomaly.md)
- [Chapter 25 — Epistemic Uncertainty (A24)](book/25-epistemic-uncertainty.md)
- [Chapter 26 — Minimax Response (A25)](book/26-minimax-response.md)
- [Chapter 27 — Patch Latency (A26)](book/27-patch-latency.md)
- [Chapter 28 — Compensating Controls (A27)](book/28-compensating-controls.md)
- [Chapter 29 — Zero-Day Hunting (A28)](book/29-zero-day-hunting.md)
- [Chapter 30 — Transfer Learning (A29)](book/30-transfer-learning.md)
- [Chapter 31 — Freeze and Isolate (A30)](book/31-freeze-and-isolate.md)

**Part IV — Theorems**

- [Chapter 32 — Derived Theorems](book/32-derived-theorems.md)

**Part V — The Pipeline and Computation**

- [Chapter 33 — The Monitoring–Response Pipeline](book/33-pipeline.md)
- [Chapter 34 — The Computational Framework](book/34-computational-framework.md)

**Part VI — Case Studies**

- [Chapter 35 — Case Study: FIN7](book/35-case-study-fin7.md)
- [Chapter 36 — Case Study: Conti Ransomware](book/36-case-study-conti.md)
- [Chapter 37 — Case Study: SolarWinds Supply Chain](book/37-case-study-solarwinds.md)
- [Chapter 38 — Case Study: Log4Shell](book/38-case-study-log4shell.md)
- [Chapter 39 — Case Study: Cryptomining Exfiltration](book/39-case-study-cryptomining.md)

**Part VII — Validation**

- [Chapter 40 — Validation](book/40-validation.md)

**Part VIII — Assessment**

- [Chapter 41 — Comparison with Other Frameworks](book/41-comparison.md)
- [Chapter 42 — Benefits and Limitations](book/42-benefits-and-limitations.md)
- [Chapter 43 — Roadmap](book/43-roadmap.md)

**Back matter**

- [Appendix A — Reference of Axioms](book/appendix-a-axioms-reference.md)
- [Appendix B — Notation and Symbols](book/appendix-b-notation-and-symbols.md)
- [References](book/references.md)

## Reproducing the computations

Every numeric example in this book is generated by a single, dependency-free
Rust program committed at [`tools/compute_examples.rs`](tools/compute_examples.rs).
To reproduce all figures:

```sh
rustc tools/compute_examples.rs -O -o compute_examples
./compute_examples
```

The program prints each quantity to full machine precision; the prose rounds
for display only. Where a value appears in the text, it can be traced to this
program.

## Citation

```bibtex
@book{asmr,
  author    = {The ASMR Project},
  title     = {Algebraic Security for Monitoring and Response: A Formal Framework for Security Operations},
  year      = {2026},
  note      = {Draft edition, version 0.1},
  url       = {https://github.com/...}
}
```

## License

The license for this manuscript is to be confirmed before publication. The
recommended license for the prose is Creative Commons Attribution 4.0
(CC BY 4.0); the computation source may be placed under a permissive software
license. No license is asserted here until the author records one.
