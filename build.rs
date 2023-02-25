use eyre::Context;

fn main() -> eyre::Result<()> {
    tonic_build::configure()
        .build_client(cfg!(feature = "client"))
        .build_server(cfg!(feature = "server"))
        .compile(
            &[
                proto_path("v1", "shuffle_service"),
                proto_path("v1", "events"),
                proto_path("v1", "types"),
            ],
            &[COIN_SHUFFLE_PROTO],
        )
        .context("failed to build protos")?;

    Ok(())
}

const COIN_SHUFFLE_PROTO: &str = "./protobuf/";

fn proto_path(version: &str, proto_file: &str) -> String {
    format!(
        "{}/coin_shuffle/{version}/{proto_file}.proto",
        COIN_SHUFFLE_PROTO
    )
}
