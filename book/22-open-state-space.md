# Chapter 22 — Open State Space (Axiom A21)

> *Part III — Zero-Day Axioms*

## 22.1 Motivation

The state space of Chapter 2 was fixed. But a zero-day vulnerability is, by
definition, a state the defender has not modeled before: a new vulnerability, a
new tool, a new attack path. When Log4Shell was disclosed, the state space
grew. A framework with a closed state space is blind to exactly the events that
matter most. This chapter opens it.

## 22.2 Axiom

**Axiom A21 (Open state space).** The state space is the union of what is
known and what is not yet known:

$$
S(t) \;=\; S_{\mathrm{known}}(t) \;\cup\; S_{\mathrm{unknown}}(t).
$$

## 22.3 Operations

**Rate of expansion.** The size of the state space changes with the rate at
which new states appear and old states are removed:

$$
\frac{d\,|S|}{dt} \;=\; \lambda_{\mathrm{new}} - \lambda_{\mathrm{removed}}.
$$

**Good–Turing estimate.** The probability that the next state is one never seen
before is estimated by the fraction of states seen exactly once:

$$
P(s_{\mathrm{new}}) \;=\; \frac{N_1}{N},
$$

where the number seen once is the number of distinct states observed exactly once and the total observations is the overall number of observations. This is the classical estimator for the mass of unseen
events, and it is the quantitative answer to "how likely is a novel state
next?"

## 22.4 Worked example

A state space grows as new vulnerabilities are disclosed:

| Date | Event | $|S|$ |
|---|---|---|
| Jan 1 | Baseline | 256 |
| Jan 15 | Log4Shell disclosed | 512 |
| Feb 1 | Spring4Shell disclosed | 768 |
| Feb 15 | New zero-day | 1024 |

Each disclosure doubles the relevant state space, because each new
vulnerability introduces a new class of states (new processes, new network
paths, new file artifacts) that the defender must now distinguish. A detector
trained on the baseline of 256 states is, after three disclosures, facing
1024 states — three-quarters of which it has never seen.

The Good–Turing estimate makes the unseen mass computable. If, among the states
observed so far, a fraction equal to the number seen once divided by the total observations appeared exactly once, that fraction is
the best estimate of the probability that the next observation is novel. The
estimate is not a guess; it is a standard, asymptotically justified estimator
for the probability of the unseen.

## 22.5 Consequences

An open state space changes the inferential problem fundamentally. In the
closed setting of Part I, the defender's uncertainty is *within* a known set;
in the open setting, it is *about* the set itself. The distinction is the
subject of Chapter 25 (epistemic versus aleatoric uncertainty). The open state
space is also why novelty (Chapter 23) and structural anomaly (Chapter 24)
exist as separate notions: they are the detectors for the *appearance* of the
unseen, rather than for the misclassification of the seen.

## 22.6 What fails without A21

Without an open state space the framework cannot represent a zero-day. It would
treat every state as already known, and a novel attack as merely an unusual
instance of a familiar one — which is precisely the error a zero-day exploits.
The remaining chapters of Part III all presuppose A21.
