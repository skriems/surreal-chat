use clap::{arg, ArgMatches, Command};

fn run_command() -> Command {
    Command::new("run")
        .about("run the event processor")
        .arg(
            arg!(-b --brokers <BROKERS> "broker list in kafka format")
                .env("KAFKA_BROKER")
                .default_value("localhost:9092"),
        )
        .arg(
            arg!(-g --"group-id" <GROUP_ID> "consumer group id")
                .env("KAFKA_GROUP_ID")
                .default_value("surreal-events"))
        .arg(
            arg!(-i --"input-topic" <INPUT_TOPIC> "topic to consume from")
                .env("KAFKA_INPUT_TOPIC")
                .default_value("commands")
        )
        .arg(
            arg!(-o --"output-topic" <OUTPUT_TOPIC> "topic to send events to")
                .env("KAFKA_OUTPUT_TOPIC")
                .default_value("events")
        )
        .arg(
            arg!(-w --workers <WORKERS> "topic to consume from")
                .env("KAFKA_WORKERS")
                .value_parser(clap::value_parser!(usize))
                .default_value("1")
        )
        .arg(
            arg!(-l --"log-conf" <LOG_CONF> "configure the logging format (example: 'rdkafka=trace')")
        )
}

pub fn get_matches() -> ArgMatches {
    Command::new("surreal-events")
        .version(option_env!("CARGO_PKG_VERSION").unwrap_or(""))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(run_command())
        .subcommand(
            Command::new("restore")
                .about("restore event stream to kafka")
                .arg_required_else_help(true)
                .arg(arg!(-d --domain <DOMAIN>).help("domain to restore")),
        )
        .subcommand(
            Command::new("backup")
                .about("backup event stream")
                .arg_required_else_help(true)
                .arg(arg!(-d --domain <DOMAIN>).help("domain to backup")),
        )
        .get_matches()
}
