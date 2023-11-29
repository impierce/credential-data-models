use crate::tests::assert_eq_json_value;
use openbadges::json_schema::{
    achievement::{Achievement, AchievementBuilder, Criteria, CriteriaBuilder, AchievementType},
    achievement_credential::{AchievementCredential, AchievementCredentialBuilder, AchievementCredentialType, CredentialSchema, CredentialSchemaBuilder, CredentialStatus, CredentialStatusBuilder, AchievementCredentialSchema},
    achievement_subject::{AchievementSubject, AchievementSubjectBuilder},
    profile::{Profile, ProfileBuilder, Address, AddressBuilder, GeoCoordinates, GeoCoordinatesBuilder}, general::{ImageBuilder, Image, RefreshService, RefreshServiceBuilder}, identity::{IdentityObject, IdentityObjectBuilder, IdentifierEntry, IdentifierEntryBuilder}, alignment::{Alignment, AlignmentBuilder, AlignmentTargetType}, endorsement::{EndorsementCredential, EndorsementCredentialBuilder, EndorsementSubjectBuilder, EndorsementSubject, EndorsementCredentialType, EndorsementCredentialSchema, EndorsementCredentialProof}, proof_evidence::{Proof, ProofBuilder, Evidence, EvidenceBuilder}, result::{ResultDescription, ResultDescriptionBuilder, RubricCriterionLevel, RubricCriterionLevelBuilder, Result_, ResultBuilder, ResultStatus},
};
use std::{fs::File, str::FromStr};

#[test]
fn full_achievement_credential() {

    // Testing if serialization and deserialization between the OBv3 examples and our rust code works as needed.

    assert_eq_json_value::<AchievementCredential>("tests/obv3_json_examples/full_achievement_credential.json");
    
    // Next, the builders are tested against the OBv3 examples 

    let full_achievement_credential: AchievementCredential = AchievementCredentialBuilder::default()
    .context([
        "https://www.w3.org/2018/credentials/v1".into(),
        "https://purl.imsglobal.org/spec/ob/v3p0/context-3.0.2.json".into(),
        "https://purl.imsglobal.org/spec/ob/v3p0/extensions.json".into()
    ])
    .id("http://1edtech.edu/credentials/3732")
    .type_(AchievementCredentialType::from(vec![
        "VerifiableCredential",
        "OpenBadgeCredential"
      ]))
    .name("1EdTech University Degree for Example Student")
    .description("1EdTech University Degree Description".to_string())
    .image({
        let image: Image = ImageBuilder::default()
        .id("https://1edtech.edu/credentials/3732/image")
        .type_("Image")
        .caption("1EdTech University Degree for Example Student".to_string())
        .try_into()
        .unwrap();

        image
    })
    .credential_subject({
        let credential_subject: AchievementSubject = AchievementSubjectBuilder::default()
        .id("did:example:ebfeb1f712ebc6f1c276e12ec21".to_string())
        .type_("AchievementSubject")
        .activity_end_date("2010-01-02T00:00:00Z"
            .parse::<chrono::DateTime<chrono::offset::Utc>>()
            .unwrap()
        )
        .activity_start_date("2010-01-01T00:00:00Z"
            .parse::<chrono::DateTime<chrono::offset::Utc>>()
            .unwrap()        
        )
        .credits_earned(42.0)
        .license_number("A-9320041".to_string())
        .role("Major Domo".to_string())
        .source({
            let source: Profile = ProfileBuilder::default()
            .id("https://school.edu/issuers/201234".to_string())
            .type_("Profile")
            .name("1EdTech College of Arts".to_string())
            .try_into()
            .unwrap();

            source
        })
        .term("Fall".to_string())
        .identifier({
            let identifier_student: IdentityObject = IdentityObjectBuilder::default()
            .type_("IdentityObject")
            .identity_hash("student@1edtech.edu")
            .identity_type("emailAddress")
            .hashed(false)
            .salt("not-used".to_string())
            .try_into()
            .unwrap();

            let identifier_somebody: IdentityObject = IdentityObjectBuilder::default()
            .type_("IdentityObject")
            .identity_hash("somebody@gmail.com")
            .identity_type("emailAddress")
            .hashed(false)
            .salt("not-used".to_string())            
            .try_into()
            .unwrap();

            vec![identifier_student, identifier_somebody]
        })
        .achievement({
            let achievement: Achievement = AchievementBuilder::default()
            .id("https://1edtech.edu/achievements/degree")
            .type_("Achievement")
            .alignment({
                let alignment_cfitem: Alignment = AlignmentBuilder::default()
                .type_("Alignment")
                .target_code("degree".to_string())
                .target_description("1EdTech University Degree programs.".to_string())
                .target_name("1EdTech University Degree")
                .target_framework("1EdTech University Program and Course Catalog".to_string())
                .target_type(AlignmentTargetType::from("CFItem".to_string()))
                .target_url("https://1edtech.edu/catalog/degree")
                .try_into()
                .unwrap();

                let alignment_ctdl: Alignment = AlignmentBuilder::default()
                .type_("Alignment")
                .target_code("degree".to_string())
                .target_description("1EdTech University Degree programs.".to_string())
                .target_name("1EdTech University Degree")
                .target_framework("1EdTech University Program and Course Catalog".to_string())
                .target_type(AlignmentTargetType::from("CTDL"))
                .target_url("https://1edtech.edu/catalog/degree")
                .try_into()
                .unwrap();

                vec![alignment_cfitem, alignment_ctdl]
            })
            .achievement_type(AchievementType::from("Degree"))
            .creator({
                let creator: Profile = ProfileBuilder::default()
                .id("https://1edtech.edu/issuers/565049")
                .type_("Profile")
                .name("1EdTech University".to_string())
                .url("https://1edtech.edu".to_string())
                .phone("1-222-333-4444".to_string())
                .description("1EdTech University provides online degree programs.".to_string())
                .endorsement({
                    let endorsement1: EndorsementCredential = EndorsementCredentialBuilder::default()
                    .context(vec![
                        "https://www.w3.org/2018/credentials/v1".into(),
                        "https://purl.imsglobal.org/spec/ob/v3p0/context-3.0.2.json".into(),
                        "https://w3id.org/security/data-integrity/v1".into()
                    ])
                    .id("http://1edtech.edu/endorsementcredential/3732")
                    .type_(vec![
                        "VerifiableCredential",
                        "EndorsementCredential"
                    ])
                    .name("SDE endorsement")
                    .issuer({
                        let profile: Profile = ProfileBuilder::default()
                        .id("https://accrediter.edu/issuers/565049")
                        .type_("Profile")
                        .name("Example Accrediting Agency".to_string())
                        .try_into()
                        .unwrap();

                        profile
                    })
                    .issuance_date("2010-01-01T00:00:00Z".parse::<chrono::DateTime<chrono::offset::Utc>>().unwrap())
                    .expiration_date("2020-01-01T00:00:00Z".parse::<chrono::DateTime<chrono::offset::Utc>>().unwrap())
                    .credential_subject({
                        let endorsement_subject: EndorsementSubject = EndorsementSubjectBuilder::default()
                        .id("https://1edtech.edu/issuers/565049")
                        .type_("EndorsementSubject")
                        .endorsement_comment("1EdTech University is in good standing".to_string())
                        .try_into()
                        .unwrap();

                        endorsement_subject
                    })
                    .credential_schema({
                        let schema1: CredentialSchema = CredentialSchemaBuilder::default()
                        .id("https://purl.imsglobal.org/spec/ob/v3p0/schema/json/ob_v3p0_endorsementcredential_schema.json")
                        .type_("1EdTechJsonSchemaValidator2019")
                        .try_into()
                        .unwrap();

                        let schema2: CredentialSchema = CredentialSchemaBuilder::default()
                        .id("https://accrediter.edu/schema/endorsementcredential.json")
                        .type_("1EdTechJsonSchemaValidator2019")
                        .try_into()
                        .unwrap();

                        EndorsementCredentialSchema::from(vec![schema1, schema2])
                    })
                    .credential_status({
                        let credential_status: CredentialStatus = CredentialStatusBuilder::default()
                        .id("https://1edtech.edu/credentials/3732/revocations")
                        .type_("1EdTechRevocationList")
                        .try_into()
                        .unwrap();

                        credential_status
                    })
                    .refresh_service({
                        let refresh_service: RefreshService = RefreshServiceBuilder::default()
                        .id("http://1edtech.edu/credentials/3732")
                        .type_("1EdTechCredentialRefresh")
                        .try_into()
                        .unwrap();

                        refresh_service
                    })
                    .proof({
                        let proof: Proof = ProofBuilder::default()
                        .type_("DataIntegrityProof")
                        .cryptosuite("eddsa-rdf-2022".to_string())
                        .created("2022-05-26T18:17:08Z".parse::<chrono::DateTime<chrono::offset::Utc>>().unwrap())
                        .verification_method("https://accrediter.edu/issuers/565049#zvPkQiUFfJrgnCRhyPkTSkgrGXbnLR15pHH5HZVYNdM4TCAwQHqG7fMeMPLtYNRnEgoV1aJdR5E61eWu5sWRYgtA".to_string())
                        .proof_purpose("assertionMethod".to_string())
                        .proof_value("zvPkQiUFfJrgnCRhyPkTSkgrGXbnLR15pHH5HZVYNdM4TCAwQHqG7fMeMPLtYNRnEgoV1aJdR5E61eWu5sWRYgtA".to_string())
                        .try_into()
                        .unwrap();

                        EndorsementCredentialProof::from(proof)
                    })
                    .try_into()
                    .unwrap();

                    let endorsement2: EndorsementCredential = EndorsementCredentialBuilder::default()
                    .context([
                        "https://www.w3.org/2018/credentials/v1".into(),
                        "https://purl.imsglobal.org/spec/ob/v3p0/context-3.0.2.json".into(),
                        "https://w3id.org/security/data-integrity/v1".into()
                    ])
                    .id("http://1edtech.edu/endorsementcredential/3733")
                    .type_(EndorsementCredentialType::from(vec![
                        "VerifiableCredential".to_string(),
                        "EndorsementCredential".to_string()
                    ]))
                    .name("SDE endorsement")
                    .issuer({
                        let profile: Profile = ProfileBuilder::default()
                        .id("https://state.gov/issuers/565049")
                        .type_("Profile")
                        .name("State Department of Education".to_string())
                        .try_into()
                        .unwrap();

                        profile
                    })
                    .issuance_date("2010-01-01T00:00:00Z".parse::<chrono::DateTime<chrono::offset::Utc>>().unwrap())
                    .expiration_date("2020-01-01T00:00:00Z".parse::<chrono::DateTime<chrono::offset::Utc>>().unwrap())
                    .credential_subject({
                        let credential_subject: EndorsementSubject = EndorsementSubjectBuilder::default()
                        .id("https://1edtech.edu/issuers/565049")
                        .type_("EndorsementSubject")
                        .endorsement_comment("1EdTech University is in good standing".to_string())
                        .try_into()
                        .unwrap();

                        credential_subject
                    })
                    .credential_schema({
                        let schema1: CredentialSchema = CredentialSchemaBuilder::default()
                        .id("https://purl.imsglobal.org/spec/ob/v3p0/schema/json/ob_v3p0_endorsementcredential_schema.json")
                        .type_("1EdTechJsonSchemaValidator2019")
                        .try_into()
                        .unwrap();

                        let schema2: CredentialSchema = CredentialSchemaBuilder::default()
                        .id("https://state.gov/schema/endorsementcredential.json")
                        .type_("1EdTechJsonSchemaValidator2019")
                        .try_into()
                        .unwrap();

                        EndorsementCredentialSchema::from(vec![schema1, schema2])                        
                    })
                    .credential_status({
                        let credential_status: CredentialStatus = CredentialStatusBuilder::default()
                        .id("https://state.gov/credentials/3732/revocations")
                        .type_("1EdTechRevocationList")
                        .try_into()
                        .unwrap();

                        credential_status
                    })
                    .refresh_service({
                        let refresh_service: RefreshService = RefreshServiceBuilder::default()
                        .id("http://state.gov/credentials/3732")
                        .type_("1EdTechCredentialRefresh")
                        .try_into()
                        .unwrap();

                        refresh_service
                    })
                    .proof({
                        let proof: Proof = ProofBuilder::default()
                        .type_("DataIntegrityProof")
                        .cryptosuite("eddsa-rdf-2022".to_string())
                        .created("2022-05-26T18:25:59Z".parse::<chrono::DateTime<chrono::offset::Utc>>().unwrap())
                        .verification_method("https://accrediter.edu/issuers/565049#z5bDnmSgDczXwZGya6ZjxKaxkdKxzsCMiVSsgEVWxnaWK7ZqbKnzcCd7mUKE9DQaAL2QMXP5AquPeW6W2CWrZ7jNC".to_string())
                        .proof_purpose("assertionMethod".to_string())
                        .proof_value("z5bDnmSgDczXwZGya6ZjxKaxkdKxzsCMiVSsgEVWxnaWK7ZqbKnzcCd7mUKE9DQaAL2QMXP5AquPeW6W2CWrZ7jNC".to_string())
                        .try_into()
                        .unwrap();

                        EndorsementCredentialProof::from(proof)
                    })
                    .try_into()
                    .unwrap();

                    vec![endorsement1, endorsement2]
                })
                .image({
                    let image: Image = ImageBuilder::default()
                    .id("https://1edtech.edu/logo.png")
                    .type_("Image")
                    .caption("1EdTech University logo".to_string())
                    .try_into()
                    .unwrap();

                    image
                })
                .email("registrar@1edtech.edu".to_string())
                .address({
                    let address: Address = AddressBuilder::default()
                    .type_("Address")
                    .address_country("USA".to_string())
                    .address_country_code("US".to_string())
                    .address_region("TX".to_string())
                    .address_locality("Austin".to_string())
                    .street_address("123 First St".to_string())
                    .post_office_box_number("1".to_string())
                    .postal_code("12345".to_string())
                    .geo({
                        let geo: GeoCoordinates = GeoCoordinatesBuilder::default()
                        .type_("GeoCoordinates")
                        .latitude(1.0)
                        .longitude(1.0)
                        .try_into()
                        .unwrap();

                        geo
                    })
                    .try_into()
                    .unwrap();

                    address
                })
                .other_identifier({
                    let identifier1: IdentifierEntry = IdentifierEntryBuilder::default()
                    .type_("IdentifierEntry")
                    .identifier("12345")
                    .identifier_type("sourcedId")
                    .try_into()
                    .unwrap();

                    let identifier2: IdentifierEntry = IdentifierEntryBuilder::default()
                    .type_("IdentifierEntry")
                    .identifier("67890")
                    .identifier_type("nationalIdentityNumber")
                    .try_into()
                    .unwrap();

                    vec![identifier1, identifier2]
                })
                .official("Horace Mann".to_string())
                .parent_org({
                    let parent_org: Profile = ProfileBuilder::default()
                    .id("did:example:123456789")
                    .type_("Profile")
                    .name("Universal Universities".to_string())
                    .try_into()
                    .unwrap();

                    Some(parent_org)
                })     
                .try_into()
                .unwrap();

                creator
            })
            .credits_available(36.0)
            .criteria({
                let criteria: Criteria = CriteriaBuilder::default()
                .id("https://1edtech.edu/achievements/degree".to_string())
                .narrative("# Degree Requirements\nStudents must complete...".to_string())
                .try_into()
                .unwrap();

                criteria
            })
            .description("1EdTech University Degree Description")
            .endorsement({
                let endorsement: EndorsementCredential = EndorsementCredentialBuilder::default()
                .context(vec!["https://www.w3.org/2018/credentials/v1".into(),
                "https://purl.imsglobal.org/spec/ob/v3p0/context-3.0.2.json".into(),
                "https://w3id.org/security/data-integrity/v1".into()])
                .id("http://1edtech.edu/endorsementcredential/3734")
                .type_(EndorsementCredentialType::from(vec![
                    "VerifiableCredential",
                    "EndorsementCredential"
                ]))
                .name("EAA endorsement")
                .issuer({
                    let issuer: Profile = ProfileBuilder::default()
                    .id("https://accrediter.edu/issuers/565049")
                    .type_("Profile")
                    .name("Example Accrediting Agency".to_string())
                    .try_into()
                    .unwrap();

                    issuer
                })
                .issuance_date("2010-01-01T00:00:00Z".parse::<chrono::DateTime<chrono::offset::Utc>>().unwrap())
                .expiration_date("2020-01-01T00:00:00Z".parse::<chrono::DateTime<chrono::offset::Utc>>().unwrap())
                .credential_subject({
                    let credential_subject: EndorsementSubject = EndorsementSubjectBuilder::default()
                    .id("https://1edtech.edu/issuers/565049")
                    .type_("EndorsementSubject")
                    .endorsement_comment("1EdTech University is in good standing".to_string())
                    .try_into()
                    .unwrap();

                    credential_subject
                })
                .credential_schema({
                    let schema1: CredentialSchema = CredentialSchemaBuilder::default()
                    .id("https://purl.imsglobal.org/spec/ob/v3p0/schema/json/ob_v3p0_endorsementcredential_schema.json")
                    .type_("1EdTechJsonSchemaValidator2019")
                    .try_into()
                    .unwrap();

                    let schema2: CredentialSchema = CredentialSchemaBuilder::default()
                    .id("https://accrediter.edu/schema/endorsementcredential.json")
                    .type_("1EdTechJsonSchemaValidator2019")
                    .try_into()
                    .unwrap();
                    
                    EndorsementCredentialSchema::from(vec![schema1, schema2])
                })
                .credential_status({
                    let credential_status: CredentialStatus = CredentialStatusBuilder::default()
                    .id("https://1edtech.edu/credentials/3732/revocations")
                    .type_("1EdTechRevocationList")
                    .try_into()
                    .unwrap();

                    credential_status
                })
                .refresh_service({
                    let refresh_service: RefreshService = RefreshServiceBuilder::default()
                    .id("http://1edtech.edu/credentials/3732")
                    .type_("1EdTechCredentialRefresh")
                    .try_into()
                    .unwrap();

                    refresh_service
                })
                .proof({
                    let proof: Proof = ProofBuilder::default()
                    .type_("DataIntegrityProof")
                    .cryptosuite("eddsa-rdf-2022".to_string())
                    .created("2022-05-26T18:17:08Z".parse::<chrono::DateTime<chrono::offset::Utc>>().unwrap())
                    .verification_method("https://accrediter.edu/issuers/565049#zvPkQiUFfJrgnCRhyPkTSkgrGXbnLR15pHH5HZVYNdM4TCAwQHqG7fMeMPLtYNRnEgoV1aJdR5E61eWu5sWRYgtA".to_string())
                    .proof_purpose("assertionMethod".to_string())
                    .proof_value("zvPkQiUFfJrgnCRhyPkTSkgrGXbnLR15pHH5HZVYNdM4TCAwQHqG7fMeMPLtYNRnEgoV1aJdR5E61eWu5sWRYgtA".to_string())
                    .try_into()
                    .unwrap();

                    EndorsementCredentialProof::from(proof)
                })
                .try_into()
                .unwrap();

                vec![endorsement]
            })
            .field_of_study("Research".to_string())
            .human_code("R1".to_string())
            .image({
                let image: Image = ImageBuilder::default()
                .id("https://1edtech.edu/achievements/degree/image")
                .type_("Image")
                .caption("1EdTech University Degree".to_string())
                .try_into()
                .unwrap();

                image
            })
            .name("1EdTech University Degree")
            .other_identifier({
                let other_identifier: IdentifierEntry = IdentifierEntryBuilder::default()
                .type_("IdentifierEntry")
                .identifier("abde")
                .identifier_type("identifier")
                .try_into()
                .unwrap();

                vec![other_identifier]
            })
            .result_description({
                let result_description1: ResultDescription = ResultDescriptionBuilder::default()
                .id("urn:uuid:f6ab24cd-86e8-4eaf-b8c6-ded74e8fd41c")
                .type_("ResultDescription")
                .alignment({
                    let alignment: Alignment = AlignmentBuilder::default()
                    .type_("Alignment")
                    .target_code("project".to_string())
                    .target_description("Project description".to_string())
                    .target_name("Final Project")
                    .target_framework("1EdTech University Program and Course Catalog".to_string())
                    .target_type(AlignmentTargetType::from("CFItem"))
                    .target_url("https://1edtech.edu/catalog/degree/project")
                    .try_into()
                    .unwrap();

                    vec![alignment]
                })
                .allowed_value(vec!["D".to_string(), "C".to_string(), "B".to_string(), "A".to_string()])
                .name("Final Project Grade")
                .required_value("C".to_string())
                .result_type("LetterGrade")
                .try_into()
                .unwrap();

                let result_description2: ResultDescription = ResultDescriptionBuilder::default()
                .id("urn:uuid:a70ddc6a-4c4a-4bd8-8277-cb97c79f40c5")
                .type_("ResultDescription")
                .alignment({
                    let alignment: Alignment = AlignmentBuilder::default()
                    .type_("Alignment")
                    .target_code("project".to_string())
                    .target_description("Project description".to_string())
                    .target_name("Final Project")
                    .target_framework("1EdTech University Program and Course Catalog".to_string())
                    .target_type(AlignmentTargetType::from("CFItem"))
                    .target_url("https://1edtech.edu/catalog/degree/project")
                    .try_into()
                    .unwrap();

                    vec![alignment]
                })
                .allowed_value(vec!["D".to_string(), "C".to_string(), "B".to_string(), "A".to_string()])
                .name("Final Project Grade")
                .required_level("urn:uuid:d05a0867-d0ad-4b03-bdb5-28fb5d2aab7a".to_string())
                .result_type("RubricCriterionLevel")
                .rubric_criterion_level({
                    let rubric1: RubricCriterionLevel = RubricCriterionLevelBuilder::default()
                    .id("urn:uuid:d05a0867-d0ad-4b03-bdb5-28fb5d2aab7a")
                    .type_("RubricCriterionLevel")
                    .alignment({
                        let alignment: Alignment = AlignmentBuilder::default()
                        .type_("Alignment")
                        .target_code("project".to_string())
                        .target_description("Project description".to_string())
                        .target_name("Final Project")
                        .target_framework("1EdTech University Program and Course Catalog".to_string())
                        .target_type(AlignmentTargetType::from("CFRubricCriterionLevel"))
                        .target_url("https://1edtech.edu/catalog/degree/project/rubric/levels/mastered")
                        .try_into()
                        .unwrap();

                        vec![alignment]
                    })
                    .description("The author demonstrated...".to_string())
                    .level("Mastered".to_string())
                    .name("Mastery")
                    .points("4".to_string())
                    .try_into()
                    .unwrap();

                    let rubric2: RubricCriterionLevel = RubricCriterionLevelBuilder::default()
                    .id("urn:uuid:6b84b429-31ee-4dac-9d20-e5c55881f80e")
                    .type_("RubricCriterionLevel")
                    .alignment({
                        let alignment: Alignment = AlignmentBuilder::default()
                        .type_("Alignment")
                        .target_code("project".to_string())
                        .target_description("Project description".to_string())
                        .target_name("Final Project")
                        .target_framework("1EdTech University Program and Course Catalog".to_string())
                        .target_type(AlignmentTargetType::from("CFRubricCriterionLevel"))
                        .target_url("https://1edtech.edu/catalog/degree/project/rubric/levels/basic")
                        .try_into()
                        .unwrap();

                        vec![alignment]
                    })
                    .description("The author demonstrated...".to_string())
                    .level("Basic".to_string())
                    .name("Basic")
                    .points("4".to_string())
                    .try_into()
                    .unwrap();

                    vec![rubric1, rubric2]
                })
                .try_into()
                .unwrap();
                
                let result_description3: ResultDescription = ResultDescriptionBuilder::default()
                .id("urn:uuid:b07c0387-f2d6-4b65-a3f4-f4e4302ea8f7")
                .type_("ResultDescription")
                .name("Project Status")
                .result_type("Status")
                .try_into()
                .unwrap();

                vec![result_description1, result_description2, result_description3]
            })
            .specialization("Computer Science Research".to_string())
            .tag(vec!["research".to_string(), "computer science".to_string()])
            .try_into()
            .unwrap();
      
            achievement
        })
        .image({
            let image: Image = ImageBuilder::default()
            .id("https://1edtech.edu/credentials/3732/image")
            .type_("Image")
            .caption("1EdTech University Degree for Example Student".to_string())
            .try_into()
            .unwrap();

            image
        })
        .narrative("There is a final project report and source code evidence.".to_string())
        .result({
            let result1: Result_ = ResultBuilder::default()
            .type_(vec!["Result"])
            .alignment({
                let alignment: Alignment = AlignmentBuilder::default()
                .type_("Alignment")
                .target_code("project".to_string())
                .target_description("Project description".to_string())
                .target_name("Final Project")
                .target_framework("1EdTech University Program and Course Catalog".to_string())
                .target_type(AlignmentTargetType::from("CFItem"))
                .target_url("https://1edtech.edu/catalog/degree/project/result/1")
                .try_into()
                .unwrap();

                vec![alignment]
            })
            .result_description("urn:uuid:f6ab24cd-86e8-4eaf-b8c6-ded74e8fd41c".to_string())
            .value("A".to_string())
            .try_into()
            .unwrap();

            let result2: Result_ = ResultBuilder::default()
            .type_("Result")
            .achieved_level("urn:uuid:d05a0867-d0ad-4b03-bdb5-28fb5d2aab7a".to_string())
            .alignment({
                let alignment: Alignment = AlignmentBuilder::default()
                .type_("Alignment")
                .target_code("project".to_string())
                .target_description("Project description".to_string())
                .target_name("Final Project")
                .target_framework("1EdTech University Program and Course Catalog".to_string())
                .target_type(AlignmentTargetType::from("CFItem"))
                .target_url("https://1edtech.edu/catalog/degree/project/result/1")
                .try_into()
                .unwrap();

                vec![alignment]
            })
            .result_description("urn:uuid:f6ab24cd-86e8-4eaf-b8c6-ded74e8fd41c".to_string())
            .try_into()
            .unwrap();

            let result3: Result_ = ResultBuilder::default()
            .type_("Result")
            .result_description("urn:uuid:f6ab24cd-86e8-4eaf-b8c6-ded74e8fd41c".to_string())
            .status(ResultStatus::from_str("Completed").unwrap())
            .try_into()
            .unwrap();

            vec![result1, result2, result3]
        })    
        .try_into()
        .unwrap();
    
        credential_subject
    })
    .endorsement({
        let endorsement: EndorsementCredential = EndorsementCredentialBuilder::default()
        .context(vec![
            "https://www.w3.org/2018/credentials/v1".into(),
            "https://purl.imsglobal.org/spec/ob/v3p0/context-3.0.2.json".into(),
            "https://w3id.org/security/data-integrity/v1".into()
        ])
        .id("http://1edtech.edu/endorsementcredential/3735")
        .type_(vec![
            "VerifiableCredential",
            "EndorsementCredential"
        ])
        .name("EAA endorsement")
        .issuer({
            let issuer: Profile = ProfileBuilder::default()
            .id("https://accrediter.edu/issuers/565049")
            .type_("Profile")
            .name("Example Accrediting Agency".to_string())
            .try_into()
            .unwrap();

            issuer
        })
        .issuance_date("2010-01-01T00:00:00Z".parse::<chrono::DateTime<chrono::offset::Utc>>().unwrap())
        .expiration_date("2020-01-01T00:00:00Z".parse::<chrono::DateTime<chrono::offset::Utc>>().unwrap())
        .credential_subject({
            let credential_subject: EndorsementSubject = EndorsementSubjectBuilder::default()
            .id("https://1edtech.edu/issuers/565049")
            .type_("EndorsementSubject")
            .endorsement_comment("1EdTech University is in good standing".to_string())
            .try_into()
            .unwrap();

            credential_subject
        })
        .credential_schema({
            let schema1: CredentialSchema = CredentialSchemaBuilder::default()
            .id("https://purl.imsglobal.org/spec/ob/v3p0/schema/json/ob_v3p0_endorsementcredential_schema.json")
            .type_("1EdTechJsonSchemaValidator2019")
            .try_into()
            .unwrap();

            let schema2: CredentialSchema = CredentialSchemaBuilder::default()
            .id("https://accrediter.edu/schema/endorsementcredential.json")
            .type_("1EdTechJsonSchemaValidator2019")
            .try_into()
            .unwrap();

            EndorsementCredentialSchema::from(vec![schema1, schema2])
        })
        .credential_status({
            let credential_status: CredentialStatus = CredentialStatusBuilder::default()
            .id("https://1edtech.edu/credentials/3732/revocations")
            .type_("1EdTechRevocationList")
            .try_into()
            .unwrap();

            credential_status
        })
        .refresh_service({
            let refresh_service: RefreshService = RefreshServiceBuilder::default()
            .id("http://1edtech.edu/credentials/3732")
            .type_("1EdTechCredentialRefresh")
            .try_into()
            .unwrap();

            refresh_service
        })
        .proof({
            let proof: Proof = ProofBuilder::default()
            .type_("DataIntegrityProof")
            .cryptosuite("eddsa-rdf-2022".to_string())
            .created("2022-05-26T18:17:08Z".parse::<chrono::DateTime<chrono::offset::Utc>>().unwrap())
            .verification_method("https://accrediter.edu/issuers/565049#zvPkQiUFfJrgnCRhyPkTSkgrGXbnLR15pHH5HZVYNdM4TCAwQHqG7fMeMPLtYNRnEgoV1aJdR5E61eWu5sWRYgtA".to_string())
            .proof_purpose("assertionMethod".to_string())
            .proof_value("zvPkQiUFfJrgnCRhyPkTSkgrGXbnLR15pHH5HZVYNdM4TCAwQHqG7fMeMPLtYNRnEgoV1aJdR5E61eWu5sWRYgtA".to_string())
            .try_into()
            .unwrap();

            EndorsementCredentialProof::from(proof)
        })
        .try_into()
        .unwrap();

        vec![endorsement]
    })
    .evidence({
        let evidence: Evidence = EvidenceBuilder::default()
        .id("https://1edtech.edu/credentials/3732/evidence/1".to_string())
        .type_("Evidence")
        .narrative("# Final Project Report \n This project was ...".to_string())
        .name("Final Project Report".to_string())
        .description("This is the final project report.".to_string())
        .genre("Research".to_string())
        .audience("Department".to_string())
        .try_into()
        .unwrap();

        let evidence2: Evidence = EvidenceBuilder::default()
        .id("https://github.com/somebody/project".to_string())
        .type_("Evidence")
        .name("Final Project Code".to_string())
        .description("This is the source code for the final project app.".to_string())
        .genre("Research".to_string())
        .audience("Department".to_string())
        .try_into()
        .unwrap();

        vec![evidence, evidence2]
    })
    .issuer({
        let issuer: Profile = ProfileBuilder::default()
        .id("https://1edtech.edu/issuers/565049")
        .type_("Profile")
        .name("1EdTech University".to_string())
        .url("https://1edtech.edu".to_string())
        .phone("1-222-333-4444".to_string())
        .description("1EdTech University provides online degree programs.".to_string())
        .endorsement({
            let endorsement: EndorsementCredential = EndorsementCredentialBuilder::default()
            .context(vec![
                "https://www.w3.org/2018/credentials/v1".into(),
                "https://purl.imsglobal.org/spec/ob/v3p0/context-3.0.2.json".into(),
                "https://w3id.org/security/data-integrity/v1".into()
              ])
            .id("http://1edtech.edu/endorsementcredential/3736")
            .type_(vec![
                "VerifiableCredential",
                "EndorsementCredential"
              ])
            .name("EAA endorsement")
            .issuer({
                let issuer: Profile = ProfileBuilder::default()
                .id("https://accrediter.edu/issuers/565049")
                .type_("Profile")
                .name("Example Accrediting Agency".to_string())
                .try_into()
                .unwrap();

                issuer
            })
            .issuance_date("2010-01-01T00:00:00Z".parse::<chrono::DateTime<chrono::offset::Utc>>().unwrap())
            .expiration_date("2020-01-01T00:00:00Z".parse::<chrono::DateTime<chrono::offset::Utc>>().unwrap())
            .credential_subject({
                let credential_subject: EndorsementSubject = EndorsementSubjectBuilder::default()
                .id("https://1edtech.edu/issuers/565049")
                .type_("EndorsementSubject")
                .endorsement_comment("1EdTech University is in good standing".to_string())
                .try_into()
                .unwrap();

                credential_subject
            })
            .credential_schema({
                let schema1: CredentialSchema = CredentialSchemaBuilder::default()
                .id("https://purl.imsglobal.org/spec/ob/v3p0/schema/json/ob_v3p0_endorsementcredential_schema.json")
                .type_("1EdTechJsonSchemaValidator2019")
                .try_into()
                .unwrap();

                let schema2: CredentialSchema = CredentialSchemaBuilder::default()
                .id("https://accrediter.edu/schema/endorsementcredential.json")
                .type_("1EdTechJsonSchemaValidator2019")
                .try_into()
                .unwrap();

                EndorsementCredentialSchema::from(vec![schema1, schema2])
            })
            .credential_status({
                let credential_status: CredentialStatus = CredentialStatusBuilder::default()
                .id("https://1edtech.edu/credentials/3732/revocations")
                .type_("1EdTechRevocationList")
                .try_into()
                .unwrap();

                credential_status
            })
            .refresh_service({
                let refresh_service: RefreshService = RefreshServiceBuilder::default()
                .id("http://1edtech.edu/credentials/3732")
                .type_("1EdTechCredentialRefresh")
                .try_into()
                .unwrap();

                refresh_service
            })
            .proof({
                let proof: Proof = ProofBuilder::default()
                .type_("DataIntegrityProof")
                .cryptosuite("eddsa-rdf-2022".to_string())
                .created("2022-05-26T18:17:08Z".parse::<chrono::DateTime<chrono::offset::Utc>>().unwrap())
                .verification_method("https://accrediter.edu/issuers/565049#zvPkQiUFfJrgnCRhyPkTSkgrGXbnLR15pHH5HZVYNdM4TCAwQHqG7fMeMPLtYNRnEgoV1aJdR5E61eWu5sWRYgtA".to_string())
                .proof_purpose("assertionMethod".to_string())
                .proof_value("zvPkQiUFfJrgnCRhyPkTSkgrGXbnLR15pHH5HZVYNdM4TCAwQHqG7fMeMPLtYNRnEgoV1aJdR5E61eWu5sWRYgtA".to_string())
                .try_into()
                .unwrap();
      
                EndorsementCredentialProof::from(proof)
            })
            .try_into()
            .unwrap();

            vec![endorsement]
        })
        .image({
            let image: Image = ImageBuilder::default()
            .id("https://1edtech.edu/logo.png")
            .type_("Image")
            .caption("1EdTech University logo".to_string())
            .try_into()
            .unwrap();

            image
        })
        .email("registrar@1edtech.edu".to_string())
        .address({
            let address: Address = AddressBuilder::default()
            .type_("Address")
            .address_country("USA".to_string())
            .address_country_code("US".to_string())
            .address_region("TX".to_string())
            .address_locality("Austin".to_string())
            .street_address("123 First St".to_string())
            .post_office_box_number("1".to_string())
            .postal_code("12345".to_string())
            .geo({
                let geo: GeoCoordinates = GeoCoordinatesBuilder::default()
                .type_("GeoCoordinates")
                .latitude(1.0)
                .longitude(1.0)
                .try_into()
                .unwrap();

                geo
            })
            .try_into()
            .unwrap();

            address
        })
        .other_identifier({
            let identifier1: IdentifierEntry = IdentifierEntryBuilder::default()
            .type_("IdentifierEntry")
            .identifier("12345")
            .identifier_type("sourcedId")
            .try_into()
            .unwrap();

            let identifier2: IdentifierEntry = IdentifierEntryBuilder::default()
            .type_("IdentifierEntry")
            .identifier("67890")
            .identifier_type("nationalIdentityNumber")
            .try_into()
            .unwrap();

            vec![identifier1, identifier2]
        })
        .official("Horace Mann".to_string())
        .parent_org({
            let parent_org: Profile = ProfileBuilder::default()
            .id("did:example:123456789")
            .type_("Profile")
            .name("Universal Universities".to_string())
            .try_into()
            .unwrap();

            Some(parent_org)
        })
        .try_into()
        .unwrap();

        issuer
    })
    .issuance_date("2010-01-01T00:00:00Z".parse::<chrono::DateTime<chrono::offset::Utc>>().unwrap())
    .expiration_date("2030-01-01T00:00:00Z".parse::<chrono::DateTime<chrono::offset::Utc>>().unwrap())
    .credential_schema({
        let schema: CredentialSchema = CredentialSchemaBuilder::default()
        .id("https://purl.imsglobal.org/spec/ob/v3p0/schema/json/ob_v3p0_achievementcredential_schema.json")
        .type_("1EdTechJsonSchemaValidator2019")
        .try_into()
        .unwrap();

        AchievementCredentialSchema::from(schema)
    })
    .credential_status({
        let credential_status: CredentialStatus = CredentialStatusBuilder::default()
        .id("https://1edtech.edu/credentials/3732/revocations")
        .type_("1EdTechRevocationList")
        .try_into()
        .unwrap();

        credential_status
    })
    .refresh_service({
        let refresh_service: RefreshService = RefreshServiceBuilder::default()
        .id("http://1edtech.edu/credentials/3732")
        .type_("1EdTechCredentialRefresh")
        .try_into()
        .unwrap();

        refresh_service
    }) 
    .try_into()
    .unwrap();

    ///// (First) Error occurs at AlignmentTargetType. String not automatically recognized as Enum value

    let file = File::open("tests/obv3_json_examples/full_achievement_credential.json").expect("Failed to open file");
    let full_ach_cred_from_file: AchievementCredential = serde_json::from_reader(file).expect("Couldn't read from file");
    // let json_value_from_file: serde_json::Value = serde_json::from_reader(file).expect("Couldn't read from file");

    assert_eq!(full_achievement_credential, full_ach_cred_from_file)
}
