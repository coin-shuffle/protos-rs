use eyre::Context;

fn main() -> eyre::Result<()> {
    tonic_build::configure()
        .build_client(cfg!(feature = "client"))
        .build_server(cfg!(feature = "server"))
        .compile(
            &[proto_path("v1", "shuffle_service")],
            &[format!("{}/v1", COIN_SHUFFLE_PROTO)],
        )
        .context("failed to build protos")?;

    Ok(())
}

const COIN_SHUFFLE_PROTO: &'static str = "./protobuf/coin_shuffle";

fn proto_path(version: &str, proto_file: &str) -> String {
    format!("{}/{version}/{proto_file}.proto", COIN_SHUFFLE_PROTO)
}
