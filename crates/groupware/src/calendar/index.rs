/*
 * SPDX-FileCopyrightText: 2020 Stalwart Labs LLC <hello@stalw.art>
 *
 * SPDX-License-Identifier: AGPL-3.0-only OR LicenseRef-SEL
 */

use crate::calendar::{ArchivedCalendarScheduling, CalendarScheduling};

use super::{
    ArchivedCalendar, ArchivedCalendarEvent, ArchivedCalendarPreferences, ArchivedDefaultAlert,
    ArchivedTimezone, Calendar, CalendarEvent, CalendarPreferences, DefaultAlert, Timezone,
};
use common::storage::index::{IndexValue, IndexableAndSerializableObject, IndexableObject};
use types::{acl::AclGrant, collection::SyncCollection, field::CalendarField};

impl IndexableObject for Calendar {
    fn index_values(&self) -> impl Iterator<Item = IndexValue<'_>> {
        [
            IndexValue::Acl {
                value: (&self.acls).into(),
            },
            IndexValue::Quota {
                used: self.dead_properties.size() as u32
                    + self.preferences.iter().map(|p| p.size()).sum::<usize>() as u32
                    + self.default_alerts.iter().map(|a| a.size()).sum::<usize>() as u32
                    + self.name.len() as u32,
            },
            IndexValue::LogContainer {
                sync_collection: SyncCollection::Calendar,
            },
        ]
        .into_iter()
    }
}

impl IndexableObject for &ArchivedCalendar {
    fn index_values(&self) -> impl Iterator<Item = IndexValue<'_>> {
        [
            IndexValue::Acl {
                value: self
                    .acls
                    .iter()
                    .map(AclGrant::from)
                    .collect::<Vec<_>>()
                    .into(),
            },
            IndexValue::Quota {
                used: self.dead_properties.size() as u32
                    + self.preferences.iter().map(|p| p.size()).sum::<usize>() as u32
                    + self.default_alerts.iter().map(|a| a.size()).sum::<usize>() as u32
                    + self.name.len() as u32,
            },
            IndexValue::LogContainer {
                sync_collection: SyncCollection::Calendar,
            },
        ]
        .into_iter()
    }
}

impl IndexableAndSerializableObject for Calendar {
    fn is_versioned() -> bool {
        true
    }
}

impl IndexableObject for CalendarEvent {
    fn index_values(&self) -> impl Iterator<Item = IndexValue<'_>> {
        [
            IndexValue::Index {
                field: CalendarField::Uid.into(),
                value: self.data.event.uids().next().into(),
            },
            IndexValue::Quota {
                used: self.dead_properties.size() as u32
                    + self.display_name.as_ref().map_or(0, |n| n.len() as u32)
                    + self.names.iter().map(|n| n.name.len() as u32).sum::<u32>()
                    + self.size,
            },
            IndexValue::LogItem {
                sync_collection: SyncCollection::Calendar,
                prefix: None,
            },
        ]
        .into_iter()
    }
}

impl IndexableObject for &ArchivedCalendarEvent {
    fn index_values(&self) -> impl Iterator<Item = IndexValue<'_>> {
        [
            IndexValue::Index {
                field: CalendarField::Uid.into(),
                value: self.data.event.uids().next().into(),
            },
            IndexValue::Quota {
                used: self.dead_properties.size() as u32
                    + self.display_name.as_ref().map_or(0, |n| n.len() as u32)
                    + self.names.iter().map(|n| n.name.len() as u32).sum::<u32>()
                    + self.size,
            },
            IndexValue::LogItem {
                sync_collection: SyncCollection::Calendar,
                prefix: None,
            },
        ]
        .into_iter()
    }
}

impl IndexableAndSerializableObject for CalendarEvent {
    fn is_versioned() -> bool {
        true
    }
}

impl IndexableObject for CalendarScheduling {
    fn index_values(&self) -> impl Iterator<Item = IndexValue<'_>> {
        [
            IndexValue::Quota { used: self.size },
            IndexValue::Index {
                field: CalendarField::Created.into(),
                value: self.created.into(),
            },
            IndexValue::LogItem {
                sync_collection: SyncCollection::CalendarScheduling,
                prefix: None,
            },
        ]
        .into_iter()
    }
}

impl IndexableObject for &ArchivedCalendarScheduling {
    fn index_values(&self) -> impl Iterator<Item = IndexValue<'_>> {
        [
            IndexValue::Quota {
                used: self.size.to_native(),
            },
            IndexValue::Index {
                field: CalendarField::Created.into(),
                value: self.created.to_native().into(),
            },
            IndexValue::LogItem {
                sync_collection: SyncCollection::CalendarScheduling,
                prefix: None,
            },
        ]
        .into_iter()
    }
}

impl IndexableAndSerializableObject for CalendarScheduling {
    fn is_versioned() -> bool {
        false
    }
}

impl CalendarPreferences {
    pub fn size(&self) -> usize {
        self.name.len()
            + self.description.as_ref().map_or(0, |n| n.len())
            + self.color.as_ref().map_or(0, |n| n.len())
            + self.time_zone.size()
    }
}

impl ArchivedCalendarPreferences {
    pub fn size(&self) -> usize {
        self.name.len()
            + self.description.as_ref().map_or(0, |n| n.len())
            + self.color.as_ref().map_or(0, |n| n.len())
            + self.time_zone.size()
    }
}

impl Timezone {
    pub fn size(&self) -> usize {
        match self {
            Timezone::IANA(_) => 2,
            Timezone::Custom(c) => c.size(),
            Timezone::Default => 0,
        }
    }
}

impl ArchivedTimezone {
    pub fn size(&self) -> usize {
        match self {
            ArchivedTimezone::IANA(_) => 2,
            ArchivedTimezone::Custom(c) => c.size(),
            ArchivedTimezone::Default => 0,
        }
    }
}

impl DefaultAlert {
    pub fn size(&self) -> usize {
        self.alert.size() + self.id.len()
    }
}

impl ArchivedDefaultAlert {
    pub fn size(&self) -> usize {
        self.alert.size() + self.id.len()
    }
}
