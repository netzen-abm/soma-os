// File Path: services/backend-rust/src/backup_scheduler.rs

use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BackupMetadataTrackingLog {
    pub anonymized_user_hash: String,
    pub last_successful_backup_epoch_secs: u64,
}

pub struct SovereignBackupScheduler;

impl SovereignBackupScheduler {
    /// Inspects tracking log timestamps and checks if a weekly physical hardware backup notification is due
    pub fn evaluate_weekly_backup_alert_requirement(
        tracking_profile: &BackupMetadataTrackingLog,
    ) -> bool {
        let current_time_secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Define a strict 7-day conversion threshold configuration setting (7 days * 24h * 3600s)
        let weekly_interval_secs: u64 = 604_800;

        // Prevent boundary calculation errors across future system date handshakes
        if current_time_secs < tracking_profile.last_successful_backup_epoch_secs {
            return false; 
        }

        let time_delta_since_last_backup = current_time_secs - tracking_profile.last_successful_backup_epoch_secs;

        if time_delta_since_last_backup >= weekly_interval_secs {
            println!(
                "⚠️ Privacy Shield Notice: User profile [{}] has passed the safe data retention window threshold without an offline backup update.",
                tracking_profile.anonymized_user_hash
            );
            return true; // System alert must be launched immediately
        }

        false // User remains inside a safe data validation tier window
    }

    /// Assembles a multi-channel message string layout reminding the user to backup data offline
    pub fn compile_backup_reminder_string() -> String {
        format!(
            "🪐 *SomaOS Sovereignty Reminder* 🪐\n\n\
            It has been over *7 days* since your last physical health vault save operation.\n\n\
            Remember: SomaOS enforces absolute *by-design user privacy*, meaning zero central copies of your medical records or Dr. Khadar Vali millet protocols exist on our servers.\n\n\
            To secure your data history, please connect an external USB drive or flash disk, navigate to your dashboard panel, and click *Export Secure Local Vault* to save an encrypted backup archive package offline."
        )
    }
}
