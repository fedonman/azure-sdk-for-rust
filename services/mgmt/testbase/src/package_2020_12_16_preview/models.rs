#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProxyResource {
    #[serde(flatten)]
    pub resource: Resource,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Tags {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    pub location: String,
    #[serde(skip_serializing)]
    pub etag: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDefinition>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDefinition {
    #[serde(skip_serializing)]
    pub code: Option<String>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
    #[serde(skip_serializing)]
    pub target: Option<String>,
    #[serde(skip_serializing)]
    pub details: Vec<ErrorDefinition>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestBaseAccountSkuListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<TestBaseAccountSku>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestBaseAccountSku {
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    pub name: String,
    pub tier: test_base_account_sku::Tier,
    #[serde(skip_serializing)]
    pub capabilities: Vec<TestBaseAccountSkuCapability>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<String>,
}
pub mod test_base_account_sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Tier {
        Basic,
        Standard,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestBaseAccountSkuCapability {
    pub name: String,
    pub value: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemData {
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "createdByType", default, skip_serializing_if = "Option::is_none")]
    pub created_by_type: Option<system_data::CreatedByType>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[serde(rename = "lastModifiedAt", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_at: Option<String>,
}
pub mod system_data {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CreatedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LastModifiedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestBaseAccountListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<TestBaseAccountResource>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestBaseAccountResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(rename = "systemData", skip_serializing)]
    pub system_data: Option<SystemData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TestBaseAccountResourceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestBaseAccountResourceProperties {
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<ProvisioningState>,
    pub sku: TestBaseAccountSku,
    #[serde(rename = "accessLevel", skip_serializing)]
    pub access_level: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ProvisioningState {
    Succeeded,
    Failed,
    Cancelled,
    Creating,
    Deleting,
    Updating,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestBaseAccountUpdateParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TestBaseAccountUpdateParameterProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestBaseAccountUpdateParameterProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<TestBaseAccountSku>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestBaseAccountUsageDataList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<TestBaseAccountUsageData>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestBaseAccountUsageData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<TestBaseAccountUsageName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "currentValue", default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestBaseAccountUsageName {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "localizedValue", default, skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetFileUploadUrlParameters {
    #[serde(rename = "blobName", default, skip_serializing_if = "Option::is_none")]
    pub blob_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileUploadUrlResponse {
    #[serde(rename = "uploadUrl", skip_serializing)]
    pub upload_url: Option<String>,
    #[serde(rename = "blobPath", skip_serializing)]
    pub blob_path: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvailableOsListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AvailableOsResource>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvailableOsResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(rename = "systemData", skip_serializing)]
    pub system_data: Option<SystemData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AvailableOsProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvailableOsProperties {
    #[serde(rename = "osId", default, skip_serializing_if = "Option::is_none")]
    pub os_id: Option<String>,
    #[serde(rename = "osName", default, skip_serializing_if = "Option::is_none")]
    pub os_name: Option<String>,
    #[serde(rename = "osVersion", default, skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    #[serde(rename = "insiderChannel", default, skip_serializing_if = "Option::is_none")]
    pub insider_channel: Option<String>,
    #[serde(rename = "osUpdateType", default, skip_serializing_if = "Option::is_none")]
    pub os_update_type: Option<String>,
    #[serde(rename = "osPlatform", default, skip_serializing_if = "Option::is_none")]
    pub os_platform: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FlightingRingListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<FlightingRingResource>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FlightingRingResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(rename = "systemData", skip_serializing)]
    pub system_data: Option<SystemData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<FlightingRingProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FlightingRingProperties {
    #[serde(rename = "actualFlightingRingName", default, skip_serializing_if = "Option::is_none")]
    pub actual_flighting_ring_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestTypeListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<TestTypeResource>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestTypeResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(rename = "systemData", skip_serializing)]
    pub system_data: Option<SystemData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TestTypeProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestTypeProperties {
    #[serde(rename = "actualTestTypeName", default, skip_serializing_if = "Option::is_none")]
    pub actual_test_type_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackageCheckNameAvailabilityParameters {
    pub name: String,
    #[serde(rename = "applicationName")]
    pub application_name: String,
    pub version: String,
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameAvailabilityResult {
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<check_name_availability_result::Reason>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
}
pub mod check_name_availability_result {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Reason {
        Invalid,
        AlreadyExists,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackageListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PackageResource>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackageResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(rename = "systemData", skip_serializing)]
    pub system_data: Option<SystemData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PackageProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackageProperties {
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<ProvisioningState>,
    #[serde(rename = "applicationName")]
    pub application_name: String,
    pub version: String,
    #[serde(rename = "testTypes", skip_serializing)]
    pub test_types: Vec<TestType>,
    #[serde(rename = "targetOSList")]
    pub target_os_list: Vec<TargetOsInfo>,
    #[serde(rename = "packageStatus", skip_serializing)]
    pub package_status: Option<package_properties::PackageStatus>,
    #[serde(rename = "lastModifiedTime", skip_serializing)]
    pub last_modified_time: Option<String>,
    #[serde(rename = "flightingRing")]
    pub flighting_ring: String,
    #[serde(rename = "isEnabled", skip_serializing)]
    pub is_enabled: Option<bool>,
    #[serde(rename = "blobPath")]
    pub blob_path: String,
    #[serde(rename = "validationResults", skip_serializing)]
    pub validation_results: Vec<PackageValidationResult>,
    pub tests: Vec<Test>,
}
pub mod package_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PackageStatus {
        Unknown,
        Registered,
        Ready,
        Error,
        ValidatingPackage,
        PreValidationCheckPass,
        Deleted,
        ValidationLongerThanUsual,
        VerifyingPackage,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TestType {
    OutOfBoxTest,
    FunctionalTest,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TargetOsInfo {
    #[serde(rename = "osUpdateType")]
    pub os_update_type: String,
    #[serde(rename = "targetOSs")]
    pub target_o_ss: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackageValidationResult {
    #[serde(rename = "validationName", skip_serializing)]
    pub validation_name: Option<String>,
    #[serde(rename = "isValid", skip_serializing)]
    pub is_valid: Option<bool>,
    #[serde(skip_serializing)]
    pub errors: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Test {
    #[serde(rename = "testType")]
    pub test_type: TestType,
    #[serde(rename = "validationRunStatus", skip_serializing)]
    pub validation_run_status: Option<test::ValidationRunStatus>,
    #[serde(rename = "isActive", default, skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    pub commands: Vec<Command>,
}
pub mod test {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ValidationRunStatus {
        Unknown,
        Pending,
        Passed,
        Failed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Command {
    pub name: String,
    pub action: command::Action,
    #[serde(rename = "contentType")]
    pub content_type: command::ContentType,
    pub content: String,
    #[serde(rename = "runElevated", default, skip_serializing_if = "Option::is_none")]
    pub run_elevated: Option<bool>,
    #[serde(rename = "restartAfter", default, skip_serializing_if = "Option::is_none")]
    pub restart_after: Option<bool>,
    #[serde(rename = "maxRunTime", default, skip_serializing_if = "Option::is_none")]
    pub max_run_time: Option<i32>,
    #[serde(rename = "runAsInteractive", default, skip_serializing_if = "Option::is_none")]
    pub run_as_interactive: Option<bool>,
    #[serde(rename = "alwaysRun", default, skip_serializing_if = "Option::is_none")]
    pub always_run: Option<bool>,
    #[serde(rename = "applyUpdateBefore", default, skip_serializing_if = "Option::is_none")]
    pub apply_update_before: Option<bool>,
}
pub mod command {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Action {
        Install,
        Launch,
        Close,
        Uninstall,
        Custom,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ContentType {
        Inline,
        File,
        Path,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackageUpdateParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PackageUpdateParameterProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackageUpdateParameterProperties {
    #[serde(rename = "targetOSList", default, skip_serializing_if = "Vec::is_empty")]
    pub target_os_list: Vec<TargetOsInfo>,
    #[serde(rename = "flightingRing", default, skip_serializing_if = "Option::is_none")]
    pub flighting_ring: Option<String>,
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[serde(rename = "blobPath", default, skip_serializing_if = "Option::is_none")]
    pub blob_path: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tests: Vec<Test>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DownloadUrlResponse {
    #[serde(rename = "downloadUrl", skip_serializing)]
    pub download_url: Option<String>,
    #[serde(rename = "expirationTime", skip_serializing)]
    pub expiration_time: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestSummaryListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<TestSummaryResource>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestSummaryResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(rename = "systemData", skip_serializing)]
    pub system_data: Option<SystemData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TestSummaryProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestSummaryProperties {
    #[serde(rename = "testSummaryId", default, skip_serializing_if = "Option::is_none")]
    pub test_summary_id: Option<String>,
    #[serde(rename = "packageId", default, skip_serializing_if = "Option::is_none")]
    pub package_id: Option<String>,
    #[serde(rename = "applicationName", default, skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    #[serde(rename = "applicationVersion", default, skip_serializing_if = "Option::is_none")]
    pub application_version: Option<String>,
    #[serde(rename = "executionStatus", default, skip_serializing_if = "Option::is_none")]
    pub execution_status: Option<TestExecutionStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grade: Option<TestGrade>,
    #[serde(rename = "testRunTime", default, skip_serializing_if = "Option::is_none")]
    pub test_run_time: Option<String>,
    #[serde(rename = "featureUpdatesTestSummary", default, skip_serializing_if = "Option::is_none")]
    pub feature_updates_test_summary: Option<OsUpdatesTestSummary>,
    #[serde(rename = "securityUpdatesTestSummary", default, skip_serializing_if = "Option::is_none")]
    pub security_updates_test_summary: Option<OsUpdatesTestSummary>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TestExecutionStatus {
    None,
    InProgress,
    Processing,
    Completed,
    NotExecuted,
    Incomplete,
    Failed,
    Succeeded,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TestGrade {
    None,
    NotAvailable,
    Pass,
    Fail,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OsUpdatesTestSummary {
    #[serde(rename = "executionStatus", default, skip_serializing_if = "Option::is_none")]
    pub execution_status: Option<TestExecutionStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grade: Option<TestGrade>,
    #[serde(rename = "testRunTime", default, skip_serializing_if = "Option::is_none")]
    pub test_run_time: Option<String>,
    #[serde(rename = "osUpdateTestSummaries", default, skip_serializing_if = "Vec::is_empty")]
    pub os_update_test_summaries: Vec<OsUpdateTestSummary>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OsUpdateTestSummary {
    #[serde(rename = "osName", default, skip_serializing_if = "Option::is_none")]
    pub os_name: Option<String>,
    #[serde(rename = "releaseName", default, skip_serializing_if = "Option::is_none")]
    pub release_name: Option<String>,
    #[serde(rename = "buildVersion", default, skip_serializing_if = "Option::is_none")]
    pub build_version: Option<String>,
    #[serde(rename = "buildRevision", default, skip_serializing_if = "Option::is_none")]
    pub build_revision: Option<String>,
    #[serde(rename = "releaseVersionDate", default, skip_serializing_if = "Option::is_none")]
    pub release_version_date: Option<String>,
    #[serde(rename = "flightingRing", default, skip_serializing_if = "Option::is_none")]
    pub flighting_ring: Option<String>,
    #[serde(rename = "executionStatus", default, skip_serializing_if = "Option::is_none")]
    pub execution_status: Option<TestExecutionStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grade: Option<TestGrade>,
    #[serde(rename = "testRunTime", default, skip_serializing_if = "Option::is_none")]
    pub test_run_time: Option<String>,
    #[serde(rename = "testType", default, skip_serializing_if = "Option::is_none")]
    pub test_type: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestResultListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<TestResultResource>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestResultResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(rename = "systemData", skip_serializing)]
    pub system_data: Option<SystemData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TestResultProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestResultProperties {
    #[serde(rename = "baselineTestResultId", default, skip_serializing_if = "Option::is_none")]
    pub baseline_test_result_id: Option<String>,
    #[serde(rename = "packageId", default, skip_serializing_if = "Option::is_none")]
    pub package_id: Option<String>,
    #[serde(rename = "applicationName", default, skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    #[serde(rename = "applicationVersion", default, skip_serializing_if = "Option::is_none")]
    pub application_version: Option<String>,
    #[serde(rename = "osName", default, skip_serializing_if = "Option::is_none")]
    pub os_name: Option<String>,
    #[serde(rename = "releaseName", default, skip_serializing_if = "Option::is_none")]
    pub release_name: Option<String>,
    #[serde(rename = "releaseVersionDate", default, skip_serializing_if = "Option::is_none")]
    pub release_version_date: Option<String>,
    #[serde(rename = "flightingRing", default, skip_serializing_if = "Option::is_none")]
    pub flighting_ring: Option<String>,
    #[serde(rename = "buildVersion", default, skip_serializing_if = "Option::is_none")]
    pub build_version: Option<String>,
    #[serde(rename = "buildRevision", default, skip_serializing_if = "Option::is_none")]
    pub build_revision: Option<String>,
    #[serde(rename = "testType", default, skip_serializing_if = "Option::is_none")]
    pub test_type: Option<String>,
    #[serde(rename = "testRunTime", default, skip_serializing_if = "Option::is_none")]
    pub test_run_time: Option<String>,
    #[serde(rename = "isDownloadDataAvailable", default, skip_serializing_if = "Option::is_none")]
    pub is_download_data_available: Option<bool>,
    #[serde(rename = "isVideoAvailable", default, skip_serializing_if = "Option::is_none")]
    pub is_video_available: Option<bool>,
    #[serde(rename = "executionStatus", default, skip_serializing_if = "Option::is_none")]
    pub execution_status: Option<TestExecutionStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grade: Option<TestGrade>,
    #[serde(rename = "kbNumber", default, skip_serializing_if = "Option::is_none")]
    pub kb_number: Option<String>,
    #[serde(rename = "packageVersion", default, skip_serializing_if = "Option::is_none")]
    pub package_version: Option<String>,
    #[serde(rename = "analysisSummaries", default, skip_serializing_if = "Vec::is_empty")]
    pub analysis_summaries: Vec<TestResultAnalysisSummary>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestResultAnalysisSummary {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "analysisStatus", default, skip_serializing_if = "Option::is_none")]
    pub analysis_status: Option<test_result_analysis_summary::AnalysisStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grade: Option<TestGrade>,
}
pub mod test_result_analysis_summary {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AnalysisStatus {
        None,
        Completed,
        InProgress,
        Failed,
        Succeeded,
        Available,
        NotAvailable,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OsUpdateListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OsUpdateResource>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OsUpdateResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(rename = "systemData", skip_serializing)]
    pub system_data: Option<SystemData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OsUpdateProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OsUpdateProperties {
    #[serde(rename = "osName", default, skip_serializing_if = "Option::is_none")]
    pub os_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub release: Option<String>,
    #[serde(rename = "flightingRing", default, skip_serializing_if = "Option::is_none")]
    pub flighting_ring: Option<String>,
    #[serde(rename = "buildVersion", default, skip_serializing_if = "Option::is_none")]
    pub build_version: Option<String>,
    #[serde(rename = "buildRevision", default, skip_serializing_if = "Option::is_none")]
    pub build_revision: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<os_update_properties::Type>,
    #[serde(rename = "releaseVersionDate", default, skip_serializing_if = "Option::is_none")]
    pub release_version_date: Option<String>,
}
pub mod os_update_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SecurityUpdate,
        FeatureUpdate,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FavoriteProcessListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<FavoriteProcessResource>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FavoriteProcessResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(rename = "systemData", skip_serializing)]
    pub system_data: Option<SystemData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<FavoriteProcessProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FavoriteProcessProperties {
    #[serde(rename = "actualProcessName")]
    pub actual_process_name: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnalysisResultListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AnalysisResultSingletonResource>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnalysisResultSingletonResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(rename = "systemData", skip_serializing)]
    pub system_data: Option<SystemData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AnalysisResultSingletonResourceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnalysisResultSingletonResourceProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grade: Option<TestGrade>,
    #[serde(rename = "analysisResultType")]
    pub analysis_result_type: analysis_result_singleton_resource_properties::AnalysisResultType,
}
pub mod analysis_result_singleton_resource_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AnalysisResultType {
        ScriptExecution,
        Reliability,
        #[serde(rename = "CPUUtilization")]
        CpuUtilization,
        MemoryUtilization,
        #[serde(rename = "CPURegression")]
        CpuRegression,
        MemoryRegression,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScriptExecutionResultSingletonResourceProperties {
    #[serde(flatten)]
    pub analysis_result_singleton_resource_properties: AnalysisResultSingletonResourceProperties,
    #[serde(rename = "scriptExecutionResults", default, skip_serializing_if = "Vec::is_empty")]
    pub script_execution_results: Vec<ScriptExecutionResult>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScriptExecutionResult {
    #[serde(rename = "scriptName", default, skip_serializing_if = "Option::is_none")]
    pub script_name: Option<String>,
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "exitCode", default, skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
    #[serde(rename = "timeOut", default, skip_serializing_if = "Option::is_none")]
    pub time_out: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReliabilityResultSingletonResourceProperties {
    #[serde(flatten)]
    pub analysis_result_singleton_resource_properties: AnalysisResultSingletonResourceProperties,
    #[serde(rename = "reliabilityResults", default, skip_serializing_if = "Vec::is_empty")]
    pub reliability_results: Vec<ReliabilityResult>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReliabilityResult {
    #[serde(rename = "fileName", default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(rename = "launchCount", default, skip_serializing_if = "Option::is_none")]
    pub launch_count: Option<i32>,
    #[serde(rename = "crashCount", default, skip_serializing_if = "Option::is_none")]
    pub crash_count: Option<i32>,
    #[serde(rename = "hangCount", default, skip_serializing_if = "Option::is_none")]
    pub hang_count: Option<i32>,
    #[serde(rename = "regressionGrade", default, skip_serializing_if = "Option::is_none")]
    pub regression_grade: Option<TestGrade>,
    #[serde(rename = "crashRegressionGrade", default, skip_serializing_if = "Option::is_none")]
    pub crash_regression_grade: Option<TestGrade>,
    #[serde(rename = "crashRegressionTestDetails", default, skip_serializing_if = "Option::is_none")]
    pub crash_regression_test_details: Option<RegressionTestDetails>,
    #[serde(rename = "hangRegressionGrade", default, skip_serializing_if = "Option::is_none")]
    pub hang_regression_grade: Option<TestGrade>,
    #[serde(rename = "hangRegressionTestDetails", default, skip_serializing_if = "Option::is_none")]
    pub hang_regression_test_details: Option<RegressionTestDetails>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegressionTestDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diff: Option<f64>,
    #[serde(rename = "isRegressed", default, skip_serializing_if = "Option::is_none")]
    pub is_regressed: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CpuUtilizationResultSingletonResourceProperties {
    #[serde(flatten)]
    pub analysis_result_singleton_resource_properties: AnalysisResultSingletonResourceProperties,
    #[serde(rename = "cpuUtilizationResults", default, skip_serializing_if = "Vec::is_empty")]
    pub cpu_utilization_results: Vec<UtilizationResult>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MemoryUtilizationResultSingletonResourceProperties {
    #[serde(flatten)]
    pub analysis_result_singleton_resource_properties: AnalysisResultSingletonResourceProperties,
    #[serde(rename = "memoryUtilizationResults", default, skip_serializing_if = "Vec::is_empty")]
    pub memory_utilization_results: Vec<UtilizationResult>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UtilizationResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process: Option<String>,
    #[serde(rename = "upperBound", default, skip_serializing_if = "Option::is_none")]
    pub upper_bound: Option<UtilizationBound>,
    #[serde(rename = "lowerBound", default, skip_serializing_if = "Option::is_none")]
    pub lower_bound: Option<UtilizationBound>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub utilization: Vec<UtilizationEntry>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UtilizationBound {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percentile: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UtilizationEntry {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CpuRegressionResultSingletonResourceProperties {
    #[serde(flatten)]
    pub analysis_result_singleton_resource_properties: AnalysisResultSingletonResourceProperties,
    #[serde(rename = "cpuRegressionResults", default, skip_serializing_if = "Vec::is_empty")]
    pub cpu_regression_results: Vec<RegressionResult>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MemoryRegressionResultSingletonResourceProperties {
    #[serde(flatten)]
    pub analysis_result_singleton_resource_properties: AnalysisResultSingletonResourceProperties,
    #[serde(rename = "memoryRegressionResults", default, skip_serializing_if = "Vec::is_empty")]
    pub memory_regression_results: Vec<RegressionResult>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegressionResult {
    #[serde(rename = "fileName", default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grade: Option<TestGrade>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diff: Option<f64>,
    #[serde(rename = "isRegressed", default, skip_serializing_if = "Option::is_none")]
    pub is_regressed: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "isDataAction", skip_serializing)]
    pub is_data_action: Option<bool>,
    #[serde(skip_serializing)]
    pub origin: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationProperties>,
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Display {
        #[serde(skip_serializing)]
        pub provider: Option<String>,
        #[serde(skip_serializing)]
        pub operation: Option<String>,
        #[serde(skip_serializing)]
        pub resource: Option<String>,
        #[serde(skip_serializing)]
        pub description: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationProperties {}
