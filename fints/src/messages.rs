use crate::se::to_string;
use chrono::prelude::*;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde_derive::{Deserialize, Serialize};

use crate::data_types::*;
use crate::segments::*;
use fints_derive::Message;

pub trait Message {
    fn prepare_message_for_sending(&self) -> String;
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Message)]
pub struct Msg_DialogSync {
    message_head: Seg_HNHBK_MessageHead,
    signature_head: Seg_HNSHK_SignatureHead,
    identification: Seg_HKIDN_Identification,
    processing_preparation: Seg_HKVVB_ProcessingPreparation,
    two_step_tan_submission: Option<Seg_HKTAN_TwoStepTanSubmission>,
    request_for_pubkey: Option<Seg_HKISA_RequestForPubkey>,
    synchronization: Seg_HKSYN_Synchronization,
    signature_end: Seg_HNSHA_SignatureEnd,
    message_end: Seg_HNHBS_MessageEnd,
}

impl Msg_DialogSync {
    pub fn new(
        bank_code: u32,
        username: &str,
        pin: &str,
        customer_system_id: &str,
        message_no: u16,
    ) -> Msg_DialogSync {
        let security_reference: String = thread_rng().sample_iter(&Alphanumeric).take(14).collect();
        let hnhbk_message_head = Seg_HNHBK_MessageHead {
            segment_head: DEG_SegmentHead {
                identifier: "HNHBK".to_string(),
                segment_no: 1,
                version: 3,
                reference_seg: None,
            },
            message_size: 0,
            hbci_version: 300,
            dialog_id: "0".to_string(),
            message_no,
            reference_msg: None,
        };

        let hnshk_signature_head = Seg_HNSHK_SignatureHead {
            segment_head: DEG_SegmentHead {
                identifier: "HNSHK".to_string(),
                segment_no: 2,
                version: 4,
                reference_seg: None,
            },
            security_profile: DEG_SecurityProfile {
                security_method_code: SecurityMethodCode::PIN,
                version: 1, // TODO This should be upgraded as soon as a better version is available.
            },
            security_function: SecurityFunction::SingleStepAuth,
            security_reference: security_reference.clone(),
            security_area: SecurityArea::SHM,
            security_role: SecurityRole::ISS,
            security_identification_details: DEG_SecurityIdentificationDetails {
                security_party_identifier: SecurityPartyIdentifier::MS,
                cardholder_identification: None,
                party_identifier: Some(customer_system_id.to_string()),
            },
            security_ref_no: 1,
            security_date: DEG_SecurityDate {
                date_identifier: DateIdentifier::STS,
                date: Local::today().naive_local(),
                time: Local::now().naive_local().time(),
            },
            hash_algorithm: DEG_HashAlgorithm {
                use_of_hash_algorithm: UseOfHashAlgorithm::OHA,
                hash_algorithm: HashAlgorithm::MutuallyAgreed,
                hash_algorithm_param_identifier: HashAlgorithmParameterIdentifier::IVC,
                param_value: None,
            },
            signature_algorithm: DEG_SignatureAlgorithm {
                use_of_signature_algorithm: UseOfSignatureAlgorithm::OSG,
                signature_algorithm: SignatureAlgorithm::RSA,
                operation_mode: OperationMode::ISO_97961,
            },
            key_name: DEG_KeyName {
                institute_identifier: DEG_InstituteIdentifier {
                    country_code: "280".to_string(), // TODO This is Germany according to https://www.girocard.eu/media/weiternutzung_iso3166-code-280_deutschland.pdf
                    bank_code: bank_code,
                },
                user_id: username.to_string(),
                key_type: KeyType::S,
                key_no: 0,
                key_version: 0,
            },
            certificate: None,
        };

        let hkidn_identification = Seg_HKIDN_Identification {
            segment_head: DEG_SegmentHead {
                identifier: "HKIDN".to_string(),
                segment_no: 3,
                version: 2,
                reference_seg: None,
            },
            institute_identifier: DEG_InstituteIdentifier {
                country_code: "280".to_string(), // TODO This is Germany according to https://www.girocard.eu/media/weiternutzung_iso3166-code-280_deutschland.pdf
                bank_code: bank_code,
            },
            customer_id: username.to_string(),
            customer_system_id: customer_system_id.to_string(),
            customer_system_status: CustomerSystemStatus::Required,
        };

        let hkvvb_processing_preparation = Seg_HKVVB_ProcessingPreparation {
            segment_head: DEG_SegmentHead {
                identifier: "HKVVB".to_string(),
                segment_no: 4,
                version: 3,
                reference_seg: None,
            },
            bpd_version: 0,
            upd_version: 0,
            dialog_lang: DialogLang::de,
            product_identifier: "fints-rs".to_string(), // TODO: Make configurable
            product_version: "0.1".to_string(),         // TODO: Make configurable
        };

        let hksyn_synchronization = Seg_HKSYN_Synchronization {
            segment_head: DEG_SegmentHead {
                identifier: "HKSYN".to_string(),
                segment_no: 5,
                version: 3,
                reference_seg: None,
            },
            synchronization_mode: SynchronizationMode::ReportNewCustomerSystemId,
        };

        let hnsha_signature_end = Seg_HNSHA_SignatureEnd {
            segment_head: DEG_SegmentHead {
                identifier: "HNSHA".to_string(),
                segment_no: 6,
                version: 2,
                reference_seg: None,
            },
            security_reference: security_reference.clone(),
            validation_result: None,
            user_defined_signature: Some(DEG_UserDefinedSignature {
                PIN: pin.to_string(),
                TAN: None,
            }),
        };

        let hnhbs_message_end = Seg_HNHBS_MessageEnd {
            segment_head: DEG_SegmentHead {
                identifier: "HNHBS".to_string(),
                segment_no: 7,
                version: 1,
                reference_seg: None,
            },
            message_no,
        };

        Msg_DialogSync {
            message_head: hnhbk_message_head,
            signature_head: hnshk_signature_head,
            identification: hkidn_identification,
            processing_preparation: hkvvb_processing_preparation,
            two_step_tan_submission: None,
            request_for_pubkey: None,
            synchronization: hksyn_synchronization,
            signature_end: hnsha_signature_end,
            message_end: hnhbs_message_end,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Message)]
pub struct Msg_DialogInit {
    message_head: Seg_HNHBK_MessageHead,
    signature_head: Seg_HNSHK_SignatureHead,
    identification: Seg_HKIDN_Identification,
    processing_preparation: Seg_HKVVB_ProcessingPreparation,
    two_step_tan_submission: Option<Seg_HKTAN_TwoStepTanSubmission>,
    request_for_pubkey: Option<Seg_HKISA_RequestForPubkey>,
    signature_end: Seg_HNSHA_SignatureEnd,
    message_end: Seg_HNHBS_MessageEnd,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_serialize() {}
}
