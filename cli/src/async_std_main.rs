mod args;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    use args::{Args, Cmd};
    use async_std_gpiod as gpiod;

    let args: Args = clap::Parser::parse();

    match args.cmd {
        Cmd::Detect => {
            let chips = futures::future::join_all(
                gpiod::Chip::list_devices()
                    .await?
                    .into_iter()
                    .map(gpiod::Chip::new),
            )
            .await
            .into_iter()
            .collect::<std::io::Result<Vec<_>>>()?;

            chips
                .iter()
                .rev() //Do it in reverse order because the numbers of the GPIO chips go from high to low
                .for_each(|f| println!("{}", f));
        }

        Cmd::Info { chip } => {
            let chips = futures::future::join_all(
                gpiod::Chip::list_devices()
                    .await?
                    .into_iter()
                    .filter(|path| {
                        chip.is_empty()
                            || chip.iter().any(|chip| {
                                path.to_str()
                                    .map(|path| path.ends_with(chip))
                                    .unwrap_or(false)
                            })
                    })
                    .map(gpiod::Chip::new),
            )
            .await
            .into_iter()
            .collect::<std::io::Result<Vec<_>>>()?;

            for index in (0..chips.len()).rev() {
                let chip = &chips[index];
                println!("{}", chip);
                for line in 0..chip.num_lines() {
                    let line_info = chip.line_info(line).await?;
                    println!("\t line \t {}: \t {}", line, line_info);
                }
            }
        }

        Cmd::Get {
            bias,
            active,
            consumer,
            chip,
            lines,
        } => {
            let chip = gpiod::Chip::new(&chip).await?;

            let input = chip
                .request_lines(
                    gpiod::Options::input(&lines)
                        .active(active)
                        .bias(bias)
                        .consumer(&consumer),
                )
                .await?;

            let values = lines.iter().map(|_| false).collect::<Vec<_>>();

            let values = input.get_values(values).await?;

            for value in values {
                print!("{} ", if value { 1 } else { 0 });
            }
            println!();
        }

        Cmd::Set {
            bias,
            active,
            drive,
            consumer,
            chip,
            line_values,
        } => {
            let chip = gpiod::Chip::new(&chip).await?;

            let (lines, values): (Vec<_>, Vec<_>) = line_values
                .into_iter()
                .map(|pair| (pair.line, pair.value))
                .unzip();

            let output = chip
                .request_lines(
                    gpiod::Options::output(&lines)
                        .active(active)
                        .bias(bias)
                        .drive(drive)
                        .values(&values)
                        .consumer(&consumer),
                )
                .await?;

            //output.set_values(values)?;
            let values = output.get_values(values).await?;

            for value in values {
                print!("{} ", if value { 1 } else { 0 });
            }
            println!();
        }

        Cmd::Mon {
            edge,
            bias,
            active,
            consumer,
            chip,
            lines,
        } => {
            let chip = gpiod::Chip::new(&chip).await?;

            let mut input = chip
                .request_lines(
                    gpiod::Options::input(&lines)
                        .active(active)
                        .edge(edge)
                        .bias(bias)
                        .consumer(&consumer),
                )
                .await?;

            loop {
                let event = input.read_event().await?;
                println!(
                    "line {}: {}-edge [{}]",
                    lines[event.line as usize],
                    event.edge,
                    event.time.as_nanos(),
                );
            }
        }

        #[cfg(feature = "complete")]
        Cmd::Complete { shell } => {
            let mut cmd = <Args as clap::CommandFactory>::command();
            let name = cmd.get_name().to_string();
            clap_complete::generate(shell, &mut cmd, name, &mut std::io::stdout());
        }
    }

    Ok(())
}
