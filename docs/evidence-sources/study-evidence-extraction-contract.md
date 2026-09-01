# SOMA Structured Study-Evidence Extraction Contract

## Objective

Create a structured representation of what a source actually reports before
any claim is assessed for clinical support.

## Required study fields

- `source_id`
- `study_design`
- `population`
- `sample_size`
- `intervention_or_exposure`
- `comparator`
- `duration_or_follow_up`
- `primary_outcomes`
- `reported_effects`
- `adverse_events_or_safety`
- `authors_conclusion`
- `limitations`
- `extraction_status`

## Extraction states

- `NOT_EXTRACTED`
- `PARTIALLY_EXTRACTED`
- `EXTRACTED`
- `REQUIRES_SOURCE_REVIEW`

## Evidence boundary

Extraction records what the source reports. Extraction does not determine
whether the source is credible, whether the study is at low risk of bias, or
whether its findings support a SOMA management claim.

## Claim-support assessment

A later governed assessment should compare:

1. the exact SOMA claim;
2. study population;
3. intervention/exposure;
4. comparator;
5. measured outcome;
6. reported direction and magnitude of effect;
7. study design;
8. limitations and risk of bias;
9. safety findings.

A source may be relevant but still fail to support the claim.

## Missing information

Unknown fields must remain `null` or explicitly marked unknown. The extractor
must not infer sample size, effect size, population, outcome, or conclusion
from a title alone.

## AI boundary

AI may propose structured extraction with confidence and source spans. It must
not silently convert proposed extraction into verified evidence.

Human or governed review is required before an extraction can be used for a
published evidence assessment.

## Reproducibility

Each extracted field should eventually carry a source location such as page,
section, table, figure, paragraph, or stable document span where the source
format permits it.

This allows a reviewer to verify the extracted information against the
original source rather than trusting an automated summary.
