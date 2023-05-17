use std::any::{Any, TypeId};
use std::error::Error;

use dialoguer::{Confirm, Select};
use structopt::StructOpt;

use crate::{create_image, utils};
use crate::requirements::{CheckResult, checks, Requirement, run_requirements};

#[derive(StructOpt)]
#[structopt(name = "vpc-migration-tools", about = "A collection of useful tools.")]
enum Cli {
    #[structopt(about = "Checks if the system is prepared to be used as an image for IBM Cloud Virtual Servers for VPC.\
     There are some requirements that might be fixed automatically.")]
    CheckRequirements,

    #[structopt(about = "Create a disk image.")]
    CreateImage {
        #[structopt(long = "image-name", help = "The name of the image to create.")]
        image_name: String,

        #[structopt(long = "directory", default_value = "/tmp", help = "The directory where the image will be created.")]
        dir: String,

        #[structopt(long = "skip-free-space", help = "Skip the creation of free space.")]
        skip_free_space: Option<bool>,
    },
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let cli = Cli::from_args();

    match cli {
        Cli::CheckRequirements => {
            run_requirements::run_requirements()
        }
        Cli::CreateImage { image_name, dir, skip_free_space } => {
            let device_list = create_image::partitions::list_available_devices()?;
            let device = ask_user_from_list(device_list, "Select a device to create the image on:")?;
            create_image::run(create_image::run_process::Options {
                skip_free_space,
                image_name,
                dir,
                // device comes from option without /
                device: device.value,
            })
        }
    }
}


fn ask_user_from_list(list: Vec<utils::Option>, message: &str) -> Result<utils::Option, Box<dyn Error>> {
    let mut list = list;
    list.sort_by_key(|a| a.to_string());

    let selection = Select::new()
        .with_prompt(message)
        .items(&list)
        .default(0)
        .interact()?;

    Ok(list.remove(selection))
}

