use axum::{
    extract::Multipart,
    http::StatusCode,
    Json,
};
use serde::Serialize;

const MAX_UPLOAD_SIZE_BYTES: usize = 10 * 1024 * 1024;
const EXCERPT_CHAR_LIMIT: usize = 200;

#[derive(Serialize)]
pub struct ExtractedReportData {
    pub text_signature_extracted: String,
    pub detected_biomarkers_count: usize,
    pub triage_escalation_required: bool,
}

#[derive(Serialize)]
pub struct ProcessedReportResponse {
    pub status: String,
    pub analysis_payload: Option<ExtractedReportData>,
    pub compliance_notice: String,
}

pub async fn handle_lab_report_upload(
    mut multipart_payload: Multipart,
) -> (StatusCode, Json<ProcessedReportResponse>) {
    let raw_text = match extract_report_bytes(&mut multipart_payload).await {
        Ok(Some(bytes)) => String::from_utf8_lossy(&bytes).into_owned(),
        Ok(None) => {
            return error_response(
                StatusCode::BAD_REQUEST,
                "MALFORMED_OR_EMPTY_PAYLOAD",
                "No supported report file was provided.",
            );
        }
        Err(UploadError::TooLarge) => {
            return error_response(
                StatusCode::PAYLOAD_TOO_LARGE,
                "REJECTED_FILE_TOO_LARGE",
                "Upload exceeds the maximum payload size.",
            );
        }
        Err(UploadError::Multipart) => {
            return error_response(
                StatusCode::BAD_REQUEST,
                "MALFORMED_MULTIPART_PAYLOAD",
                "The report upload could not be read.",
            );
        }
    };

    let analysis = analyze_report_text(&raw_text);

    (
        StatusCode::OK,
        Json(ProcessedReportResponse {
            status: "ANALYSIS_COMPLETE".to_string(),
            analysis_payload: Some(analysis),
            compliance_notice: "Report content was processed locally by this endpoint."
                .to_string(),
        }),
    )
}

async fn extract_report_bytes(
    multipart_payload: &mut Multipart,
) -> Result<Option<Vec<u8>>, UploadError> {
    while let Some(mut field) = multipart_payload
        .next_field()
        .await
        .map_err(|_| UploadError::Multipart)?
    {
        if field.name() != Some("lab_report_pdf") {
            continue;
        }

        let mut bytes = Vec::new();

        while let Some(chunk) = field
            .chunk()
            .await
            .map_err(|_| UploadError::Multipart)?
        {
            bytes.extend_from_slice(&chunk);

            if bytes.len() > MAX_UPLOAD_SIZE_BYTES {
                return Err(UploadError::TooLarge);
            }
        }

        return Ok(Some(bytes));
    }

    Ok(None)
}

fn analyze_report_text(raw_text: &str) -> ExtractedReportData {
    let lower_text = raw_text.to_lowercase();
    let contains_indicators = lower_text.contains("alkaline phosphatase")
        || lower_text.contains("osteolytic");

    ExtractedReportData {
        text_signature_extracted: raw_text.chars().take(EXCERPT_CHAR_LIMIT).collect(),
        detected_biomarkers_count: usize::from(contains_indicators),
        triage_escalation_required: contains_indicators,
    }
}

fn error_response(
    status: StatusCode,
    code: &str,
    notice: &str,
) -> (StatusCode, Json<ProcessedReportResponse>) {
    (
        status,
        Json(ProcessedReportResponse {
            status: code.to_string(),
            analysis_payload: None,
            compliance_notice: notice.to_string(),
        }),
    )
}

#[derive(Debug)]
enum UploadError {
    TooLarge,
    Multipart,
}
