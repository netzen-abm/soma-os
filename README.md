# SOMA OS

A privacy-first, modular infrastructure for evidence-informed,
life-oriented systems.

## Core principles

- Privacy and safety are defaults, not optional features.
- Personal and sensitive personal data are not collected by default.
- User data remains on the user's device whenever possible.
- Any required external data transfer must be explicit, minimized,
  and protected by appropriate encryption.
- Shared capabilities belong in shared infrastructure first.
- Client surfaces must consume shared capabilities rather than duplicate
  business, evidence, privacy, or safety logic.
- Evidence is multi-source and traceable.
- Research discovery is not clinical diagnosis or treatment.
- Management information must not be presented as a cure or remedy.

## Architecture

```text
Client surfaces
      |
      v
Shared SOMA infrastructure
      |
      +-- Evidence research
      +-- Privacy and data policy
      +-- Safety policy
      +-- Protocol framework
      +-- Identity and access
      +-- Verification
      |
      v
Provider adapters and external sources
```

## Evidence research

SOMA does not depend on a single research database.

Relevant research can be discovered across:

- biomedical literature;
- systematic reviews;
- clinical-trial registries;
- Indian research repositories;
- international and global repositories;
- food and nutrition databases;
- traditional-medicine research sources;
- safety and regulatory sources;
- botanical and species databases.

Every evidence record should preserve source provenance and a
user-verifiable original source whenever available.

Multiple database records for the same underlying study are not counted
as independent evidence.

Provider failure is never interpreted as absence of evidence.

## Health and management boundary

SOMA may present evidence-informed management information where the
underlying evidence and safety context justify doing so.

It does not diagnose, prescribe, promise cures, or instruct users to
replace prescribed treatment.

Users should consult an appropriately qualified professional for
individual medical decisions.

## Privacy architecture

The default data path is:

```text
User device
    |
    +--> Local data and computation
    |
    +--> Explicitly requested external capability
             |
             +--> Minimized data
             +--> Protected transport
```

Personal data should not be sent to research providers merely to answer a
research question.

## Repository structure

```text
apps/
services/
database/
health-vault/
protocols/
legal-shields/
public/
ops/
scripts/
docs/
.github/
```

The repository is being consolidated around shared infrastructure.
Obsolete material is archived before removal.

## Code maintainability

Code should remain easy to inspect and maintain.

- Prefer small modules and functions.
- Prefer clear names over compressed expressions.
- Avoid unnecessarily long lines.
- Keep security-sensitive operations isolated.
- Keep provider-specific code behind adapter interfaces.
- Add tests for important failure states.
- Do not describe unfinished security as production security.

Rust formatting is enforced through `rustfmt.toml`.

## Development status

SOMA is under active architectural stabilization.

Security-sensitive placeholder implementations are not considered
production-ready until they use an appropriate, reviewed cryptographic
design and pass dedicated tests.

## License

License and contribution terms will be finalized as the repository
stabilizes.
