# SomaOS Telegram WebApp Front-End UI Layout Blueprint
# File Path: docs/ui-ux/telegram_webapp_layout_map.md

## 🎨 Global Design System Variables
- **Background Color (Dark Space Core):** #0B132B
- **Surface Level Color (Card Elements):** #1C2541
- **Primary Branding Color (Cyber Jade):** #5BC0BE
- **Accent Highlighting (Biometric Neon):** #6FFFE9
- **Typography Base:** System-UI / Inter Font Stack, text alignment left-justified.

---

## 📱 SCREEN FRAME 1: The Secure Gateway Landing (The Initialization View)
*Render Condition: First-time application initialization within Telegram WebView frame context.*

### [Component 1.1] Header Branding Block
- **Layout Element:** Top container, fixed position height: 72px.
- **Visuals:** Left-aligned Title text `🪐 SomaOS` (White bold, 24px). Sub-label text `Sovereign Network Architecture` (Cyber Jade, 11px).
- **Interactions:** None (Static).

### [Component 1.2] Mixed Privacy Status Banner
- **Layout Element:** Centered horizontal notification banner box directly under header.
- **Visuals:** Background: #111A33. Thin outline border tracking: #5BC0BE (30% opacity).
- **Embedded Text String:** `🔒 Metadata Blinding via NYM Mixnet Layer Activated` (Neon accent color, 12px, centered).

### [Component 1.3] Initialization Interactive CTA Button
- **Layout Element:** Bottom sticky action row box layout.
- **Visuals:** Fixed primary colored solid box block padding: 16px. Background: #5BC0BE. Text: `GENERATE SECURE LOCAL VAULT` (Deep Space Dark #0B132B, 16px font-weight: 700).
- **Interactions (OnClick Trigger):**
  1. Trigger loading spinner interface state.
  2. Fire `crypto::SovereignCryptoEngine::sign_health_milestone()`.
  3. Seamless transition animation path to Screen Frame 2 (Dashboard Context).

---

## 📱 SCREEN FRAME 2: The Core Vitality Cockpit (The Analytical View)
*Render Condition: Post-authorization phase, state verified.*

### [Component 2.1] Interactive Vitality Hero Card
- **Layout Element:** High-contrast centered display box block.
- **Visuals:** Linear gradient matrix overlay (#1C2541 to #111A33). Large score readout: `88` (Neon Accent #6FFFE9, 64px tracking: -0.02em).
- **Subtext Display String:** `Salud Protocol V1 Compliant Analytics Profile` (#A5A1AA, 12px).
- **Interactions (OnClick Trigger):** Expands detailed time-series graph panel showing historical score progression over a rolling 30-day window.

### [Component 2.2] Split Biometric Twin-Grid Mappings
- **Layout Element:** Multi-column flex layout container width: 100%.
- **Left Column Box [Heart Rate Variability Monitor]:**
  - **Visuals:** Title: `HRV Base` (#A5A1AA, 11px). Live readout value: `64 ms` (White, 20px).
- **Right Column Box [ZKP Asset Status Verification Indicator]:**
  - **Visuals:** Title: `Anonymous Proof` (#A5A1AA, 11px). Status code badge string: `0x7f3B...Verified` (Cyber Jade #5BC0BE, 14px).

### [Component 2.3] Predictive Traditional Bot Intervention Dialogue Block
- **Layout Element:** Full width container item bottom margin: 24px.
- **Visuals:** Soft colored translucent notification box styling. Background: rgba(91, 192, 190, 0.1). Left structural indicator icon bar: 4px width accent element line color: #5BC0BE.
- **Dynamic Text Data Payload String:** `💡 Systemic recovery indicator stable. Recommended Botanical Strategy: Ashwagandha extract formulations to stabilize systemic cortisol.` (White/Grey text mix, 13px line-height: 1.4).

### [Component 2.4] Footer Structural Action Group
- **Layout Element:** Bottom aligned button tray layout matrix.
- **Button Left [Download Medical Report]:** Outlined box style. Triggers `generate_reports.py` execution sequence outputting non-custodial PDF documents.
- **Button Right [Mesh Sync]:** Secondary border styling. Blinks active state indicators if local Reticulum mesh radio connectivity modules are operational nearby.
