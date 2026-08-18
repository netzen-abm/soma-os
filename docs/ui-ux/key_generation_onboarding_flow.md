┌──────────────────────────────────────────────┐│    SCREEN 1: SOVEREIGN RE-POSITIONING HUB    │└──────────────────────┬───────────────────────┘│▼ [Action: Hit "Generate Keys"]┌──────────────────────────────────────────────┐│   SCREEN 2: LOCAL ENTROPY CALCULATOR LOOP    │└──────────────────────┬───────────────────────┘│▼ [Action: Auto-derive Seed]┌──────────────────────────────────────────────┐│    SCREEN 3: THE PRIVATE KEY BACKUP VAULT    │└──────────────────────────────────────────────┘
---

## 📱 SCREEN 1: Sovereign Re-Positioning Hub (The Autonomy Screen)
*Render Condition: Initial cold initialization. Device contains zero local state parameters.*

### 🛠️ UI Elements & Layout Mapping
- **Central Card Container:** Background variable: `#1C2541` (Surface). Border mapping: 1px width color: `#5BC0BE` (Cyber Jade).
- **Core Educational Prompt Title:** `Take Absolute Ownership of Your Health History` (White bold typography font-scale, 20px).
- **Explanatory Body Description:** `SomaOS operates with no central user directory databases. Your data is owned entirely by you, locked under cryptographically secure keys generated directly on this smartphone device.` (Muted slate, 13px line-height: 1.5).

### ⚡ User Interactions & Core Transitions
- **Button [GENERATE PRIVATE HEALTH KEYPAIR]:** Primary background color `#5BC0BE`. Padding: 16px.
- **Trigger Event:** Clicking this button initiates a transition to Screen 2.

---

## 📱 SCREEN 2: Local Entropy Calculator Loop (The Generation Screen)
*Render Condition: Execution of keygen loop confirmation parameters.*

### 🛠️ UI Elements & Layout Mapping
- **Full Frame Canvas Void:** Background color: `#0B132B`.
- **Active Processing Indicator:** Centered, pulsing neon loop widget graphic. Color: `#6FFFE9` (Biometric Neon).
- **Instruction String Text:** `Calculating localized device entropy matrices... Please wait while your unique cryptographic seed is generated.` (Muted violet text, 12px, centered layout).

### ⚡ Technical Execution Under the Hood
- **Low-Level Code Operations:**
  1. Frontend invokes `ring::rand::SystemRandom` to capture 32 bytes of secure system noise.
  2. Passes data into BIP-39 mnemonic conversion functions to build a 12-word recovery phrase.
  3. Uses standard `ed25519` seed derivation profiles to build the user's public and private key signatures.
  4. Automatically writes secret values directly to local secure hardware enclaves (**iOS Keychain / Android Keystore**).

---

## 📱 SCREEN 3: The Private Key Backup Vault (The Protection Screen)
*Render Condition: Successful local cryptographic generation completion.*

### 🛠️ UI Elements & Layout Mapping
- **Warning Title Element:** `🚨 Safeguard Your Master Recovery Key` (Warning Crimson text color `#D9534F`, bold).
- **Mnemonic Recovery Box Display Frame:** Background color: `#111A33`. Heavy padded bounding container.
  - Displays the 12 generated recovery words in an individual grid block. Font: Monospace style, bold text color: `#6FFFE9`.
- **Checkbox Explicit Commitment Component:** `[ ] I certify that I have copied these 12 words down offline. If I lose this recovery seed phrase, my medical history cannot be recovered by anyone.` (White text, 11px font scale).

### ⚡ User Interactions & Core Transitions
- **Primary Operational Action Button [ENTER APPLICATION]:** Background changes from gray to solid Cyber Jade once the explicit commitment checkbox is checked.
- **Trigger Event:** Clicking this button unlocks the main app cockpit layout (`ap