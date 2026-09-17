# Chapter 27 — Patch Latency (Axiom A26)

> *Part III — Zero-Day Axioms*

## 27.1 Motivation

The usual remedy for a vulnerability is a patch, and a patch takes time. For a
zero-day there is, by definition, no patch yet. This chapter formalizes patch
latency and its extreme case.

## 27.2 Axiom

**Axiom A26 (Patch latency).** The time from detection of a vulnerability to
the availability of a patch is a random variable depending on the vendor, the
severity, and the complexity of the fix:

$$
T_{\mathrm{patch}}  =  f(\text{vendor}, \text{severity}, \text{complexity}).
$$

## 27.3 Operations

**Distribution.** Patch latency is modeled as log-normally distributed,

$$
T_{\mathrm{patch}} \sim \mathrm{LogNormal}(\mu, \sigma^2),
$$

whose expectation is

$$
\mathbb{E}[T_{\mathrm{patch}}] = e^{\mu + \sigma^2 / 2}.
$$

The log-normal form is appropriate because patch times are positive, skewed,
and bounded below by zero.

**Zero-day case.** For a zero-day, no patch exists, so

$$
T_{\mathrm{patch}} = \infty.
$$

This is not a large finite number; it is the absence of the usual remedy.

## 27.4 Worked example

Reported remediation times for high-profile vulnerabilities span orders of
magnitude (representative magnitudes, as publicly reported):

| Vulnerability | Remediation time |
|---|---|
| Log4Shell (CVE-2021-44228) | on the order of two weeks for the initial fix |
| Spring4Shell (CVE-2022-22965) | on the order of one week for the initial fix |
| SolarWinds (SUNBURST) | on the order of months for full remediation |
| A new zero-day | unknown — $T_{\mathrm{patch}} = \infty$ |

The point is not the exact durations but their structure. Even for a
well-resourced, widely deployed vulnerability, the patch arrives in days or
weeks; during that window the defender must operate *without* the patch. For a
zero-day the window has no known endpoint. The defender's response problem is
therefore: what to do in the interval from zero to the patch time, when
the patch time is unknown and possibly infinite.

## 27.5 Consequences

Patch latency forces the compensating-controls problem of Chapter 28: if the
patch is unavailable, the defender must reduce risk by other means. It also
informs the freeze response of Chapter 31, because the decision to freeze — to
stop processing and isolate — is taken precisely when the expected patch time
is long relative to the cost of freezing. The formal connection is the theorem
of Chapter 32: when novelty is high and patch latency is effectively infinite,
freeze is the minimax-optimal response.

## 27.6 What fails without A26

Without patch latency the framework would assume a patch is always available,
and would have no account of the interval in which the vulnerability is live
but unfixed — which is exactly the interval in which a zero-day does its damage.
