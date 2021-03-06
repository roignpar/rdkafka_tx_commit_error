use rdkafka::{
    error::KafkaResult,
    producer::{Producer, ThreadedProducer},
    util::Timeout,
    ClientConfig,
};

fn main() -> KafkaResult<()> {
    env_logger::init();

    let mut config = ClientConfig::new();

    config.set("bootstrap.servers", "localhost:19092");
    config.set("transactional.id", "transaction_test");
    config.set("debug", "eos");

    let prod: ThreadedProducer<_> = config.create()?;

    prod.init_transactions(Timeout::Never)?;
    log::info!("Init transactions successful");

    prod.begin_transaction()?;
    log::info!("Begin transaction successful");

    prod.commit_transaction(Timeout::Never)?;
    log::info!("Commit transaction successful");

    Ok(())
}
