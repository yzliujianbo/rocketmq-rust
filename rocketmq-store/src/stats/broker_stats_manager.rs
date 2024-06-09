/*
 * Licensed to the Apache Software Foundation (ASF) under one or more
 * contributor license agreements.  See the NOTICE file distributed with
 * this work for additional information regarding copyright ownership.
 * The ASF licenses this file to You under the Apache License, Version 2.0
 * (the "License"); you may not use this file except in compliance with
 * the License.  You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use std::{collections::HashMap, sync::Arc};

use rocketmq_common::common::{
    broker::broker_config::BrokerConfig,
    statistics::state_getter::StateGetter,
    stats::{moment_stats_item_set::MomentStatsItemSet, stats_item_set::StatsItemSet, Stats},
};

pub struct BrokerStatsManager {
    stats_table: Arc<HashMap<String, StatsItemSet>>,
    cluster_name: String,
    enable_queue_stat: bool,
    moment_stats_item_set_fall_size: MomentStatsItemSet,
    moment_stats_item_set_fall_time: MomentStatsItemSet,
    producer_state_getter: Option<Arc<Box<dyn StateGetter>>>,
    consumer_state_getter: Option<Arc<Box<dyn StateGetter>>>,
    broker_config: Option<Arc<BrokerConfig>>,
}

impl BrokerStatsManager {
    pub const TOPIC_PUT_LATENCY: &'static str = "TOPIC_PUT_LATENCY";
    pub const GROUP_ACK_NUMS: &'static str = "GROUP_ACK_NUMS";
    pub const GROUP_CK_NUMS: &'static str = "GROUP_CK_NUMS";
    pub const DLQ_PUT_NUMS: &'static str = "DLQ_PUT_NUMS";
    pub const BROKER_ACK_NUMS: &'static str = "BROKER_ACK_NUMS";
    pub const BROKER_CK_NUMS: &'static str = "BROKER_CK_NUMS";
    pub const BROKER_GET_NUMS_WITHOUT_SYSTEM_TOPIC: &'static str =
        "BROKER_GET_NUMS_WITHOUT_SYSTEM_TOPIC";
    pub const BROKER_PUT_NUMS_WITHOUT_SYSTEM_TOPIC: &'static str =
        "BROKER_PUT_NUMS_WITHOUT_SYSTEM_TOPIC";
    pub const SNDBCK2DLQ_TIMES: &'static str = "SNDBCK2DLQ_TIMES";

    pub const COMMERCIAL_OWNER: &'static str = "Owner";

    pub const ACCOUNT_OWNER_PARENT: &'static str = "OWNER_PARENT";
    pub const ACCOUNT_OWNER_SELF: &'static str = "OWNER_SELF";

    pub const ACCOUNT_STAT_INVERTAL: u64 = 60 * 1000;
    pub const ACCOUNT_AUTH_TYPE: &'static str = "AUTH_TYPE";

    pub const ACCOUNT_SEND: &'static str = "SEND";
    pub const ACCOUNT_RCV: &'static str = "RCV";
    pub const ACCOUNT_SEND_BACK: &'static str = "SEND_BACK";
    pub const ACCOUNT_SEND_BACK_TO_DLQ: &'static str = "SEND_BACK_TO_DLQ";
    pub const ACCOUNT_AUTH_FAILED: &'static str = "AUTH_FAILED";
    pub const ACCOUNT_SEND_REJ: &'static str = "SEND_REJ";
    pub const ACCOUNT_REV_REJ: &'static str = "RCV_REJ";

    pub const MSG_NUM: &'static str = "MSG_NUM";
    pub const MSG_SIZE: &'static str = "MSG_SIZE";
    pub const SUCCESS_MSG_NUM: &'static str = "SUCCESS_MSG_NUM";
    pub const FAILURE_MSG_NUM: &'static str = "FAILURE_MSG_NUM";
    pub const COMMERCIAL_MSG_NUM: &'static str = "COMMERCIAL_MSG_NUM";
    pub const SUCCESS_REQ_NUM: &'static str = "SUCCESS_REQ_NUM";
    pub const FAILURE_REQ_NUM: &'static str = "FAILURE_REQ_NUM";
    pub const SUCCESS_MSG_SIZE: &'static str = "SUCCESS_MSG_SIZE";
    pub const FAILURE_MSG_SIZE: &'static str = "FAILURE_MSG_SIZE";
    pub const RT: &'static str = "RT";
    pub const INNER_RT: &'static str = "INNER_RT";

    #[deprecated]
    pub const GROUP_GET_FALL_SIZE: &'static str = "GROUP_GET_FALL_SIZE";
    #[deprecated]
    pub const GROUP_GET_FALL_TIME: &'static str = "GROUP_GET_FALL_TIME";
    // Pull Message Latency
    #[deprecated]
    pub const GROUP_GET_LATENCY: &'static str = "GROUP_GET_LATENCY";

    // Consumer Register Time
    pub const CONSUMER_REGISTER_TIME: &'static str = "CONSUMER_REGISTER_TIME";
    // Producer Register Time
    pub const PRODUCER_REGISTER_TIME: &'static str = "PRODUCER_REGISTER_TIME";
    pub const CHANNEL_ACTIVITY: &'static str = "CHANNEL_ACTIVITY";
    pub const CHANNEL_ACTIVITY_CONNECT: &'static str = "CONNECT";
    pub const CHANNEL_ACTIVITY_IDLE: &'static str = "IDLE";
    pub const CHANNEL_ACTIVITY_EXCEPTION: &'static str = "EXCEPTION";
    pub const CHANNEL_ACTIVITY_CLOSE: &'static str = "CLOSE";
}

impl BrokerStatsManager {
    pub fn new(broker_config: Arc<BrokerConfig>) -> Self {
        let stats_table = Arc::new(HashMap::new());
        let moment_stats_item_set_fall_size =
            MomentStatsItemSet::new(Stats::GROUP_GET_FALL_SIZE.to_string());
        let moment_stats_item_set_fall_time =
            MomentStatsItemSet::new(Stats::GROUP_GET_FALL_TIME.to_string());
        let enable_queue_stat = broker_config.enable_detail_stat;
        let cluster_name = broker_config
            .broker_identity
            .broker_cluster_name
            .to_string();
        BrokerStatsManager {
            stats_table,
            cluster_name,
            enable_queue_stat,
            moment_stats_item_set_fall_size,
            moment_stats_item_set_fall_time,
            producer_state_getter: None,
            consumer_state_getter: None,
            broker_config: None,
        }
    }

    pub fn get_stats_table(&self) -> Arc<HashMap<String, StatsItemSet>> {
        Arc::clone(&self.stats_table)
    }

    pub fn get_cluster_name(&self) -> &str {
        &self.cluster_name
    }

    pub fn get_enable_queue_stat(&self) -> bool {
        self.enable_queue_stat
    }

    pub fn get_moment_stats_item_set_fall_size(&self) -> &MomentStatsItemSet {
        &self.moment_stats_item_set_fall_size
    }

    pub fn get_moment_stats_item_set_fall_time(&self) -> &MomentStatsItemSet {
        &self.moment_stats_item_set_fall_time
    }

    pub fn get_broker_puts_num_without_system_topic(&self) -> u64 {
        0
    }

    pub fn get_broker_gets_num_without_system_topic(&self) -> u64 {
        0
    }
}

pub fn build_commercial_stats_key(owner: &str, topic: &str, group: &str, type_: &str) -> String {
    format!("{}@{}@{}@{}", owner, topic, group, type_)
}

pub fn build_account_stats_key(
    account_owner_parent: &str,
    account_owner_self: &str,
    instance_id: &str,
    topic: &str,
    group: &str,
    msg_type: &str,
) -> String {
    format!(
        "{}@{}@{}@{}@{}@{}",
        account_owner_parent, account_owner_self, instance_id, topic, group, msg_type
    )
}

pub fn build_account_stats_key_with_flowlimit(
    account_owner_parent: &str,
    account_owner_self: &str,
    instance_id: &str,
    topic: &str,
    group: &str,
    msg_type: &str,
    flowlimit_threshold: &str,
) -> String {
    format!(
        "{}@{}@{}@{}@{}@{}@{}",
        account_owner_parent,
        account_owner_self,
        instance_id,
        topic,
        group,
        msg_type,
        flowlimit_threshold
    )
}

pub fn build_account_stat_key(
    owner: &str,
    instance_id: &str,
    topic: &str,
    group: &str,
    msg_type: &str,
) -> String {
    format!("{}|{}|{}|{}|{}", owner, instance_id, topic, group, msg_type)
}

pub fn build_account_stat_key_with_flowlimit(
    owner: &str,
    instance_id: &str,
    topic: &str,
    group: &str,
    msg_type: &str,
    flowlimit_threshold: &str,
) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}",
        owner, instance_id, topic, group, msg_type, flowlimit_threshold
    )
}

pub fn split_account_stat_key(account_stat_key: &str) -> Vec<&str> {
    account_stat_key.split('|').collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_commercial_stats_key_creates_correct_key() {
        let key = build_commercial_stats_key("owner1", "topic1", "group1", "type1");
        assert_eq!(key, "owner1@topic1@group1@type1");
    }

    #[test]
    fn build_account_stats_key_creates_correct_key() {
        let key = build_account_stats_key("parent1", "self1", "id1", "topic1", "group1", "type1");
        assert_eq!(key, "parent1@self1@id1@topic1@group1@type1");
    }

    #[test]
    fn build_account_stats_key_with_flowlimit_creates_correct_key() {
        let key = build_account_stats_key_with_flowlimit(
            "parent1", "self1", "id1", "topic1", "group1", "type1", "limit1",
        );
        assert_eq!(key, "parent1@self1@id1@topic1@group1@type1@limit1");
    }

    #[test]
    fn build_account_stat_key_creates_correct_key() {
        let key = build_account_stat_key("owner1", "id1", "topic1", "group1", "type1");
        assert_eq!(key, "owner1|id1|topic1|group1|type1");
    }

    #[test]
    fn build_account_stat_key_with_flowlimit_creates_correct_key() {
        let key = build_account_stat_key_with_flowlimit(
            "owner1", "id1", "topic1", "group1", "type1", "limit1",
        );
        assert_eq!(key, "owner1|id1|topic1|group1|type1|limit1");
    }

    #[test]
    fn split_account_stat_key_splits_correctly() {
        let parts = split_account_stat_key("part1|part2|part3|part4|part5");
        assert_eq!(parts, vec!["part1", "part2", "part3", "part4", "part5"]);
    }
}