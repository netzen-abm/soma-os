import os
import json
from reportlab.lib.pagesizes import letter
from reportlab.platypus import SimpleDocTemplate, Paragraph, Spacer
from reportlab.lib.styles import getSampleStyleSheet, ParagraphStyle
from reportlab.lib import colors

def create_sovereign_pdf_package(json_data_path, output_pdf_path):
    # Ensure source directories exist securely
    if not os.path.exists(json_data_path):
        raise FileNotFoundError(f"Missing essential source data mapping: {json_data_path}")

    # Explicit JSON Parse File Read
    with open(json_data_path, 'r') as file:
        health_data = json.load(file)

    # Document Layout Setup
    doc = SimpleDocTemplate(
        output_pdf_path,
        pagesize=letter,
        rightMargin=36, leftMargin=36, topMargin=36, bottomMargin=36
    )
    
    styles = getSampleStyleSheet()
    story = []

    # Custom Core Design Typography Templates
    title_style = ParagraphStyle(
        'DocTitle',
        parent=styles['Heading1'],
        fontName='Helvetica-Bold',
        fontSize=24,
        textColor=colors.HexColor('#0B132B'),
        spaceAfter=12
    )
    
    body_style = ParagraphStyle(
        'DocBody',
        parent=styles['Normal'],
        fontName='Helvetica',
        fontSize=10,
        textColor=colors.HexColor('#1C2541'),
        leading=14,
        spaceAfter=10
    )

    legal_style = ParagraphStyle(
        'DocLegal',
        parent=styles['Normal'],
        fontName='Helvetica-Bold',
        fontSize=9,
        textColor=colors.HexColor('#D9534F'),
        leading=12,
        spaceAfter=8
    )

    # Content Processing Step 1: Health Analysis Summary Section
    story.append(Paragraph("SomaOS Sovereign Health Assessment", title_style))
    story.append(Paragraph(f"Protocol Footprint Hash Verification ID: {health_data.get('protocol_id', 'UNKNOWN')}", body_style))
    story.append(Spacer(1, 12))

    story.append(Paragraph("<b>Biomarker Baseline Status:</b>", body_style))
    for key, value in health_data.get('biomarker_mapping', {}).items():
        metric_str = f"• {key.replace('_', ' ').upper()}: Target: {value.get('optimal_threshold')}"
        story.append(Paragraph(metric_str, body_style))

    story.append(Spacer(1, 16))

    # Content Processing Step 2: Formulating Notice of Liability Shell
    story.append(Paragraph("NOTICE OF AUTONOMOUS LIABILITY & REFUSAL FRAMEWORK", title_style))
    legal_text = (
        "<b>LEGAL SHIELD MANDATE:</b> The user retains absolute sovereign authority over "
        "their biometric records and bodily choice framework. This documentation establishes an unalterable "
        "record of informed request under self-governed health jurisdiction. Any external entity "
        "interfering with this non-custodial wellness tracking protocol or forcing standard healthcare access "
        "mechanisms without explicit identity consent directly breaches basic systemic metadata liberty."
    )
    story.append(Paragraph(legal_text, legal_style))

    # Compile the final physical PDF
    doc.build(story)
    print(f"Sovereign PDF Package written successfully to: {output_pdf_path}")

if __name__ == "__main__":
    # Point internally directly to your project schema directories
    create_sovereign_pdf_package(
        "database/medical-core/salud_schema.json",
        "legal-shields/SomaOS_Shield_Package.pdf"
    )
