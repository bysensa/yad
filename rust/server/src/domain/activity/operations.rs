use entrait::entrait;

#[entrait(pub ActivityOperations)]
pub mod implementation {
    use serde::Deserialize;

    use serde_with::{serde_as, DurationSeconds};

    use crate::{domain::{
        activity::Activity,
        types::DateTime,
    }, db::{GetEntity, UpdateEntity, Persistent}};

    #[derive(Deserialize)]
    pub struct CreateActivityData {
        pub summary: String,
    }

    #[derive(Deserialize)]
    pub struct ChangeActivitySummary {
        pub id: String,
        pub summary: String,
    }

    #[derive(Deserialize)]
    pub struct ChangeActivityStartedAt {
        pub id: String,
        pub started_at: DateTime,
    }

    #[serde_as]
    #[derive(Deserialize, Debug)]
    pub struct ChangeActivityDuration {
        pub id: String,
        #[serde_as(as = "DurationSeconds<i64>")]
        pub duration: crate::domain::types::Duration,
    }

    pub async fn create_activity(
        deps: &impl crate::db::CreateEntity,
        data: CreateActivityData,
    ) -> anyhow::Result<Activity> {
        let activity = Activity::default().with_summary(data.summary);
        let persistent = crate::db::Persistent::try_from(&activity)?;
        let persistent = deps.create_entity(persistent).await?;
        let activity = persistent.try_extract::<Activity>()?;
        Ok(activity)
    }

    pub async fn change_activity_summary(
        deps: &(impl GetEntity + UpdateEntity),
        data: ChangeActivitySummary,
    ) -> anyhow::Result<Activity> {
        let ChangeActivitySummary { id, summary } = data;
        let activity = deps.get_entity(id).await?.try_extract::<Activity>()?;
        let activity = activity.with_summary(summary);
        let persistent = Persistent::try_from(&activity)?;
        let activity = deps.update_entity(persistent).await?.try_extract::<Activity>()?;
        Ok(activity)
    }

    pub async fn change_activity_started_at(
        deps: &(impl GetEntity + UpdateEntity),
        data: ChangeActivityStartedAt,
    ) -> anyhow::Result<Activity> {
        let ChangeActivityStartedAt { id, started_at } = data;
        let activity = deps.get_entity(id).await?.try_extract::<Activity>()?;
        let activity = activity.with_started_at(started_at);
        let persistent = Persistent::try_from(&activity)?;
        let activity = deps.update_entity(persistent).await?.try_extract::<Activity>()?;
        Ok(activity)
    }

    pub async fn change_activity_duration(
        deps: &(impl GetEntity + UpdateEntity),
        data: ChangeActivityDuration,
    ) -> anyhow::Result<Activity> {
        let ChangeActivityDuration { id, duration } = data;
        let activity = deps.get_entity(id).await?.try_extract::<Activity>()?;
        let activity = activity.with_duration(duration);
        let persistent = Persistent::try_from(&activity)?;
        let activity = deps.update_entity(persistent).await?.try_extract::<Activity>()?;
        Ok(activity)
    }
}
