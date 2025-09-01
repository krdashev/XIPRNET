# Security Policy

## Supported Versions
- `main` branch: actively supported
- Release tags: supported for 6 months

## Reporting a Vulnerability
Please report security issues **privately** via email: security@xiprnet.org or via discord: N_.13
Do **not** open a public GitHub issue.  

Reports should include:
- Affected version/commit
- Steps to reproduce
- Proof-of-concept exploit (if possible)
- Impact assessment (what data/keys are at risk)

We will acknowledge within 72h, triage within 7 days, and target a fix within 30 days.  

## Scope
- Core Rust crypto (`/core/`)
- Server components (`/server/`)
- Mobile/desktop clients (`/clients/`)

## Out of Scope
- Third-party dependencies (report upstream)
- Forked or modified builds
