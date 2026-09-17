# Chapter 42 — Benefits and Limitations

> *Part VIII — Assessment*

## 42.1 A statement of honesty

A framework is best assessed by stating both what it delivers and what it does
not. This chapter does both, and it separates claims that follow from the
axioms from claims that require empirical confirmation. The reader should treat
the two categories as different in kind.

## 42.2 Benefits

The benefits that follow from the framework's structure are these.

**Provable.** The framework produces theorems with proofs (Chapter 32). The gap
characterization, the optimal threshold, and the master equation are not
assertions but consequences of the axioms, and the first two are proven in
full.

**Predictive.** The axioms make directional predictions. The gap predicts the
false-positive rate; the threshold predicts the analyst's decision; the
resource shortfall predicts unreviewed volume. These are the hypotheses of
Chapter 40, and they are testable.

**Comparable.** The framework defines quantities that can be compared across
systems: the gap ratio (Chapter 2), the channel capacity (Chapter 16), the
blast radius (Chapter 14), and the availability (Chapter 21). Two deployments
can be compared on these axes without a common toolset.

**Optimal.** The decision rule is optimal by construction: it minimizes expected
cost (Chapter 9), with a closed-form threshold (Chapter 32), and falls back to
minimax regret where a prior is absent (Chapter 26).

**Governed.** The action space is filtered by autonomy policy (Chapter 13), so
that optimality and permissibility are separate, and the system cannot
automate what must not be automated.

**Robust.** Because the framework models an adaptive adversary (Chapters 8 and
15), its decision criteria are minimax where robustness is required, rather
than optimized against a fixed adversary.

## 42.3 Operational claims

Operational figures are sometimes quoted for the framework — reductions in
mean time to recovery, reductions in false-positive rate, reductions in cost,
and so on. **None of these are established in this edition.** They are the kind
of measurement that the validation program of Chapter 40 is designed to
produce, and until that program is executed they remain claims, not results.
The framework's *derived* quantities (posteriors, thresholds, blast radii) are
computed and reproducible; its *empirical* effects are open.

## 42.4 Limitations

The limitations are stated plainly.

**Consistency is unproven.** There is no proof in this edition that the thirty
axioms are mutually consistent, independent, or minimal. It is possible that
some axioms imply others, or that some are stronger than needed. This is a
theorem to be proven, not an assumption to be made.

**Validation is incomplete.** The validation of Chapter 40 is a program with
hypotheses, not a body of completed measurements. The framework has not been
tested at scale, and its empirical predictions are unconfirmed.

**Rationality is assumed.** The decision model of Chapter 9 and the game of
Chapter 8 assume that agents — defender and attacker — act to optimize their
objectives. Human analysts are subject to bias, fatigue, and error, and a model
that assumes rationality will mispredict their behavior where those factors
dominate. The resource model of Chapter 12 captures fatigue only as a capacity
shortfall, not as a cognitive bias.

**The zero-day problem is only partially addressed.** The open state space of
Chapter 22 and the transfer learning of Chapter 30 provide principled responses
to novelty, but the detection of a genuinely novel attack remains bounded by
Theorem 5 of Chapter 32: it cannot be detected by a detector trained only on
the known. Hunting (Chapter 29) is a mitigation, not a solution.

**Complexity.** Thirty axioms, eight parts, and a pipeline are a substantial
investment. The framework's unity is purchased at the cost of a steep learning
curve, and a practitioner who needs only a single result may find the whole
structure more than is warranted.

**Adoption.** The framework has no installed base, no ecosystem, and no
community of practice. These are the ordinary obstacles of any new formalism,
and they are recorded here rather than ignored.

## 42.5 Summary

The honest summary is this: ASMR provides a rigorous, unified, and computable
language for security monitoring and response, with proven theorems and
reproducible calculations, but its empirical validation, its formal
consistency proof, and its operational adoption are all open work. The roadmap
of Chapter 43 orders that work.
