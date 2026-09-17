# Chapter 36 — Case Study: Conti Ransomware

> *Part VI — Case Studies*

## 36.1 Background

Conti is a ransomware operation, publicly reported and widely attributed, whose
execution chain is notable for its deliberate destruction of recovery options
before encryption. The characteristic commands, as documented in public threat
intelligence, are:

```
vssadmin delete shadows /all /quiet
bcdedit /set {default} recoveryenabled No
wbadmin delete catalog -quiet
```

The first deletes volume shadow copies, the second disables the recovery
environment, and the third deletes the backup catalog. Together they remove the
two ordinary paths to recovery — shadow copies and backups — before the
encryption step begins. This is an attack against the *resilience* of
Chapter 21, executed before the attack against availability.

## 36.2 Detection rule

The detection rule is a conjunction over the observed commands:

```
IF proc = cmd
AND command CONTAINS "vssadmin delete shadows"
AND command CONTAINS "bcdedit"
AND command CONTAINS "recoveryenabled"
AND command CONTAINS "wbadmin delete catalog"
THEN alert "ransomware"
```

Each conjunct is individually weak — a system administrator might legitimately
run any one of them — but their conjunction is the signature. This is the
structural-anomaly logic of Chapter 24: the *combination* of events, not any
single value, is what is anomalous.

## 36.3 Model and computation

Because the commands are individually rare and jointly rarer still, the
likelihood ratios are large. Set the prior at 0.01 (the host is under
active investigation), and assign:

| Event | $P(o \mid M)$ | $P(o \mid \neg M)$ | Likelihood ratio |
|---|---|---|---|
| `vssadmin delete shadows` | 0.90 | 0.01 | 90.00 |
| `bcdedit` recovery disabled | 0.70 | 0.005 | 140.00 |
| `wbadmin delete catalog` | 0.80 | 0.01 | 80.00 |
| Bulk file-extension change | 0.95 | 0.002 | 475.00 |

The sequential posterior is:

| After observing | Likelihood ratio | Posterior $P(M \mid I)$ |
|---|---|---|
| `vssadmin delete shadows` | 90.00 | 0.476190 |
| `bcdedit` recovery disabled | 140.00 | 0.992204 |
| `wbadmin delete catalog` | 80.00 | 0.999902 |
| Bulk file-extension change | 475.00 | $\approx 1.000000$ |

The first command alone is enough to roughly even the odds; the second makes
compromise overwhelmingly likely; by the third the posterior is effectively
certain. The fourth observation — the encryption itself — is confirmatory.

## 36.4 Decision and response

At the posterior after the second command, 0.992204, the optimal response
under the cost model of Chapter 9 is **isolate the host**. Its expected cost is

$$
0.992204 \cdot 5{,}000 + 0.007796 \cdot 50{,}000 \approx \$5{,}351,
$$

which is far below the expected cost of doing nothing at this confidence (on
the order of 4,000,000 times the posterior$4{,}000{,}000 \cdot p$). Isolation is the correct response because
the attack's remaining steps — encryption and exfiltration — are blocked by
cutting the host off from the rest of the environment, which is precisely the
containment operation of Chapter 31.

The governance dimension matters here as in Chapter 13: isolation of a host is
a human-approved action, not an autonomous one, because the cost of isolating
a production host in error is high. The framework therefore *escalates* the
derived action to a human for approval rather than executing it automatically.
The decision is optimal; the authority to execute it is delegated.

## 36.5 Discussion

The Conti case is a study in *structure*. The individual commands are benign in
isolation; the sequence is not. The framework detects this not with a novel
signature but with the sequential Bayesian update of Chapter 10 applied to a
conjunction of otherwise-weak signals, followed by the cost–utility decision of
Chapter 9 and the governance filter of Chapter 13. The resilience dimension of
Chapter 21 is the reason the attack is dangerous: by deleting the recovery
options first, the attacker raises the MTTR of the eventual response, which is
exactly the quantity the framework's recovery model tracks.
