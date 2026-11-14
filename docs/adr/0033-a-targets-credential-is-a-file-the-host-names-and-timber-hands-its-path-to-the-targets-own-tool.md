---
number: 33
title: A target's credential is a file the host names, and timber hands its path to the target's own tool
status: accepted
date: 2026-11-14
decision-makers:
- boomdrive-design
---

# A target's credential is a file the host names, and timber hands its path to the target's own tool

Status: accepted, boomdrive-design's own choice, made on the rules written and cheap to change.

Deciders: boomdrive-design, on afterthought's question of 2026-11-14, carried by boomdrive-planner, of how timber's Dash0 publish takes a token file.
Sources: `boomdrivedev/blueprints:design/timber/requirements.md` 45, 64, 92, 118 and C.12, on branch `design/timber-deploys-dash0-dashboards`; record 0013; rule core.21; afterthought/swancloud's record 0062 as amended on 2026-11-14; the README of Dash0's CLI (`dash0hq/dash0-cli`), read 2026-11-14.

## Context and problem statement

A repository of Dash0 dashboards, one Perses file each with a fixed identifier, is to be deployed by timber from its main branch, through Dash0's own CLI. That is the first target whose tool needs a secret timber must arrange for. The Nix cache's tool and the fleet's deploy tool each sign in on their own on the host.

The statement already settles who holds what:

- The repository binds its targets, and the operator binds the host (92).
- A reconciler acts with the environment's own credentials in its own place (45).
- timber keeps no credential, reaching it at the moment of use (64).
- The host's file names a key's path and never holds a key (record 0013).

It doesn't settle how a target's tool is given a token, or where the target's address and its dataset are named.

Dash0's CLI takes each setting from a flag first, then a `DASH0_*` variable, then a profile under `~/.dash0`. Its `--auth-token-file` reads the token from a file that holds only the token. The same host runs the person's own `dash0`, with a profile and a read-only token of its own.

## Considered options

**The token.**

1. **A file the host's bindings name, handed to the tool by its path (chosen).** The environment's target table names a credential by a name of its own. The host's file maps that name to a file. timber passes the path to the tool, Dash0's `--auth-token-file`, so the tool reads the token and timber reads none of it.
2. **An environment file the publish loads.** Not taken: every variable in it reaches every command the publish runs.
3. **The tool's own profile on the host.** Not taken: it shares `~/.dash0` with the person's own `dash0` and its read-only token, and the CLI rewrites that file as it signs in.
4. **A path in the repository's file.** Not taken: it ties the repository to one host's layout, and puts the host's secrets' whereabouts in the repository.

**Where the dashboards go.**

1. **In the environment's target table, in the repository (chosen).** The API's address and the dataset are each environment's place, as a Nix cache's `<server>:<cache>` is. They are not secret, and they are reviewed with the dashboards they receive.
2. **In files of the host's.** Not taken: the same repository would deploy somewhere else on each host. The collector's OTLP address is also not the API's address, which `apply` needs.

**What a version removes.**

1. **What an earlier version applied and this one doesn't hold (chosen).** A dashboard someone made in Dash0 by hand is never timber's to remove.
2. **Everything in the dataset that the version doesn't hold.** Not taken, for that reason.

Dash0's Terraform provider was not chosen, by afterthought's record 0062 as amended.

## Decision outcome

Option 1 of each, written as C.12's row for Dash0, as 118's credentials, and as rule core.21.

- The host's file gains `[credentials]`, each key a credential's name and its value the absolute path of a file that holds only that credential.
- The environment's `dash0` table names `api`, the API's address, `dataset`, the dataset's identifier, and `credential`, the name of the host's credential.
- Each run of the tool gets every setting it needs as a flag, so no `DASH0_*` variable and no profile of the host's is read. Its configuration folder is one of the run's own, empty, which no other run of `dash0` shares.
- An environment whose credential the host's file doesn't name is unread, naming the credential and the file, and nothing is applied.

### Consequences

- Good: no part of timber holds the token, and the person's own `dash0`, with its read-only token, never meets the publish's.
- Good: a later target whose tool reads a token from a file takes the same table, a package registry's among them.
- Good: a host on agenix or sops points `[credentials]` at the decrypted file and writes nothing else.
- Bad: a target whose tool takes a token only in a variable or on its command line needs its own way to be handed one. Such a target reads the file at the moment of use and passes the token to that one run alone (64).
