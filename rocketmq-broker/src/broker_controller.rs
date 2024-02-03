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
use rocketmq_common::common::broker::broker_config::BrokerConfig;
use rocketmq_store::config::message_store_config::MessageStoreConfig;

pub struct BrokerController {
    pub(crate) broker_config: BrokerConfig,
    pub(crate) store_config: MessageStoreConfig,
}

impl BrokerController {
    pub fn new(broker_config: BrokerConfig, store_config: MessageStoreConfig) -> Self {
        Self {
            broker_config,
            store_config,
        }
    }
}

impl BrokerController {
    pub fn start(&mut self) {}
}