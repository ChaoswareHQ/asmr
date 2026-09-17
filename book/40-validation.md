# Chapter 40 — Validation

> *Part VII — Validation*

## 40.1 The principle

An axiom is useful only to the extent that it makes a testable prediction. This
chapter states the validation program of the framework: for each core axiom, a
hypothesis that can be tested against data, and the quantity to be measured.
It distinguishes three kinds of claim, and the reader should keep them
separate:

1. **Derived results**, which follow from the axioms and are computed in this
   book (the gap ratio, the threshold, the Fano bound, the posteriors of
   Part VI). These are proven or computed, and are reproducible through the
   reference program of Chapter 34.

2. **Testable predictions**, which state how a derived quantity should covary
   with an observable quantity (for example, how the gap ratio should predict
   the false-positive rate). These are hypotheses to be tested.

3. **Reported measurements**, which are empirical numbers that have been
   reported but not independently verified in this edition. These are marked as
   such, and are targets for reproduction rather than established facts.

## 40.2 The hypotheses

| Axiom | Hypothesis | Predicted covariate |
|---|---|---|
| A3 | The gap predicts the false-positive rate | Gap ratio $\to$ FP rate |
| A8 | The threshold predicts analyst behavior | $p$ vs. $p^{*}$ $\to$ accept/decline |
| A7 | The master equation predicts response | Game solution $\to$ observed response |
| A11 | Resources predict fatigue | $R(t)$ vs. $C$ $\to$ unreviewed volume |

Each hypothesis has a direction, not merely a correlation. A3 predicts that a
larger gap — more states collapsed into the same observation — produces more
false positives, because more benign states are observationally
indistinguishable from the alerting state. A8 predicts that analysts accept an
alert when the posterior exceeds the cost threshold and decline it otherwise.
A11 predicts that unreviewed-alert volume is the shortfall
$\max(0, \text{workload} - \text{capacity})$.

## 40.3 Method

The natural datasets are:

- **CICIDS2017**, a public intrusion-detection dataset with labeled flows,
  for testing A3 and the capacity analysis of Chapter 16;
- **MITRE ATT&CK** and public threat-intelligence reporting, for testing A7
  against documented defender responses;
- **SOC telemetry**, for testing A11 and the triage calculus of Chapter 12.

For A3, the procedure is: fix a projection $\pi$ (full, process-only,
network-only, file-only), compute the gap ratio $|\Delta| / |S|$ exactly as in
Chapter 2, fit a detector, and measure its false-positive rate. The hypothesis
is that the two quantities are monotonically related. The gap ratio itself is
computed here; the false-positive rate is the quantity to be measured.

For A8, the procedure is: for a set of alerts, compute the posterior $p$ and
the threshold $p^{*}$ from a cost table, and record the analyst's accept or
decline decision. The hypothesis is that acceptance tracks the comparison
$p > p^{*}$. The threshold is computed here; the analyst's decision is the
quantity to be measured.

## 40.4 Reported results, to be reproduced

The following tables report results that have been stated for the framework but
are **not independently verified in this edition**. They are included to make
the validation program concrete, and are targets for reproduction, not
established facts.

**A3 — gap versus false-positive rate** (reported, unverified):

| Projection $\pi$ | $|\Delta|$ (reported) | FP rate (reported) |
|---|---|---|
| Full | 0.0 | 0.01 |
| Process only | 0.4 | 0.15 |
| Network only | 0.6 | 0.30 |
| File only | 0.5 | 0.25 |

**A8 — threshold versus analyst decision** (reported, unverified):

| $p$ | $p^{*}$ | Analyst says yes (reported) |
|---|---|---|
| 0.01 | 0.9 | 0% |
| 0.50 | 0.9 | 10% |
| 0.95 | 0.9 | 95% |

**A7 — master equation versus observed response** (reported, unverified):

| Scenario | Framework prediction | Reported defender | Match (reported) |
|---|---|---|---|
| APT29 | Block C2 | Block C2 | yes |
| FIN7 | Isolate host | Isolate host | yes |
| Conti | Restore backup | Restore backup | yes |

**A11 — resources versus unreviewed volume** (reported, unverified):

| SOC | $R(t)$ | $C$ | Unreviewed (reported) |
|---|---|---|---|
| Small | 1,000 | 500 | 500 |
| Medium | 5,000 | 2,000 | 3,000 |
| Large | 10,000 | 9,600 | 400 |

The reader should treat the direction of these hypotheses as the substantive
claim, and the specific numbers as the values that a reproduction must either
confirm or revise.

## 40.5 What validation would establish

If the hypotheses of Section 40.2 are confirmed, then the framework earns its
name: its axioms are not merely consistent but *predictive*, in that they
convert a structural quantity (a gap, a threshold, a resource shortfall) into
an observable outcome (a false-positive rate, a decision, a fatigue level). If
they are not confirmed, then the framework's axioms are wrong or incomplete in
a way that the data will localize — which is itself useful. The validation
program is thus not a decoration but a test, and its outcome is recorded
honestly in Chapter 42 as an open question.
