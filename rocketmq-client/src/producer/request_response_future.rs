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
use std::error::Error;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicPtr;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;

use rocketmq_common::common::message::message_single::Message;
use tokio::sync::Notify;

use crate::producer::request_callback::RequestCallbackFn;

type AtomicMessagePtr = AtomicPtr<Option<Message>>;
type AtomicCausePtr = AtomicPtr<Box<dyn Error + Send + Sync>>;

pub struct RequestResponseFuture {
    correlation_id: String,
    request_callback: Option<RequestCallbackFn>,
    begin_timestamp: Instant,
    request_msg: Option<Message>,
    timeout_millis: u64,
    notify: Arc<Notify>,
    //response_msg: Arc<Mutex<Option<Message>>>,
    response_msg: AtomicMessagePtr,
    send_request_ok: Arc<AtomicBool>,
    cause: AtomicCausePtr,
    //cause: Arc<Mutex<Option<Box<dyn Error + Send + Sync>>>>,
}

impl RequestResponseFuture {
    pub fn new(
        correlation_id: String,
        timeout_millis: u64,
        request_callback: Option<RequestCallbackFn>,
    ) -> Self {
        Self {
            correlation_id,
            request_callback,
            begin_timestamp: Instant::now(),
            request_msg: None,
            timeout_millis,
            notify: Arc::new(Notify::new()),
            // response_msg: Arc::new(Mutex::new(None)),
            response_msg: AtomicPtr::new(std::ptr::null_mut()),
            send_request_ok: Arc::new(AtomicBool::new(false)),
            cause: AtomicPtr::new(std::ptr::null_mut()),
            //cause: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn execute_request_callback(&self) {
        /*if let Some(callback) = &self.request_callback {
            let send_request_ok = self.send_request_ok.load(Ordering::Acquire);
            let cause = self.cause.lock().await;
            let response_msg = self.response_msg.lock().await;
            if send_request_ok && cause.is_none() {
                if let Some(response) = &*response_msg {
                    callback(Some(response), None);
                }
            } else if let Some(e) = &*cause {
                callback(None, Some(&**e));
            }
        }*/
        unimplemented!()
    }

    pub fn is_timeout(&self) -> bool {
        self.begin_timestamp.elapsed() > Duration::from_millis(self.timeout_millis)
    }

    pub async fn wait_response_message(&self, timeout: Duration) -> Option<Message> {
        /*if tokio::time::timeout(timeout, self.notify.notified())
            .await
            .is_ok()
        {
            return self.get_response_msg();
        }
        None*/

        match tokio::time::timeout(timeout, self.notify.notified()).await {
            Ok(_) => self.get_response_msg(),
            Err(error) => {
                self.set_cause(Box::new(error));
                None
            }
        }
    }

    pub async fn put_response_message(&self, response_msg: Option<Message>) {
        let raw = Box::into_raw(Box::new(response_msg));
        self.response_msg.store(raw, Ordering::Release);
        self.notify.notify_waiters();
    }

    // Getters and setters
    pub fn get_correlation_id(&self) -> &str {
        &self.correlation_id
    }

    pub fn get_timeout_millis(&self) -> u64 {
        self.timeout_millis
    }

    pub fn set_timeout_millis(&mut self, timeout_millis: u64) {
        self.timeout_millis = timeout_millis;
    }

    pub fn get_request_callback(&self) -> Option<&RequestCallbackFn> {
        self.request_callback.as_ref()
    }

    pub fn get_begin_timestamp(&self) -> Instant {
        self.begin_timestamp
    }

    pub fn get_notify(&self) -> Arc<Notify> {
        Arc::clone(&self.notify)
    }

    #[inline]
    pub fn get_response_msg(&self) -> Option<Message> {
        let raw = self.response_msg.load(Ordering::Acquire);
        if raw.is_null() {
            return None;
        }
        let response_msg = unsafe { Box::from_raw(raw) };
        *response_msg
    }

    pub fn set_response_msg(&self, response_msg: Message) {
        let raw = Box::into_raw(Box::new(Some(response_msg)));
        self.response_msg.store(raw, Ordering::Release);
    }

    pub async fn is_send_request_ok(&self) -> bool {
        self.send_request_ok.load(Ordering::Acquire)
    }

    pub fn set_send_request_ok(&self, send_request_ok: bool) {
        self.send_request_ok.store(false, Ordering::Release)
    }

    pub fn get_request_msg(&self) -> Option<&Message> {
        self.request_msg.as_ref()
    }

    pub fn get_cause(&self) -> Option<Box<dyn Error + Send + Sync>> {
        //self.cause.lock().await.clone()
        let raw = self.cause.load(Ordering::Acquire);
        if raw.is_null() {
            return None;
        }
        let cause = unsafe { Box::from_raw(raw) };
        Some(*cause)
    }

    pub fn set_cause(&self, cause: Box<dyn Error + Send + Sync>) {
        /*let mut err = self.cause.lock().await;
         *err = Some(cause); */
        let raw = Box::into_raw(Box::new(cause));
        self.cause.store(raw, Ordering::Release);
    }
}
