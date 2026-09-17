# Chapter 5 — Time and Causality (Axiom A4)

> *Part I — Foundations*

## 5.1 Motivation

Cross-host correlation requires more than timestamps. Two events recorded at the
same wall-clock instant may be unrelated, and two events recorded far apart may
be causally linked. This chapter distinguishes local time from global
causality, and provides the standard device — the vector clock — for deciding
which of two events came before the other.

## 5.2 Axiom

**Axiom A4 (Temporal–causal structure).** Local time on each host is a total
order; global causality across hosts is a partial order.

Formally, each host h carries a totally ordered time domain
(a set of time values with a total order). The set E of all events carries a partial order, the causal order,
where e1 precedes e2 means "e1 causally precedes e2". Two events that are
incomparable under the causal order are *concurrent*.

## 5.3 Vector clocks

A vector clock assigns to each event a vector of natural numbers, one component
per host:

$$
VC : E \to \mathbb{N}^{n}.
$$

The fundamental property is that causality is decided componentwise:

$$
e_1 \prec e_2 \;\Longleftrightarrow\; VC(e_1) < VC(e_2),
$$

where the less-than relation is the componentwise strict order with strictness in at least one
coordinate. Concurrency is the negation of both directions:

$$
e_1 \parallel e_2 \;\Longleftrightarrow\; \neg(e_1 \prec e_2) \wedge \neg(e_2 \prec e_1).
$$

Vector clocks are the correct data structure for correlating events across
hosts because they track causal dependencies rather than wall-clock order.
Wall-clock order is a poor proxy: clock skew, network delay, and buffering can
each reorder events that are in fact causally ordered, or align events that are
in fact unrelated.

## 5.4 Worked example

Consider two hosts.

Host A:

```
10:00:01  Word opens
10:00:02  PowerShell runs
10:00:03  PS1 downloaded
```

Host B:

```
10:00:02  DNS query
10:00:04  C2 connection
```

There is a causal link between the script download on A and the C2 connection
on B — the script, once downloaded, is what initiates the connection — so

$$
\texttt{PS1 downloaded} \prec \texttt{C2 connection}.
$$

The Word-open event on A and the DNS query on B are unrelated by any causal
path, so they are concurrent:

$$
\texttt{Word opens} \parallel \texttt{DNS query}.
$$

Assigning vector clocks with two components (A, B), where A's events advance
the first component and B's the second, one obtains, after the third event of
each host,

$$
VC(e_A) = (3, 0),\qquad VC(e_B) = (0, 2).
$$

Neither vector is componentwise less than the other, so the two events are
concurrent — exactly as required, because neither host's third event was caused
by the other.

## 5.5 Consequences

The partial order is what makes traces *windows* well-defined. A
detection window is an antichain or an interval in the causal order, not merely
a wall-clock interval. This matters for detection: an attack whose stages are
spread across hosts and separated by network delay may appear, in wall-clock
time, as a set of unrelated events, yet be a single causal chain. Correlation
that respects the causal order recovers the chain; correlation that respects only
wall-clock time may not.

The causal order also underpins the information sets of Chapter 10: what the
defender knows at time t is indexed by causal time, not clock time, so that
knowledge propagates only along causal links.

## 5.6 What fails without A4

Without a distinction between local order and global causality there is no
coherent notion of "before" across hosts. Without a partial order there are no
concurrent events, and a framework is forced either to treat every event as
ordered (which is false) or to treat none as ordered (which forfeits
correlation). Without vector clocks there is no finite, computable witness to
causality. The traces of Chapter 3 and the windows they define presuppose A4.
