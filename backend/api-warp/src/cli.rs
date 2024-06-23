use clap::{arg, ArgMatches, Command};

/// # Example
///
/// ```rust
/// let matches = process_command.get_matches_from(vec!["run", "-d" "commands"]).unwrap();
/// assert!(matches.contains_id("domain"));
/// ```
fn run_command() -> Command {
    Command::new("run")
        .about("run the warp server")
        .arg(
            arg!(-d --domain <DOMAIN> "Limit processing to a specific domain (can be more than one!).\nIf not set, all are being processed.\n")
                .next_line_help(true)
                .env("DOMAINS")
                .value_delimiter(',')
                .value_parser(["commands", "events"])
                .default_value("commands,events")
        )
        .arg(
            arg!(-b --brokers <BROKERS> "broker list in kafka format")
                .env("KAFKA_BROKER")
                .default_value("localhost:9092"),
        )
        .arg(
            arg!(-g --"group-id" <GROUP_ID> "consumer group id")
                .env("KAFKA_GROUP_ID")
                .default_value("warp"))
        .arg(
            arg!(-l --"log-conf" <LOG_CONF> "configure the logging format (example: 'rdkafka=trace')")
        )
}

pub fn get_matches() -> ArgMatches {
    Command::new("warp")
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_command() {
        assert!(
            run_command()
            .get_matches_from(vec!["process", "-d", "backpacks"])
            .contains_id("domain")
        );

        assert_eq!(
            run_command()
                .get_matches_from(vec!["process", "-d", "backpacks"])
                .get_many::<String>("domain")
                .unwrap_or_default().map(|v| v.as_str()).collect::<Vec<_>>(),
            vec!["backpacks"]
        );

        assert_eq!(
            run_command()
                .get_matches_from(vec!["process", "-d", "backpacks", "-d", "articles"])
                .get_many::<String>("domain")
                .unwrap_or_default().map(|v| v.as_str()).collect::<Vec<_>>(),
            vec!["backpacks", "articles"]
        );
    }
}
