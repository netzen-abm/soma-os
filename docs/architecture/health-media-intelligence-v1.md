# SOMA Health Media Intelligence v1

**Status:** Architecture contract / design baseline  
**Date:** 2026-09-08

## 1. Purpose

SOMA Health Media Intelligence is a governed discovery, organization, playback, transcript, claim-extraction, and evidence-linking capability for publicly accessible health-related media and educational material.

The purpose is not to become a generic video platform. SOMA should give users a compelling reason to install and return to SOMA by turning a fragmented health-media landscape into a privacy-first, health-focused intelligence layer.

Primary content domains include:

- health and wellness
- nutrition and food
- medical and biomedical research
- clinical education
- public-health information
- traditional, complementary, and integrative medicine
- health interviews and lectures
- podcasts and long-form discussions
- patient and lived-experience content
- emerging and unconventional health claims

## 2. Product principle

> SOMA does not merely collect health content. SOMA helps people understand what the content claims, where it came from, what evidence supports or contradicts it, who it may apply to, and what remains uncertain.

The system must remain **medical-paradigm-neutral but evidence-strict**.

No source receives automatic epistemic authority because it is popular, conventional, alternative, institutional, independent, suppressed, or controversial.

## 3. Why media belongs in SOMA

Health applications commonly lose users when they do not provide continuing usefulness. Research on mHealth engagement identifies perceived usefulness, accuracy, transparency, support, feedback, and tailored content as important factors in uptake and retention.

Media creates a recurring user utility loop without turning SOMA into an attention-maximization platform:

`Discover → Watch/Listen → Understand → Verify → Save → Apply safely → Measure → Revisit`

The long-term SOMA value is the layer around the media:

`Media → Claims → Evidence → Context → Personal relevance → Action → Outcome → Learning`

## 4. User value proposition

A user should have reasons to return to SOMA even when they are not entering health measurements.

### Daily utility

- personalized health-media discovery
- new research and educational material
- topic/channel/source subscriptions
- daily or weekly health intelligence briefings
- saved watch/listen queues
- cross-source search
- transcripts and summaries where permitted
- claim-level evidence links
- contradiction and uncertainty indicators
- topic timelines
- follow a condition, intervention, researcher, practitioner, knowledge system, or research question

### Personal utility

With explicit user choice and minimum necessary data, SOMA may connect media topics to the user's own health context.

Example:

> "You saved 14 items about insulin resistance. Here are the recurring claims, the evidence supporting them, important contradictory findings, and which claims are relevant to your stated goal."

Personal health context must never be silently exposed to external media providers.

## 5. Source model

SOMA uses adapters rather than hard-coded assumptions about individual platforms.

Potential source adapters include:

- YouTube
- Vimeo
- PeerTube
- X
- Facebook public/page surfaces where permitted
- Rumble
- Odysee
- BitChute
- Brighteon
- podcast/RSS feeds
- institutional video repositories
- university/research repositories
- public-health organizations
- medical journals and research publishers
- independent research organizations
- Padaav/VCPCRF and similar research archives

A source may support some capabilities but not others.

Example capability matrix:

| Capability | Meaning |
|---|---|
| SEARCH | permitted discovery/search |
| METADATA | title, creator, date, category, etc. |
| EMBED | permitted embedded playback |
| PLAYBACK | permitted source playback |
| TRANSCRIPT | source-provided or permitted transcript |
| INDEX | SOMA may retain searchable metadata/index data |
| DOWNLOAD | media download is legally/technically permitted |
| OFFLINE_REFERENCE | SOMA can retain a user reference for later access |

`DOWNLOAD` must never be assumed merely because a video is publicly viewable.

## 6. Public access is not public domain

SOMA must distinguish:

- public URL
- publicly viewable
- embeddable
- licensed for reuse
- Creative Commons/open license
- public domain
- downloadable under provider terms
- SOMA-local storage permitted

SOMA should prefer source-native playback or embedding where appropriate.

Where local offline storage is permitted, SOMA may store the permitted media representation. Otherwise SOMA should store a reference, metadata, permitted transcript, user notes, and evidence graph links rather than copying the media.

## 7. Media-to-evidence pipeline

```text
Source
  ↓
Source Adapter
  ↓
Integrity / Access / License Metadata
  ↓
Media Normalization
  ↓
Health Relevance Classification
  ↓
Transcript / Audio / Visual Extraction (where permitted)
  ↓
Claim Extraction
  ↓
Knowledge-System Identification
  ↓
Research Method Identification
  ↓
Evidence Retrieval
  ↓
Contradiction / Negative Evidence Search
  ↓
Safety Assessment
  ↓
Uncertainty / Applicability
  ↓
Health Evidence Graph
  ↓
Evidence Passport + User Experience
```

## 8. Content is not evidence

Every media item must preserve the distinction:

`Source → Content → Claim → Evidence → Interpretation → Recommendation`

Examples:

- a patient testimonial is lived experience, not controlled efficacy evidence
- an interview with a clinician is expert commentary, not automatically clinical evidence
- a research lecture may contain evidence, interpretation, and speculation simultaneously
- a traditional medical teaching may represent a coherent knowledge system without proving every underlying causal claim
- a viral claim remains a claim until independently assessed

## 9. Health Media Passport

Each indexed item should be capable of exposing:

- source/platform
- canonical source URL
- creator/channel
- publication date
- language
- duration
- media type
- license/access status
- health topics
- knowledge system
- claims detected
- research references
- evidence status
- safety flags
- contradictory evidence
- uncertainty
- applicability
- provenance
- transcript provenance
- processing version
- last reviewed

The passport is a view over canonical SOMA records, not a second evidence source of truth.

## 10. User controls

The user should be able to select discovery modes such as:

- All health media
- Evidence-first
- Research only
- Clinical education
- Nutrition
- Wellness
- Traditional/complementary medicine
- Emerging research
- Patient experiences
- Podcasts/interviews
- Courses/lectures
- Saved sources

These are discovery preferences, not evidence rankings.

SOMA must not silently suppress a source merely because it is outside a dominant medical paradigm. It should expose epistemic status and evidence limitations.

## 11. Recommendation engine

Recommendations must not optimize only for watch time.

The ranking model should consider:

1. health-topic relevance
2. user-selected interests
3. source provenance
4. recency where meaningful
5. evidence relevance
6. novelty/diversity
7. contradiction coverage
8. source diversity
9. knowledge-system diversity when requested
10. safety signals
11. user feedback
12. duplicate/near-duplicate suppression

Watch time may be a product metric, but it must not become the primary health-information objective.

## 12. A better reason to download SOMA

The strongest product proposition is not:

> "SOMA lets you watch videos."

It is:

> **"SOMA is your health intelligence library: discover the world's health knowledge and media, understand what it actually claims, verify it against evidence, save what matters, and connect learning to your own health journey."**

This creates a durable utility loop:

`Discover → Understand → Verify → Save → Act → Measure → Learn`

Media is therefore an acquisition and retention surface; evidence, personal context, safety, provenance, and learning are the moat.

## 13. Offline / download strategy

Offline functionality should be a first-class user benefit, but it must be rights-aware.

### Level A — Offline reference

Always preferred when media copying is not permitted:

- source URL
- title/creator metadata
- permitted thumbnail/reference
- transcript if permitted
- SOMA summary generated from permitted content
- claims
- evidence links
- notes
- tags
- last-seen state

### Level B — Offline permitted media

For media explicitly licensed or otherwise permitted for local storage:

- encrypted local media cache
- transcript
- metadata
- provenance
- license record
- expiration/revocation handling

### Level C — Source-native offline

If the provider controls offline playback, SOMA should deep-link the user to the provider's native offline feature rather than circumventing provider controls.

## 14. Privacy boundary

Media discovery must not leak protected health information to source providers by default.

Bad design:

`User health record → external media platform search`

Preferred design:

`Local/SOMA health context → local topic abstraction → governed media search → minimal query`

For example, a user's diagnosis or lab result should not be sent to a media provider merely because SOMA is recommending educational material.

## 15. Safety boundary

SOMA may discover, organize, summarize, compare, and contextualize health content.

It must not transform media into an automatic diagnosis, prescription, or guarantee of cure.

High-consequence claims require stronger evidence, safety assessment, provenance, and human oversight.

## 16. Special treatment of unconventional claims

Unusual or controversial health claims should be discoverable when lawful and relevant, but clearly classified.

SOMA should support:

- `UNVERIFIED`
- `PRELIMINARY`
- `SUPPORTED`
- `MIXED`
- `CONTRADICTED`
- `SPECULATIVE`
- `SAFETY_RESTRICTED`

The goal is neither automatic endorsement nor automatic dismissal.

> Investigate without prejudice. Conclude according to evidence.

## 17. Initial source strategy

Do not launch with every platform.

### Tier 1 — first implementation targets

- YouTube
- Vimeo
- PeerTube
- podcast/RSS
- research/institutional video sources

### Tier 2 — governed discovery adapters

- X
- Brighteon
- BitChute
- Rumble
- Odysee
- Facebook public surfaces

### Tier 3 — specialist sources

- Padaav/VCPCRF
- university repositories
- public-health media
- practitioner education archives
- additional federated/local platforms

The source registry must allow adapters to be enabled/disabled without changing the canonical health model.

## 18. Non-goals

v1 is not:

- a replacement for YouTube
- a social network
- a generic entertainment recommendation engine
- an unrestricted video downloader
- an autonomous medical advisor
- an authority-ranking system that assumes one medical paradigm is always correct
- a repository for copying all public media

## 19. Architectural rule

Media is an application capability around the canonical SOMA domain.

The dependency direction is:

`Media → Health Intelligence → Evidence Graph`

not:

`Health Intelligence → YouTube`

No single media provider may become a core dependency or a health-data trust boundary.
