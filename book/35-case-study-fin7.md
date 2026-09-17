# Chapter 35 — Case Study: FIN7

> *Part VI — Case Studies*

## 35.1 Background

FIN7 is a financially motivated intrusion group, publicly reported since
roughly 2015, known for targeting retail and hospitality organizations through
spearphishing, PowerShell, and custom tooling. Its observed techniques map onto
the MITRE ATT&CK taxonomy as follows:

| Phase | Technique | Identifier |
|---|---|---|
| Initial access | Spearphishing attachment | T1566.001 |
| Execution | PowerShell | T1059.001 |
| Persistence | Registry run keys | T1547.001 |
| Command and control | Web protocols (HTTPS) | T1071.001 |

This case study walks the framework's pipeline over a representative attack
chain of the kind FIN7 has been reported to use.

## 35.2 The attack chain

A Word document arrives as an email attachment; it contains a macro. The chain,
as it would appear in Sysmon telemetry, is:

```
09:23:41.123  WINWORD.EXE starts
09:23:44.456  powershell.exe spawns (parent: WINWORD)
09:23:47.789  powershell -> 185.220.101.42:443
09:23:50.012  update.ps1 written
09:23:53.345  Run key set
```

In the four-factor state space of Chapter 2, this is the trace:

| Time | Event | State |
|---|---|---|
| 09:23:41 | Word opens | `(winword, no_conn, no_download, normal)` |
| 09:23:44 | PowerShell spawns | `(powershell, no_conn, no_download, normal)` |
| 09:23:47 | C2 connection | `(powershell, c2_conn, no_download, normal)` |
| 09:23:50 | PS1 written | `(powershell, c2_conn, ps1, normal)` |
| 09:23:53 | Run key set | `(powershell, c2_conn, ps1, run_key)` |

## 35.3 Model and computation

Set the base rate of compromise for a random endpoint at 0.001, and
assign each observed event a likelihood under the two hypotheses:

| Event | $P(o \mid M)$ | $P(o \mid \neg M)$ | Likelihood ratio |
|---|---|---|---|
| Word macro executes | 0.30 | 0.05 | 6.00 |
| PowerShell spawned by Word | 0.60 | 0.02 | 30.00 |
| Outbound C2 to known-bad IP | 0.85 | 0.02 | 42.50 |
| PS1 payload written | 0.70 | 0.01 | 70.00 |
| Run key persistence | 0.80 | 0.005 | 160.00 |

Applying the sequential update of Chapter 10, the running posterior after each
observation is:

| After observing | Likelihood ratio | Posterior $P(M \mid I)$ |
|---|---|---|
| Word macro executes | 6.00 | 0.005970 |
| PowerShell spawned by Word | 30.00 | 0.152672 |
| Outbound C2 | 42.50 | 0.884495 |
| PS1 payload written | 70.00 | 0.998138 |
| Run key persistence | 160.00 | 0.999988 |

The progression is the quantitative form of the claim of Chapter 10: a single
event (the macro) leaves the posterior below 1%; the accumulation of five
events drives it to virtual certainty. The decisive step is the C2 connection —
after the first three events the posterior is already 0.884495 — but the two
subsequent events remove the remaining doubt.

## 35.4 Decision and response

At the final posterior 0.999988, the cost model of Chapter 9 is
evaluated. Using the cost table of that chapter:

| Action | Expected cost |
|---|---|
| Do nothing | ≈ \$3,999,952 |
| Alert | \$50 |
| Block IP | ≈ \$0.006 |
| Isolate host | ≈ \$5,000 |

The posterior is far above the block-versus-alert threshold of $p^{*} = 0.9$
(Chapter 32), so the optimal action is **block the IP**, autonomously, per the
governance policy of Chapter 13. The block is cheap when right and cheap when
wrong at this confidence; isolation is unnecessary because the posterior has
effectively settled the question.

## 35.5 Discussion

The case exercises the full pipeline: state (Chapter 2), trace (Chapter 3),
sequential inference (Chapter 10), decision (Chapter 9), and governance
(Chapter 13). Its lesson is methodological rather than sensational: the
framework does not require a novel detection rule for FIN7, only the sequential
application of Bayes' rule with defensible likelihoods, followed by a
thresholded cost decision. The numbers that matter — the likelihood ratios and
the running posterior — are computed, not asserted, and are reproduced by the
reference program of Chapter 34.
