# Chapter 17 — Temporal Decay (Axiom A16)

> *Part II — Extended Axioms*

## 17.1 Motivation

Evidence ages. A command-and-control connection observed five minutes ago bears
on the present state; the same connection observed five days ago does not. A
framework that weights all evidence equally will treat stale evidence as
current, and will be misled. This chapter formalizes decay.

## 17.2 Axiom

**Axiom A16 (Temporal decay).** Evidence decays exponentially in age:

$$
w(t, t')  =  e^{-\lambda (t - t')},
$$

where the decay rate is positive and the age of the evidence is time t minus time t'.

## 17.3 Operations

**Half-life.** The time at which evidence loses half its weight is

$$
t_{1/2}  =  \frac{\ln 2}{\lambda}.
$$

**Weighted posterior.** The posterior weights each observation by its decayed
relevance:

$$
P(M \mid o_{\le t})  \propto  P(M) \prod_{t'=0}^{t} P(o_{t'} \mid M)^{ w(t, t')}.
$$

An observation older than several half-lives contributes essentially nothing;
an observation within a half-life contributes nearly its full weight.

## 17.4 Worked example

Take an evidence type with a half-life of five minutes — a reasonable value for
an active C2 signal. Then

$$
\lambda = \frac{\ln 2}{5} = 0.138629\ \mathrm{min}^{-1}.
$$

At ten minutes of age, which is exactly two half-lives, the weight is

$$
w(10) = e^{-0.138629 \cdot 10} = e^{-1.38629} = 2^{-2} = 0.25.
$$

The equality of the decay weight to one quarter at two half-lives is exact: after two
half-lives, evidence retains exactly one quarter of its weight. This is the
clean form of the exponential-decay rule.

Reference decay rates for common evidence types are:

| Evidence | Half-life | $\lambda$ (min$^{-1}$) |
|---|---|---|
| Active C2 | 5 minutes | 0.138629 |
| Failed login | 1 hour | 0.011552 |
| Malware hash | 30 days | 0.000016 |
| Vulnerability | 90 days | 0.000005 |

The decay rates differ by orders of magnitude, which is the quantitative
content of the intuition that "a C2 signal is transient while a vulnerability
signal is persistent." A detection rule must use a decay rate matched to the
evidence it consumes; using a five-minute decay for a 90-day vulnerability
would discard the vulnerability signal almost immediately.

## 17.5 Consequences

Decay makes the defender's posterior a *windowed* quantity, aligned with the
causal windows of Chapter 5. It also disciplines the information set of
Chapter 10: since old evidence decays, the bounded information set need not
retain everything, only what is still above a weight threshold. And it feeds
the recovery calculus of Chapter 21, where the rate at which trust is restored
after an incident is itself a decay process.

## 17.6 What fails without A16

Without decay there is no forgetting, no staleness, and no notion of relevance.
The defender would aggregate evidence forever, letting a five-day-old signal
trigger a five-minute response. The pipeline of Chapter 33 applies decay in its
update step for exactly this reason.
