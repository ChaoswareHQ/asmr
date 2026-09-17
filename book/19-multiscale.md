# Chapter 19 — Multi-Scale Hierarchy (Axiom A18)

> *Part II — Extended Axioms*

## 19.1 Motivation

Attacks span scales. A single-packet detector sees the exploit but not the
campaign; an enterprise-level detector sees the campaign but not the exploit.
Defense in depth is not a slogan but a structural fact: the state space is
organized into a hierarchy of scales, and detection must occur at every level.

## 19.2 Axiom

**Axiom A18 (Multi-scale hierarchy).** The state spaces at successive scales
form an increasing chain,

$$
S^{(1)} \subset S^{(2)} \subset \cdots \subset S^{(L)}.
$$

## 19.3 Operations

**Abstraction.** Each scale maps upward by an abstraction map

$$
\alpha_l : S^{(l)} \to S^{(l+1)}.
$$

**Refinement.** Each scale maps downward by a refinement relation

$$
\rho_l : S^{(l+1)} \to \mathcal{P}(S^{(l)}),
$$

which returns the set of finer states consistent with a coarse state.

**Consistency.** Abstraction and refinement are mutually consistent:

$$
\alpha_l\bigl(\rho_l(s^{(l+1)})\bigr) = s^{(l+1)}.
$$

**Information loss.** Each abstraction step discards information; the loss is

$$
\Delta_l  =  H(S^{(l)}) - H(S^{(l+1)}).
$$

**Multi-scale detection.** The posterior over the attack hypothesis combines
evidence across all scales:

$$
P(M \mid o^{(1)}, \ldots, o^{(L)})  \propto  P(M) \prod_{l=1}^{L} P(o^{(l)} \mid M).
$$

## 19.4 Worked example

A representative hierarchy is:

| Level | Scale | Example |
|---|---|---|
| 1 | Packet | TCP flags |
| 2 | Flow | Source/destination, bytes |
| 3 | Session | Login sequence |
| 4 | Host | Process tree |
| 5 | Subnet | Lateral movement |
| 6 | Enterprise | Campaign |

Abstraction maps concrete events to their coarse summaries:

$$
\alpha_1(\texttt{TCP SYN}) = \texttt{flow start},\qquad
\alpha_2(\texttt{flow start}) = \texttt{session start}.
$$

The information-loss quantity measures what is discarded at each
step: a flow summary discards the individual packets, a session summary
discards the flow, and so on. Each level is blind to the detail below it and
the context above it. A packet-level detector cannot see that a session is part
of a campaign; an enterprise-level detector cannot see the malformed packet
that began the session. Only the *product* of likelihoods across all levels,
in the multi-scale detection formula above, uses all of the evidence.

## 19.5 Consequences

Multi-scale structure is the formal content of defense in depth. It implies
that no single level is sufficient, because each level discards information the
others retain, and that the correct detector fuses evidence across levels
rather than choosing one. It also disciplines cross-layer correlation: the
consistency condition — the abstraction map composed with the refinement map equals the identity — is the requirement
that a refinement of a coarse state, re-abstracted, returns the same coarse
state — the bookkeeping that keeps the levels from diverging.

## 19.6 What fails without A18

Without the hierarchy there is no defense in depth and no cross-layer
correlation. The framework would be forced to choose a single scale and accept
blindness at every other scale, or to treat all scales as independent, missing
the correlation between a packet-level exploit and an enterprise-level
campaign. The case studies of Part VI — in particular the cryptomining case of
Chapter 39, where packet-level retransmission and cluster-level CPU anomalies
must be fused — presuppose A18.
