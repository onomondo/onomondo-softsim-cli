use chrono::Local;
use clap::Parser;
use env_logger::Builder;
use log::LevelFilter;
use models::fs;
use models::profile;
use models::ss_error;

use std::error::Error;
use std::io::Write;
use std::path::PathBuf;

mod config;
mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let arg = config::Args::parse();
    let verbosity = arg.verbosity;
    let log_level: LevelFilter = match verbosity {
        1 | 2 => LevelFilter::Debug,
        3 => LevelFilter::Trace,
        _ => LevelFilter::Info,
    };

    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, log_level)
        .init();

    log::trace!("{:?}", arg);

    let res = match arg.cmd {
        config::SubCommand::Fetch {
            api_key,
            num_of_profiles,
            output,
            url,
        } => {
            let api_config = models::profile::api::Config {
                api_key: api_key.to_string(),
                url: url.to_string(),
            };
            // match fetch_profiles(&api_config, num_of_profiles, &mut output.clone()).await {
            //     Ok(_) => log::info!("Ok"),
            //     Err(_) => log::info!("Exiting due to error"),
            // }
            fetch_profiles(&api_config, num_of_profiles, &mut output.clone()).await
        }
        config::SubCommand::Next {
            key: private_key,
            set_of_profiles: base_path,
            format,
        } => next(&private_key, &base_path.unwrap(), format),
    };

    if let Err(e) = res {
        match e.downcast_ref::<models::ss_error::GenericError>() {
            Some(err) => {
                log::error!("Exiting due to error: {}", err.message);
            }
            _ => {
                log::error!("Exiting due to error: {}", e);
            }
        }
        std::process::exit(1);
    }

    Ok(())
}

fn get_next(base_path: &PathBuf) -> Result<std::fs::DirEntry, Box<dyn Error>> {
    // find next unused profile.
    let next = std::fs::read_dir(base_path).unwrap().find(|p| {
        if let Ok(entry) = p {
            let path = entry.path();
            let current_path = path
                .file_stem()
                .unwrap_or_else(|| std::ffi::OsStr::new(""))
                .to_str()
                .unwrap();
            if !current_path.starts_with("__") && !current_path.starts_with("profiles") {
                return true;
            }
        }
        false
    });

    match next {
        None => {
            log::error!("No profiles found at {}", base_path.display());
            Err(Box::new(ss_error::GenericError::new(
                "No profiles was found".to_string(),
            )))
        }
        Some(val) => Ok(val?),
    }
}

fn mark_exported(path: &std::fs::DirEntry) -> Result<(), Box<dyn Error>> {
    let mut to = path.path();
    let mut filename = path.file_name().into_string().unwrap();
    filename.insert_str(0, "__");

    to.set_file_name(filename);
    std::fs::rename(path.path(), to)?;

    Ok(())
}

fn read_and_decrypt(
    path: &PathBuf,
    key: &profile::crypto::Key,
) -> Result<profile::Profile, Box<dyn Error>> {
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    let encrypted_profile: models::profile::EncryptedProfile = serde_json::from_reader(reader)?;

    let mut profile = key.decrypt(encrypted_profile.profile())?;
    if profile.iccid.is_none() {
        profile.iccid = Some(encrypted_profile.iccid().clone());
    }

    Ok(profile)
}

async fn fetch_profiles(
    api_config: &models::profile::api::Config,
    profile_count: u32,
    store_at: &mut PathBuf,
) -> Result<(), Box<dyn Error>> {
    log::info!(
        "Fetching {} profiles from {}",
        profile_count,
        api_config.url
    );
    log::debug!("Storing profiles at {}/profiles.json", store_at.display());

    let create_dir = !std::path::Path::new(&store_at).is_dir();
    if create_dir {
        log::debug!("Creating directory {}", store_at.display());
        std::fs::create_dir(&store_at)?;
    }
    let mut profiles_json = store_at.clone();
    profiles_json.push("profiles.json");
    // fail early if file exists
    let mut file = match std::fs::File::options()
        .append(true)
        .create_new(true)
        .write(true)
        .open(&profiles_json)
    {
        Ok(f) => f,
        Err(e) => {
            log::debug!("Failed to open file: {}", e);
            if e.kind() == std::io::ErrorKind::AlreadyExists {
                log::error!(
                    "File already exists at {}. ss_cli won't overwrite existing files.",
                    store_at.display()
                );
                return Err("".into());
            }
            return Err(e.into());
        }
    };

    let profiles = models::profile::api::get(api_config, profile_count).await;
    match profiles {
        Ok(p) => {
            let json = serde_json::to_string(&p)?;

            file.write_all(json.as_bytes())?;

            drop(file);

            p.into_iter().for_each(|profile| {
                fs::store(&profile, store_at, profile.iccid(), "json").unwrap();
            });

            log::info!("Stored profiles in: {}", store_at.display());
            Ok(())
        }
        Err(e) => {
            log::info!("Removing file: {}", profiles_json.display());
            std::fs::remove_file(&profiles_json)?;
            Err(e)
        }
    }
}

fn next(
    key_path: &PathBuf,
    base_path: &PathBuf,
    format: config::Format,
) -> Result<(), Box<dyn Error>> {
    let key = models::profile::crypto::Key::new(key_path)?;

    let profile_path = get_next(base_path)?;
    let profile = read_and_decrypt(&profile_path.path(), &key)?;
    mark_exported(&profile_path)?;

    // formatted_

    // switch on formatting options
    match format {
        config::Format::Hex => {
            std::io::stdout().write_all(profile::encoder::profile_to_hex(&profile).as_bytes())?;
        }

        config::Format::Json => {
            std::io::stdout().write_all(profile::encoder::profile_to_json(&profile)?.as_bytes())?;
        }
    }

    Ok(())
}
