/*
 * Licensed to the Apache Software Foundation (ASF) under one or more
 * contributor license agreements. See the NOTICE file distributed with
 * this work for additional information regarding copyright ownership.
 * The ASF licenses this file to You under the Apache License, Version 2.0
 * (the "License"); you may not use this file except in compliance with
 * the License. You may obtain a copy of the License at
 *
 *    http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
package org.apache.kafka.clients.admin;

import org.apache.kafka.common.KafkaFuture;
import org.apache.kafka.common.TopicPartitionReplica;
import org.apache.kafka.common.requests.DescribeLogDirsResponse;

import java.util.Collection;
import java.util.HashMap;
import java.util.Map;
import java.util.concurrent.ExecutionException;

/**
 * The result of {@link Admin#describeReplicaLogDirs(Collection)}.
 */
public class DescribeReplicaLogDirsResult {
    private final Map<TopicPartitionReplica, KafkaFuture<ReplicaLogDirInfo>> futures;

    DescribeReplicaLogDirsResult(Map<TopicPartitionReplica, KafkaFuture<ReplicaLogDirInfo>> futures) {
        this.futures = futures;
    }

    /**
     * Return a map from replica to future which can be used to check the log directory information of individual replicas
     */
    public Map<TopicPartitionReplica, KafkaFuture<ReplicaLogDirInfo>> values() {
        return futures;
    }

    /**
     * Return a future which succeeds if log directory information of all replicas are available
     */
    public KafkaFuture<Map<TopicPartitionReplica, ReplicaLogDirInfo>> all() {
        return KafkaFuture.allOf(futures.values().toArray(new KafkaFuture[0]))
            .thenApply(v -> {
                Map<TopicPartitionReplica, ReplicaLogDirInfo> replicaLogDirInfos = new HashMap<>();
                for (Map.Entry<TopicPartitionReplica, KafkaFuture<ReplicaLogDirInfo>> entry : futures.entrySet()) {
                    try {
                        replicaLogDirInfos.put(entry.getKey(), entry.getValue().get());
                    } catch (InterruptedException | ExecutionException e) {
                        // This should be unreachable, because allOf ensured that all the futures completed successfully.
                        throw new RuntimeException(e);
                    }
                }
                return replicaLogDirInfos;
            });
    }

    public static class ReplicaLogDirInfo {
        
        private final String currentReplicaLogDir;
        private final long currentReplicaOffsetLag;
        private final String futureReplicaLogDir;
        private final long futureReplicaOffsetLag;

        ReplicaLogDirInfo() {
            this(null, DescribeLogDirsResponse.INVALID_OFFSET_LAG, null, DescribeLogDirsResponse.INVALID_OFFSET_LAG);
        }

        ReplicaLogDirInfo(String currentReplicaLogDir,
                          long currentReplicaOffsetLag,
                          String futureReplicaLogDir,
                          long futureReplicaOffsetLag) {
            this.currentReplicaLogDir = currentReplicaLogDir;
            this.currentReplicaOffsetLag = currentReplicaOffsetLag;
            this.futureReplicaLogDir = futureReplicaLogDir;
            this.futureReplicaOffsetLag = futureReplicaOffsetLag;
        }

        /**
         * The current log directory of the replica of this partition on the given broker.
         * Null if no replica is not found for this partition on the given broker.
         */
        public String getCurrentReplicaLogDir() {
            return currentReplicaLogDir;
        }

        /**
         * Defined as max(HW of partition - LEO of the replica, 0).
         */
        public long getCurrentReplicaOffsetLag() {
            return currentReplicaOffsetLag;
        }

        /**
         * The future log directory of the replica of this partition on the given broker.
         * Null if the replica of this partition is not being moved to another log directory on the given broker.
         */
        public String getFutureReplicaLogDir() {
            return futureReplicaLogDir;
        }

        /**
         * The LEO of the replica - LEO of the future log of this replica in the destination log directory.
         * -1 if either there is not replica for this partition or the replica of this partition is not being moved to another log directory on the given broker.
         */
        public long getFutureReplicaOffsetLag() {
            return futureReplicaOffsetLag;
        }

        @Override
        public String toString() {
            StringBuilder builder = new StringBuilder();
            if (futureReplicaLogDir != null) {
                builder.append("(currentReplicaLogDir=")
                    .append(currentReplicaLogDir)
                    .append(", futureReplicaLogDir=")
                    .append(futureReplicaLogDir)
                    .append(", futureReplicaOffsetLag=")
                    .append(futureReplicaOffsetLag)
                    .append(")");
            } else {
                builder.append("ReplicaLogDirInfo(currentReplicaLogDir=").append(currentReplicaLogDir).append(")");
            }
            return builder.toString();
        }
    }
}
