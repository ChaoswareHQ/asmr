# Chapter 28 — Compensating Controls (Axiom A27)

> *Part III — Zero-Day Axioms*

## 28.1 Motivation

When there is no patch, the defender still has options: a web-application
firewall rule, a configuration change, a network block, isolation. These are
*compensating controls* — workarounds that reduce risk without fixing the
underlying vulnerability. This chapter formalizes their selection.

## 28.2 Axiom

**Axiom A27 (Compensating controls).** For a vulnerability with no patch,
the set of compensating controls is

$$
\mathrm{Compensate}(v) \;=\; \{\, a \in A \mid \mathrm{reduces}(a, v) \ \wedge\ \neg \mathrm{patch}(v) \,\}.
$$

## 28.3 Operations

**Effectiveness.** The effectiveness of a control is the fractional reduction in
risk:

$$
\mathrm{Eff}(a, v) \;=\; 1 - \frac{\mathrm{Risk}_{\mathrm{after}}(a, v)}{\mathrm{Risk}_{\mathrm{before}}(v)}.
$$

**Selection.** The defender chooses the control maximizing effectiveness net of
cost:

$$
a^{*} \;=\; \arg\max_{a}\ \bigl( \mathrm{Eff}(a, v) - \lambda \cdot C(a) \bigr),
$$

where the cost weight converts cost into the same units as effectiveness.

## 28.4 Worked example

For Log4Shell, the compensating controls available before the patch included:

| Control | Effectiveness | Cost |
|---|---|---|
| WAF rule | 0.70 | \$1,000 |
| Disable JNDI | 0.95 | \$10,000 |
| Network block | 0.50 | \$500 |
| Isolate affected hosts | 0.99 | \$50,000 |

The choice depends on the objective. If the defender has a hard budget of
\$15,000, the affordable controls are the WAF rule (\$1,000), disabling JNDI
(\$10,000), and the network block (\$500); isolation exceeds the budget. Among
the affordable controls, disabling JNDI has the highest effectiveness, 0.95.
If instead the defender's only goal is maximum risk reduction regardless of
cost, isolation is best at 0.99. And if the defender wants the most risk
reduction per dollar, the network block is the cheapest nontrivial control.

The example illustrates the general form: the control is *not* chosen by a rule
of thumb but by evaluating effectiveness against cost under an explicit
objective — a budget constraint, a cost weight, or a minimax criterion.
This is the same decision-theoretic structure as Chapter 9, applied to the
no-patch regime.

## 28.5 Consequences

Compensating controls are the operational content of "defense without a patch."
They also compose with detection: a control that reduces the *risk* of a
vulnerability also changes the *likelihoods* in the observation model of
Chapter 4, because the control alters what the adversary can do and therefore
what the defender observes. The framework's quantities — effectiveness, cost,
posterior — remain the same; only the available actions change.

## 28.6 What fails without A27

Without compensating controls, the framework would model the no-patch interval
as one in which nothing can be done — a paralysis that is false in practice and
unacceptable in operation. The case study of Log4Shell (Chapter 38) presupposes
A27.
