# Chapter 7 — Actions (Axiom A6)

> *Part I — Foundations*

## 7.1 Motivation

Inference is not the end of the pipeline. The defender must act, and to act it
must know what actions exist, when they may be taken, and what they do. This
chapter defines the action space.

## 7.2 Axiom

**Axiom A6 (Action space).** There exists a set A of actions. Each action is
a tuple

$$
a \;=\; (\mathrm{pre}_a,\ \mathrm{eff}_a,\ C_a,\ \ell_a),
$$

where

- the precondition is a function from the state space to true or false,
  true exactly when the action is applicable;
- the effect is a function from the state space to itself, the transition the action induces;
- the cost structure of the action (developed in Chapter 9);
- the autonomy level of the action (developed in Chapter 13).

The set of actions applicable at a state is

$$
A(s) \;=\; \{\, a \in A \mid \mathrm{pre}_a(s) = 1 \,\},
$$

and executing an action carries the state to the state produced by its effect.

## 7.3 Worked example

A SOAR playbook might contain the following actions:

```yaml
actions:
  - name: block_ip
    pre: "ip in c2_list"
    eff: "firewall.block(ip)"
    cost_M: 0
    cost_notM: 500
    autonomy: autonomous

  - name: isolate_host
    pre: "host.online"
    eff: "edr.isolate(host)"
    cost_M: 5000
    cost_notM: 50000
    autonomy: human_approved

  - name: alert
    pre: "true"
    eff: "soc.notify()"
    cost_M: 1
    cost_notM: 5
    autonomy: autonomous
```

At the state

```python
s = {"proc": "powershell", "net": "c2_conn", "file": "ps1"}
```

the applicable set is computed by evaluating each precondition:

```python
applicable = [a for a in actions if a.pre(s)]
# [block_ip, isolate_host, alert]
```

Here `block_ip` applies because `c2_conn` is in the C2 list, `isolate_host`
applies because the host is online, and `alert` applies unconditionally. The
choice among these applicable actions is not made here; it is made by the
cost–utility rule of Chapter 9, subject to the resource constraints of
Chapter 12 and the governance filter of Chapter 13.

## 7.4 Consequences

The action space defines the defender's *capabilities*. A framework that cannot
name its actions cannot reason about response at all; the action space, together with
its preconditions and effects, is the boundary of what the defender can do. The
action space also carries the two dimensions that make response non-trivial:
cost (Chapter 9), which makes the choice among actions a decision problem, and
autonomy (Chapter 13), which makes some actions unavailable without human
approval.

Note that actions and events (Chapter 3) are both functions from the state space to itself. The
difference is not in their type but in their provenance and their cost. An event
is something that *happens* in the system, attributable to the adversary or to
the environment; an action is something the *defender does*, with an associated
cost and an associated level of governance. The same formal mechanism —
composition, choice, iteration — applies to both, which is why a single algebra
can describe playbooks and kill chains alike.

## 7.5 What fails without A6

Without an action space there is no response, no cost, and no governance. The
pipeline of Chapter 1 terminates at inference and the framework becomes
monitoring without response — which, as the name of the framework makes clear,
is only half the subject. Every chapter concerned with what the defender *does*
(Chapters 9, 12, 13, 18, 21, 28, 31) presupposes A6.
