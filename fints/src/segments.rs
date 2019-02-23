use serde_derive::{Deserialize, Serialize};

use crate::data_types::*;

mod pad_to_12 {
    use serde::{Deserializer, Serializer, Deserialize};

    pub fn serialize<S>(number: &u64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{:012}", number);
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse::<u64>().map_err(serde::de::Error::custom)
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Seg_HNHBK_MessageHead {
    // Segmentkopf
    pub segment_head: DEG_SegmentHead,

    // Nachrichtengröße
    #[serde(with = "pad_to_12")]
    pub message_size: u64,

    // HBCI-Version
    pub hbci_version: u16,

    // Dialog-ID
    pub dialog_id: String,

    // Nachrichtennummer
    pub message_no: u16,

    // Bezugsnachricht
    pub reference_msg: Option<ReferenceMessage>,
}

/// B.5.1 Signaturkopf
#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Seg_HNSHK_SignatureHead {
    // Segmentkopf
    pub segment_head: DEG_SegmentHead,

    // Sicherheitsprofil
    pub security_profile: DEG_SecurityProfile,

    // Sicherheitsfunktion, kodiert
    pub security_function: SecurityFunction,

    // Sicherheitskontrollreferenz
    pub security_reference: String,

    // Bereich der Sicherheitsapplikation, kodiert
    pub security_area: SecurityArea,

    // Rolle des Sicherheitslieferanten, kodiert
    pub security_role: SecurityRole,

    // Sicherheitsidentifikation, Details
    pub security_identification_details: DEG_SecurityIdentificationDetails,

    // Sicherheitsreferenznummer
    pub security_ref_no: u64,

    // Sicherheitsdatum und -uhrzeit
    pub security_date: DEG_SecurityDate,

    // Hashalgorithmus
    pub hash_algorithm: DEG_HashAlgorithm,

    // Signaturalgorithmus
    pub signature_algorithm: DEG_SignatureAlgorithm,

    // Schlüsselname
    pub key_name: DEG_KeyName,

    // Zertifikat
    pub certificate: Option<DEG_Certificate>,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Seg_HNHBS_MessageEnd {
    // Segmentkopf
    pub segment_head: DEG_SegmentHead,

    // Nachrichtennummer
    pub message_no: u16,
}

// C.3.1.2 Segment: Identifikation
#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Seg_HKIDN_Identification {
    // Segmentkopf
    pub segment_head: DEG_SegmentHead,

    // Kreditinstitutskennung
    pub institute_identifier: DEG_InstituteIdentifier,

    // Kunden-ID
    pub customer_id: String,

    // Kundensystem-ID
    pub customer_system_id: String,

    // Kundensystem-Status
    pub customer_system_status: CustomerSystemStatus,
}

// C.3.1.3 Segment: Verarbeitungsvorbereitung
#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Seg_HKVVB_ProcessingPreparation {
    // Segmentkopf
    pub segment_head: DEG_SegmentHead,

    // BPD-Version
    pub bpd_version: u16,

    // UPD-Version
    pub upd_version: u16,

    // Dialogsprache
    pub dialog_lang: DialogLang,

    // Produktbezeichnung
    pub product_identifier: String,

    // Produktversion
    pub product_version: String,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Seg_HKTAN_TwoStepTanSubmission {
    // Segmentkopf
    pub segment_head: DEG_SegmentHead,

    // TAN-Prozess
    pub tan_process: TanProcess,

    // Segmentkennung
    pub segment_identifier: Option<String>,

    // Kontoverbindung international Auftraggeber
    pub account_international_issuer: Option<DEG_AccountInternationalIssuer>,

    // Auftrags-Hashwert
    pub job_hash_value: Option<Vec<u8>>,

    // Auftragsreferenz
    pub job_reference: Option<String>,
}

// C.3.1.4 Segment: Anforderung eines öffentlichen Schlüssels
#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Seg_HKISA_RequestForPubkey {
    // Segmentkopf
    pub segment_head: DEG_SegmentHead,

    // Nachrichtenbeziehung, kodiert
    pub message_relationship: MessageRelationship,

    // Bezeichner für Funktionstyp
    pub function_type_identifier: FunctionTypeIdentifier,

    // Sicherheitsprofil
    pub security_profile: DEG_SecurityProfile,

    // Schlüsselname
    pub key_name: DEG_KeyName,

    // Zertifikat
    pub certificate: Option<DEG_Certificate>,
}

// C.8.1.2 Segment: Synchronisierung
#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Seg_HKSYN_Synchronization {
    // Segmentkopf
    pub segment_head: DEG_SegmentHead,

    // Synchronisierungsmodus
    pub synchronization_mode: SynchronizationMode,
}

// B.5.2 Segment: Signaturabschluss
#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Seg_HNSHA_SignatureEnd {
    // Segmentkopf
    pub segment_head: DEG_SegmentHead,

    // Sicherheitskontrollreferenz
    pub security_reference: String,

    // Validierungsresultat
    pub validation_result: Option<Vec<u8>>,

    // Benutzerdefinierte Signatur
    pub user_defined_signature: Option<DEG_UserDefinedSignature>,
}
