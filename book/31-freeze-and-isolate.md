# Chapter 31 — Freeze and Isolate (Axiom A30)

> *Part III — Zero-Day Axioms*

## 31.1 Motivation

When the defender does not know what is happening — high novelty, no patch,
epistemic uncertainty — the safest action is often to stop changing things and
cut off the affected region. This chapter formalizes freeze and isolation, the
extreme-case response of the zero-day regime.

## 31.2 Axiom

**Axiom A30 (Freeze and isolate).** There exists a freeze operator and an
isolation operator:

$$
\mathrm{Freeze}(s) = s_{\mathrm{frozen}},\qquad
\mathrm{Isolate}(s) = s_{\mathrm{isolated}}.
$$

## 31.3 Operations

**Freeze.** A frozen state is one in which no effect is permitted:

$$
s_{\mathrm{frozen}} = s \quad\text{with}\quad \mathrm{eff}_a = \mathrm{id}\ \ \forall a.
$$

Freezing halts change: deploys stop, processing pauses, the system is held in
place while the defender determines what is happening.

**Isolate.** Isolation removes the affected component from interaction with the
rest:

$$
s_{\mathrm{isolated}} : \mathrm{Reach}(s) \cap \text{(rest)} = \varnothing.
$$

**Contain.** Containment is the requirement that the compromised region have no
reachable path to the rest:

$$
\mathrm{Contain}(c) \;=\; \{\, s \mid \mathrm{Reach}(c) \cap s = \varnothing \,\}.
$$

## 31.4 Worked example

The Log4Shell response followed exactly this sequence:

1. **Freeze.** Stop deploys, so that no further vulnerable artifact is
   introduced while the scope is assessed.
2. **Isolate.** Isolate the affected servers from the rest of the network.
3. **Contain.** Block outbound LDAP, cutting the path the exploit uses.
4. **Patch.** Update Log4j (Chapter 27's patch, once available).
5. **Unfreeze.** Resume normal operation.

Each step is an instance of an axiom developed earlier. Freeze is the minimax
response of Chapter 26 in its extreme form — under maximal epistemic
uncertainty, the action with minimal worst-case regret is to stop. Isolate and
contain are the composition algebra of Chapter 14 used defensively: cutting
the reachable region is precisely the operation that shrinks the blast radius.
Patch is the endpoint of the latency of Chapter 27, and the compensating
controls of Chapter 28 bridge the interval until it arrives. Unfreeze is the
recovery of Chapter 21.

## 31.5 Consequences

Freeze-and-isolate is the terminal response of the zero-day regime, and it is
*derived*, not merely recommended: the theorem of Chapter 32 states that when
novelty exceeds threshold and patch latency is effectively infinite, freeze is
minimax-optimal. The response also has a governance dimension (Chapter 13):
freeze and isolate are often human-approved actions, not autonomous ones,
because their cost when wrong — taking production offline — is high. The
framework's governance filter ensures the derived action is taken only with the
required approval.

## 31.6 What fails without A30

Without freeze and isolate, the framework has no response for the worst case —
the unknown attack with no patch — and would continue to act (change state,
allow interaction) exactly when acting is most dangerous. The case studies of
Part VI, especially Log4Shell (Chapter 38), presuppose A30.
