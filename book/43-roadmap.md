# Chapter 43 — Roadmap

> *Part VIII — Assessment*

## 43.1 Versioning

The framework is versioned, and each version is defined by what it *proves* and
what it *delivers*, not merely by what it adds.

## 43.2 Version 1.0 — the present edition

Version 1.0 is what this book describes.

- The twelve core axioms (A1–A12).
- The eight extended axioms (A13–A20).
- The ten zero-day axioms (A21–A30).
- The derived theorems (Chapter 32).
- The pipeline and its computational specification (Chapters 33–34).
- Five case studies with exact computations (Chapters 35–39).
- A validation program with stated hypotheses (Chapter 40).

The computation source for every worked example is committed and reproducible.
What version 1.0 does *not* contain is equally explicit: a proof of the
consistency and independence of the axioms, completed empirical validation, and
a shipped reference implementation of the API of Chapter 34.

## 43.3 Version 2.0 — proof and evidence

Version 2.0 has two objectives, one formal and one empirical.

**Formal.** Prove the consistency of the axioms, their independence, and their
minimality — or, if any axiom is found redundant or dependent, revise the set
and record the revision. This is the counterpart of the proof-theoretic work
that separates a collection of definitions from a *system*.

**Empirical.** Execute the validation program of Chapter 40:

- Reproduce the gap-versus-false-positive hypothesis on CICIDS2017.
- Reproduce the threshold-versus-decision hypothesis against SOC records.
- Reproduce the master-equation-versus-response hypothesis against
  MITRE ATT&CK and public reporting.
- Reproduce the resource-versus-fatigue hypothesis against SOC telemetry.

**Integration.** Provide the reference implementation of the API of Chapter 34,
and connectors to Wazuh, Splunk, and Elastic, so that the pipeline of
Chapter 33 can be run against live telemetry.

## 43.4 Version 3.0 — application and community

Version 3.0 turns a validated framework into an adopted one.

- A web interface for the pipeline and the case studies.
- Cloud deployment of the reference implementation.
- A community of plugins for new sensors, actions, and policies.
- Publication of this manuscript as a printed textbook.
- Engagement with standards bodies toward a common notation for the
  quantities the framework defines (gap, capacity, blast radius, autonomy).

## 43.5 A note on ordering

The order is deliberate. Proof precedes evidence, and evidence precedes
adoption. A framework that claims adoption before validation is a product; a
framework that claims validation before proof is a hypothesis. This book
positions ASMR as a hypothesis with reproducible calculations and a testable
program, and the roadmap is the schedule for turning that hypothesis into a
demonstrated system.
