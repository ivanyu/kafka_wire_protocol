// This file was generated. Do not edit.

/// The utility for getting info about header versions and API keys.
pub struct ApiMessageType {
    pub name: &'static str,
    pub api_key: i16,
    pub lowest_supported_version: i16,
    pub highest_supported_version: i16,
    pub lowest_deprecated_version: i16,
    pub highest_deprecated_version: i16,
    pub latest_version_unstable: bool,
}

impl ApiMessageType {
    pub const PRODUCE: ApiMessageType = ApiMessageType { name: "Produce", api_key: 0, lowest_supported_version: 3, highest_supported_version: 12, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const FETCH: ApiMessageType = ApiMessageType { name: "Fetch", api_key: 1, lowest_supported_version: 4, highest_supported_version: 17, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const LIST_OFFSETS: ApiMessageType = ApiMessageType { name: "ListOffsets", api_key: 2, lowest_supported_version: 1, highest_supported_version: 10, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const METADATA: ApiMessageType = ApiMessageType { name: "Metadata", api_key: 3, lowest_supported_version: 0, highest_supported_version: 13, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const OFFSET_COMMIT: ApiMessageType = ApiMessageType { name: "OffsetCommit", api_key: 8, lowest_supported_version: 2, highest_supported_version: 9, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const OFFSET_FETCH: ApiMessageType = ApiMessageType { name: "OffsetFetch", api_key: 9, lowest_supported_version: 1, highest_supported_version: 9, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const FIND_COORDINATOR: ApiMessageType = ApiMessageType { name: "FindCoordinator", api_key: 10, lowest_supported_version: 0, highest_supported_version: 6, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const JOIN_GROUP: ApiMessageType = ApiMessageType { name: "JoinGroup", api_key: 11, lowest_supported_version: 2, highest_supported_version: 9, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const HEARTBEAT: ApiMessageType = ApiMessageType { name: "Heartbeat", api_key: 12, lowest_supported_version: 0, highest_supported_version: 4, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const LEAVE_GROUP: ApiMessageType = ApiMessageType { name: "LeaveGroup", api_key: 13, lowest_supported_version: 0, highest_supported_version: 5, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const SYNC_GROUP: ApiMessageType = ApiMessageType { name: "SyncGroup", api_key: 14, lowest_supported_version: 0, highest_supported_version: 5, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const DESCRIBE_GROUPS: ApiMessageType = ApiMessageType { name: "DescribeGroups", api_key: 15, lowest_supported_version: 0, highest_supported_version: 6, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const LIST_GROUPS: ApiMessageType = ApiMessageType { name: "ListGroups", api_key: 16, lowest_supported_version: 0, highest_supported_version: 5, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const SASL_HANDSHAKE: ApiMessageType = ApiMessageType { name: "SaslHandshake", api_key: 17, lowest_supported_version: 0, highest_supported_version: 1, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const API_VERSIONS: ApiMessageType = ApiMessageType { name: "ApiVersions", api_key: 18, lowest_supported_version: 0, highest_supported_version: 4, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const CREATE_TOPICS: ApiMessageType = ApiMessageType { name: "CreateTopics", api_key: 19, lowest_supported_version: 2, highest_supported_version: 7, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const DELETE_TOPICS: ApiMessageType = ApiMessageType { name: "DeleteTopics", api_key: 20, lowest_supported_version: 1, highest_supported_version: 6, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const DELETE_RECORDS: ApiMessageType = ApiMessageType { name: "DeleteRecords", api_key: 21, lowest_supported_version: 0, highest_supported_version: 2, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const INIT_PRODUCER_ID: ApiMessageType = ApiMessageType { name: "InitProducerId", api_key: 22, lowest_supported_version: 0, highest_supported_version: 5, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const OFFSET_FOR_LEADER_EPOCH: ApiMessageType = ApiMessageType { name: "OffsetForLeaderEpoch", api_key: 23, lowest_supported_version: 2, highest_supported_version: 4, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const ADD_PARTITIONS_TO_TXN: ApiMessageType = ApiMessageType { name: "AddPartitionsToTxn", api_key: 24, lowest_supported_version: 0, highest_supported_version: 5, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const ADD_OFFSETS_TO_TXN: ApiMessageType = ApiMessageType { name: "AddOffsetsToTxn", api_key: 25, lowest_supported_version: 0, highest_supported_version: 4, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const END_TXN: ApiMessageType = ApiMessageType { name: "EndTxn", api_key: 26, lowest_supported_version: 0, highest_supported_version: 5, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const WRITE_TXN_MARKERS: ApiMessageType = ApiMessageType { name: "WriteTxnMarkers", api_key: 27, lowest_supported_version: 1, highest_supported_version: 1, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const TXN_OFFSET_COMMIT: ApiMessageType = ApiMessageType { name: "TxnOffsetCommit", api_key: 28, lowest_supported_version: 0, highest_supported_version: 5, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const DESCRIBE_ACLS: ApiMessageType = ApiMessageType { name: "DescribeAcls", api_key: 29, lowest_supported_version: 1, highest_supported_version: 3, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const CREATE_ACLS: ApiMessageType = ApiMessageType { name: "CreateAcls", api_key: 30, lowest_supported_version: 1, highest_supported_version: 3, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const DELETE_ACLS: ApiMessageType = ApiMessageType { name: "DeleteAcls", api_key: 31, lowest_supported_version: 1, highest_supported_version: 3, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const DESCRIBE_CONFIGS: ApiMessageType = ApiMessageType { name: "DescribeConfigs", api_key: 32, lowest_supported_version: 1, highest_supported_version: 4, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const ALTER_CONFIGS: ApiMessageType = ApiMessageType { name: "AlterConfigs", api_key: 33, lowest_supported_version: 0, highest_supported_version: 2, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const ALTER_REPLICA_LOG_DIRS: ApiMessageType = ApiMessageType { name: "AlterReplicaLogDirs", api_key: 34, lowest_supported_version: 1, highest_supported_version: 2, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const DESCRIBE_LOG_DIRS: ApiMessageType = ApiMessageType { name: "DescribeLogDirs", api_key: 35, lowest_supported_version: 1, highest_supported_version: 4, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const SASL_AUTHENTICATE: ApiMessageType = ApiMessageType { name: "SaslAuthenticate", api_key: 36, lowest_supported_version: 0, highest_supported_version: 2, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const CREATE_PARTITIONS: ApiMessageType = ApiMessageType { name: "CreatePartitions", api_key: 37, lowest_supported_version: 0, highest_supported_version: 3, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const CREATE_DELEGATION_TOKEN: ApiMessageType = ApiMessageType { name: "CreateDelegationToken", api_key: 38, lowest_supported_version: 1, highest_supported_version: 3, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const RENEW_DELEGATION_TOKEN: ApiMessageType = ApiMessageType { name: "RenewDelegationToken", api_key: 39, lowest_supported_version: 1, highest_supported_version: 2, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const EXPIRE_DELEGATION_TOKEN: ApiMessageType = ApiMessageType { name: "ExpireDelegationToken", api_key: 40, lowest_supported_version: 1, highest_supported_version: 2, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const DESCRIBE_DELEGATION_TOKEN: ApiMessageType = ApiMessageType { name: "DescribeDelegationToken", api_key: 41, lowest_supported_version: 1, highest_supported_version: 3, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const DELETE_GROUPS: ApiMessageType = ApiMessageType { name: "DeleteGroups", api_key: 42, lowest_supported_version: 0, highest_supported_version: 2, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const ELECT_LEADERS: ApiMessageType = ApiMessageType { name: "ElectLeaders", api_key: 43, lowest_supported_version: 0, highest_supported_version: 2, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const INCREMENTAL_ALTER_CONFIGS: ApiMessageType = ApiMessageType { name: "IncrementalAlterConfigs", api_key: 44, lowest_supported_version: 0, highest_supported_version: 1, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const ALTER_PARTITION_REASSIGNMENTS: ApiMessageType = ApiMessageType { name: "AlterPartitionReassignments", api_key: 45, lowest_supported_version: 0, highest_supported_version: 0, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const LIST_PARTITION_REASSIGNMENTS: ApiMessageType = ApiMessageType { name: "ListPartitionReassignments", api_key: 46, lowest_supported_version: 0, highest_supported_version: 0, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const OFFSET_DELETE: ApiMessageType = ApiMessageType { name: "OffsetDelete", api_key: 47, lowest_supported_version: 0, highest_supported_version: 0, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const DESCRIBE_CLIENT_QUOTAS: ApiMessageType = ApiMessageType { name: "DescribeClientQuotas", api_key: 48, lowest_supported_version: 0, highest_supported_version: 1, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const ALTER_CLIENT_QUOTAS: ApiMessageType = ApiMessageType { name: "AlterClientQuotas", api_key: 49, lowest_supported_version: 0, highest_supported_version: 1, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const DESCRIBE_USER_SCRAM_CREDENTIALS: ApiMessageType = ApiMessageType { name: "DescribeUserScramCredentials", api_key: 50, lowest_supported_version: 0, highest_supported_version: 0, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const ALTER_USER_SCRAM_CREDENTIALS: ApiMessageType = ApiMessageType { name: "AlterUserScramCredentials", api_key: 51, lowest_supported_version: 0, highest_supported_version: 0, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const VOTE: ApiMessageType = ApiMessageType { name: "Vote", api_key: 52, lowest_supported_version: 0, highest_supported_version: 2, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const BEGIN_QUORUM_EPOCH: ApiMessageType = ApiMessageType { name: "BeginQuorumEpoch", api_key: 53, lowest_supported_version: 0, highest_supported_version: 1, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const END_QUORUM_EPOCH: ApiMessageType = ApiMessageType { name: "EndQuorumEpoch", api_key: 54, lowest_supported_version: 0, highest_supported_version: 1, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const DESCRIBE_QUORUM: ApiMessageType = ApiMessageType { name: "DescribeQuorum", api_key: 55, lowest_supported_version: 0, highest_supported_version: 2, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const ALTER_PARTITION: ApiMessageType = ApiMessageType { name: "AlterPartition", api_key: 56, lowest_supported_version: 2, highest_supported_version: 3, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const UPDATE_FEATURES: ApiMessageType = ApiMessageType { name: "UpdateFeatures", api_key: 57, lowest_supported_version: 0, highest_supported_version: 2, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const ENVELOPE: ApiMessageType = ApiMessageType { name: "Envelope", api_key: 58, lowest_supported_version: 0, highest_supported_version: 0, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const FETCH_SNAPSHOT: ApiMessageType = ApiMessageType { name: "FetchSnapshot", api_key: 59, lowest_supported_version: 0, highest_supported_version: 1, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const DESCRIBE_CLUSTER: ApiMessageType = ApiMessageType { name: "DescribeCluster", api_key: 60, lowest_supported_version: 0, highest_supported_version: 2, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const DESCRIBE_PRODUCERS: ApiMessageType = ApiMessageType { name: "DescribeProducers", api_key: 61, lowest_supported_version: 0, highest_supported_version: 0, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const BROKER_REGISTRATION: ApiMessageType = ApiMessageType { name: "BrokerRegistration", api_key: 62, lowest_supported_version: 0, highest_supported_version: 4, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const BROKER_HEARTBEAT: ApiMessageType = ApiMessageType { name: "BrokerHeartbeat", api_key: 63, lowest_supported_version: 0, highest_supported_version: 1, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const UNREGISTER_BROKER: ApiMessageType = ApiMessageType { name: "UnregisterBroker", api_key: 64, lowest_supported_version: 0, highest_supported_version: 0, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const DESCRIBE_TRANSACTIONS: ApiMessageType = ApiMessageType { name: "DescribeTransactions", api_key: 65, lowest_supported_version: 0, highest_supported_version: 0, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const LIST_TRANSACTIONS: ApiMessageType = ApiMessageType { name: "ListTransactions", api_key: 66, lowest_supported_version: 0, highest_supported_version: 1, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const ALLOCATE_PRODUCER_IDS: ApiMessageType = ApiMessageType { name: "AllocateProducerIds", api_key: 67, lowest_supported_version: 0, highest_supported_version: 0, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const CONSUMER_GROUP_HEARTBEAT: ApiMessageType = ApiMessageType { name: "ConsumerGroupHeartbeat", api_key: 68, lowest_supported_version: 0, highest_supported_version: 1, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const CONSUMER_GROUP_DESCRIBE: ApiMessageType = ApiMessageType { name: "ConsumerGroupDescribe", api_key: 69, lowest_supported_version: 0, highest_supported_version: 1, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const CONTROLLER_REGISTRATION: ApiMessageType = ApiMessageType { name: "ControllerRegistration", api_key: 70, lowest_supported_version: 0, highest_supported_version: 0, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const GET_TELEMETRY_SUBSCRIPTIONS: ApiMessageType = ApiMessageType { name: "GetTelemetrySubscriptions", api_key: 71, lowest_supported_version: 0, highest_supported_version: 0, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const PUSH_TELEMETRY: ApiMessageType = ApiMessageType { name: "PushTelemetry", api_key: 72, lowest_supported_version: 0, highest_supported_version: 0, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const ASSIGN_REPLICAS_TO_DIRS: ApiMessageType = ApiMessageType { name: "AssignReplicasToDirs", api_key: 73, lowest_supported_version: 0, highest_supported_version: 0, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const LIST_CLIENT_METRICS_RESOURCES: ApiMessageType = ApiMessageType { name: "ListClientMetricsResources", api_key: 74, lowest_supported_version: 0, highest_supported_version: 0, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const DESCRIBE_TOPIC_PARTITIONS: ApiMessageType = ApiMessageType { name: "DescribeTopicPartitions", api_key: 75, lowest_supported_version: 0, highest_supported_version: 0, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const SHARE_GROUP_HEARTBEAT: ApiMessageType = ApiMessageType { name: "ShareGroupHeartbeat", api_key: 76, lowest_supported_version: 0, highest_supported_version: 0, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: true };
    pub const SHARE_GROUP_DESCRIBE: ApiMessageType = ApiMessageType { name: "ShareGroupDescribe", api_key: 77, lowest_supported_version: 0, highest_supported_version: 0, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: true };
    pub const SHARE_FETCH: ApiMessageType = ApiMessageType { name: "ShareFetch", api_key: 78, lowest_supported_version: 0, highest_supported_version: 0, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: true };
    pub const SHARE_ACKNOWLEDGE: ApiMessageType = ApiMessageType { name: "ShareAcknowledge", api_key: 79, lowest_supported_version: 0, highest_supported_version: 0, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: true };
    pub const ADD_RAFT_VOTER: ApiMessageType = ApiMessageType { name: "AddRaftVoter", api_key: 80, lowest_supported_version: 0, highest_supported_version: 0, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const REMOVE_RAFT_VOTER: ApiMessageType = ApiMessageType { name: "RemoveRaftVoter", api_key: 81, lowest_supported_version: 0, highest_supported_version: 0, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const UPDATE_RAFT_VOTER: ApiMessageType = ApiMessageType { name: "UpdateRaftVoter", api_key: 82, lowest_supported_version: 0, highest_supported_version: 0, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: false };
    pub const INITIALIZE_SHARE_GROUP_STATE: ApiMessageType = ApiMessageType { name: "InitializeShareGroupState", api_key: 83, lowest_supported_version: 0, highest_supported_version: 0, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: true };
    pub const READ_SHARE_GROUP_STATE: ApiMessageType = ApiMessageType { name: "ReadShareGroupState", api_key: 84, lowest_supported_version: 0, highest_supported_version: 0, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: true };
    pub const WRITE_SHARE_GROUP_STATE: ApiMessageType = ApiMessageType { name: "WriteShareGroupState", api_key: 85, lowest_supported_version: 0, highest_supported_version: 0, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: true };
    pub const DELETE_SHARE_GROUP_STATE: ApiMessageType = ApiMessageType { name: "DeleteShareGroupState", api_key: 86, lowest_supported_version: 0, highest_supported_version: 0, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: true };
    pub const READ_SHARE_GROUP_STATE_SUMMARY: ApiMessageType = ApiMessageType { name: "ReadShareGroupStateSummary", api_key: 87, lowest_supported_version: 0, highest_supported_version: 0, lowest_deprecated_version: 0, highest_deprecated_version: -1, latest_version_unstable: true };
    
    /// Get request header version for the message version.
    pub fn request_header_version(&self, _version: i16) -> i16 {
        match self.api_key {
            0 => {  // Produce
                if _version >= 9 { 2 } else { 1 }
            }

            1 => {  // Fetch
                if _version >= 12 { 2 } else { 1 }
            }

            2 => {  // ListOffsets
                if _version >= 6 { 2 } else { 1 }
            }

            3 => {  // Metadata
                if _version >= 9 { 2 } else { 1 }
            }

            8 => {  // OffsetCommit
                if _version >= 8 { 2 } else { 1 }
            }

            9 => {  // OffsetFetch
                if _version >= 6 { 2 } else { 1 }
            }

            10 => {  // FindCoordinator
                if _version >= 3 { 2 } else { 1 }
            }

            11 => {  // JoinGroup
                if _version >= 6 { 2 } else { 1 }
            }

            12 => {  // Heartbeat
                if _version >= 4 { 2 } else { 1 }
            }

            13 => {  // LeaveGroup
                if _version >= 4 { 2 } else { 1 }
            }

            14 => {  // SyncGroup
                if _version >= 4 { 2 } else { 1 }
            }

            15 => {  // DescribeGroups
                if _version >= 5 { 2 } else { 1 }
            }

            16 => {  // ListGroups
                if _version >= 3 { 2 } else { 1 }
            }

            17 => {  // SaslHandshake
                1
            }

            18 => {  // ApiVersions
                if _version >= 3 { 2 } else { 1 }
            }

            19 => {  // CreateTopics
                if _version >= 5 { 2 } else { 1 }
            }

            20 => {  // DeleteTopics
                if _version >= 4 { 2 } else { 1 }
            }

            21 => {  // DeleteRecords
                if _version >= 2 { 2 } else { 1 }
            }

            22 => {  // InitProducerId
                if _version >= 2 { 2 } else { 1 }
            }

            23 => {  // OffsetForLeaderEpoch
                if _version >= 4 { 2 } else { 1 }
            }

            24 => {  // AddPartitionsToTxn
                if _version >= 3 { 2 } else { 1 }
            }

            25 => {  // AddOffsetsToTxn
                if _version >= 3 { 2 } else { 1 }
            }

            26 => {  // EndTxn
                if _version >= 3 { 2 } else { 1 }
            }

            27 => {  // WriteTxnMarkers
                2
            }

            28 => {  // TxnOffsetCommit
                if _version >= 3 { 2 } else { 1 }
            }

            29 => {  // DescribeAcls
                if _version >= 2 { 2 } else { 1 }
            }

            30 => {  // CreateAcls
                if _version >= 2 { 2 } else { 1 }
            }

            31 => {  // DeleteAcls
                if _version >= 2 { 2 } else { 1 }
            }

            32 => {  // DescribeConfigs
                if _version >= 4 { 2 } else { 1 }
            }

            33 => {  // AlterConfigs
                if _version >= 2 { 2 } else { 1 }
            }

            34 => {  // AlterReplicaLogDirs
                if _version >= 2 { 2 } else { 1 }
            }

            35 => {  // DescribeLogDirs
                if _version >= 2 { 2 } else { 1 }
            }

            36 => {  // SaslAuthenticate
                if _version >= 2 { 2 } else { 1 }
            }

            37 => {  // CreatePartitions
                if _version >= 2 { 2 } else { 1 }
            }

            38 => {  // CreateDelegationToken
                if _version >= 2 { 2 } else { 1 }
            }

            39 => {  // RenewDelegationToken
                if _version >= 2 { 2 } else { 1 }
            }

            40 => {  // ExpireDelegationToken
                if _version >= 2 { 2 } else { 1 }
            }

            41 => {  // DescribeDelegationToken
                if _version >= 2 { 2 } else { 1 }
            }

            42 => {  // DeleteGroups
                if _version >= 2 { 2 } else { 1 }
            }

            43 => {  // ElectLeaders
                if _version >= 2 { 2 } else { 1 }
            }

            44 => {  // IncrementalAlterConfigs
                if _version >= 1 { 2 } else { 1 }
            }

            45 => {  // AlterPartitionReassignments
                2
            }

            46 => {  // ListPartitionReassignments
                2
            }

            47 => {  // OffsetDelete
                1
            }

            48 => {  // DescribeClientQuotas
                if _version >= 1 { 2 } else { 1 }
            }

            49 => {  // AlterClientQuotas
                if _version >= 1 { 2 } else { 1 }
            }

            50 => {  // DescribeUserScramCredentials
                2
            }

            51 => {  // AlterUserScramCredentials
                2
            }

            52 => {  // Vote
                2
            }

            53 => {  // BeginQuorumEpoch
                if _version >= 1 { 2 } else { 1 }
            }

            54 => {  // EndQuorumEpoch
                if _version >= 1 { 2 } else { 1 }
            }

            55 => {  // DescribeQuorum
                2
            }

            56 => {  // AlterPartition
                2
            }

            57 => {  // UpdateFeatures
                2
            }

            58 => {  // Envelope
                2
            }

            59 => {  // FetchSnapshot
                2
            }

            60 => {  // DescribeCluster
                2
            }

            61 => {  // DescribeProducers
                2
            }

            62 => {  // BrokerRegistration
                2
            }

            63 => {  // BrokerHeartbeat
                2
            }

            64 => {  // UnregisterBroker
                2
            }

            65 => {  // DescribeTransactions
                2
            }

            66 => {  // ListTransactions
                2
            }

            67 => {  // AllocateProducerIds
                2
            }

            68 => {  // ConsumerGroupHeartbeat
                2
            }

            69 => {  // ConsumerGroupDescribe
                2
            }

            70 => {  // ControllerRegistration
                2
            }

            71 => {  // GetTelemetrySubscriptions
                2
            }

            72 => {  // PushTelemetry
                2
            }

            73 => {  // AssignReplicasToDirs
                2
            }

            74 => {  // ListClientMetricsResources
                2
            }

            75 => {  // DescribeTopicPartitions
                2
            }

            76 => {  // ShareGroupHeartbeat
                2
            }

            77 => {  // ShareGroupDescribe
                2
            }

            78 => {  // ShareFetch
                2
            }

            79 => {  // ShareAcknowledge
                2
            }

            80 => {  // AddRaftVoter
                2
            }

            81 => {  // RemoveRaftVoter
                2
            }

            82 => {  // UpdateRaftVoter
                2
            }

            83 => {  // InitializeShareGroupState
                2
            }

            84 => {  // ReadShareGroupState
                2
            }

            85 => {  // WriteShareGroupState
                2
            }

            86 => {  // DeleteShareGroupState
                2
            }

            87 => {  // ReadShareGroupStateSummary
                2
            }

            _ => {
                panic!("Unsupported API key {}", self.api_key);
            }
        }
    }
    
    /// Get response header version for the message version.
    pub fn response_header_version(&self, _version: i16) -> i16 {
        match self.api_key {
            0 => {  // Produce
                if _version >= 9 { 1 } else { 0 }
            }

            1 => {  // Fetch
                if _version >= 12 { 1 } else { 0 }
            }

            2 => {  // ListOffsets
                if _version >= 6 { 1 } else { 0 }
            }

            3 => {  // Metadata
                if _version >= 9 { 1 } else { 0 }
            }

            8 => {  // OffsetCommit
                if _version >= 8 { 1 } else { 0 }
            }

            9 => {  // OffsetFetch
                if _version >= 6 { 1 } else { 0 }
            }

            10 => {  // FindCoordinator
                if _version >= 3 { 1 } else { 0 }
            }

            11 => {  // JoinGroup
                if _version >= 6 { 1 } else { 0 }
            }

            12 => {  // Heartbeat
                if _version >= 4 { 1 } else { 0 }
            }

            13 => {  // LeaveGroup
                if _version >= 4 { 1 } else { 0 }
            }

            14 => {  // SyncGroup
                if _version >= 4 { 1 } else { 0 }
            }

            15 => {  // DescribeGroups
                if _version >= 5 { 1 } else { 0 }
            }

            16 => {  // ListGroups
                if _version >= 3 { 1 } else { 0 }
            }

            17 => {  // SaslHandshake
                0
            }

            18 => {  // ApiVersions
                // ApiVersionsResponse always includes a v0 header.
                // See KIP-511 for details.
                0
            }
            19 => {  // CreateTopics
                if _version >= 5 { 1 } else { 0 }
            }

            20 => {  // DeleteTopics
                if _version >= 4 { 1 } else { 0 }
            }

            21 => {  // DeleteRecords
                if _version >= 2 { 1 } else { 0 }
            }

            22 => {  // InitProducerId
                if _version >= 2 { 1 } else { 0 }
            }

            23 => {  // OffsetForLeaderEpoch
                if _version >= 4 { 1 } else { 0 }
            }

            24 => {  // AddPartitionsToTxn
                if _version >= 3 { 1 } else { 0 }
            }

            25 => {  // AddOffsetsToTxn
                if _version >= 3 { 1 } else { 0 }
            }

            26 => {  // EndTxn
                if _version >= 3 { 1 } else { 0 }
            }

            27 => {  // WriteTxnMarkers
                1
            }

            28 => {  // TxnOffsetCommit
                if _version >= 3 { 1 } else { 0 }
            }

            29 => {  // DescribeAcls
                if _version >= 2 { 1 } else { 0 }
            }

            30 => {  // CreateAcls
                if _version >= 2 { 1 } else { 0 }
            }

            31 => {  // DeleteAcls
                if _version >= 2 { 1 } else { 0 }
            }

            32 => {  // DescribeConfigs
                if _version >= 4 { 1 } else { 0 }
            }

            33 => {  // AlterConfigs
                if _version >= 2 { 1 } else { 0 }
            }

            34 => {  // AlterReplicaLogDirs
                if _version >= 2 { 1 } else { 0 }
            }

            35 => {  // DescribeLogDirs
                if _version >= 2 { 1 } else { 0 }
            }

            36 => {  // SaslAuthenticate
                if _version >= 2 { 1 } else { 0 }
            }

            37 => {  // CreatePartitions
                if _version >= 2 { 1 } else { 0 }
            }

            38 => {  // CreateDelegationToken
                if _version >= 2 { 1 } else { 0 }
            }

            39 => {  // RenewDelegationToken
                if _version >= 2 { 1 } else { 0 }
            }

            40 => {  // ExpireDelegationToken
                if _version >= 2 { 1 } else { 0 }
            }

            41 => {  // DescribeDelegationToken
                if _version >= 2 { 1 } else { 0 }
            }

            42 => {  // DeleteGroups
                if _version >= 2 { 1 } else { 0 }
            }

            43 => {  // ElectLeaders
                if _version >= 2 { 1 } else { 0 }
            }

            44 => {  // IncrementalAlterConfigs
                if _version >= 1 { 1 } else { 0 }
            }

            45 => {  // AlterPartitionReassignments
                1
            }

            46 => {  // ListPartitionReassignments
                1
            }

            47 => {  // OffsetDelete
                0
            }

            48 => {  // DescribeClientQuotas
                if _version >= 1 { 1 } else { 0 }
            }

            49 => {  // AlterClientQuotas
                if _version >= 1 { 1 } else { 0 }
            }

            50 => {  // DescribeUserScramCredentials
                1
            }

            51 => {  // AlterUserScramCredentials
                1
            }

            52 => {  // Vote
                1
            }

            53 => {  // BeginQuorumEpoch
                if _version >= 1 { 1 } else { 0 }
            }

            54 => {  // EndQuorumEpoch
                if _version >= 1 { 1 } else { 0 }
            }

            55 => {  // DescribeQuorum
                1
            }

            56 => {  // AlterPartition
                1
            }

            57 => {  // UpdateFeatures
                1
            }

            58 => {  // Envelope
                1
            }

            59 => {  // FetchSnapshot
                1
            }

            60 => {  // DescribeCluster
                1
            }

            61 => {  // DescribeProducers
                1
            }

            62 => {  // BrokerRegistration
                1
            }

            63 => {  // BrokerHeartbeat
                1
            }

            64 => {  // UnregisterBroker
                1
            }

            65 => {  // DescribeTransactions
                1
            }

            66 => {  // ListTransactions
                1
            }

            67 => {  // AllocateProducerIds
                1
            }

            68 => {  // ConsumerGroupHeartbeat
                1
            }

            69 => {  // ConsumerGroupDescribe
                1
            }

            70 => {  // ControllerRegistration
                1
            }

            71 => {  // GetTelemetrySubscriptions
                1
            }

            72 => {  // PushTelemetry
                1
            }

            73 => {  // AssignReplicasToDirs
                1
            }

            74 => {  // ListClientMetricsResources
                1
            }

            75 => {  // DescribeTopicPartitions
                1
            }

            76 => {  // ShareGroupHeartbeat
                1
            }

            77 => {  // ShareGroupDescribe
                1
            }

            78 => {  // ShareFetch
                1
            }

            79 => {  // ShareAcknowledge
                1
            }

            80 => {  // AddRaftVoter
                1
            }

            81 => {  // RemoveRaftVoter
                1
            }

            82 => {  // UpdateRaftVoter
                1
            }

            83 => {  // InitializeShareGroupState
                1
            }

            84 => {  // ReadShareGroupState
                1
            }

            85 => {  // WriteShareGroupState
                1
            }

            86 => {  // DeleteShareGroupState
                1
            }

            87 => {  // ReadShareGroupStateSummary
                1
            }

            _ => {
                panic!("Unsupported API key {}", self.api_key);
            }
        }
    }
    
    pub fn from_api_key(api_key: i16) -> ApiMessageType {
        match api_key {
            0 => Self::PRODUCE,
            1 => Self::FETCH,
            2 => Self::LIST_OFFSETS,
            3 => Self::METADATA,
            8 => Self::OFFSET_COMMIT,
            9 => Self::OFFSET_FETCH,
            10 => Self::FIND_COORDINATOR,
            11 => Self::JOIN_GROUP,
            12 => Self::HEARTBEAT,
            13 => Self::LEAVE_GROUP,
            14 => Self::SYNC_GROUP,
            15 => Self::DESCRIBE_GROUPS,
            16 => Self::LIST_GROUPS,
            17 => Self::SASL_HANDSHAKE,
            18 => Self::API_VERSIONS,
            19 => Self::CREATE_TOPICS,
            20 => Self::DELETE_TOPICS,
            21 => Self::DELETE_RECORDS,
            22 => Self::INIT_PRODUCER_ID,
            23 => Self::OFFSET_FOR_LEADER_EPOCH,
            24 => Self::ADD_PARTITIONS_TO_TXN,
            25 => Self::ADD_OFFSETS_TO_TXN,
            26 => Self::END_TXN,
            27 => Self::WRITE_TXN_MARKERS,
            28 => Self::TXN_OFFSET_COMMIT,
            29 => Self::DESCRIBE_ACLS,
            30 => Self::CREATE_ACLS,
            31 => Self::DELETE_ACLS,
            32 => Self::DESCRIBE_CONFIGS,
            33 => Self::ALTER_CONFIGS,
            34 => Self::ALTER_REPLICA_LOG_DIRS,
            35 => Self::DESCRIBE_LOG_DIRS,
            36 => Self::SASL_AUTHENTICATE,
            37 => Self::CREATE_PARTITIONS,
            38 => Self::CREATE_DELEGATION_TOKEN,
            39 => Self::RENEW_DELEGATION_TOKEN,
            40 => Self::EXPIRE_DELEGATION_TOKEN,
            41 => Self::DESCRIBE_DELEGATION_TOKEN,
            42 => Self::DELETE_GROUPS,
            43 => Self::ELECT_LEADERS,
            44 => Self::INCREMENTAL_ALTER_CONFIGS,
            45 => Self::ALTER_PARTITION_REASSIGNMENTS,
            46 => Self::LIST_PARTITION_REASSIGNMENTS,
            47 => Self::OFFSET_DELETE,
            48 => Self::DESCRIBE_CLIENT_QUOTAS,
            49 => Self::ALTER_CLIENT_QUOTAS,
            50 => Self::DESCRIBE_USER_SCRAM_CREDENTIALS,
            51 => Self::ALTER_USER_SCRAM_CREDENTIALS,
            52 => Self::VOTE,
            53 => Self::BEGIN_QUORUM_EPOCH,
            54 => Self::END_QUORUM_EPOCH,
            55 => Self::DESCRIBE_QUORUM,
            56 => Self::ALTER_PARTITION,
            57 => Self::UPDATE_FEATURES,
            58 => Self::ENVELOPE,
            59 => Self::FETCH_SNAPSHOT,
            60 => Self::DESCRIBE_CLUSTER,
            61 => Self::DESCRIBE_PRODUCERS,
            62 => Self::BROKER_REGISTRATION,
            63 => Self::BROKER_HEARTBEAT,
            64 => Self::UNREGISTER_BROKER,
            65 => Self::DESCRIBE_TRANSACTIONS,
            66 => Self::LIST_TRANSACTIONS,
            67 => Self::ALLOCATE_PRODUCER_IDS,
            68 => Self::CONSUMER_GROUP_HEARTBEAT,
            69 => Self::CONSUMER_GROUP_DESCRIBE,
            70 => Self::CONTROLLER_REGISTRATION,
            71 => Self::GET_TELEMETRY_SUBSCRIPTIONS,
            72 => Self::PUSH_TELEMETRY,
            73 => Self::ASSIGN_REPLICAS_TO_DIRS,
            74 => Self::LIST_CLIENT_METRICS_RESOURCES,
            75 => Self::DESCRIBE_TOPIC_PARTITIONS,
            76 => Self::SHARE_GROUP_HEARTBEAT,
            77 => Self::SHARE_GROUP_DESCRIBE,
            78 => Self::SHARE_FETCH,
            79 => Self::SHARE_ACKNOWLEDGE,
            80 => Self::ADD_RAFT_VOTER,
            81 => Self::REMOVE_RAFT_VOTER,
            82 => Self::UPDATE_RAFT_VOTER,
            83 => Self::INITIALIZE_SHARE_GROUP_STATE,
            84 => Self::READ_SHARE_GROUP_STATE,
            85 => Self::WRITE_SHARE_GROUP_STATE,
            86 => Self::DELETE_SHARE_GROUP_STATE,
            87 => Self::READ_SHARE_GROUP_STATE_SUMMARY,
            _ => panic!("Unsupported API key {}", api_key)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_version() {
        assert_eq!(ApiMessageType::PRODUCE.request_header_version(0), 1);
        assert_eq!(ApiMessageType::PRODUCE.response_header_version(0), 0);
        
        assert_eq!(ApiMessageType::PRODUCE.request_header_version(1), 1);
        assert_eq!(ApiMessageType::PRODUCE.response_header_version(1), 0);
        
        assert_eq!(ApiMessageType::CREATE_TOPICS.request_header_version(4), 1);
        assert_eq!(ApiMessageType::CREATE_TOPICS.response_header_version(4), 0);
        
        assert_eq!(ApiMessageType::CREATE_TOPICS.request_header_version(5), 2);
        assert_eq!(ApiMessageType::CREATE_TOPICS.response_header_version(5), 1);
        
    }
}
