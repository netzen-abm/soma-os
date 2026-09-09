# SOMA OS

A privacy-first, modular infrastructure for health, wellbeing, research,
and evidence-informed health intelligence.

> **Project boundary:** SOMA-OS and Janavani are separate projects. SOMA-OS
> focuses on health and wellbeing, medical-stream integration, health research,
and health-related policy. Janavani is not part of the SOMA-OS product scope.

## Mission

SOMA-OS is intended to make health knowledge, research, data, capabilities,
and governed intelligence more interoperable, traceable, privacy-preserving,
and useful across health domains.

Integration does not mean claiming that all medical systems or interventions
have equal evidentiary support. SOMA preserves evidence quality, uncertainty,
provenance, safety context, and disagreement.

## Core principles

- Privacy and safety are defaults, not optional features.
- Personal and sensitive personal data are not collected by default.
- User data remains on the user's device whenever possible.
- External data transfer is explicit, minimized, and appropriately protected.
- Shared capabilities belong in shared infrastructure first.
- Client surfaces consume shared capabilities rather than duplicate business,
  evidence, privacy, safety, provenance, or policy logic.
- Evidence is multi-source and traceable.
- Research discovery is not clinical diagnosis or treatment.
- Management information is not a promise of cure or a substitute for
  qualified medical care.

## Health scope

SOMA-OS covers:

- health and wellbeing;
- integration and interoperability across medical streams;
- biomedical and health research;
- evidence discovery, verification, and synthesis;
- health-related policy research and analysis;
- privacy-first health data infrastructure;
- governed health intelligence and decision support;
- health provider and research-source adapters.

Potential future applications include evidence/research intelligence,
longitudinal health knowledge, professional decision support, health-policy
intelligence, institutional health intelligence, and governed health agents.

These are application directions, not a commitment to implement everything at
once. The immediate objective is one reliable end-to-end health reference flow.

## Architecture

```text
Health applications / surfaces
              |
              v
      Shared SOMA infrastructure
              |
       +------+------+------+------+------+
       |      |      |      |      |      |
    Identity Policy Evidence Safety Privacy Audit
       |      |      |      |      |      |
       +------+------+------+------+------+
              |
              v
       Provider / research adapters
```

The core capability spine is:

```text
Identity
  -> Capability
  -> Policy
  -> Gateway
  -> Execution
  -> Evidence
  -> Audit
```

AI, agents, MCP, decentralized protocols, mobile surfaces, and other
interfaces are adapters/capabilities around this core. They are not the
product identity.

## Evidence research

SOMA does not depend on a single research database.

Relevant research can be discovered across biomedical literature, systematic
reviews, clinical-trial registries, government and international sources,
Indian research repositories, institutional publications, nutrition and food
databases, traditional-medicine research sources, safety/regulatory sources,
and botanical/species databases.

Evidence records should preserve original source provenance and enough
metadata for independent verification. Duplicate records for the same
underlying study are not independent evidence.

Provider failure is never interpreted as absence of evidence.

Sources must retain their epistemic status. Traditional, historical,
mechanistic, observational, expert, commercial, and self-published material
must not be silently promoted to clinical evidence.

## Health and safety boundary

SOMA may support evidence discovery, education, synthesis, management
information, research, policy analysis, and decision support where appropriate.

It does not diagnose, prescribe, promise cures, or instruct users to replace
prescribed treatment.

High-consequence health use requires stronger safety controls and appropriate
professional oversight.

## Privacy architecture

The default data path is:

```text
User device
    |
    +--> Local data and computation where possible
    |
    +--> Explicitly requested external capability
             |
             +--> Minimized data
             +--> Protected transport
```

Personal health data should not be sent to research providers merely to answer
a research question.

## Shared infrastructure rule

If a capability is reusable across multiple SOMA surfaces, implement its
contract and core behavior in shared infrastructure first unless a documented
reason requires otherwise.

Applications should be thin compositions of capabilities. Authorization,
privacy, evidence, safety, provenance, and audit logic must not be copied into
individual surfaces.

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

The repository is also the institutional memory of SOMA-OS. Important
architecture decisions, audits, security findings, research-model decisions,
and roadmap changes belong in `docs/`.

## Code maintainability

- Prefer small modules and functions.
- Prefer clear names over compressed expressions.
- Keep lines readable.
- Keep security-sensitive operations isolated.
- Keep provider-specific code behind adapter interfaces.
- Test important failure states.
- Do not describe unfinished security as production security.

Rust formatting is enforced through `rustfmt.toml`.

## Development status

SOMA is under active architectural stabilization.

Security-sensitive placeholders are not production-ready until replaced with
appropriate, reviewed designs and dedicated tests.

See `docs/SOMA-OS-PROJECT-MEMORY.md` for the consolidated project history and
`docs/decisions/` for explicit architecture decisions.

## License

SOMA-OS is licensed under the **Apache License 2.0**.

See the [`LICENSE`](LICENSE) file for the complete license terms.
