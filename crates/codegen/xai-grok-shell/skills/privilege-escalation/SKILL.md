---
name: privilege-escalation
description: Privilege escalation methodology across Linux, Windows, containers, CI runners, and cloud IAM for authorized engagements.
---

# Privilege Escalation Playbook

## Universal

1. Who am I / what can I already do?
2. Local recon — users, groups, sudo/privileges, scheduled tasks, services, sockets.
3. Interesting files — configs, histories, keys, tokens, backups.
4. Misconfigs — writable services, PATH hijacks, capabilities, weak ACLs.
5. Kernel/container/cloud-specific paths only after cheaper wins fail.
6. Prove escalation with a clear before/after evidence artifact.

## Linux

- `id`, `sudo -l`, SUID/SGID, capabilities, systemd units, cron, docker.sock, writable mounts.

## Windows

- Token privileges, service ACLs, unquoted paths, AlwaysInstallElevated, stored creds, UAC bypass only when needed.

## Cloud / K8s / CI

- Instance metadata, role assumption chains, pod SA tokens, node escape, pipeline secret inheritance.

Save proofs under `.grok/redteam/artifacts/privesc/`.
