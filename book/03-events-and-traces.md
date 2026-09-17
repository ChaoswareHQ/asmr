# Chapter 3 — Events and Traces (Axiom A2)

> *Part I — Foundations*

## 3.1 Motivation

Security is about sequences, not snapshots. A single state may be innocuous; the
sequence of states that led to it may be the signature of an intrusion. This
chapter defines the discrete notion of change and the notion of a trace of such
changes.

## 3.2 Axiom

**Axiom A2 (Event transition).** An *event* is a partial function
e from the state space to itself that carries one state to another. Events are *sparse*: an event
changes a small number of factors and leaves the rest unchanged.

A *trace* is a sequence of states connected by events,

$$
\sigma \;=\; s_0 \xrightarrow{\,e_1\,} s_1 \xrightarrow{\,e_2\,} s_2 \xrightarrow{\,e_3\,} \cdots ,
$$

where the state at time t equals the event at step t applied to the previous state.

## 3.3 Operations

**Sparsity.** The set of factors changed by an event is

$$
\Delta(s, s') \;=\; \{\, i \in I \mid s_i \neq s'_i \,\},
$$

and sparsity is the condition that the number of changed factors is much smaller than the total number of factors.

**Replay.** Given a sequence of events, the state after time t is the
functional composition of the events applied to the initial state:

$$
s_t \;=\; e_t\bigl(e_{t-1}\bigl(\cdots e_1(s_0)\cdots\bigr)\bigr).
$$

**Sequential composition.** The composition of two events is their functional
composition:

$$
(e_1\,;\,e_2)(s) \;=\; e_2(e_1(s)).
$$

**Choice.** The choice of two events is their disjunction, wherever both are
defined:

$$
(e_1 + e_2)(s) \;=\; e_1(s) \lor e_2(s).
$$

**Iteration.** The Kleene iteration of an event is the join of all its finite
powers:

$$
e^{*}(s) \;=\; \bigvee_{n=0}^{\infty} e^{n}(s).
$$

**Identity and abort.** The identity event leaves every state fixed — the identity applied to any
state returns that state — and the abort event is undefined everywhere,
returning the undefined value for any state.

Together, sequential composition, choice, iteration, identity, and abort give
the events the structure of a Kleene algebra, which is the algebraic content
the framework's name refers to.

## 3.4 Worked example

Consider the following Sysmon record, in which an attacker moves from a Word
document through PowerShell to a command-and-control connection and persistence:

```
09:23:41.123  WINWORD.EXE starts
09:23:44.456  powershell.exe spawns (parent: WINWORD)
09:23:47.789  powershell -> 185.220.101.42:443
09:23:50.012  update.ps1 written
09:23:53.345  Run key set
```

Mapping each record to the four-factor state of Chapter 2:

| Time | Event | State |
|---|---|---|
| 09:23:41 | Word opens | `(winword, no_conn, no_download, normal)` |
| 09:23:44 | PowerShell spawns | `(powershell, no_conn, no_download, normal)` |
| 09:23:47 | C2 connection | `(powershell, c2_conn, no_download, normal)` |
| 09:23:50 | PS1 written | `(powershell, c2_conn, ps1, normal)` |
| 09:23:53 | Run key set | `(powershell, c2_conn, ps1, run_key)` |

Each transition changes exactly one of the four factors, so the sparsity of
every event in this trace is

$$
|\Delta(s_t, s_{t+1})| = 1 \quad \text{out of } 4,
$$

a changed-factor fraction of 0.25, consistent with the sparsity condition
that the number of changed factors is much smaller than the number of factors whenever the number of factors is large.

The trace itself is the composition

$$
s_5 = e_5\,;\,e_4\,;\,e_3\,;\,e_2\,;\,e_1\,(s_0).
$$

The individual events are unremarkable in isolation. The *sequence* — a
document spawning an interpreter that then establishes an outbound connection,
writes a script, and sets a persistence key — is the signature. This is the
sense in which detection is a property of traces rather than of states.

## 3.5 Consequences

Because traces are closed under composition, a detector can be defined as a
function on traces, not merely on the latest observation. This is what makes
correlation possible: the detector may be sensitive to the *order* of events,
which is the subject of Chapter 5, and to their accumulation, which is the
subject of Chapter 10.

The Kleene-algebra structure also provides a uniform language for describing
both the defender's playbooks and the attacker's kill chains. A playbook is a
program built from events with sequential composition, choice, and iteration;
a kill chain is the same. The difference between them is not in their structure
but in their cost and governance, developed in Chapters 9 and 13.

## 3.6 What fails without A2

Without events there are no traces; without traces there is no replay; without
replay there is no correlation. Detection is reduced to classifying isolated
snapshots, and the sequence that constitutes an intrusion — the only thing that
distinguishes it from an isolated anomaly — is invisible. Every subsequent
chapter that reasons over time (Chapters 5, 10, 17, and the pipeline of
Chapter 33) presupposes A2.
