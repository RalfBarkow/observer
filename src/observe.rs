use chrono::Utc;
use crate::{context::Context, event::Event, frame::Frame, Result};

pub fn observe<F, T, E>(ctx: &Context, closure: F) -> Result<T>
where
    F: FnOnce() -> std::result::Result<(T, E), (failure::Error, E)>,
    T: std::fmt::Debug + Event<T, E> + serde::Serialize,
{
    unimplemented!()
    /*
    ctx.start_frame();
    match closure() {
        Ok((res, evt)) => {
            ctx.end_frame_success(evt);
            Ok(res)
        }
        Err((e, evt)) => {
            ctx.end_frame_error(e, evt);
            Err(e)
        }
    }
    */
    /*
    let new_frame = Frame::new(event.name());
    let temp1 = ctx.get_frame();

    ctx.modify_context(new_frame);

    let result = closure();

    ctx.update_end_ts(Utc::now());
    ctx.update_breadcrumbs(serde_json::to_value(event.map(ctx, &result)).unwrap());

    let temp2 = ctx.get_frame().into_inner();
    if event.is_critical() {
        ctx.enqueue(temp2.clone());
    } else {
        ctx.save_on_local(event.destination(), temp2.clone());
    }

    ctx.modify_context(temp1.into_inner());
    ctx.modify_add(temp2);

    result
    */
}
