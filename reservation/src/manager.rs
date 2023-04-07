use crate::{ReservationId, ReservationManager, Rsvp};
use abi::{Error, Reservation, ReservationQuery};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::postgres::types::PgRange;
use sqlx::{PgPool, Row};

#[async_trait]
impl Rsvp for ReservationManager {
    async fn reserve(&self, mut rsvp: Reservation) -> Result<Reservation, Error> {
        if rsvp.start.is_none() || rsvp.end.is_none() {
            return Err(Error::InvalidTime);
        }
        // let status = abi::ReservationStatus::from_i32(rsvp.status)
        //     .unwrap_or(abi::ReservationStatus::Pending);

        let start = abi::convert_to_utc_time(rsvp.start.as_ref().unwrap().clone());
        let end = abi::convert_to_utc_time(rsvp.end.as_ref().unwrap().clone());

        let timespan: PgRange<DateTime<Utc>> = (start..end).into();

        // generate a insert sql for the reservation
        // execute the sql
        let id = sqlx::query(
            "INSERT INTO reservation (user_id, resource_id, timespan, note, status) VALUES ($1, $2, $3, $4, $5) RETURNING id"
        )
            .bind(rsvp.user_id.clone())
            .bind(rsvp.resource_id.clone())
            .bind(timespan)
            .bind(rsvp.note.clone())
            .bind(rsvp.status)
            .fetch_one(&self.pool)
            .await?.get(0);

        rsvp.id = id;

        Ok(rsvp)
    }

    async fn change_status(&self, _id: ReservationId) -> Result<Reservation, Error> {
        todo!()
    }

    async fn update_note(&self, _id: ReservationId, _note: String) -> Result<Reservation, Error> {
        todo!()
    }

    async fn delete(&self, _id: ReservationId) -> Result<(), Reservation> {
        todo!()
    }

    async fn get(&self, _id: ReservationId) -> Result<Reservation, Error> {
        todo!()
    }

    async fn query(&self, _query: ReservationQuery) -> Result<Vec<Reservation>, Error> {
        todo!()
    }
}

impl ReservationManager {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

// #[cfg(test)]
// mod tests {
// use super::*;

// #[sqlx_database_tester::test(pool(variable = "migrated_pool", migrations = "../migrations"))]
// async fn reserve_should_work_for_valid_window() {
//     let manager = ReservationManager::new(migrated_pool.clone());
//
//     let rsvp = abi::Reservation::new_pending(
//         "liweiid",
//         "ocean-view-room-713",
//         "2022-12-25T15:00:00-0700".parse().unwrap(),
//         "2022-12-28T12:00:00-0700".parse().unwrap(),
//         "I'll arrive at 3pm. Please help to upgrade to execuitive room if possible.",
//     );
//
//     let rsvp = manager.reserve(rsvp).await.unwrap();
//     assert!(!rsvp.id.is_empty());
// }
//
// #[sqlx_database_tester::test(pool(variable = "migrated_pool", migrations = "../migrations"))]
// async fn reserve_conflict_reservation_should_reject() {
//     let manger = ReservationManager::new(migrated_pool.clone());
//     let rsvp1 = abi::Reservation::new_pending(
//         "tyrid",
//         "ocean-view-room-713",
//         "2022-12-25T15:00:00-0700".parse().unwrap(),
//         "2022-12-28T12:00:00-0700".parse().unwrap(),
//         "hello.",
//     );
//     let rsvp2 = abi::Reservation::new_pending(
//         "aliceid",
//         "ocean-view-room-713",
//         "2022-12-26T15:00:00-0700".parse().unwrap(),
//         "2022-12-30T12:00:00-0700".parse().unwrap(),
//         "hello.",
//     );
//
//     let _rsvp1 = manger.reserve(rsvp1).await.unwrap();
//     let err = manger.reserve(rsvp2).await.unwrap_err();
//     println!("{:?}", err);
//     if let abi::Error::ConflictReservation(_info) = err {
//         // assert_eq!(info, "conflict");
//         // assert_eq!(info.resource_id, "ocean-view-room-713");
//         // assert_eq!(
//         //     info.timespan.start,
//         //     "2022-12-26T15:00:00-0700".parse().unwrap()
//         // );
//         // assert_eq!(
//         //     info.timespan.end,
//         //     "2022-12-28T12:00:00-0700".parse().unwrap()
//         // );
//     }
// }
// }
