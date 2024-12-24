use serde::{Deserialize, Serialize};




/// An AdvanceRelocateBucketOperation request.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvanceRelocateBucketOperationRequest {
    
    
    
    /// Specifies the time when the relocation will revert to the sync stage if the relocation hasn't succeeded.
    
    #[serde(rename = "expireTime", skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<String>,
    
    
    /// Specifies the duration after which the relocation will revert to the sync stage if the relocation hasn't succeeded. Optional, if not supplied, a default value of 12h will be used.
    
    #[serde(rename = "ttl", skip_serializing_if = "Option::is_none")]
    pub ttl: Option<String>,
    
    
}

impl AdvanceRelocateBucketOperationRequest {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for AdvanceRelocateBucketOperationRequest {
    fn default() -> Self {
        Self {
            
            
            expire_time: None,
            
            ttl: None,
            
            
        }
    }
}



/// An Anywhere Cache instance.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnywhereCache {
    
    
    
    /// The cache-level entry admission policy.
    
    #[serde(rename = "admissionPolicy", skip_serializing_if = "Option::is_none")]
    pub admission_policy: Option<String>,
    
    
    /// The ID of the Anywhere cache instance.
    
    #[serde(rename = "anywhereCacheId", skip_serializing_if = "Option::is_none")]
    pub anywhere_cache_id: Option<String>,
    
    
    /// The name of the bucket containing this cache instance.
    
    #[serde(rename = "bucket", skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    
    
    /// The creation time of the cache instance in RFC 3339 format.
    
    #[serde(rename = "createTime", skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    
    
    /// The ID of the resource, including the project number, bucket name and anywhere cache ID.
    
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    
    
    /// The kind of item this is. For Anywhere Cache, this is always storage#anywhereCache.
    
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    
    
    /// True if the cache instance has an active Update long-running operation.
    
    #[serde(rename = "pendingUpdate", skip_serializing_if = "Option::is_none")]
    pub pending_update: Option<String>,
    
    
    /// The link to this cache instance.
    
    #[serde(rename = "selfLink", skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
    
    
    /// The current state of the cache instance.
    
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    
    
    /// The TTL of all cache entries in whole seconds. e.g., "7200s".
    
    #[serde(rename = "ttl", skip_serializing_if = "Option::is_none")]
    pub ttl: Option<String>,
    
    
    /// The modification time of the cache instance metadata in RFC 3339 format.
    
    #[serde(rename = "updateTime", skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    
    
    /// The zone in which the cache instance is running. For example, us-central1-a.
    
    #[serde(rename = "zone", skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
    
    
}

impl AnywhereCache {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for AnywhereCache {
    fn default() -> Self {
        Self {
            
            
            admission_policy: None,
            
            anywhere_cache_id: None,
            
            bucket: None,
            
            create_time: None,
            
            id: None,
            
            kind: None,
            
            pending_update: None,
            
            self_link: None,
            
            state: None,
            
            ttl: None,
            
            update_time: None,
            
            zone: None,
            
            
        }
    }
}



/// A list of Anywhere Caches.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnywhereCaches {
    
    
    
    /// The list of items.
    
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<String>,
    
    
    /// The kind of item this is. For lists of Anywhere Caches, this is always storage#anywhereCaches.
    
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    
    
    /// The continuation token, used to page through large result sets. Provide this value in a subsequent request to return the next page of results.
    
    #[serde(rename = "nextPageToken", skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    
    
}

impl AnywhereCaches {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for AnywhereCaches {
    fn default() -> Self {
        Self {
            
            
            items: None,
            
            kind: None,
            
            next_page_token: None,
            
            
        }
    }
}



/// A bucket.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bucket {
    
    
    
    /// Access controls on the bucket.
    
    #[serde(rename = "acl", skip_serializing_if = "Option::is_none")]
    pub acl: Option<String>,
    
    
    /// The bucket's Autoclass configuration.
    
    #[serde(rename = "autoclass", skip_serializing_if = "Option::is_none")]
    pub autoclass: Option<String>,
    
    
    /// The bucket's billing configuration.
    
    #[serde(rename = "billing", skip_serializing_if = "Option::is_none")]
    pub billing: Option<String>,
    
    
    /// The bucket's Cross-Origin Resource Sharing (CORS) configuration.
    
    #[serde(rename = "cors", skip_serializing_if = "Option::is_none")]
    pub cors: Option<String>,
    
    
    /// The bucket's custom placement configuration for Custom Dual Regions.
    
    #[serde(rename = "customPlacementConfig", skip_serializing_if = "Option::is_none")]
    pub custom_placement_config: Option<String>,
    
    
    /// The default value for event-based hold on newly created objects in this bucket. Event-based hold is a way to retain objects indefinitely until an event occurs, signified by the hold's release. After being released, such objects will be subject to bucket-level retention (if any). One sample use case of this flag is for banks to hold loan documents for at least 3 years after loan is paid in full. Here, bucket-level retention is 3 years and the event is loan being paid in full. In this example, these objects will be held intact for any number of years until the event has occurred (event-based hold on the object is released) and then 3 more years after that. That means retention duration of the objects begins from the moment event-based hold transitioned from true to false. Objects under event-based hold cannot be deleted, overwritten or archived until the hold is removed.
    
    #[serde(rename = "defaultEventBasedHold", skip_serializing_if = "Option::is_none")]
    pub default_event_based_hold: Option<String>,
    
    
    /// Default access controls to apply to new objects when no ACL is provided.
    
    #[serde(rename = "defaultObjectAcl", skip_serializing_if = "Option::is_none")]
    pub default_object_acl: Option<String>,
    
    
    /// Encryption configuration for a bucket.
    
    #[serde(rename = "encryption", skip_serializing_if = "Option::is_none")]
    pub encryption: Option<String>,
    
    
    /// HTTP 1.1 Entity tag for the bucket.
    
    #[serde(rename = "etag", skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    
    
    /// The generation of this bucket.
    
    #[serde(rename = "generation", skip_serializing_if = "Option::is_none")]
    pub generation: Option<String>,
    
    
    /// The hard delete time of the bucket in RFC 3339 format.
    
    #[serde(rename = "hardDeleteTime", skip_serializing_if = "Option::is_none")]
    pub hard_delete_time: Option<String>,
    
    
    /// The bucket's hierarchical namespace configuration.
    
    #[serde(rename = "hierarchicalNamespace", skip_serializing_if = "Option::is_none")]
    pub hierarchical_namespace: Option<String>,
    
    
    /// The bucket's IAM configuration.
    
    #[serde(rename = "iamConfiguration", skip_serializing_if = "Option::is_none")]
    pub iam_configuration: Option<String>,
    
    
    /// The ID of the bucket. For buckets, the id and name properties are the same.
    
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    
    
    /// The bucket's IP filter configuration. Specifies the network sources that are allowed to access the operations on the bucket, as well as its underlying objects. Only enforced when the mode is set to 'Enabled'.
    
    #[serde(rename = "ipFilter", skip_serializing_if = "Option::is_none")]
    pub ip_filter: Option<String>,
    
    
    /// The kind of item this is. For buckets, this is always storage#bucket.
    
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    
    
    /// User-provided labels, in key/value pairs.
    
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<String>,
    
    
    /// The bucket's lifecycle configuration. See [Lifecycle Management](https://cloud.google.com/storage/docs/lifecycle) for more information.
    
    #[serde(rename = "lifecycle", skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<String>,
    
    
    /// The location of the bucket. Object data for objects in the bucket resides in physical storage within this region. Defaults to US. See the [Developer's Guide](https://cloud.google.com/storage/docs/locations) for the authoritative list.
    
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    
    
    /// The type of the bucket location.
    
    #[serde(rename = "locationType", skip_serializing_if = "Option::is_none")]
    pub location_type: Option<String>,
    
    
    /// The bucket's logging configuration, which defines the destination bucket and optional name prefix for the current bucket's logs.
    
    #[serde(rename = "logging", skip_serializing_if = "Option::is_none")]
    pub logging: Option<String>,
    
    
    /// The metadata generation of this bucket.
    
    #[serde(rename = "metageneration", skip_serializing_if = "Option::is_none")]
    pub metageneration: Option<String>,
    
    
    /// The name of the bucket.
    
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    
    
    /// The bucket's object retention config.
    
    #[serde(rename = "objectRetention", skip_serializing_if = "Option::is_none")]
    pub object_retention: Option<String>,
    
    
    /// The owner of the bucket. This is always the project team's owner group.
    
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    
    
    /// The project number of the project the bucket belongs to.
    
    #[serde(rename = "projectNumber", skip_serializing_if = "Option::is_none")]
    pub project_number: Option<String>,
    
    
    /// The bucket's retention policy. The retention policy enforces a minimum retention time for all objects contained in the bucket, based on their creation time. Any attempt to overwrite or delete objects younger than the retention period will result in a PERMISSION_DENIED error. An unlocked retention policy can be modified or removed from the bucket via a storage.buckets.update operation. A locked retention policy cannot be removed or shortened in duration for the lifetime of the bucket. Attempting to remove or decrease period of a locked retention policy will result in a PERMISSION_DENIED error.
    
    #[serde(rename = "retentionPolicy", skip_serializing_if = "Option::is_none")]
    pub retention_policy: Option<String>,
    
    
    /// The Recovery Point Objective (RPO) of this bucket. Set to ASYNC_TURBO to turn on Turbo Replication on a bucket.
    
    #[serde(rename = "rpo", skip_serializing_if = "Option::is_none")]
    pub rpo: Option<String>,
    
    
    /// Reserved for future use.
    
    #[serde(rename = "satisfiesPZI", skip_serializing_if = "Option::is_none")]
    pub satisfies_pzi: Option<String>,
    
    
    /// Reserved for future use.
    
    #[serde(rename = "satisfiesPZS", skip_serializing_if = "Option::is_none")]
    pub satisfies_pzs: Option<String>,
    
    
    /// The URI of this bucket.
    
    #[serde(rename = "selfLink", skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
    
    
    /// The bucket's soft delete policy, which defines the period of time that soft-deleted objects will be retained, and cannot be permanently deleted.
    
    #[serde(rename = "softDeletePolicy", skip_serializing_if = "Option::is_none")]
    pub soft_delete_policy: Option<String>,
    
    
    /// The soft delete time of the bucket in RFC 3339 format.
    
    #[serde(rename = "softDeleteTime", skip_serializing_if = "Option::is_none")]
    pub soft_delete_time: Option<String>,
    
    
    /// The bucket's default storage class, used whenever no storageClass is specified for a newly-created object. This defines how objects in the bucket are stored and determines the SLA and the cost of storage. Values include MULTI_REGIONAL, REGIONAL, STANDARD, NEARLINE, COLDLINE, ARCHIVE, and DURABLE_REDUCED_AVAILABILITY. If this value is not specified when the bucket is created, it will default to STANDARD. For more information, see [Storage Classes](https://cloud.google.com/storage/docs/storage-classes).
    
    #[serde(rename = "storageClass", skip_serializing_if = "Option::is_none")]
    pub storage_class: Option<String>,
    
    
    /// The creation time of the bucket in RFC 3339 format.
    
    #[serde(rename = "timeCreated", skip_serializing_if = "Option::is_none")]
    pub time_created: Option<String>,
    
    
    /// The modification time of the bucket in RFC 3339 format.
    
    #[serde(rename = "updated", skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    
    
    /// The bucket's versioning configuration.
    
    #[serde(rename = "versioning", skip_serializing_if = "Option::is_none")]
    pub versioning: Option<String>,
    
    
    /// The bucket's website configuration, controlling how the service behaves when accessing bucket contents as a web site. See the [Static Website Examples](https://cloud.google.com/storage/docs/static-website) for more information.
    
    #[serde(rename = "website", skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    
    
}

impl Bucket {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Bucket {
    fn default() -> Self {
        Self {
            
            
            acl: None,
            
            autoclass: None,
            
            billing: None,
            
            cors: None,
            
            custom_placement_config: None,
            
            default_event_based_hold: None,
            
            default_object_acl: None,
            
            encryption: None,
            
            etag: None,
            
            generation: None,
            
            hard_delete_time: None,
            
            hierarchical_namespace: None,
            
            iam_configuration: None,
            
            id: None,
            
            ip_filter: None,
            
            kind: None,
            
            labels: None,
            
            lifecycle: None,
            
            location: None,
            
            location_type: None,
            
            logging: None,
            
            metageneration: None,
            
            name: None,
            
            object_retention: None,
            
            owner: None,
            
            project_number: None,
            
            retention_policy: None,
            
            rpo: None,
            
            satisfies_pzi: None,
            
            satisfies_pzs: None,
            
            self_link: None,
            
            soft_delete_policy: None,
            
            soft_delete_time: None,
            
            storage_class: None,
            
            time_created: None,
            
            updated: None,
            
            versioning: None,
            
            website: None,
            
            
        }
    }
}



/// An access-control entry.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BucketAccessControl {
    
    
    
    /// The name of the bucket.
    
    #[serde(rename = "bucket", skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    
    
    /// The domain associated with the entity, if any.
    
    #[serde(rename = "domain", skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    
    
    /// The email address associated with the entity, if any.
    
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    
    
    /// The entity holding the permission, in one of the following forms: 
user-userId 
user-email 
group-groupId 
group-email 
domain-domain 
project-team-projectId 
allUsers 
allAuthenticatedUsers Examples: 
The user liz@example.com would be user-liz@example.com. 
The group example@googlegroups.com would be group-example@googlegroups.com. 
To refer to all members of the Google Apps for Business domain example.com, the entity would be domain-example.com.
    
    #[serde(rename = "entity", skip_serializing_if = "Option::is_none")]
    pub entity: Option<String>,
    
    
    /// The ID for the entity, if any.
    
    #[serde(rename = "entityId", skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    
    
    /// HTTP 1.1 Entity tag for the access-control entry.
    
    #[serde(rename = "etag", skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    
    
    /// The ID of the access-control entry.
    
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    
    
    /// The kind of item this is. For bucket access control entries, this is always storage#bucketAccessControl.
    
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    
    
    /// The project team associated with the entity, if any.
    
    #[serde(rename = "projectTeam", skip_serializing_if = "Option::is_none")]
    pub project_team: Option<String>,
    
    
    /// The access permission for the entity.
    
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    
    
    /// The link to this access-control entry.
    
    #[serde(rename = "selfLink", skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
    
    
}

impl BucketAccessControl {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for BucketAccessControl {
    fn default() -> Self {
        Self {
            
            
            bucket: None,
            
            domain: None,
            
            email: None,
            
            entity: None,
            
            entity_id: None,
            
            etag: None,
            
            id: None,
            
            kind: None,
            
            project_team: None,
            
            role: None,
            
            self_link: None,
            
            
        }
    }
}



/// An access-control list.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BucketAccessControls {
    
    
    
    /// The list of items.
    
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<String>,
    
    
    /// The kind of item this is. For lists of bucket access control entries, this is always storage#bucketAccessControls.
    
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    
    
}

impl BucketAccessControls {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for BucketAccessControls {
    fn default() -> Self {
        Self {
            
            
            items: None,
            
            kind: None,
            
            
        }
    }
}



/// The storage layout configuration of a bucket.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BucketStorageLayout {
    
    
    
    /// The name of the bucket.
    
    #[serde(rename = "bucket", skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    
    
    /// The bucket's custom placement configuration for Custom Dual Regions.
    
    #[serde(rename = "customPlacementConfig", skip_serializing_if = "Option::is_none")]
    pub custom_placement_config: Option<String>,
    
    
    /// The bucket's hierarchical namespace configuration.
    
    #[serde(rename = "hierarchicalNamespace", skip_serializing_if = "Option::is_none")]
    pub hierarchical_namespace: Option<String>,
    
    
    /// The kind of item this is. For storage layout, this is always storage#storageLayout.
    
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    
    
    /// The location of the bucket.
    
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    
    
    /// The type of the bucket location.
    
    #[serde(rename = "locationType", skip_serializing_if = "Option::is_none")]
    pub location_type: Option<String>,
    
    
}

impl BucketStorageLayout {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for BucketStorageLayout {
    fn default() -> Self {
        Self {
            
            
            bucket: None,
            
            custom_placement_config: None,
            
            hierarchical_namespace: None,
            
            kind: None,
            
            location: None,
            
            location_type: None,
            
            
        }
    }
}



/// A list of buckets.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Buckets {
    
    
    
    /// The list of items.
    
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<String>,
    
    
    /// The kind of item this is. For lists of buckets, this is always storage#buckets.
    
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    
    
    /// The continuation token, used to page through large result sets. Provide this value in a subsequent request to return the next page of results.
    
    #[serde(rename = "nextPageToken", skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    
    
}

impl Buckets {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Buckets {
    fn default() -> Self {
        Self {
            
            
            items: None,
            
            kind: None,
            
            next_page_token: None,
            
            
        }
    }
}



/// A bulk restore objects request.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkRestoreObjectsRequest {
    
    
    
    /// If false (default), the restore will not overwrite live objects with the same name at the destination. This means some deleted objects may be skipped. If true, live objects will be overwritten resulting in a noncurrent object (if versioning is enabled). If versioning is not enabled, overwriting the object will result in a soft-deleted object. In either case, if a noncurrent object already exists with the same name, a live version can be written without issue.
    
    #[serde(rename = "allowOverwrite", skip_serializing_if = "Option::is_none")]
    pub allow_overwrite: Option<String>,
    
    
    /// If true, copies the source object's ACL; otherwise, uses the bucket's default object ACL. The default is false.
    
    #[serde(rename = "copySourceAcl", skip_serializing_if = "Option::is_none")]
    pub copy_source_acl: Option<String>,
    
    
    /// Restores only the objects matching any of the specified glob(s). If this parameter is not specified, all objects will be restored within the specified time range.
    
    #[serde(rename = "matchGlobs", skip_serializing_if = "Option::is_none")]
    pub match_globs: Option<String>,
    
    
    /// Restores only the objects that were soft-deleted after this time.
    
    #[serde(rename = "softDeletedAfterTime", skip_serializing_if = "Option::is_none")]
    pub soft_deleted_after_time: Option<String>,
    
    
    /// Restores only the objects that were soft-deleted before this time.
    
    #[serde(rename = "softDeletedBeforeTime", skip_serializing_if = "Option::is_none")]
    pub soft_deleted_before_time: Option<String>,
    
    
}

impl BulkRestoreObjectsRequest {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for BulkRestoreObjectsRequest {
    fn default() -> Self {
        Self {
            
            
            allow_overwrite: None,
            
            copy_source_acl: None,
            
            match_globs: None,
            
            soft_deleted_after_time: None,
            
            soft_deleted_before_time: None,
            
            
        }
    }
}



/// An notification channel used to watch for resource changes.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    
    
    
    /// The address where notifications are delivered for this channel.
    
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    
    
    /// Date and time of notification channel expiration, expressed as a Unix timestamp, in milliseconds. Optional.
    
    #[serde(rename = "expiration", skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
    
    
    /// A UUID or similar unique string that identifies this channel.
    
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    
    
    /// Identifies this as a notification channel used to watch for changes to a resource, which is "api#channel".
    
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    
    
    /// Additional parameters controlling delivery channel behavior. Optional.
    
    #[serde(rename = "params", skip_serializing_if = "Option::is_none")]
    pub params: Option<String>,
    
    
    /// A Boolean value to indicate whether payload is wanted. Optional.
    
    #[serde(rename = "payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
    
    
    /// An opaque ID that identifies the resource being watched on this channel. Stable across different API versions.
    
    #[serde(rename = "resourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    
    
    /// A version-specific identifier for the watched resource.
    
    #[serde(rename = "resourceUri", skip_serializing_if = "Option::is_none")]
    pub resource_uri: Option<String>,
    
    
    /// An arbitrary string delivered to the target address with each notification delivered over this channel. Optional.
    
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    
    
    /// The type of delivery mechanism used for this channel.
    
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    
    
}

impl Channel {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Channel {
    fn default() -> Self {
        Self {
            
            
            address: None,
            
            expiration: None,
            
            id: None,
            
            kind: None,
            
            params: None,
            
            payload: None,
            
            resource_id: None,
            
            resource_uri: None,
            
            token: None,
            
            r#type: None,
            
            
        }
    }
}



/// A Compose request.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComposeRequest {
    
    
    
    /// Properties of the resulting object.
    
    #[serde(rename = "destination", skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    
    
    /// The kind of item this is.
    
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    
    
    /// The list of source objects that will be concatenated into a single object.
    
    #[serde(rename = "sourceObjects", skip_serializing_if = "Option::is_none")]
    pub source_objects: Option<String>,
    
    
}

impl ComposeRequest {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for ComposeRequest {
    fn default() -> Self {
        Self {
            
            
            destination: None,
            
            kind: None,
            
            source_objects: None,
            
            
        }
    }
}



/// Represents an expression text. Example: title: "User account presence" description: "Determines whether the request has a user account" expression: "size(request.user) > 0"

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expr {
    
    
    
    /// An optional description of the expression. This is a longer text which describes the expression, e.g. when hovered over it in a UI.
    
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    
    /// Textual representation of an expression in Common Expression Language syntax. The application context of the containing message determines which well-known feature set of CEL is supported.
    
    #[serde(rename = "expression", skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    
    
    /// An optional string indicating the location of the expression for error reporting, e.g. a file name and a position in the file.
    
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    
    
    /// An optional title for the expression, i.e. a short string describing its purpose. This can be used e.g. in UIs which allow to enter the expression.
    
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    
    
}

impl Expr {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Expr {
    fn default() -> Self {
        Self {
            
            
            description: None,
            
            expression: None,
            
            location: None,
            
            title: None,
            
            
        }
    }
}



/// A folder. Only available in buckets with hierarchical namespace enabled.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Folder {
    
    
    
    /// The name of the bucket containing this folder.
    
    #[serde(rename = "bucket", skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    
    
    /// The creation time of the folder in RFC 3339 format.
    
    #[serde(rename = "createTime", skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    
    
    /// The ID of the folder, including the bucket name, folder name.
    
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    
    
    /// The kind of item this is. For folders, this is always storage#folder.
    
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    
    
    /// The version of the metadata for this folder. Used for preconditions and for detecting changes in metadata.
    
    #[serde(rename = "metageneration", skip_serializing_if = "Option::is_none")]
    pub metageneration: Option<String>,
    
    
    /// The name of the folder. Required if not specified by URL parameter.
    
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    
    
    /// Only present if the folder is part of an ongoing rename folder operation. Contains information which can be used to query the operation status.
    
    #[serde(rename = "pendingRenameInfo", skip_serializing_if = "Option::is_none")]
    pub pending_rename_info: Option<String>,
    
    
    /// The link to this folder.
    
    #[serde(rename = "selfLink", skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
    
    
    /// The modification time of the folder metadata in RFC 3339 format.
    
    #[serde(rename = "updateTime", skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    
    
}

impl Folder {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Folder {
    fn default() -> Self {
        Self {
            
            
            bucket: None,
            
            create_time: None,
            
            id: None,
            
            kind: None,
            
            metageneration: None,
            
            name: None,
            
            pending_rename_info: None,
            
            self_link: None,
            
            update_time: None,
            
            
        }
    }
}



/// A list of folders.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Folders {
    
    
    
    /// The list of items.
    
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<String>,
    
    
    /// The kind of item this is. For lists of folders, this is always storage#folders.
    
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    
    
    /// The continuation token, used to page through large result sets. Provide this value in a subsequent request to return the next page of results.
    
    #[serde(rename = "nextPageToken", skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    
    
}

impl Folders {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Folders {
    fn default() -> Self {
        Self {
            
            
            items: None,
            
            kind: None,
            
            next_page_token: None,
            
            
        }
    }
}



/// The response message for storage.buckets.operations.list.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleLongrunningListOperationsResponse {
    
    
    
    /// The kind of item this is. For lists of operations, this is always storage#operations.
    
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    
    
    /// The continuation token, used to page through large result sets. Provide this value in a subsequent request to return the next page of results.
    
    #[serde(rename = "nextPageToken", skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    
    
    /// A list of operations that matches the specified filter in the request.
    
    #[serde(rename = "operations", skip_serializing_if = "Option::is_none")]
    pub operations: Option<String>,
    
    
}

impl GoogleLongrunningListOperationsResponse {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for GoogleLongrunningListOperationsResponse {
    fn default() -> Self {
        Self {
            
            
            kind: None,
            
            next_page_token: None,
            
            operations: None,
            
            
        }
    }
}



/// This resource represents a long-running operation that is the result of a network API call.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleLongrunningOperation {
    
    
    
    /// If the value is "false", it means the operation is still in progress. If "true", the operation is completed, and either "error" or "response" is available.
    
    #[serde(rename = "done", skip_serializing_if = "Option::is_none")]
    pub done: Option<String>,
    
    
    /// The error result of the operation in case of failure or cancellation.
    
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    
    
    /// The kind of item this is. For operations, this is always storage#operation.
    
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    
    
    /// Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any.
    
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
    
    
    /// The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the "name" should be a resource name ending with "operations/{operationId}".
    
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    
    
    /// The normal response of the operation in case of success. If the original method returns no data on success, such as "Delete", the response is google.protobuf.Empty. If the original method is standard Get/Create/Update, the response should be the resource. For other methods, the response should have the type "XxxResponse", where "Xxx" is the original method name. For example, if the original method name is "TakeSnapshot()", the inferred response type is "TakeSnapshotResponse".
    
    #[serde(rename = "response", skip_serializing_if = "Option::is_none")]
    pub response: Option<String>,
    
    
    /// The link to this long running operation.
    
    #[serde(rename = "selfLink", skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
    
    
}

impl GoogleLongrunningOperation {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for GoogleLongrunningOperation {
    fn default() -> Self {
        Self {
            
            
            done: None,
            
            error: None,
            
            kind: None,
            
            metadata: None,
            
            name: None,
            
            response: None,
            
            self_link: None,
            
            
        }
    }
}



/// The "Status" type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each "Status" message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors).

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleRpcStatus {
    
    
    
    /// The status code, which should be an enum value of google.rpc.Code.
    
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    
    
    /// A list of messages that carry the error details. There is a common set of message types for APIs to use.
    
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    
    
    /// A developer-facing error message, which should be in English.
    
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    
    
}

impl GoogleRpcStatus {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for GoogleRpcStatus {
    fn default() -> Self {
        Self {
            
            
            code: None,
            
            details: None,
            
            message: None,
            
            
        }
    }
}



/// JSON template to produce a JSON-style HMAC Key resource for Create responses.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HmacKey {
    
    
    
    /// The kind of item this is. For HMAC keys, this is always storage#hmacKey.
    
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    
    
    /// Key metadata.
    
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
    
    
    /// HMAC secret key material.
    
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    
    
}

impl HmacKey {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for HmacKey {
    fn default() -> Self {
        Self {
            
            
            kind: None,
            
            metadata: None,
            
            secret: None,
            
            
        }
    }
}



/// JSON template to produce a JSON-style HMAC Key metadata resource.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HmacKeyMetadata {
    
    
    
    /// The ID of the HMAC Key.
    
    #[serde(rename = "accessId", skip_serializing_if = "Option::is_none")]
    pub access_id: Option<String>,
    
    
    /// HTTP 1.1 Entity tag for the HMAC key.
    
    #[serde(rename = "etag", skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    
    
    /// The ID of the HMAC key, including the Project ID and the Access ID.
    
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    
    
    /// The kind of item this is. For HMAC Key metadata, this is always storage#hmacKeyMetadata.
    
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    
    
    /// Project ID owning the service account to which the key authenticates.
    
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    
    
    /// The link to this resource.
    
    #[serde(rename = "selfLink", skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
    
    
    /// The email address of the key's associated service account.
    
    #[serde(rename = "serviceAccountEmail", skip_serializing_if = "Option::is_none")]
    pub service_account_email: Option<String>,
    
    
    /// The state of the key. Can be one of ACTIVE, INACTIVE, or DELETED.
    
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    
    
    /// The creation time of the HMAC key in RFC 3339 format.
    
    #[serde(rename = "timeCreated", skip_serializing_if = "Option::is_none")]
    pub time_created: Option<String>,
    
    
    /// The last modification time of the HMAC key metadata in RFC 3339 format.
    
    #[serde(rename = "updated", skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    
    
}

impl HmacKeyMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for HmacKeyMetadata {
    fn default() -> Self {
        Self {
            
            
            access_id: None,
            
            etag: None,
            
            id: None,
            
            kind: None,
            
            project_id: None,
            
            self_link: None,
            
            service_account_email: None,
            
            state: None,
            
            time_created: None,
            
            updated: None,
            
            
        }
    }
}



/// A list of hmacKeys.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HmacKeysMetadata {
    
    
    
    /// The list of items.
    
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<String>,
    
    
    /// The kind of item this is. For lists of hmacKeys, this is always storage#hmacKeysMetadata.
    
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    
    
    /// The continuation token, used to page through large result sets. Provide this value in a subsequent request to return the next page of results.
    
    #[serde(rename = "nextPageToken", skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    
    
}

impl HmacKeysMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for HmacKeysMetadata {
    fn default() -> Self {
        Self {
            
            
            items: None,
            
            kind: None,
            
            next_page_token: None,
            
            
        }
    }
}



/// A managed folder.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedFolder {
    
    
    
    /// The name of the bucket containing this managed folder.
    
    #[serde(rename = "bucket", skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    
    
    /// The creation time of the managed folder in RFC 3339 format.
    
    #[serde(rename = "createTime", skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    
    
    /// The ID of the managed folder, including the bucket name and managed folder name.
    
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    
    
    /// The kind of item this is. For managed folders, this is always storage#managedFolder.
    
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    
    
    /// The version of the metadata for this managed folder. Used for preconditions and for detecting changes in metadata.
    
    #[serde(rename = "metageneration", skip_serializing_if = "Option::is_none")]
    pub metageneration: Option<String>,
    
    
    /// The name of the managed folder. Required if not specified by URL parameter.
    
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    
    
    /// The link to this managed folder.
    
    #[serde(rename = "selfLink", skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
    
    
    /// The last update time of the managed folder metadata in RFC 3339 format.
    
    #[serde(rename = "updateTime", skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    
    
}

impl ManagedFolder {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for ManagedFolder {
    fn default() -> Self {
        Self {
            
            
            bucket: None,
            
            create_time: None,
            
            id: None,
            
            kind: None,
            
            metageneration: None,
            
            name: None,
            
            self_link: None,
            
            update_time: None,
            
            
        }
    }
}



/// A list of managed folders.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedFolders {
    
    
    
    /// The list of items.
    
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<String>,
    
    
    /// The kind of item this is. For lists of managed folders, this is always storage#managedFolders.
    
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    
    
    /// The continuation token, used to page through large result sets. Provide this value in a subsequent request to return the next page of results.
    
    #[serde(rename = "nextPageToken", skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    
    
}

impl ManagedFolders {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for ManagedFolders {
    fn default() -> Self {
        Self {
            
            
            items: None,
            
            kind: None,
            
            next_page_token: None,
            
            
        }
    }
}



/// A subscription to receive Google PubSub notifications.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    
    
    
    /// An optional list of additional attributes to attach to each Cloud PubSub message published for this notification subscription.
    
    #[serde(rename = "custom_attributes", skip_serializing_if = "Option::is_none")]
    pub custom_attributes: Option<String>,
    
    
    /// HTTP 1.1 Entity tag for this subscription notification.
    
    #[serde(rename = "etag", skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    
    
    /// If present, only send notifications about listed event types. If empty, sent notifications for all event types.
    
    #[serde(rename = "event_types", skip_serializing_if = "Option::is_none")]
    pub event_types: Option<String>,
    
    
    /// The ID of the notification.
    
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    
    
    /// The kind of item this is. For notifications, this is always storage#notification.
    
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    
    
    /// If present, only apply this notification configuration to object names that begin with this prefix.
    
    #[serde(rename = "object_name_prefix", skip_serializing_if = "Option::is_none")]
    pub object_name_prefix: Option<String>,
    
    
    /// The desired content of the Payload.
    
    #[serde(rename = "payload_format", skip_serializing_if = "Option::is_none")]
    pub payload_format: Option<String>,
    
    
    /// The canonical URL of this notification.
    
    #[serde(rename = "selfLink", skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
    
    
    /// The Cloud PubSub topic to which this subscription publishes. Formatted as: '//pubsub.googleapis.com/projects/{project-identifier}/topics/{my-topic}'
    
    #[serde(rename = "topic", skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    
    
}

impl Notification {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Notification {
    fn default() -> Self {
        Self {
            
            
            custom_attributes: None,
            
            etag: None,
            
            event_types: None,
            
            id: None,
            
            kind: None,
            
            object_name_prefix: None,
            
            payload_format: None,
            
            self_link: None,
            
            topic: None,
            
            
        }
    }
}



/// A list of notification subscriptions.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notifications {
    
    
    
    /// The list of items.
    
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<String>,
    
    
    /// The kind of item this is. For lists of notifications, this is always storage#notifications.
    
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    
    
}

impl Notifications {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Notifications {
    fn default() -> Self {
        Self {
            
            
            items: None,
            
            kind: None,
            
            
        }
    }
}



/// An object.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Object {
    
    
    
    /// Access controls on the object.
    
    #[serde(rename = "acl", skip_serializing_if = "Option::is_none")]
    pub acl: Option<String>,
    
    
    /// The name of the bucket containing this object.
    
    #[serde(rename = "bucket", skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    
    
    /// Cache-Control directive for the object data. If omitted, and the object is accessible to all anonymous users, the default will be public, max-age=3600.
    
    #[serde(rename = "cacheControl", skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<String>,
    
    
    /// Number of underlying components that make up this object. Components are accumulated by compose operations.
    
    #[serde(rename = "componentCount", skip_serializing_if = "Option::is_none")]
    pub component_count: Option<String>,
    
    
    /// Content-Disposition of the object data.
    
    #[serde(rename = "contentDisposition", skip_serializing_if = "Option::is_none")]
    pub content_disposition: Option<String>,
    
    
    /// Content-Encoding of the object data.
    
    #[serde(rename = "contentEncoding", skip_serializing_if = "Option::is_none")]
    pub content_encoding: Option<String>,
    
    
    /// Content-Language of the object data.
    
    #[serde(rename = "contentLanguage", skip_serializing_if = "Option::is_none")]
    pub content_language: Option<String>,
    
    
    /// Content-Type of the object data. If an object is stored without a Content-Type, it is served as application/octet-stream.
    
    #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    
    
    /// CRC32c checksum, as described in RFC 4960, Appendix B; encoded using base64 in big-endian byte order. For more information about using the CRC32c checksum, see [Data Validation and Change Detection](https://cloud.google.com/storage/docs/data-validation).
    
    #[serde(rename = "crc32c", skip_serializing_if = "Option::is_none")]
    pub crc_32_c: Option<String>,
    
    
    /// A timestamp in RFC 3339 format specified by the user for an object.
    
    #[serde(rename = "customTime", skip_serializing_if = "Option::is_none")]
    pub custom_time: Option<String>,
    
    
    /// Metadata of customer-supplied encryption key, if the object is encrypted by such a key.
    
    #[serde(rename = "customerEncryption", skip_serializing_if = "Option::is_none")]
    pub customer_encryption: Option<String>,
    
    
    /// HTTP 1.1 Entity tag for the object.
    
    #[serde(rename = "etag", skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    
    
    /// Whether an object is under event-based hold. Event-based hold is a way to retain objects until an event occurs, which is signified by the hold's release (i.e. this value is set to false). After being released (set to false), such objects will be subject to bucket-level retention (if any). One sample use case of this flag is for banks to hold loan documents for at least 3 years after loan is paid in full. Here, bucket-level retention is 3 years and the event is the loan being paid in full. In this example, these objects will be held intact for any number of years until the event has occurred (event-based hold on the object is released) and then 3 more years after that. That means retention duration of the objects begins from the moment event-based hold transitioned from true to false.
    
    #[serde(rename = "eventBasedHold", skip_serializing_if = "Option::is_none")]
    pub event_based_hold: Option<String>,
    
    
    /// The content generation of this object. Used for object versioning.
    
    #[serde(rename = "generation", skip_serializing_if = "Option::is_none")]
    pub generation: Option<String>,
    
    
    /// This is the time (in the future) when the soft-deleted object will no longer be restorable. It is equal to the soft delete time plus the current soft delete retention duration of the bucket.
    
    #[serde(rename = "hardDeleteTime", skip_serializing_if = "Option::is_none")]
    pub hard_delete_time: Option<String>,
    
    
    /// The ID of the object, including the bucket name, object name, and generation number.
    
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    
    
    /// The kind of item this is. For objects, this is always storage#object.
    
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    
    
    /// Not currently supported. Specifying the parameter causes the request to fail with status code 400 Bad Request.
    
    #[serde(rename = "kmsKeyName", skip_serializing_if = "Option::is_none")]
    pub kms_key_name: Option<String>,
    
    
    /// MD5 hash of the data; encoded using base64. For more information about using the MD5 hash, see [Data Validation and Change Detection](https://cloud.google.com/storage/docs/data-validation).
    
    #[serde(rename = "md5Hash", skip_serializing_if = "Option::is_none")]
    pub md_5_hash: Option<String>,
    
    
    /// Media download link.
    
    #[serde(rename = "mediaLink", skip_serializing_if = "Option::is_none")]
    pub media_link: Option<String>,
    
    
    /// User-provided metadata, in key/value pairs.
    
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
    
    
    /// The version of the metadata for this object at this generation. Used for preconditions and for detecting changes in metadata. A metageneration number is only meaningful in the context of a particular generation of a particular object.
    
    #[serde(rename = "metageneration", skip_serializing_if = "Option::is_none")]
    pub metageneration: Option<String>,
    
    
    /// The name of the object. Required if not specified by URL parameter.
    
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    
    
    /// The owner of the object. This will always be the uploader of the object.
    
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    
    
    /// Restore token used to differentiate deleted objects with the same name and generation. This field is only returned for deleted objects in hierarchical namespace buckets.
    
    #[serde(rename = "restoreToken", skip_serializing_if = "Option::is_none")]
    pub restore_token: Option<String>,
    
    
    /// A collection of object level retention parameters.
    
    #[serde(rename = "retention", skip_serializing_if = "Option::is_none")]
    pub retention: Option<String>,
    
    
    /// A server-determined value that specifies the earliest time that the object's retention period expires. This value is in RFC 3339 format. Note 1: This field is not provided for objects with an active event-based hold, since retention expiration is unknown until the hold is removed. Note 2: This value can be provided even when temporary hold is set (so that the user can reason about policy without having to first unset the temporary hold).
    
    #[serde(rename = "retentionExpirationTime", skip_serializing_if = "Option::is_none")]
    pub retention_expiration_time: Option<String>,
    
    
    /// The link to this object.
    
    #[serde(rename = "selfLink", skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
    
    
    /// Content-Length of the data in bytes.
    
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    
    
    /// The time at which the object became soft-deleted in RFC 3339 format.
    
    #[serde(rename = "softDeleteTime", skip_serializing_if = "Option::is_none")]
    pub soft_delete_time: Option<String>,
    
    
    /// Storage class of the object.
    
    #[serde(rename = "storageClass", skip_serializing_if = "Option::is_none")]
    pub storage_class: Option<String>,
    
    
    /// Whether an object is under temporary hold. While this flag is set to true, the object is protected against deletion and overwrites. A common use case of this flag is regulatory investigations where objects need to be retained while the investigation is ongoing. Note that unlike event-based hold, temporary hold does not impact retention expiration time of an object.
    
    #[serde(rename = "temporaryHold", skip_serializing_if = "Option::is_none")]
    pub temporary_hold: Option<String>,
    
    
    /// The creation time of the object in RFC 3339 format.
    
    #[serde(rename = "timeCreated", skip_serializing_if = "Option::is_none")]
    pub time_created: Option<String>,
    
    
    /// The time at which the object became noncurrent in RFC 3339 format. Will be returned if and only if this version of the object has been deleted.
    
    #[serde(rename = "timeDeleted", skip_serializing_if = "Option::is_none")]
    pub time_deleted: Option<String>,
    
    
    /// The time when the object was finalized.
    
    #[serde(rename = "timeFinalized", skip_serializing_if = "Option::is_none")]
    pub time_finalized: Option<String>,
    
    
    /// The time at which the object's storage class was last changed. When the object is initially created, it will be set to timeCreated.
    
    #[serde(rename = "timeStorageClassUpdated", skip_serializing_if = "Option::is_none")]
    pub time_storage_class_updated: Option<String>,
    
    
    /// The modification time of the object metadata in RFC 3339 format. Set initially to object creation time and then updated whenever any metadata of the object changes. This includes changes made by a requester, such as modifying custom metadata, as well as changes made by Cloud Storage on behalf of a requester, such as changing the storage class based on an Object Lifecycle Configuration.
    
    #[serde(rename = "updated", skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    
    
}

impl Object {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Object {
    fn default() -> Self {
        Self {
            
            
            acl: None,
            
            bucket: None,
            
            cache_control: None,
            
            component_count: None,
            
            content_disposition: None,
            
            content_encoding: None,
            
            content_language: None,
            
            content_type: None,
            
            crc_32_c: None,
            
            custom_time: None,
            
            customer_encryption: None,
            
            etag: None,
            
            event_based_hold: None,
            
            generation: None,
            
            hard_delete_time: None,
            
            id: None,
            
            kind: None,
            
            kms_key_name: None,
            
            md_5_hash: None,
            
            media_link: None,
            
            metadata: None,
            
            metageneration: None,
            
            name: None,
            
            owner: None,
            
            restore_token: None,
            
            retention: None,
            
            retention_expiration_time: None,
            
            self_link: None,
            
            size: None,
            
            soft_delete_time: None,
            
            storage_class: None,
            
            temporary_hold: None,
            
            time_created: None,
            
            time_deleted: None,
            
            time_finalized: None,
            
            time_storage_class_updated: None,
            
            updated: None,
            
            
        }
    }
}



/// An access-control entry.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectAccessControl {
    
    
    
    /// The name of the bucket.
    
    #[serde(rename = "bucket", skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    
    
    /// The domain associated with the entity, if any.
    
    #[serde(rename = "domain", skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    
    
    /// The email address associated with the entity, if any.
    
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    
    
    /// The entity holding the permission, in one of the following forms: 
user-userId 
user-email 
group-groupId 
group-email 
domain-domain 
project-team-projectId 
allUsers 
allAuthenticatedUsers Examples: 
The user liz@example.com would be user-liz@example.com. 
The group example@googlegroups.com would be group-example@googlegroups.com. 
To refer to all members of the Google Apps for Business domain example.com, the entity would be domain-example.com.
    
    #[serde(rename = "entity", skip_serializing_if = "Option::is_none")]
    pub entity: Option<String>,
    
    
    /// The ID for the entity, if any.
    
    #[serde(rename = "entityId", skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    
    
    /// HTTP 1.1 Entity tag for the access-control entry.
    
    #[serde(rename = "etag", skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    
    
    /// The content generation of the object, if applied to an object.
    
    #[serde(rename = "generation", skip_serializing_if = "Option::is_none")]
    pub generation: Option<String>,
    
    
    /// The ID of the access-control entry.
    
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    
    
    /// The kind of item this is. For object access control entries, this is always storage#objectAccessControl.
    
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    
    
    /// The name of the object, if applied to an object.
    
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    
    
    /// The project team associated with the entity, if any.
    
    #[serde(rename = "projectTeam", skip_serializing_if = "Option::is_none")]
    pub project_team: Option<String>,
    
    
    /// The access permission for the entity.
    
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    
    
    /// The link to this access-control entry.
    
    #[serde(rename = "selfLink", skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
    
    
}

impl ObjectAccessControl {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for ObjectAccessControl {
    fn default() -> Self {
        Self {
            
            
            bucket: None,
            
            domain: None,
            
            email: None,
            
            entity: None,
            
            entity_id: None,
            
            etag: None,
            
            generation: None,
            
            id: None,
            
            kind: None,
            
            object: None,
            
            project_team: None,
            
            role: None,
            
            self_link: None,
            
            
        }
    }
}



/// An access-control list.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectAccessControls {
    
    
    
    /// The list of items.
    
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<String>,
    
    
    /// The kind of item this is. For lists of object access control entries, this is always storage#objectAccessControls.
    
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    
    
}

impl ObjectAccessControls {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for ObjectAccessControls {
    fn default() -> Self {
        Self {
            
            
            items: None,
            
            kind: None,
            
            
        }
    }
}



/// A list of objects.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Objects {
    
    
    
    /// The list of items.
    
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<String>,
    
    
    /// The kind of item this is. For lists of objects, this is always storage#objects.
    
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    
    
    /// The continuation token, used to page through large result sets. Provide this value in a subsequent request to return the next page of results.
    
    #[serde(rename = "nextPageToken", skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    
    
    /// The list of prefixes of objects matching-but-not-listed up to and including the requested delimiter.
    
    #[serde(rename = "prefixes", skip_serializing_if = "Option::is_none")]
    pub prefixes: Option<String>,
    
    
}

impl Objects {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Objects {
    fn default() -> Self {
        Self {
            
            
            items: None,
            
            kind: None,
            
            next_page_token: None,
            
            prefixes: None,
            
            
        }
    }
}



/// A bucket/object/managedFolder IAM policy.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    
    
    
    /// An association between a role, which comes with a set of permissions, and members who may assume that role.
    
    #[serde(rename = "bindings", skip_serializing_if = "Option::is_none")]
    pub bindings: Option<String>,
    
    
    /// HTTP 1.1  Entity tag for the policy.
    
    #[serde(rename = "etag", skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    
    
    /// The kind of item this is. For policies, this is always storage#policy. This field is ignored on input.
    
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    
    
    /// The ID of the resource to which this policy belongs. Will be of the form projects/_/buckets/bucket for buckets, projects/_/buckets/bucket/objects/object for objects, and projects/_/buckets/bucket/managedFolders/managedFolder. A specific generation may be specified by appending #generationNumber to the end of the object name, e.g. projects/_/buckets/my-bucket/objects/data.txt#17. The current generation can be denoted with #0. This field is ignored on input.
    
    #[serde(rename = "resourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    
    
    /// The IAM policy format version.
    
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    
    
}

impl Policy {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Policy {
    fn default() -> Self {
        Self {
            
            
            bindings: None,
            
            etag: None,
            
            kind: None,
            
            resource_id: None,
            
            version: None,
            
            
        }
    }
}



/// A Relocate Bucket request.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelocateBucketRequest {
    
    
    
    /// The bucket's new custom placement configuration if relocating to a Custom Dual Region.
    
    #[serde(rename = "destinationCustomPlacementConfig", skip_serializing_if = "Option::is_none")]
    pub destination_custom_placement_config: Option<String>,
    
    
    /// The new location the bucket will be relocated to.
    
    #[serde(rename = "destinationLocation", skip_serializing_if = "Option::is_none")]
    pub destination_location: Option<String>,
    
    
    /// If true, validate the operation, but do not actually relocate the bucket.
    
    #[serde(rename = "validateOnly", skip_serializing_if = "Option::is_none")]
    pub validate_only: Option<String>,
    
    
}

impl RelocateBucketRequest {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for RelocateBucketRequest {
    fn default() -> Self {
        Self {
            
            
            destination_custom_placement_config: None,
            
            destination_location: None,
            
            validate_only: None,
            
            
        }
    }
}



/// A rewrite response.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewriteResponse {
    
    
    
    /// true if the copy is finished; otherwise, false if the copy is in progress. This property is always present in the response.
    
    #[serde(rename = "done", skip_serializing_if = "Option::is_none")]
    pub done: Option<String>,
    
    
    /// The kind of item this is.
    
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    
    
    /// The total size of the object being copied in bytes. This property is always present in the response.
    
    #[serde(rename = "objectSize", skip_serializing_if = "Option::is_none")]
    pub object_size: Option<String>,
    
    
    /// A resource containing the metadata for the copied-to object. This property is present in the response only when copying completes.
    
    #[serde(rename = "resource", skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    
    
    /// A token to use in subsequent requests to continue copying data. This token is present in the response only when there is more data to copy.
    
    #[serde(rename = "rewriteToken", skip_serializing_if = "Option::is_none")]
    pub rewrite_token: Option<String>,
    
    
    /// The total bytes written so far, which can be used to provide a waiting user with a progress indicator. This property is always present in the response.
    
    #[serde(rename = "totalBytesRewritten", skip_serializing_if = "Option::is_none")]
    pub total_bytes_rewritten: Option<String>,
    
    
}

impl RewriteResponse {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for RewriteResponse {
    fn default() -> Self {
        Self {
            
            
            done: None,
            
            kind: None,
            
            object_size: None,
            
            resource: None,
            
            rewrite_token: None,
            
            total_bytes_rewritten: None,
            
            
        }
    }
}



/// A subscription to receive Google PubSub notifications.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceAccount {
    
    
    
    /// The ID of the notification.
    
    #[serde(rename = "email_address", skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    
    
    /// The kind of item this is. For notifications, this is always storage#notification.
    
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    
    
}

impl ServiceAccount {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for ServiceAccount {
    fn default() -> Self {
        Self {
            
            
            email_address: None,
            
            kind: None,
            
            
        }
    }
}



/// A storage.(buckets|objects|managedFolders).testIamPermissions response.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestIamPermissionsResponse {
    
    
    
    /// The kind of item this is.
    
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    
    
    /// The permissions held by the caller. Permissions are always of the format storage.resource.capability, where resource is one of buckets, objects, or managedFolders. The supported permissions are as follows:  
storage.buckets.delete Delete bucket.  
storage.buckets.get Read bucket metadata.  
storage.buckets.getIamPolicy Read bucket IAM policy.  
storage.buckets.create Create bucket.  
storage.buckets.list List buckets.  
storage.buckets.setIamPolicy Update bucket IAM policy.  
storage.buckets.update Update bucket metadata.  
storage.objects.delete Delete object.  
storage.objects.get Read object data and metadata.  
storage.objects.getIamPolicy Read object IAM policy.  
storage.objects.create Create object.  
storage.objects.list List objects.  
storage.objects.setIamPolicy Update object IAM policy.  
storage.objects.update Update object metadata. 
storage.managedFolders.delete Delete managed folder.  
storage.managedFolders.get Read managed folder metadata.  
storage.managedFolders.getIamPolicy Read managed folder IAM policy.  
storage.managedFolders.create Create managed folder.  
storage.managedFolders.list List managed folders.  
storage.managedFolders.setIamPolicy Update managed folder IAM policy.
    
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<String>,
    
    
}

impl TestIamPermissionsResponse {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for TestIamPermissionsResponse {
    fn default() -> Self {
        Self {
            
            
            kind: None,
            
            permissions: None,
            
            
        }
    }
}


