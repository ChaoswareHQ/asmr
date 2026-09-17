# Chapter 11 — Trust and Integrity (Axiom A10)

> *Part I — Foundations*

## 11.1 Motivation

Not all sensors are honest. A sensor may be noisy (Chapter 4), but it may also
be *compromised*, in which case its reports are chosen by the adversary. The
SolarWinds compromise demonstrated the extreme case: a signed, fully trusted
update channel was itself the attack vector. This chapter formalizes trust and
its limits.

## 11.2 Axiom

**Axiom A10 (Trust and integrity).** Observations carry a trust level, drawn from a trust lattice. Trust is propagated through
composition and is used to weight likelihoods.

## 11.3 Operations

**Trust propagation.** When two sources are composed across a boundary, their combined trust is

$$
\tau(s_1 \circ s_2)  =  \tau(s_1) \odot \tau(s_2) \odot \kappa(b_{12}),
$$

where the meet operation is the lattice meet (combining evidence
conservatively) and the boundary integrity is the integrity of the boundary
between them.

**Trust-weighted likelihood.** An observation reported with a given level of trust has
likelihood

$$
P(o \mid s, \tau)  =  \tau \cdot P_{\mathrm{honest}}(o \mid s) + (1 - \tau) \cdot P_{\mathrm{adversarial}}(o \mid s).
$$

With probability equal to the trust the report comes from an honest source;
with probability one minus the trust it comes from a source controlled by the
adversary.

**Trust threshold.** An action is permitted only if trust is sufficient:

$$
\mathrm{Allowed}(a)  \Longleftrightarrow  \tau(a) \ge \tau_{\min}(a).
$$

## 11.4 Worked example

Consider the following trust assignments:

| Source | Trust | Reason |
|---|---|---|
| Sysmon (signed) | 0.95 | Kernel-level telemetry |
| Windows Event Log | 0.90 | Operating-system level |
| Firewall | 0.80 | Network-level |
| DNS | 0.60 | Susceptible to spoofing |
| Proxy | 0.70 | Susceptible to bypass |

For the likelihood computation, take an honest channel that reports `c2` with
probability 0.85 when the attack is present and 0.02 when it is not, and an
adversarial channel in which the compromised source is forced by the adversary
to report `c2` constantly, with probability 1. With a base rate of attack of
0.001, the trust-weighted likelihoods and posterior are:

| $\tau$ | $P(\texttt{c2} \mid M)$ | $P(\texttt{c2} \mid \neg M)$ | $P(M \mid \texttt{c2})$ |
|---|---|---|---|
| 0.80 | 0.8800 | 0.2160 | 0.004062 |
| 0.95 | 0.8575 | 0.0690 | 0.012287 |

Two observations follow. First, *lower trust degrades the posterior*: at trust 0.80 the posterior is 0.004062, whereas the honest-source posterior
computed in Chapter 6 was 0.040807 — an order of magnitude higher. The reason
is that a source which may be compromised can no longer be taken at its word,
and its report of `c2` is partly discounted as possible adversary
manipulation. Second, the effect is nonlinear and pronounced: a small drop in
trust (from 0.95 to 0.80) cuts the posterior by roughly a factor of three.

The converse case is also instructive. If the adversary instead *suppresses*
the C2 signal — the compromised source always reports benign, so the
adversarial channel reports `c2` with probability 0 — then at trust 0.80,

$$
P(\texttt{c2} \mid M) = 0.68,\qquad P(\texttt{c2} \mid \neg M) = 0.016,\qquad P(M \mid \texttt{c2}) = 0.040807.
$$

The posterior equals the honest value, because a report of `c2` can now
only have come from the honest fraction of the source. This asymmetry — a
compromised source can *hide* the attack without distorting the meaning of a
positive report, but can *fabricate* positives that destroy the meaning of a
positive report — is the essence of why trust, and not merely noise, must be
modeled.

## 11.5 The SolarWinds lesson

The SolarWinds compromise is the canonical failure mode. The signed Orion
update channel carried trust equal to 1.0, and yet the update was malicious.
Trust in the *source* was perfect while the *content* was hostile. The
conclusion is that trust must be paired with integrity: a cryptographic check
that the content is what it claims to be,

$$
\mathrm{Integrity}(s)  =  \bigl(\mathrm{hash}(s) = \text{expected hash}\bigr).
$$

If the hash differs, trust is set to zero regardless of the reputation of the
source:

$$
\tau(s) = 0.
$$

Trust, without integrity, is an assertion of confidence without a check; the
two are complementary, and Axiom A10 requires both.

## 11.6 Consequences

Trust weights every likelihood, and therefore every posterior, and therefore
every decision downstream. It also composes through boundaries, which makes it
the foundation for the composition algebra of Chapter 14 and the blast-radius
calculation there. And it gates actions, which connects it to the governance of
Chapter 13: an action may be prohibited by policy or by insufficient trust.

## 11.7 What fails without A10

Without trust there is no deception resistance, no supply-chain security, and
no zero-trust reasoning. A framework that treats every sensor as equally honest
will be manipulated by exactly the adversary who learns to corrupt a single
sensor. Chapter 15 (adaptation), Chapter 18 (deception), and Chapter 37 (the
SolarWinds case study) all presuppose A10.
