# Preface

## Purpose of this book

This book develops a single, self-contained framework for security monitoring
and response. Its claim is not that security operations are easy, but that they
are *structured*: that the pipeline from a hidden system state, through
observation, inference, decision, and governed action, and back to the state,
obeys a small set of axioms whose consequences can be stated precisely and
computed exactly.

The framework is called **Algebraic Security for Monitoring and Response**,
abbreviated **ASMR**. The name is deliberate. The word *algebraic* signals that
the subject is treated as a formal system — objects are defined, operations on
them are defined, and identities and bounds are derived — rather than as a
catalog of tools. The word *security* names the domain. The phrase *monitoring
and response* delimits the subject: the continuous process of inferring the
state of a defended system and acting on that inference.

## What the book assumes

The book assumes familiarity with elementary probability theory, information
theory, and discrete mathematics at the level of an undergraduate course. The
necessary background is summarized in Appendix B. No prior knowledge of
security operations is assumed; the relevant facts about attacks, defenses,
and organizational constraints are introduced as they are needed, in the
chapters that use them.

## What the book proves, and what it does not

The book proves results of two kinds.

First, *internal* results: given the axioms, certain theorems follow. These
include the characterization of the observation gap, the optimal decision
threshold, the interaction of trust and channel capacity, and the master
equation for defender and attacker. These results are proven or sketched in
Chapter 32.

Second, *computational* results: every worked example is evaluated by a
committed program, so that no numerical claim depends on arithmetic performed
in prose. The program is described in the front matter of the repository and
printed in full in Chapter 34.

The book does **not** claim that ASMR is complete, that its axioms are mutually
independent, or that its predictions have been verified at scale. These are
open questions, stated explicitly in Chapter 42. Where a number is an empirical
measurement rather than a computation, the text says so and marks it for
reproduction.

## Conventions

Mathematical notation follows standard conventions and is collected in
Appendix B. The thirty axioms are numbered linearly, $\mathrm{A1}$ through
$\mathrm{A30}$, in the order in which they are constructed. The core axioms
$\mathrm{A1}$–$\mathrm{A12}$ lay the foundation; the extended axioms
$\mathrm{A13}$–$\mathrm{A20}$ build on that foundation; and the zero-day axioms
$\mathrm{A21}$–$\mathrm{A30}$ build on both. The numbering is the build order:
each axiom presupposes the ones that precede it and extends them, and a later
chapter may refer to an earlier one but never the reverse. The book is meant to
be read from front to back for exactly this reason.

Each chapter states one axiom, motivates it, gives its formal definition, works
one example with exact arithmetic, records its main consequences, and closes
with a short note on what fails in its absence. The reader who wants only the
definitions can consult Appendix A; the reader who wants only the results can
consult Chapter 32 and the case studies of Part VI.

## How the book came to be written

This manuscript was prepared as a single document describing a framework,
then restructured into chapters for publication. The case studies in Part VI
draw on publicly documented intrusions — FIN7, Conti, the SolarWinds supply
chain compromise, Log4Shell, and a cryptomining exfiltration scenario — and use
them to illustrate the axioms, not to claim novel attribution. Where threat
intelligence is cited, it is public reporting of the kind collected in the
References.

## Acknowledgments

A book of this form is indebted to the long lines of prior work that it
attempts to unify: the taxonomies of adversarial behavior, the management
frameworks for organizational risk, the formal methods for protocol
verification, and the information-theoretic study of detection. These debts are
recorded in Chapter 41 and the References. Any error that remains is the
author's own.
