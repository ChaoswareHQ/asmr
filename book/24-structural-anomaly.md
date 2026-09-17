# Chapter 24 — Structural Anomaly (Axiom A23)

> *Part III — Zero-Day Axioms*

## 24.1 Motivation

A zero-day often differs from known attacks not in the *values* of its
observations but in their *structure* — the pattern of relationships among
processes, hosts, and connections. This chapter defines anomaly over structure
as well as value.

## 24.2 Axiom

**Axiom A23 (Structural anomaly).** Anomaly decomposes into a value component
and a structure component:

$$
\mathrm{Anomaly}(s)  =  \alpha \cdot \mathrm{ValueAnomaly}(s) + \beta \cdot \mathrm{StructureAnomaly}(s).
$$

## 24.3 Operations

**Value anomaly.** The negative log-probability of the state:

$$
\mathrm{ValueAnomaly}(s)  =  -\log P(s).
$$

**Structure anomaly.** The negative log-probability of the state's relationship
graph:

$$
\mathrm{StructureAnomaly}(s)  =  -\log P(G_s),
$$

where the relationship graph of the state is the graph induced by the state (processes as vertices, edges as
spawns, connections, and file writes).

**Graph edit distance.** The distance between two relationship graphs is the
minimum number of edits to transform one into the other:

$$
d_{\mathrm{graph}}(G_1, G_2)  =  \min_{\mathrm{edit}} |\mathrm{edit}|.
$$

## 24.4 Worked example

Consider Log4Shell. The normal request graph is

$$
\texttt{browser} \to \texttt{server} \to \texttt{database},
$$

while the exploited request graph is

$$
\texttt{browser} \to \texttt{server} \to \texttt{LDAP} \to \texttt{attacker.com} \to \texttt{server}.
$$

The *values* in the two traces may overlap heavily — both involve a browser, a
server, and network traffic — but the *structure* differs: the attack inserts a
new hop, an outbound LDAP connection to an untrusted host, that the normal
graph lacks. A detector that measures only value anomaly may see an ordinary
web request; a detector that measures structure anomaly sees a foreign edge in
the graph.

The magnitude of the structural anomaly is given by the negative log-probability of the relationship graph of the state: an edge
that is rare in the normal graph contributes a large negative log-probability.
The specific numeric value depends on the fitted graph distribution and is a
computed quantity, not a constant; what matters here is that the anomaly is
*located in the structure*, which is exactly where value-only detectors are
blind.

## 24.5 Consequences

Structural anomaly is the detector for zero-days that reuse known tools in
novel combinations — the dominant pattern of modern intrusions, which favor
"living off the land" over bespoke malware. It complements novelty
(Chapter 23) along a different axis: a state can be structurally novel while
value-familiar, or value-novel while structurally familiar, and only the joint
score catches both. The hypothesis-driven hunt of Chapter 29 searches over
structural anomalies.

## 24.6 What fails without A23

Without structural anomaly, the framework is blind to exactly the zero-days
that reuse known primitives in new patterns. It would see the Log4Shell request
as ordinary traffic and miss the one foreign edge that distinguishes it.
