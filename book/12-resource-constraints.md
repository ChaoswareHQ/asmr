# Chapter 12 — Resource Constraints (Axiom A11)

> *Part I — Foundations*

## 12.1 Motivation

Real systems are bounded. Compute, memory, storage, and — most critically —
analyst attention are finite. A framework that ignores these bounds will
produce decisions that are optimal in an unbounded world and infeasible in the
actual one. This chapter makes the bounds explicit.

## 12.2 Axiom

**Axiom A11 (Resource constraints).** There exist resource constraints

$$
R(t) \;\le\; C,
$$

where the resources used up to time t are a vector of per-resource amounts,
and the capacities are a vector of per-resource limits.

## 12.3 Operations

**Resource-aware action set.** At time t, an action is available only if its
resource requirement fits within the remaining capacity:

$$
A_R(s, t) \;=\; \{\, a \in A(s) \mid \mathrm{req}(a) \le C - R(t) \,\}.
$$

The action space of Chapter 7 is thus *filtered* by resources before the
cost-minimizing decision of Chapter 9 is applied.

## 12.4 Worked example

Consider a security operations center (SOC) with the following constraint
structure:

| Resource | Capacity | Usage |
|---|---|---|
| Analysts | 20 | 20 |
| Hours per day | 160 | 160 |
| Alerts per day | 10,000 | 10,000 |
| Time per alert | 1 minute | 1 minute |

The workload of reviewing every alert is

$$
10{,}000 \text{ alerts} \times 1 \text{ minute} = 10{,}000 \text{ minutes} = 166.67 \text{ hours}.
$$

The available analyst-hours are

$$
20 \text{ analysts} \times 8 \text{ hours} = 160 \text{ hours}.
$$

Since 166.67 is greater than 160, the constraint is violated:

$$
166.67 > 160 \;\Longrightarrow\; \text{infeasible}.
$$

At one minute per alert, the team can review at most

$$
\frac{160 \text{ hours}}{1 \text{ minute per alert}} = 9600 \text{ alerts},
$$

leaving 400 alerts unreviewed. The conclusion is forced by arithmetic, not by
preference: *every alert cannot be reviewed*. Triage is not an optional
optimization; it is a logical consequence of the resource constraint.

## 12.5 Consequences

The resource constraint is what turns detection into prioritization. Given that
not every alert can be reviewed, the defender must rank alerts by expected
value — the product of the posterior probability that the alert is a true
attack and the cost of missing it — and allocate scarce analyst attention to the
top of that ranking. The posterior of Chapter 6 and the cost of Chapter 9 are
therefore not merely inputs to a decision; they are inputs to a *scheduling*
problem under the bound that the resources used do not exceed capacity.

The resource constraint also disciplines the design of detection itself. A
detector that produces more alerts than the SOC can review has, in effect,
converted a detection problem into a prioritization problem without improving
outcome. The Fano bound of Chapter 4 and the triage calculus of this chapter
together imply that the *rate* and the *quality* of alerts must be traded off
against analyst capacity.

## 12.6 What fails without A11

Without resource constraints there is no realism, no triage, and no
prioritization. The framework would assume unbounded attention, which is the
single assumption most obviously contradicted by practice. Chapter 40 returns
to this in the validation program, where analyst fatigue and unreviewed-alert
volume are proposed as observable consequences of the constraint.
