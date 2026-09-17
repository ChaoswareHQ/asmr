# Chapter 14 — Composition Algebra (Axiom A13)

> *Part II — Extended Axioms*

## 14.1 Motivation

Systems compose, and so do compromises. A foothold on a workstation becomes a
compromise of a file server, then of a domain controller, then of a database.
This chapter formalizes composition and the quantity that governs how far a
compromise spreads: the blast radius.

## 14.2 Axiom

**Axiom A13 (Composition algebra).** There exists a composition operator on
states,

$$
\circ : S_i \times S_j \to S_{i \cup j},
$$

which is associative:

$$
(s_1 \circ s_2) \circ s_3  =  s_1 \circ (s_2 \circ s_3).
$$

## 14.3 Operations

**Trust propagation.** Trust composes through a boundary, weighted by the
boundary's integrity:

$$
\tau(s_1 \circ s_2)  =  \tau(s_1) \odot \tau(s_2) \odot \kappa(b_{12}),
$$

where the multiplication operator is multiplication in the numerical case and
the boundary integrity is the integrity of the boundary between the two components.

**Reachability.** The set of states reachable from a compromised component
is

$$
\mathrm{Reach}(c)  =  \lbrace  s_j \mid \exists\ \text{a path from } c \text{ to } s_j  \rbrace.
$$

**Blast radius.** The expected impact of a compromise at that component is the
impact-weighted, trust-weighted sum over the reachable set:

$$
\mathrm{BR}(c)  =  \sum_{s_j \in \mathrm{Reach}(c)} \mathrm{Impact}(s_j) \cdot \tau(s_j).
$$

## 14.4 Worked example

Consider a chain of four components,

$$
\texttt{WS} \to \texttt{FS} \to \texttt{DC} \to \texttt{DB},
$$

with the following trust and boundary-integrity values:

$$
\tau(\texttt{WS}) = 0.8,\quad \tau(\texttt{FS}) = 0.9,\quad \tau(\texttt{DC}) = 0.95,\quad \tau(\texttt{DB}) = 0.99,
$$

$$
\kappa(\texttt{WS--FS}) = 0.9,\quad \kappa(\texttt{FS--DC}) = 0.5,\quad \kappa(\texttt{DC--DB}) = 0.7.
$$

Trust propagates multiplicatively through each boundary. Starting from the
workstation:

$$
\tau(\texttt{WS} \circ \texttt{FS}) = 0.8 \cdot 0.9 \cdot 0.9 = 0.648000,
$$

$$
\tau(\texttt{FS} \circ \texttt{DC}) = 0.648000 \cdot 0.95 \cdot 0.5 = 0.307800,
$$

$$
\tau(\texttt{DC} \circ \texttt{DB}) = 0.307800 \cdot 0.99 \cdot 0.7 = 0.213305.
$$

The trust in the far end of the chain — the database — is 0.213305, roughly a
quarter of the workstation's own trust, attenuated by the two weak boundaries in
between.

Assigning impacts to the reachable components,

$$
\mathrm{Impact}(\texttt{FS}) = 100,\quad \mathrm{Impact}(\texttt{DC}) = 1000,\quad \mathrm{Impact}(\texttt{DB}) = 5000,
$$

the blast radius is

$$
\mathrm{BR} = 100 \cdot 0.648000 + 1000 \cdot 0.307800 + 5000 \cdot 0.213305 = 1439.127.
$$

The blast radius is the single number that summarizes "how bad is a compromise
at the workstation, on expectation." It is a product of two facts: what is
reachable (the path from WS to FS to DC to DB)
and how trustworthy each step of that path is (the trust and boundary integrity values).
Weakening the boundary integrity of the FS--DC boundary from 0.5 to 0.1 would cut
the database's trust to 0.0427 and the blast radius accordingly — which is
the quantitative justification for segmentation.

## 14.5 Consequences

Composition is how lateral movement, blast radius, and supply-chain risk are
expressed. The same operator that composes states composes trust (Chapter 11)
and propagates compromise, so the framework need not introduce a separate
mechanism for each: one algebra covers them all. The blast radius also provides
the objective for the containment response of Chapter 31 and the recovery
response of Chapter 21: containment is the act of cutting the reachable set,
and recovery is the act of restoring trust along the chain.

## 14.6 What fails without A13

Without composition there is no lateral movement, no blast radius, and no
supply-chain analysis. The framework would treat each component in isolation
and miss exactly the cross-component spread that turns a single foothold into an
enterprise compromise. Chapter 37 (SolarWinds) is the case study of this axiom.
