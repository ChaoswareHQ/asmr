# Chapter 9 — Cost and Utility (Axiom A8)

> *Part I — Foundations*

## 9.1 Motivation

Every action has a cost, and the cost depends on whether the action is taken
against a real attack or a false alarm. This chapter turns the posterior of
Chapter 6 into a decision: among the applicable actions, choose the one that
minimizes expected cost.

## 9.2 Axiom

**Axiom A8 (Cost–utility model).** For each action there are two costs — the
cost when there *is* an attack and when there *is not* — and a utility for the
defender.

## 9.3 Operations

**Expected cost.** Given a posterior probability of attack p, the expected cost of an action is

$$
\mathbb{E}[C(a)]  =  p \cdot C_M(a) + (1 - p) \cdot C_{\neg M}(a).
$$

**Optimal action.** The optimal action minimizes expected cost:

$$
a^{*}  =  \arg\min_{a} \mathbb{E}[C(a)].
$$

**Threshold.** For two actions, the probability at which they are
equally costly is

$$
p^{*}  =  \frac{C_{\neg M}(a) - C_{\neg M}(a')}{C_{\neg M}(a) - C_{\neg M}(a') + C_M(a') - C_M(a)}.
$$

Below the threshold one action is cheaper; above it, the other. The threshold is
derived by setting the expected cost of the two actions equal and solving for the posterior;
the proof appears in Chapter 32.

## 9.4 Worked example

Consider the following cost table, stated in dollars, for a large organization:

| Action | $C_M$ | $C_{\neg M}$ |
|---|---|---|
| Do nothing | 4,000,000 | 0 |
| Alert | 50 | 50 |
| Block IP | 0 | 500 |
| Isolate host | 5,000 | 50,000 |

The intuition of the table is that "do nothing" is free when benign but
catastrophic when it is in fact an attack, while "isolate host" is cheap when it
is a real attack but expensive when it is a false alarm that takes a production
host offline.

Take the posterior computed in Chapter 6, which was 0.040807. The expected cost of
each action is:

| Action | Computation | Expected cost |
|---|---|---|
| Do nothing | $0.040807 \cdot 4{,}000{,}000$ | \$163,226.12 |
| Alert | $0.040807 \cdot 50 + 0.959193 \cdot 50$ | \$50.00 |
| Block IP | $0.959193 \cdot 500$ | \$479.60 |
| Isolate host | $0.040807 \cdot 5{,}000 + 0.959193 \cdot 50{,}000$ | \$48,163.71 |

The optimal action is **Alert**, at \$50. The point is that the dramatic
actions — blocking an IP or isolating a host — are *not* warranted at this
posterior, because the probability that the observation is a false alarm is 0.959193, and false-alarm costs dominate.

The threshold between blocking and alerting is

$$
p^{*} = \frac{500 - 50}{500 - 50 + 50 - 0} = \frac{450}{500} = 0.9.
$$

Only when the posterior exceeds 0.9 does blocking become cheaper than
alerting. This single number is a complete decision rule for the choice between
those two actions, and it is derived, not chosen.

## 9.5 Consequences

The cost–utility model is the decision-theoretic heart of the framework. It
converts a posterior — a number between zero and one — into an action, by
minimizing expected cost. It also explains a familiar pattern in security
operations: that the correct response to an uncertain alarm is usually a cheap,
reversible action (alert, gather more evidence), not an expensive, irreversible
one (isolate, destroy), because at low posterior the false-alarm cost of the
latter dominates. The threshold theorem makes this precise, and it is the
reason the framework can be automated safely: the decision rule is a closed
form, not a judgment call.

## 9.6 What fails without A8

Without costs there is no decision, no threshold, and no optimization. The
posterior of Chapter 6 is a number without a consumer; the framework knows how
likely an attack is but not what to do about it. Every response-oriented
chapter that follows — governance (Chapter 13), recovery (Chapter 21),
compensating controls (Chapter 28), and the pipeline (Chapter 33) — presupposes
A8.
