use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

const WEEKLY_INTERVAL_SECS: u64 = 604_800;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BackupMetadataTrackingLog {
    pub anonymized_user_hash: String,
    pub last_successful_backup_epoch_secs: u64,
}

pub struct SovereignBackupScheduler;

impl SovereignBackupScheduler {
    pub fn evaluate_weekly_backup_alert_requirement(
        tracking_profile: &BackupMetadataTrackingLog,
    ) -> bool {
        let current_time_secs = match SystemTime::now()
            .duration_since(UNIX_EPOCH)
        {
            Ok(duration) => duration.as_secs(),
            Err(_) => return false,
        };

        if current_time_secs < tracking_profile.last_successful_backup_epoch_secs {
            return false;
        }

        let elapsed = current_time_secs
            - tracking_profile.last_successful_backup_epoch_secs;

        elapsed >= WEEKLY_INTERVAL_SECS
    }

    pub fn compile_backup_reminder_string() -> String {
        [
            "SOMA local backup reminder.",
            "Your last recorded backup is over 7 days old.",
            "Use the supported local backup flow when available.",
            "Do not assume an encrypted backup exists until",
            "authenticated encryption has been implemented.",
        ]
        .join("\n\n")
    }
}
