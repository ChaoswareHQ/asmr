# Chapter 29 — Zero-Day Hunting (Axiom A28)

> *Part III — Zero-Day Axioms*

## 29.1 Motivation

Detection waits for an alert; hunting goes looking. A zero-day, by definition,
will not be caught by rules written for known attacks, so the defender must
search proactively, guided by hypotheses. This chapter formalizes the hunt.

## 29.2 Axiom

**Axiom A28 (Zero-day hunting).** A hunt is a search over events whose
structural anomaly exceeds a threshold:

$$
\mathrm{Hunt}(h) \;=\; \{\, e \mid \mathrm{StructureAnomaly}(e) > \theta \,\},
$$

driven by a hypothesis.

## 29.3 Operations

**Hypothesis space.** The defender's hypotheses are the classes of novelty it
can imagine:

$$
H \;=\; \{\,\text{new TTPs},\ \text{new tools},\ \text{new infrastructure}\,\}.
$$

**Search.** Given a hypothesis, the defender searches for the events most
likely under it:

$$
\mathrm{Hunt}(h) \;=\; \arg\max_{e}\ P(e \mid h).
$$

## 29.4 Worked example

Consider the hypothesis: "the attacker is using WMI to execute code." The
corresponding Sysmon search is for process-creation events whose parent is the
WMI provider host and whose child is not a known, expected child:

```sql
SELECT * FROM sysmon
WHERE EventID = 1
  AND ParentImage LIKE '%wmiprvse.exe%'
  AND Image NOT IN (known_children)
```

Here `EventID = 1` selects process-creation records, `ParentImage` identifies
the WMI provider host as the parent, and the `NOT IN` clause excludes the
children that are known and benign. What remains is the set of processes that
were *unexpectedly* spawned by WMI — the structural anomaly the hypothesis
predicts. A result of, say, five such events is the starting point of an
investigation, not its conclusion; the hunt surfaces candidates, and the
posterior of Chapter 6 then evaluates them.

## 29.5 Consequences

Hunting is the proactive complement to detection. It converts a hypothesis
about the unseen into a search over structural anomalies (Chapter 24), and it
is where the defender's *prior* knowledge — its imagination of what a zero-day
might look like — enters the framework explicitly. It also closes the loop with
transfer learning (Chapter 30): the hypotheses a defender can form are drawn
from known attacks, transferred by similarity to the unknown.

## 29.6 What fails without A28

Without hunting, the defender is purely reactive: it sees only what its existing
rules fire on, and a zero-day, by construction, is not in those rules. The
framework would cede the initiative to the adversary precisely when the
adversary is most novel.
