# Chapter 4 — Observation and Partial Observability (Axiom A3)

> *Part I — Foundations*

## 4.1 Motivation

The defender does not observe the state; it observes a projection of the state,
corrupted by the sensor. This chapter formalizes that corruption and derives
the central negative result of the framework: there is a hard lower bound on
the error of any detector fed by a limited channel.

## 4.2 Axiom

**Axiom A3 (Observation projection).** The observation at time t is a
noisy projection of the state,

$$
o_t \;=\; \eta_t\bigl(\pi(s_t),\, \theta_t\bigr),
$$

where

- the projection, which maps the state space to the observation space, is the observation map, selecting which coordinates of the
  state are visible and in what form;
- the noise of the sensor, which may drop, delay, or corrupt the
  projected value; and
- the *context* is time, location, sensor type, and configuration.

The projection determines the fiber of an observation,

$$
\pi^{-1}(o) \;=\; \{\, s \in S \mid \pi(s) = o \,\},
$$

the set of states consistent with the observation.

## 4.3 The coverage bound and the Fano bound

Two consequences follow immediately.

**Coverage bound.** A decision rule that depends on a factor not present in the
image of the projection cannot be made reliable. If a rule asks "is the process
malicious?" but the projection does not observe the process factor, then no observation
can answer the question. The rule's error is bounded away from zero regardless
of its design.

**Fano bound.** Consider the entropy of the state, the mutual information
between state and observation, and the number of states. Then the error
probability of any detector satisfies

$$
P_e \;\ge\; \frac{H(S) - C - 1}{\log_2 |S|}.
$$

This is Fano's inequality applied to the observation channel. It states that
the defender's error is controlled by the *residual* entropy, the entropy of the state given the observation, which
equals the entropy of the state minus the mutual information: however much of the state the sensor fails to convey, that much
ambiguity remains and must appear as error somewhere.

## 4.4 Worked example

Let the full state be

$$
s = (\texttt{powershell},\ \texttt{c2\_conn},\ \texttt{ps1},\ \texttt{normal}).
$$

A firewall observes only the network factor, so

$$
\pi_{\mathrm{net}}(s) = \texttt{c2\_conn}.
$$

If the sensor drops the packet, the noise term acts:

$$
\eta(\texttt{c2\_conn}) = \texttt{no\_conn}.
$$

The fiber of the uncorrupted observation has size

$$
|\pi_{\mathrm{net}}^{-1}(\texttt{c2\_conn})| = 5 \cdot 1 \cdot 5 \cdot 4 = 100,
$$

as computed in Chapter 2. A rule that asks "is the process
`malicious_ps`?" cannot fire at all, because the network projection does
not observe the process factor. This is the coverage bound.

For the Fano bound, take a canonical, internally consistent instance. Let the
state be uniform over 1024 states (2 to the 10th power), so that the entropy of the state is 10 bits.
Suppose the observation channel conveys 5 bits of mutual information between state and observation,
so that the residual entropy, the entropy of the state given the observation, is 5 bits. Then

$$
P_e \;\ge\; \frac{10 - 5 - 1}{10} = \frac{4}{10} = 0.4.
$$

A sensor that conveys half the state's entropy leaves a residual of five bits,
and no detector can do better than a 40% error rate on it. This is not a
statement about the quality of any particular detector; it is a statement about
the channel. Better engineering cannot beat it; only a better sensor — one with
greater mutual information — can.

## 4.5 Consequences

The coverage bound and the Fano bound convert a qualitative complaint ("we
cannot see everything") into a quantitative one ("we can see this much, and no
more"). They provide the vocabulary for sensor design: to reduce error below a
target, one must either raise the mutual information (add sensors, richer telemetry) or reduce the entropy of the state (coarsen the state space), because the detector's error is bounded below
by what the channel omits. Chapter 16 revisits this from the channel's side,
and Chapter 11 shows how trust in the sensor further degrades the effective
channel.

## 4.6 What fails without A3

Without the observation projection there is no gap, no coverage bound, and no
Fano bound. The defender's uncertainty is unquantified, and every claim of
detection quality is unanchored. Without the noise term, the framework
assumes a perfect sensor, which is precisely the assumption that fails first in
practice. The Bayesian update of Chapter 6 depends on the likelihood
of the observation given the state introduced here, so A3 is load-bearing for the entire inferential
part of the framework.
