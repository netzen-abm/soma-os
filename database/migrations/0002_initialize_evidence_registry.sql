-- SOMA Evidence Registry
-- Shared infrastructure for source-derived knowledge, management protocols,
-- research evidence, safety findings and user-verifiable references.
-- IMPORTANT: These tables are for source/scientific metadata. They are not
-- a store for personal health records.

CREATE TABLE IF NOT EXISTS evidence_sources (
    source_id TEXT PRIMARY KEY,
    source_type TEXT NOT NULL,
    canonical_title TEXT NOT NULL,
    publisher TEXT,
    author TEXT,
    language TEXT,
    publication_date DATE,
    version TEXT,
    canonical_url TEXT NOT NULL,
    accessed_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    license_status TEXT,
    provenance_level TEXT,
    notes TEXT
);

CREATE TABLE IF NOT EXISTS evidence_claims (
    claim_id TEXT PRIMARY KEY,
    source_id TEXT NOT NULL REFERENCES evidence_sources(source_id),
    source_location TEXT,
    subject_id TEXT NOT NULL,
    predicate TEXT NOT NULL,
    object_id TEXT,
    object_text TEXT,
    claim_type TEXT NOT NULL,
    paraphrase TEXT NOT NULL,
    reported_by TEXT,
    extracted_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    review_status TEXT NOT NULL DEFAULT 'PENDING'
);

CREATE TABLE IF NOT EXISTS management_protocols (
    protocol_id TEXT PRIMARY KEY,
    source_id TEXT NOT NULL REFERENCES evidence_sources(source_id),
    name TEXT NOT NULL,
    reported_indication TEXT NOT NULL,
    protocol_class TEXT NOT NULL,
    purpose_statement TEXT NOT NULL,
    preparation TEXT,
    administration TEXT,
    schedule TEXT,
    duration TEXT,
    recommendation_status TEXT NOT NULL DEFAULT 'INFORMATIONAL_MANAGEMENT_ONLY',
    review_status TEXT NOT NULL DEFAULT 'PENDING'
);

CREATE TABLE IF NOT EXISTS protocol_claim_links (
    protocol_id TEXT NOT NULL REFERENCES management_protocols(protocol_id) ON DELETE CASCADE,
    claim_id TEXT NOT NULL REFERENCES evidence_claims(claim_id) ON DELETE CASCADE,
    PRIMARY KEY (protocol_id, claim_id)
);

CREATE TABLE IF NOT EXISTS research_evidence (
    research_id TEXT PRIMARY KEY,
    source_id TEXT NOT NULL REFERENCES evidence_sources(source_id),
    title TEXT NOT NULL,
    year INTEGER,
    journal TEXT,
    doi TEXT,
    pubmed_id TEXT,
    source_url TEXT NOT NULL,
    verification_url TEXT NOT NULL,
    study_type TEXT NOT NULL,
    population TEXT,
    sample_size INTEGER,
    intervention TEXT,
    comparator TEXT,
    duration TEXT,
    outcomes TEXT,
    main_findings TEXT NOT NULL,
    limitations TEXT,
    safety_findings TEXT,
    funding TEXT,
    conflicts_of_interest TEXT,
    search_context_id TEXT
);

CREATE TABLE IF NOT EXISTS protocol_research_links (
    protocol_id TEXT NOT NULL REFERENCES management_protocols(protocol_id) ON DELETE CASCADE,
    research_id TEXT NOT NULL REFERENCES research_evidence(research_id) ON DELETE CASCADE,
    PRIMARY KEY (protocol_id, research_id)
);

CREATE TABLE IF NOT EXISTS evidence_assessments (
    assessment_id TEXT PRIMARY KEY,
    protocol_id TEXT NOT NULL REFERENCES management_protocols(protocol_id) ON DELETE CASCADE,
    research_id TEXT REFERENCES research_evidence(research_id) ON DELETE CASCADE,
    directness TEXT NOT NULL,
    study_quality TEXT,
    evidence_consistency TEXT,
    precision TEXT,
    external_validity TEXT,
    safety_certainty TEXT,
    overall_status TEXT NOT NULL,
    supporting_or_contradictory TEXT NOT NULL,
    assessor TEXT,
    assessed_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    rationale TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS safety_records (
    safety_id TEXT PRIMARY KEY,
    subject_id TEXT NOT NULL,
    protocol_id TEXT REFERENCES management_protocols(protocol_id) ON DELETE CASCADE,
    population TEXT,
    contraindications TEXT,
    interactions TEXT,
    adverse_events TEXT,
    dose_context TEXT,
    contamination_risk TEXT,
    identity_risk TEXT,
    regulatory_status TEXT,
    confidence TEXT,
    reviewed_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS evidence_search_contexts (
    search_context_id TEXT PRIMARY KEY,
    searched_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    systems TEXT NOT NULL,
    query_terms TEXT NOT NULL,
    date_range TEXT,
    inclusion_criteria TEXT,
    exclusion_criteria TEXT,
    result_count INTEGER NOT NULL DEFAULT 0,
    notes TEXT
);

CREATE TABLE IF NOT EXISTS verification_references (
    verification_id TEXT PRIMARY KEY,
    research_id TEXT REFERENCES research_evidence(research_id) ON DELETE CASCADE,
    source_id TEXT REFERENCES evidence_sources(source_id) ON DELETE CASCADE,
    label TEXT NOT NULL,
    url TEXT NOT NULL,
    link_type TEXT NOT NULL,
    accessed_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CHECK (research_id IS NOT NULL OR source_id IS NOT NULL)
);

CREATE INDEX IF NOT EXISTS idx_claims_source ON evidence_claims(source_id);
CREATE INDEX IF NOT EXISTS idx_protocols_source ON management_protocols(source_id);
CREATE INDEX IF NOT EXISTS idx_assessments_protocol ON evidence_assessments(protocol_id);
CREATE INDEX IF NOT EXISTS idx_assessments_directness ON evidence_assessments(directness);
CREATE INDEX IF NOT EXISTS idx_research_pubmed ON research_evidence(pubmed_id);

COMMENT ON TABLE evidence_sources IS 'Public/scientific source provenance. Must not contain personal health records.';
COMMENT ON TABLE management_protocols IS 'Source-derived disease-management/supportive protocols; never remedies or cures.';
COMMENT ON TABLE research_evidence IS 'Structured research evidence with original verification references.';
COMMENT ON TABLE evidence_assessments IS 'Explicit directness, quality, consistency and evidence-status assessment.';
