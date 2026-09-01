# SOMA Audit Scripts

These scripts are intentionally conservative.

## Legacy medical JSON audit

Run:

```text
python scripts/audit_legacy_medical_json.py \
  database/medical-core/preventive_botanicals.json
```

The command is **read-only**. It does not rewrite, normalize, delete, or
publish medical records.

It reports:

- how many JSON documents are concatenated in a legacy file;
- source-like fields that require verification;
- treatment/cure/remedy-style language requiring review.

This is the first stage of migration. A later stage may create canonical
records only after source preservation, validation, safety review, and tests.
