/*
 * SPDX-FileCopyrightText: 2020 Stalwart Labs LLC <hello@stalw.art>
 *
 * SPDX-License-Identifier: AGPL-3.0-only OR LicenseRef-SEL
 */

use super::metadata::MessageMetadata;
use common::Server;
use mail_parser::Message;
use spam_filter::{
    SpamFilterInput, analysis::init::SpamFilterInit, modules::bayes::BayesClassifier,
};
use std::future::Future;
use store::write::{TaskQueueClass, now};
use trc::StoreEvent;
use types::{blob_hash::BlobHash, collection::Collection, field::EmailField};

pub trait EmailBayesTrain: Sync + Send {
    fn email_bayes_train(
        &self,
        account_id: u32,
        span_id: u64,
        message: Message<'_>,
        learn_spam: bool,
    ) -> impl Future<Output = ()> + Send;

    fn email_bayes_queue_task_build(
        &self,
        account_id: u32,
        document_id: u32,
        learn_spam: bool,
    ) -> impl Future<Output = trc::Result<TaskQueueClass>> + Send;
}

impl EmailBayesTrain for Server {
    async fn email_bayes_train(
        &self,
        account_id: u32,
        span_id: u64,
        message: Message<'_>,
        learn_spam: bool,
    ) {
        self.bayes_train_if_balanced(
            &self.spam_filter_init(SpamFilterInput::from_account_message(
                &message, account_id, span_id,
            )),
            learn_spam,
        )
        .await
    }

    async fn email_bayes_queue_task_build(
        &self,
        account_id: u32,
        document_id: u32,
        learn_spam: bool,
    ) -> trc::Result<TaskQueueClass> {
        let metadata = self
            .get_archive_by_property(
                account_id,
                Collection::Email,
                document_id,
                EmailField::Metadata.into(),
            )
            .await?
            .ok_or_else(|| {
                StoreEvent::NotFound
                    .into_err()
                    .account_id(account_id)
                    .document_id(document_id)
            })?;

        Ok(TaskQueueClass::BayesTrain {
            due: now(),
            hash: BlobHash::from(&metadata.unarchive::<MessageMetadata>()?.blob_hash),
            learn_spam,
        })
    }
}
