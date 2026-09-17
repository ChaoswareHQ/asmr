# Chapter 16 — Observation Channel Capacity (Axiom A15)

> *Part II — Extended Axioms*

## 16.1 Motivation

A sensor conveys information, and there is a hard limit to how much. That limit
is the channel capacity, and it is the quantity that tells the defender whether
a detection goal is *possible* before asking how to achieve it. This chapter
makes the limit precise.

## 16.2 Axiom

**Axiom A15 (Observation channel capacity).** The observation channel has a
capacity

$$
C \;=\; \max_{P(s)} I(S; O),
$$

the maximum mutual information between state and observation over all input
distributions.

## 16.3 Operations

**Mutual information.** The information the observation carries about the state
is

$$
I(S; O) \;=\; H(S) - H(S \mid O),
$$

the entropy of the state minus the entropy remaining after observation.

**Fano bound.** As in Chapter 4, the capacity bounds the achievable error of any
detector:

$$
P_e \;\ge\; \frac{H(S) - C - 1}{\log_2 |S|}.
$$

**Sample complexity.** To identify the state to within a target error, the
number of observations must grow at least like

$$
n \;\gtrsim\; \frac{\log_2 |S|}{\delta^2},
$$

where delta is the target accuracy. Fewer observations than this cannot
disambiguate the state space, regardless of the detector's design.

## 16.4 Worked example

Different sensors have different capacities, roughly:

| Sensor | Capacity | States distinguishable |
|---|---|---|
| DNS logs | 2 bits | 4 |
| NetFlow | 5 bits | 32 |
| EDR | 10 bits | 1,024 |
| Full packet capture | 20 bits | 1,048,576 |

Return to the canonical instance of Chapter 4: a state uniform over
a state space of 1024 states, so the entropy of the state is 10 bits, and a channel of capacity
5 bits. Then the entropy of the state given the observation is 5 bits, and

$$
P_e \;\ge\; \frac{10 - 5 - 1}{10} = 0.4.
$$

The interpretation is direct. The sensor can distinguish at most 32 states
(its 5 bits), but the state space has 1024 states (the defender needs 10
bits). The missing 5 bits must appear as error somewhere, and the Fano bound
says that error is at least 40%. No algorithm can do better on this channel.

This is what makes capacity a *design* quantity rather than a curiosity. If the
defender needs to distinguish 1024 states, it must procure a sensor with at
least 10 bits of capacity — EDR, in the table above, or a combination of
sensors whose joint capacity reaches 10 bits. NetFlow alone, at 5 bits,
cannot support the goal, and no amount of tuning changes that.

## 16.5 Consequences

Capacity is the bridge between "what we want to know" and "what we can know."
It converts a detection requirement into a sensor requirement: to reduce error
below a target, raise the channel capacity. It also interacts with trust
(Chapter 11): a sensor of lower trust has, in effect, lower capacity, because
its reports are partially discounted as possible manipulation. The
trust–capacity tradeoff is stated as a theorem in Chapter 32.

## 16.6 What fails without A15

Without capacity there is no detection bound and no feasibility analysis. The
framework could not distinguish "this detector is bad" from "this detector is
impossible," and would waste effort tuning a detector against a channel that
cannot support it. Chapter 40 proposes capacity as the quantity to measure when
validating the framework against real datasets.
