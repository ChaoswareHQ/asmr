# Chapter 1 — Introduction

## 1.1 The subject

A defended system changes over time. At any moment it is in some *state*: a
user runs a process, a process opens a connection, a file is written, a
privilege is elevated. Most of these changes are benign. A small fraction are
not.

The organization charged with defending the system does not see the state
directly. It sees *observations*: logs, alerts, network flows, endpoint
telemetry. From these it must infer whether an attack is occurring, decide what
to do, and act under constraints of cost, capacity, and authority.

This is the subject of the book: the structure of that inference–decision–action
cycle, made precise.

## 1.2 Why the subject is difficult

Security monitoring is difficult for reasons that are structural, not merely
incidental. They can be listed.

1. **Partial observability.** The defender never sees the full state. At best it
   sees a projection of the state, and often a projection that omits exactly the
   dimensions that would settle the question.

2. **Noise.** Sensors drop, delay, and corrupt observations. A sensor that never
   errs does not exist in practice, and a framework that assumes one will draw
   conclusions stronger than the evidence supports.

3. **Adversarial adaptation.** Unlike a physical process, the thing being
   measured is an intelligent adversary who learns the defender's rules and
   changes behavior in response. A rule that works today is a signal to the
   adversary tomorrow.

4. **Resource constraints.** Analysts, compute, memory, and storage are finite.
   Not every alert can be reviewed, and a framework that pretends otherwise
   will silently assume away the bottleneck that dominates practice.

5. **Governance.** Actions have legal, contractual, and operational
   consequences. A framework that does not distinguish "the system may act
   autonomously" from "the system must obtain human approval" cannot be
   deployed.

6. **Temporal decay.** Evidence ages. A connection observed five minutes ago
   bears on the present state in a way that a connection observed five days ago
   does not.

7. **Composition.** Systems compose. A compromise in one component can spread
   through a boundary to another, and the reachability of that spread — the
   *blast radius* — is a quantity that must be computed, not assumed.

8. **Multi-scale structure.** Attacks span scales, from a single packet to an
   enterprise campaign. A detector that lives at one scale is blind to the
   others.

9. **Unknown threats.** A zero-day vulnerability, by definition, lies outside
   the set of states the defender has previously modeled. The framework must
   handle the case in which the state space itself expands.

A useful framework must address these difficulties together, because a real
deployment faces them together. This is the gap the present book sets out to
fill.

## 1.3 Existing approaches

The field has produced a number of frameworks, each of which addresses part of
the problem and none of which addresses the whole.

| Framework | What it provides | What it omits |
|---|---|---|
| MITRE ATT&CK | A taxonomy of adversary behavior | No mathematics; no model of response |
| NIST CSF | A management framework for organizational risk | Deliberately non-technical; no decision procedure |
| Zero Trust | A network architecture assuming compromise | Narrow scope; no detection theory |
| Game theory | A model of adversarial interaction | Narrow scope; no operational pipeline |
| Formal methods | Rigorous verification of protocols | Narrow scope; no monitoring or response |

Each of these is valuable on its own terms. MITRE ATT&CK provides a shared
vocabulary for describing what an adversary does; NIST CSF provides an
organizational language for risk; Zero Trust provides a design principle;
game theory provides a decision-theoretic lens on two interacting rational
agents; formal methods provide rigorous proofs for bounded systems.

What is missing is a structure in which these are *instances* of a single
system, so that results can be compared, composed, and, where possible,
proven. ASMR is an attempt at that structure.

## 1.4 The core insight

The central observation of this book is that security monitoring is a pipeline
with a fixed shape:

$$
\text{state}\ \xrightarrow{\ \pi\ }\ \text{observation}\ \xrightarrow{\ \text{update}\ }\ \text{history}\ \xrightarrow{\ \text{inference}\ }\ \text{posterior}\ \xrightarrow{\ \text{decision}\ }\ \text{governed action}\ \xrightarrow{\ \text{effect}\ }\ \text{state}.
$$

Each arrow has a mathematical structure. The first is a *projection* of the
state onto what is observable, possibly composed with noise. The second is an
*update* that extends the defender's history. The third is a *Bayesian* (or
robust) inference. The fourth is a *cost-minimizing* decision. The fifth is a
*governance* filter. The sixth is an *effect* that returns the system to a new
state, from which the cycle repeats.

ASMR makes each of these structures explicit as an axiom, and then studies the
consequences. The result is a body of definitions (Part I), extensions
(Part II and Part III), theorems (Part IV), a pipeline and its computation
(Part V), and applications (Part VI).

## 1.5 Outline of the book

The book is organized as follows.

**Part I** (Chapters 2–13) develops the twelve core axioms, which define the
objects and relations of the framework: state, event, observation, time,
uncertainty, action, agent, cost, information, trust, resource, and governance.

**Part II** (Chapters 14–21) develops eight extended axioms that enrich the core:
composition, adversarial adaptation, channel capacity, temporal decay,
deception, multi-scale structure, privacy, and recovery.

**Part III** (Chapters 22–31) develops the ten zero-day axioms, which address the
case in which the state space itself is open and the defender must act under
epistemic uncertainty.

**Part IV** (Chapter 32) collects the derived theorems and their proofs.

**Part V** (Chapters 33–34) assembles the axioms into the monitoring–response
pipeline and describes the computational realization.

**Part VI** (Chapters 35–39) applies the framework to five documented intrusions.

**Part VII** (Chapter 40) states the validation program by which the framework's
predictions are to be tested.

**Part VIII** (Chapters 41–43) compares ASMR to existing work, states its benefits
and limitations honestly, and gives a roadmap.

The appendices collect the axioms, the notation, and the references.
