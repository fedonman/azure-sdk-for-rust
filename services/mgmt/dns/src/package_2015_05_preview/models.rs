#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[doc = "An A record."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ARecord {
    #[doc = "Gets or sets the IPv4 address of this A record in string notation."]
    #[serde(rename = "ipv4Address", default, skip_serializing_if = "Option::is_none")]
    pub ipv4_address: Option<String>,
}
impl ARecord {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An AAAA record."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AaaaRecord {
    #[doc = "Gets or sets the IPv6 address of this AAAA record in string notation."]
    #[serde(rename = "ipv6Address", default, skip_serializing_if = "Option::is_none")]
    pub ipv6_address: Option<String>,
}
impl AaaaRecord {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An error response from the service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "An error response from the service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<CloudErrorBody>,
}
impl CloudError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An error response from the service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudErrorBody {
    #[doc = "An identifier for the error. Codes are invariant and are intended to be consumed programmatically."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "A message describing the error, intended to be suitable for display in a user interface."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The target of the particular error. For example, the name of the property in error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "A list of additional details about the error."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CloudErrorBody>,
}
impl CloudErrorBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A CNAME record."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CnameRecord {
    #[doc = "Gets or sets the canonical name for this record without a terminating dot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cname: Option<String>,
}
impl CnameRecord {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An MX record."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MxRecord {
    #[doc = "Gets or sets the preference metric for this record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preference: Option<i32>,
    #[doc = "Gets or sets the domain name of the mail host, without a terminating dot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exchange: Option<String>,
}
impl MxRecord {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An NS record."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NsRecord {
    #[doc = "Gets or sets the name server name for this record, without a terminating dot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nsdname: Option<String>,
}
impl NsRecord {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A PTR record."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PtrRecord {
    #[doc = "Gets or sets the PTR target domain name for this record without a terminating dot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ptrdname: Option<String>,
}
impl PtrRecord {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a DNS record set (a collection of DNS records with the same name and type)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecordSet {
    #[doc = "The ID of the record set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the record set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the record set."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The etag of the record set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Represents the properties of the records in the RecordSet."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RecordSetProperties>,
}
impl RecordSet {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response to a RecordSet List operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecordSetListResult {
    #[doc = "Gets or sets information about the RecordSets in the response."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RecordSet>,
    #[doc = "Gets or sets the continuation token for the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl RecordSetListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the properties of the records in the RecordSet."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecordSetProperties {
    #[doc = "Gets or sets the TTL of the records in the RecordSet."]
    #[serde(rename = "TTL", default, skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i64>,
    #[doc = "Fully qualified domain name of the record set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[doc = "Gets or sets the list of A records in the RecordSet."]
    #[serde(rename = "ARecords", default, skip_serializing_if = "Vec::is_empty")]
    pub a_records: Vec<ARecord>,
    #[doc = "Gets or sets the list of AAAA records in the RecordSet."]
    #[serde(rename = "AAAARecords", default, skip_serializing_if = "Vec::is_empty")]
    pub aaaa_records: Vec<AaaaRecord>,
    #[doc = "Gets or sets the list of MX records in the RecordSet."]
    #[serde(rename = "MXRecords", default, skip_serializing_if = "Vec::is_empty")]
    pub mx_records: Vec<MxRecord>,
    #[doc = "Gets or sets the list of NS records in the RecordSet."]
    #[serde(rename = "NSRecords", default, skip_serializing_if = "Vec::is_empty")]
    pub ns_records: Vec<NsRecord>,
    #[doc = "Gets or sets the list of PTR records in the RecordSet."]
    #[serde(rename = "PTRRecords", default, skip_serializing_if = "Vec::is_empty")]
    pub ptr_records: Vec<PtrRecord>,
    #[doc = "Gets or sets the list of SRV records in the RecordSet."]
    #[serde(rename = "SRVRecords", default, skip_serializing_if = "Vec::is_empty")]
    pub srv_records: Vec<SrvRecord>,
    #[doc = "Gets or sets the list of TXT records in the RecordSet."]
    #[serde(rename = "TXTRecords", default, skip_serializing_if = "Vec::is_empty")]
    pub txt_records: Vec<TxtRecord>,
    #[doc = "A CNAME record."]
    #[serde(rename = "CNAMERecord", default, skip_serializing_if = "Option::is_none")]
    pub cname_record: Option<CnameRecord>,
    #[doc = "An SOA record."]
    #[serde(rename = "SOARecord", default, skip_serializing_if = "Option::is_none")]
    pub soa_record: Option<SoaRecord>,
}
impl RecordSetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Common fields that are returned in the response for all Azure Resource Manager resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Fully qualified resource ID for the resource. Ex - /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource. E.g. \"Microsoft.Compute/virtualMachines\" or \"Microsoft.Storage/storageAccounts\""]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An SOA record."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SoaRecord {
    #[doc = "Gets or sets the domain name of the authoritative name server, without a terminating dot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[doc = "Gets or sets the email for this record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "Gets or sets the serial number for this record."]
    #[serde(rename = "serialNumber", default, skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<i64>,
    #[doc = "Gets or sets the refresh value for this record."]
    #[serde(rename = "refreshTime", default, skip_serializing_if = "Option::is_none")]
    pub refresh_time: Option<i64>,
    #[doc = "Gets or sets the retry time for this record."]
    #[serde(rename = "retryTime", default, skip_serializing_if = "Option::is_none")]
    pub retry_time: Option<i64>,
    #[doc = "Gets or sets the expire time for this record."]
    #[serde(rename = "expireTime", default, skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<i64>,
    #[doc = "Gets or sets the minimum TTL value for this record."]
    #[serde(rename = "minimumTTL", default, skip_serializing_if = "Option::is_none")]
    pub minimum_ttl: Option<i64>,
}
impl SoaRecord {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An SRV record."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SrvRecord {
    #[doc = "Gets or sets the priority metric for this record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[doc = "Gets or sets the weight metric for this record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    #[doc = "Gets or sets the port of the service for this record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[doc = "Gets or sets the domain name of the target for this record, without a terminating dot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
impl SrvRecord {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubResource {
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl SubResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource model definition for an Azure Resource Manager tracked top level resource which has 'tags' and a 'location'"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The geo-location where the resource lives"]
    pub location: String,
}
impl TrackedResource {
    pub fn new(location: String) -> Self {
        Self {
            resource: Resource::default(),
            tags: None,
            location,
        }
    }
}
#[doc = "A TXT record."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TxtRecord {
    #[doc = "Gets or sets the text value of this record."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<String>,
}
impl TxtRecord {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a DNS zone."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Zone {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Gets or sets the ETag of the zone that is being updated, as received from a Get operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Represents the properties of the zone."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ZoneProperties>,
}
impl Zone {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            etag: None,
            properties: None,
        }
    }
}
#[doc = "The response to a Zone List or ListAll operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ZoneListResult {
    #[doc = "Gets or sets information about the zones in the response."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Zone>,
    #[doc = "Gets or sets the continuation token for the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ZoneListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the properties of the zone."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ZoneProperties {
    #[doc = "Gets or sets the maximum number of record sets that can be created in this zone."]
    #[serde(rename = "maxNumberOfRecordSets", default, skip_serializing_if = "Option::is_none")]
    pub max_number_of_record_sets: Option<i64>,
    #[doc = "The maximum number of records per record set that can be created in this DNS zone.  This is a read-only property and any attempt to set this value will be ignored."]
    #[serde(rename = "maxNumberOfRecordsPerRecordSet", default, skip_serializing_if = "Option::is_none")]
    pub max_number_of_records_per_record_set: Option<i64>,
    #[doc = "Gets or sets the current number of record sets in this zone."]
    #[serde(rename = "numberOfRecordSets", default, skip_serializing_if = "Option::is_none")]
    pub number_of_record_sets: Option<i64>,
}
impl ZoneProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
