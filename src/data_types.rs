use chrono::{NaiveDate, NaiveTime};
use serde_derive::{Deserialize, Serialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct DEG_InstituteIdentifier {
    pub country_code: String,
    pub bank_code: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DEG_SegmentHead {
    pub identifier: String,
    pub segment_no: u16,
    pub version: u16,
    pub reference_seg: Option<u16>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReferenceMessage {
    pub dialog_id: u16,
    pub message_no: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SecurityMethodCode {
    RAH,
    PIN,
}

impl fmt::Display for SecurityMethodCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SecurityMethodCode::RAH => write!(f, "RAH"),
            SecurityMethodCode::PIN => write!(f, "PIN"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DEG_SecurityProfile {
    pub security_method_code: SecurityMethodCode,
    pub version: u8,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum SecurityFunction {
    NRO = 1,
    AUT = 2,
    ENC = 4,
    SingleStepAuth = 999,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum SecurityArea {
    SHM = 1,
    SHT = 2,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum SecurityRole {
    ISS = 1,
    CON = 3,
    WIT = 4,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum SecurityPartyIdentifier {
    MS = 1,
    MR = 2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DEG_SecurityIdentificationDetails {
    pub security_party_identifier: SecurityPartyIdentifier,
    pub cardholder_identification: Option<Vec<u8>>,
    pub party_identifier: Option<String>,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum DateIdentifier {
    // Sicherheitszeitstempel
    STS = 1,

    // Certificate Revocation Time
    CRT = 6,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DEG_SecurityDate {
    // Datum- und Zeitbezeichner, kodiert
    pub date_identifier: DateIdentifier,

    // Datum
    #[serde(with = "fints_date_format")]
    pub date: NaiveDate,

    // Uhrzeit
    #[serde(with = "fints_time_format")]
    pub time: NaiveTime,
}

mod fints_date_format {
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Deserializer, Serializer};

    /// ISO 8601
    const FORMAT: &'static str = "%Y%m%d";

    pub fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

mod fints_time_format {
    use chrono::NaiveTime;
    use serde::{self, Deserialize, Deserializer, Serializer};

    /// ISO 8601
    const FORMAT: &'static str = "%H%M%S";

    pub fn serialize<S>(time: &NaiveTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", time.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum UseOfHashAlgorithm {
    // Owner Hashing
    OHA = 1,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum HashAlgorithm {
    SHA1 = 1,
    SHA256 = 3,
    SHA384 = 4,
    SHA512 = 5,
    SHA256SHA256 = 6,

    // Gegenseitig vereinbart (ZZZ) (nicht zugelassen)
    MutuallyAgreed = 999,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum HashAlgorithmParameterIdentifier {
    // Initialization value, clear text
    IVC = 1,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DEG_HashAlgorithm {
    // Verwendung des Hashalgorithmus, kodiert
    pub use_of_hash_algorithm: UseOfHashAlgorithm,

    // Hashalgorithmus, kodiert
    pub hash_algorithm: HashAlgorithm,

    // Bezeichner für Hashalgorithmusparameter
    pub hash_algorithm_param_identifier: HashAlgorithmParameterIdentifier,

    // Wert des Hashalgorithmusparameters
    pub param_value: Option<Vec<u8>>,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum UseOfSignatureAlgorithm {
    // Owner Signing
    OSG = 6,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum SignatureAlgorithm {
    // Nicht zugelassen
    NA = 1,

    // RSA-Algorithmus (bei RAH)
    RSA = 10,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum OperationMode {
    // Cipher Block Chaining (CBC) Nur für Verschlüsselung erlaubt (vgl. [HBCI], Kapitel VI.2.)
    CBC = 2,

    // ISO 9796-1 (bei RDH), (nicht zugelassen)
    ISO_97961 = 16,

    // ISO 9796-2 mit Zufallszahl (bei RDH) (nicht zugelassen)
    ISO_97962 = 17,

    // RSASSA-PKCS#1 V1.5 (bei RDH) (nicht zugelassen) bzw.
    // RSAES-PKCS#1 V1.5 (bei RAH, RDH) (Nur für Verschlüsselung erlaubt)
    RSASSA_PKCS = 18,

    // RSASSA-PSS (bei RAH, RDH) Nur für Signatur erlaubt
    RSASSA_PSS = 19,

    // Gegenseitig vereinbart (ZZZ) (nicht zugelassen)
    MutuallyAgreed = 999,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DEG_SignatureAlgorithm {
    // Verwendung des Signaturalgorithmus, kodiert
    pub use_of_signature_algorithm: UseOfSignatureAlgorithm,

    // Signaturalgorithmus, kodiert
    pub signature_algorithm: SignatureAlgorithm,

    // Operationsmodus, kodiert
    pub operation_mode: OperationMode,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum KeyType {
    // Schlüssel zur Erzeugung digitaler Signaturen (DS-Schlüssel)
    D,

    // Signierschlüssel
    S,

    // Chiffrierschlüssel
    V,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DEG_KeyName {
    // Kreditinstitutskennung
    pub institute_identifier: DEG_InstituteIdentifier,

    // Benutzerkennung
    pub user_id: String,

    // Schlüsselart
    pub key_type: KeyType,

    // Schlüsselnummer
    pub key_no: u16,

    // Schlüsselversion
    pub key_version: u16,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum CertificateType {
    // ZKA
    ZKA = 1,

    // UN/EDIFACT
    UN_EDIFACT = 2,

    // X.509 v3 (gemäß [ISIS/MTT])
    X509v3 = 3,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DEG_Certificate {
    // Zertifikatstyp
    pub certificate_type: CertificateType,

    // Zertifikatsinhalt
    pub value: Vec<u8>,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum CustomerSystemStatus {
    // Kundensystem-ID wird nicht benötigt (HBCI DDV-Verfahren und
    // chipkartenbasierte Verfahren ab Sicherheitsprofil-Version 3)
    NotRequired = 0,

    // Kundensystem-ID wird benötigt (sonstige HBCI RAH- /
    // RDH- und PIN/TAN-Verfahren)
    Required = 1,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum DialogLang {
    Standard = 0,
    de = 1,
    en = 2,
    fr = 3,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum TanProcess {
    // Alle
    All = 0,

    // Aktiv
    Active = 2,

    // Verfügbar
    Available = 3,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DEG_AccountInternationalIssuer {
    pub iban: String,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum MessageRelationship {
    // Key-Management-Nachricht ist Antwort
    IsAnswer = 1,

    // Key-Management-Nachricht erwartet Antwort
    ExpectAnswer = 2,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum FunctionTypeIdentifier {
    // ‘Certificate Replacement’ (Ersatz des Zertifikats) im Zusammenhang mit der Schlüsseländerung
    CertificateReplacement = 112,

    // ‘Certificate Status Request’ im Zusammenhang mit der Anfrage für einen öffentlichen Schlüssel
    CertficateStatusRequest = 124,

    // ‘Certificate Status Notice’ im Zusammenhang mit der Übermittlung eines öffentlichen
    // Schlüssels
    CertificateStatusNotice = 224,

    // ‘Certificate Revocation’ (Zertifikatswiderruf) im Zusammenhang mit der Schlüsselsperrung
    CertificateRevocation = 130,

    // ‘Revocation Confirmation’ (Bestätigung des Zertifikatswiderrufs) im Zusammenhang mit der
    // Bestätigung der Schlüsselsperrung
    RevocationConfirmation = 231,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DEG_UserDefinedSignature {
    pub PIN: String,
    pub TAN: Option<String>,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum SynchronizationMode {
    ReportNewCustomerSystemId = 0,
    ReportLastProcessedMessageNo = 1,
    ReportSignatureId = 2,
}
