use crate::domain::activity::*;

use super::RpcApi;

pub fn register_methods(module: &mut RpcApi) -> anyhow::Result<()> {
    module.register_async_method("create_activity", |params, ctx| async move {
        let data = params.one::<CreateActivityData>().unwrap();
        let res = ctx.create_activity(data).await.unwrap();
        Ok(res)
    })?;

    module.register_async_method("change_activity_summary", |params, ctx| async move {
        let data = params.one::<ChangeActivitySummary>().unwrap();
        let res = ctx.change_activity_summary(data).await.unwrap();
        Ok(res)
    })?;

    module.register_async_method("change_activity_started_at", |params, ctx| async move {
        let data = params.one::<ChangeActivityStartedAt>().unwrap();
        let res = ctx.change_activity_started_at(data).await.unwrap();
        Ok(res)
    })?;

    module.register_async_method("change_activity_duration", |params, ctx| async move {
        let data = params.one::<ChangeActivityDuration>().unwrap();
        let res = ctx.change_activity_duration(data).await.unwrap();
        Ok(res)
    })?;

    Ok(())
}
