# Chapter 39 — Case Study: Cryptomining Exfiltration

> *Part VI — Case Studies*

## 39.1 Scenario

A compromised pod in a risk-analytics Kubernetes cluster begins mining
cryptocurrency and exfiltrating data. Unlike the previous cases, this is a
constructed scenario rather than a named public incident; it is included
because it illustrates the multi-scale structure of Chapter 19, where the
signal is distributed across several correlated telemetry streams rather than
concentrated in a single alarming event.

## 39.2 The telemetry

The relevant metrics over a fourteen-minute window are:

| Time | CPU (%) | Memory (%) | Net out (KB/s) | TCP retransmission (%) |
|---|---|---|---|---|
| 03:00 | 55 | 72 | 7,600 | 0.01 |
| 03:14 | 78 | 74 | 8,100 | 0.02 |
| 03:16 | 92 | 81 | 12,400 | 0.08 |
| 03:18 | 94 | 85 | 28,700 | 0.15 |

No single metric is alarming on its own: a CPU spike is ordinary, a network
spike is ordinary, and a small retransmission rate is ordinary. What is
anomalous is their *correlation*: CPU, memory, outbound traffic, and TCP
retransmission all rise together. This is the structural-anomaly signature of
Chapter 24 — the anomaly is in the joint pattern, not in any single value — and
it spans the packet level (retransmission), the flow level (net out), and the
host level (CPU, memory), which is the multi-scale fusion of Chapter 19.

## 39.3 Model and computation

Treat the *correlated* rise in CPU, network, and retransmission as a single
composite observation. With a prior $P(M) = 0.05$ (the cluster is under
observation) and likelihoods

$$
P(o \mid M) = 0.88,\qquad P(o \mid \neg M) = 0.01,
$$

the posterior is

$$
P(M \mid o) = \frac{0.88 \cdot 0.05}{0.88 \cdot 0.05 + 0.01 \cdot 0.95} = \frac{0.044}{0.0535} = 0.822430.
$$

The posterior is high but not certain, which is appropriate for a scenario in
which the signal is a correlation rather than a smoking gun. The composite
likelihood already encodes the correlation: the joint event "CPU and network
and retransmission all rise together" is far more likely under compromise than
under ordinary operation.

## 39.4 Decision and response

At $p = 0.822430$, the cost model of Chapter 9 is evaluated. This posterior is
below the block-versus-alert threshold of $p^{*} = 0.9$ (Chapter 32) but well
above the point at which alerting is warranted. The expected cost of blocking
is

$$
(1 - 0.822430) \cdot 500 = 88.79\ \text{dollars},
$$

while the expected cost of doing nothing is

$$
0.822430 \cdot 4{,}000{,}000 = 3{,}289{,}720\ \text{dollars}.
$$

The correct response is **block the offending network egress** — an autonomous
action under the governance policy of Chapter 13 — because the expected cost of
acting is two orders of magnitude below the expected cost of inaction. A full
multi-signal sequential model, fusing the four streams rather than collapsing
them into one composite observation, would push the posterior higher, which is
the point of the multi-scale detection formula of Chapter 19.

## 39.5 Discussion

The cryptomining scenario is the multi-scale complement to the single-chain
cases. Where FIN7 (Chapter 35) and Conti (Chapter 36) are sequences of discrete
events, this case is a *field* of correlated signals across scales, and its
detection requires the fusion that Chapter 19 formalizes. It also illustrates
the resource dimension of Chapter 12: the anomalous pod competes for CPU and
network with legitimate workloads, so the compromise is not only a security
event but a *resource* event, and the framework's resource model tracks it as
such.
