use crate::constants::*;
use crate::engine::Request;
use macrograph_package_api::{exec_fn, package::Package};

pub fn create_streaming_schemas(package: &mut Package) {
    package.add_exec_schema(
        TOGGLE_STREAM,
        |io| {
            io.bool_output("Streaming");
        },
        exec_fn!(|_io, ctx| ctx.send(Request::ToggleStream)), // TODO
    );

    package.add_exec_schema(
        START_STREAM,
        |_| {},
        exec_fn!(|_io, ctx| ctx.send(Request::StartStream)),
    );

    package.add_exec_schema(
        STOP_STREAM,
        |_| {},
        exec_fn!(|_io, ctx| ctx.send(Request::StopStream)),
    );
}
