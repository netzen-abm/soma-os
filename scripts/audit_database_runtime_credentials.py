#!/usr/bin/env python3
"""Fail-closed audit for database runtime credential separation."""

from __future__ import annotations

import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
COMPOSE = ROOT / "docker-compose.yml"

FORBIDDEN_RUNTIME_ROLES = {
    "somaos_vault_admin",
    "somaos_migrator",
    "somaos_legacy_promotion_owner",
    "somaos_legacy_promotion_executor",
    "somaos_legacy_preflight_owner",
}


def main() -> int:
    if not COMPOSE.exists():
        print("FINDING: docker-compose.yml is missing")
        return 1

    text = COMPOSE.read_text(encoding="utf-8", errors="replace")
    findings: list[str] = []

    kernel_match = re.search(
        r"(?ms)^  somaos-kernel:\\n.*?^  showcase-brochure:",
        text,
    )
    if not kernel_match:
        findings.append("docker-compose.yml: somaos-kernel service could not be located")
    else:
        kernel = kernel_match.group(0)
        if "DATABASE_URL=${SOMA_APP_DATABASE_URL:?SOMA_APP_DATABASE_URL must be set}" not in kernel:
            findings.append(
                "docker-compose.yml: somaos-kernel must consume the separately provisioned SOMA_APP_DATABASE_URL"
            )
        for role in FORBIDDEN_RUNTIME_ROLES:
            if role in kernel:
                findings.append(
                    f"docker-compose.yml: forbidden privileged database role appears in application runtime service: {role}"
                )
        if re.search(r"DATABASE_URL=.*postgres(?:ql)?://[^\\s}]+:[^\\s}@]+@", kernel):
            findings.append(
                "docker-compose.yml: application DATABASE_URL contains a literal password"
            )

    db_match = re.search(r"(?ms)^  somaos-db:\\n.*?^  somaos-kernel:", text)
    if not db_match:
        findings.append("docker-compose.yml: somaos-db service could not be located")
    else:
        db = db_match.group(0)
        if "POSTGRES_PASSWORD: ${SOMA_DB_BOOTSTRAP_PASSWORD:?SOMA_DB_BOOTSTRAP_PASSWORD must be set}" not in db:
            findings.append(
                "docker-compose.yml: database bootstrap password must be externally supplied"
            )
        if "secret_zk_infrastructure_password_9988" in db:
            findings.append(
                "docker-compose.yml: committed database bootstrap password detected"
            )

    if findings:
        print("Database runtime credential separation audit")
        print("FINDINGS:")
        for finding in findings:
            print(f"- {finding}")
        return 1

    print("PASS: application runtime uses a separately provisioned database credential.")
    print("PASS: privileged/migration database roles are not wired into the application container.")
    print("PASS: database bootstrap password is not committed.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
