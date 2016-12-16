use std::io::{self, Write, BufRead, BufReader, StdinLock, Result as IoResult};
use std::process::{Command, Stdio, ChildStdin};
use std::fs::{read_dir, File};
use std::path::{Path};
use std::iter::Iterator;
use std::ops::Range;
use std::str::FromStr;
use std::fmt::Display;
use std::env;

use libudev::{Result as UdevResult, Context, Enumerator};
use num_cpus;
use config::{Config, MachineConfig, StorageDevice, SetupConfig};
use pci_device::PciDevice;
use qemu;

struct Wizard<'a> {
    stdin: StdinLock<'a>,
    udev: Context,
}

impl<'a> Wizard<'a> {
    fn udev_select_gpu(&mut self) -> IoResult<Option<String>> {
        let mut iter = Enumerator::new(&self.udev)?;
        iter.match_subsystem("drm")?;
        let devs: Vec<_> = iter.scan_devices()?.collect();

        println!("");
        let mut devs: Vec<_> = devs.iter().flat_map(|x| x.parent()).filter(|x| x.subsystem() == "pci").map(PciDevice::new).collect();
        devs.sort();
        devs.dedup();
        for (i, dev) in devs.iter().enumerate() {
            println!("[{}]\t{}", i, dev);
        }

        let selection = ask_numeric(&mut self.stdin, "Please select the graphics device you would like to pass through", 0..devs.len());

        let selected = devs.drain(..).skip(selection).next().unwrap();
        //let selected = PciDevice::new(ctx.device_from_syspath(Path::new("/sys/devices/pci0000:00/00:00/0000:00:01.0/0000:01:00.0"))?);
        let iommu_dir = selected.dev.syspath().join("iommu_group").join("devices");
        assert!(iommu_dir.is_dir());

        let mut unrelated_devices = Vec::new();
        let mut related_devices = Vec::new();
        for entry in read_dir(&iommu_dir)? {
            let dev = PciDevice::new(self.udev.device_from_syspath(&entry?.path())?);
            if dev.pci_device() == selected.pci_device() {
                // these are ours
                related_devices.push(dev);
            } else if dev.pci_class == "60400"/*pci bridge*/ {
                // According to the Arch wiki (https://wiki.archlinux.org/index.php/PCI_passthrough_via_OVMF)
                // passing a PCI bridge is fine, so we just ignore those.
                // TODO: This check probably doesn't catch everything so maybe improve it one day.
            } else {
                // now the rest is actually unrelated and therefore problematic
                unrelated_devices.push(dev);
            }
        }

        if unrelated_devices.len() > 0 {
            println!("Warning: There are unrelated devices grouped with the selected GPU:");
            for dev in unrelated_devices {
                println!("\t{}", dev);
            }

            println!("This is a problem as it means that the GPU is not properly isolated - you can only pass entire groups to a VM. All or nothing.");
            println!("While there ARE fixes for this issue, it's not supported by this tool (yet), so you're on your own.");

            if !ask_yesno(&mut self.stdin, "Ignore this and carry on? (not recommended unless you know exactly what you're doing)") {
                println!("Aborted.");
                return Ok(None);
            }
        }

        Ok(Some(related_devices.iter().map(|x| x.id).fold(String::new(), |s, (v, d)| s + &format!("{:04x}:{:04x},", v, d))))
    }

    fn get_passthrough_devs(&self) -> UdevResult<Vec<(u16, u16)>> {
        let mut iter = Enumerator::new(&self.udev)?;
        iter.match_property("DRIVER", "vfio-pci")?;

        Ok(iter.scan_devices()?.map(PciDevice::new).map(|x| x.id).collect())
    }

    fn autoconfigure_mkinitcpio(&mut self, has_modconf: &mut bool) -> IoResult<bool> {
        static MKINITCPIO_CONF: &'static str = "/etc/mkinitcpio.conf";
        if let Ok(f) = File::open(MKINITCPIO_CONF) {
            println!("It seems you are using mkinitcpio. (If you aren't, select NO here!)");

            if ask_yesno(&mut self.stdin, "Would you like me to try to edit the config file for you?") {
                let mut hooks_added = false;
                let mut already_added = false;
                *has_modconf = false;
                let mut mkic_conf = Vec::new();
                for line in BufReader::new(f).lines().flat_map(|x| x.ok()) {
                    static MODULES_PREFIX: &'static str = "MODULES=\"";
                    static HOOKS_PREFIX: &'static str = "HOOKS=\"";
                    if line.starts_with(MODULES_PREFIX) {
                        if line.contains(KERNEL_MODULES) {
                            // Already added.
                            // Now this might still be at a different location (not at the start)
                            // but those users probably know what they're doing anyways.
                            already_added = true;
                            hooks_added = true;
                        } else if line.contains("vfio") || hooks_added {
                            // bail out if there's already vfio stuff in the file
                            // or if there's two MODULES lines for some reason
                            return Ok(false);
                        } else {
                            mkic_conf.push(MODULES_PREFIX.to_owned() + KERNEL_MODULES + " " + &line[MODULES_PREFIX.len()..]);
                            hooks_added = true;
                        }
                    } else {
                        if line.starts_with(HOOKS_PREFIX) && line.contains("modconf") {
                            *has_modconf = true;
                        }
                        mkic_conf.push(line);
                    }
                }

                if already_added {
                    return Ok(true);
                }

                if hooks_added {
                    return sudo_write_file(MKINITCPIO_CONF, |writer| {
                        for line in mkic_conf {
                            writeln!(writer, "{}", line)?;
                        }
                        Ok(())
                    });
                }
            }
        }

        Ok(false)
    }

    fn run(&mut self, target: &Path, workdir: &Path, datadir: &Path) {
        println!("Welcome!");
        println!("This wizard will help you configure a PCI passthrough setup!");
        println!("Some steps are automated, others have to be done manually. Note that answering no ('n') to those will abort the wizard.");
        println!("You can abort the wizard at any point without risking corruptions except where it specifically tells you not to.");
        println!("When you restart the wizard after aborting it, it may lead you through some steps again even though you already completed them. This is fine, just verify that everything is the way it should be and move on.");
        println!("");
        if !ask_yesno(&mut self.stdin, "Understood?") {
            println!("Aborted.");
            return;
        }

        println!("");

        if !is_iommu_enabled() {
            println!("Step 1: Enable IOMMU");
            println!("It's as simple as adding 'intel_iommu=on' or 'amd_iommu=on' to your kernel command line.");
            println!("Do this now, then continue here. Don't reboot yet, there's more we need to configure.");
            println!("");
            if !ask_yesno(&mut self.stdin, "Done?") {
                println!("Aborted.");
                return;
            }
            println!("");
        }

        let passthrough_devs = self.get_passthrough_devs().expect("Failed to check gpu passthrough with udev");
        if passthrough_devs.is_empty() {
            println!("Step 2: Setting up the vfio driver");

            let vfio_params = match self.udev_select_gpu().expect("Failed to select GPU") {
                Some(x) => x,
                None => return,
            };
            println!("Success!");
            println!("");

            let mut has_modconf = false;
            let mut skip_ask = false;
            if self.autoconfigure_mkinitcpio(&mut has_modconf).unwrap_or(false) {
                println!("Success!");
                if !has_modconf {
                    println!("However, it looks like your mkinitcpio is using a nonstandard configuration that does not use the 'modconf' hook.");
                    println!("This hook inserts a config file that tells the vfio driver what PCI devices it should bind to, so things won't work without it.");
                    println!("If our detection just bugged and you actually have the hook enabled, things are obviously fine.");
                    println!("Alternatively, you have to make sure that our configuration at /etc/modprobe.d/vfio.conf (creating right now) is properly applied.");
                    if !ask_yesno(&mut self.stdin, "Done?") {
                        println!("Aborted.");
                        return;
                    }
                } else {
                    skip_ask = true;
                }
            } else {
                println!("Falling back to manual mode.");
                println!("");
                println!("Please tell your initramfs generator to load these kernel modules: {}", KERNEL_MODULES);
                println!("Make sure that they are loaded BEFORE any graphics drivers!");
                println!("For mkinitcpio users, adding them at the START of your MODULES line in /etc/mkinitcpio.conf will take care of this.");
                println!("");
                if !ask_yesno(&mut self.stdin, "Done?") {
                    println!("Aborted.");
                    return;
                }

                println!("Now that the vfio is added, it needs to know what PCI devices it should bind to.");
                println!("We configure this in /etc/modprobe.d/vfio.conf (creating right now) but your initramfs generator needs to understand it.");
                if has_modconf {
                    println!("Looks like your mkinitcpio contains the hook that does this ('modconf') but perhaps you'd like to double-check.");
                } else {
                    println!("Since you're either not using mkinitcpio at all or heavily customized your configuration, you're on your own here. Good luck.");
                }
            }

            let modconf_success = sudo_write_file("/etc/modprobe.d/vfio.conf", |x| {
                writeln!(x, "options vfio-pci ids={}", vfio_params)
            }).unwrap_or(false);
            if !modconf_success {
                panic!("Failed to write modconf");
            }

            if !skip_ask {
                if !ask_yesno(&mut self.stdin, "Done?") {
                    println!("Aborted.");
                    return;
                }
            }

            println!("");
            println!("Step 3: Update initramfs");
            let mut skip_ask = false;
            if ask_yesno(&mut self.stdin, "Are you using the default kernel ('linux')?") {
                let status = Command::new("/usr/bin/sudo").arg("/usr/bin/mkinitcpio")
                    .arg("-p").arg("linux").status().expect("Failed to run mkinitcpio");
                if !status.success() {
                    println!("Got an error from mkinitcpio. Sorry, but you have to fix this on your own.");
                } else {
                    skip_ask = true;
                }
            } else {
                println!("Please run your initramfs generator now and verify that everything works.");
            }

            if !skip_ask {
                if !ask_yesno(&mut self.stdin, "Done?") {
                    println!("Aborted.");
                    return;
                }
            }

            println!("");
            println!("Step 4: Reboot");
            println!("Now that everything is properly configured, the initramfs should load vfio which should then grab your graphics card.");
            println!("Before you boot into Linux again, please check your firmware/bios settings to make sure that the guest GPU is NOT your \
                      primary graphics device. If everything works correctly, the host's graphics drivers will no longer see the card so \
                      remember to plug your monitor to a different output (e.g. Intel onboard gfx).");
            println!("Some firmware implementations initialize a UEFI framebuffer on the card anyways. This is no problem for VFIO but \
                      it may cause your monitor to pick up a black image from the discrete graphics if it's already plugged into both. \
                      If you get no video output whatsoever, your system is probably not creating a virtual console and instead relies \
                      on the UEFI framebuffer entirely. This too can be fixed but that's a lot harder to do and pretty much impossible \
                      if you don't have an emergency/live OS you can boot into.");
            println!("If you don't have anything like that, now is the time to change your mind and undo these changes. (Removing the vfio \
                      module from your initramfs should almost certainly leave you with a perfectly working system.)");
            println!("You have been warned.");
            println!("");
            println!("With that out of the way, your next step after the reboot is simply to launch this wizard again and we can move on!");
        } else { // if something is actually passed through properly
            println!("Step 5: VM setup");
            println!("Looks like everything is working fine so far! Time to configure your VM!");

            let logical_cores = num_cpus::get();
            let physical_cores = num_cpus::get_physical();

            let mut cfg = Config {
                samba: None,
                machine: MachineConfig {
                    memory: ask_anything(&mut self.stdin, "How much memory would you like to assign to it?",
                                         "8G", |x| Some(x.to_owned())),
                    cores: physical_cores,
                    network: None,
                    storage: Vec::new(),

                    hugepages: None,
                    threads: None,
                },
                setup: Some(SetupConfig {
                    cdrom: None,
                    floppy: None,
                    gui: false,
                }),
            };


            if logical_cores == physical_cores * 2 {
                // hyperthreading detected
                cfg.machine.threads = Some(2);
            } else if logical_cores != physical_cores {
                println!("Warning: You have {} logical on {} physical cores. No idea how to handle this.",
                         logical_cores, physical_cores);
            }

            {
                let setup = cfg.setup.as_mut().unwrap();
                if env::var("DISPLAY").is_ok() {
                    println!("It seems you're running this setup in a graphical environment. This can make things a lot easier!");
                    println!("While our objective is of course VGA passthrough, running a virtual display during setup is very convenient for many reasons. We strongly recommend using this.");
                    if ask_yesno(&mut self.stdin, "Would you like to enable virtual graphics (only during setup)?") {
                        setup.gui = true;
                    }
                }
                if !setup.gui {
                    // TODO: Here we would hook them up with mouse-only passthrough so they can
                    // do the setup without losing control over the machine.
                    unimplemented!();
                }

                let mut ask_file = |q| ask_anything(&mut self.stdin, q, "", |x| {
                    if Path::new(x).exists() {
                        Some(x.to_owned())
                    } else {
                        None
                    }
                });
                println!("Configuring VM root hard disk. Only raw disks are supported for now (sorry).");
                println!("WARNING: ALL DATA ON THE BLOCK DEVICE YOU SELECT HERE WILL BE DELETED!");
                cfg.machine.storage.push(StorageDevice {
                    cache: "none".to_owned(),
                    format: "raw".to_owned(),
                    path: ask_file("Please enter the path to the VM root block device"),
                });

                // Stage 1: First boot and Windows setup
                setup.cdrom = Some(ask_file("Please enter the path to your Windows ISO"));
                setup.floppy = Some(datadir.join("virtio-win.vfd").to_str().unwrap().to_owned());
            }
            println!("Your VM is going to boot now.");
            println!("Just install Windows and shut it down cleanly as soon as that's done so we can continue.");
            println!("");
            println!("Note: Windows probably won't pick up the virtio-scsi storage device right away. You can load the drivers from the attached floppy drive.");
            if !ask_yesno(&mut self.stdin, "Ready?") {
                println!("Aborted.");
                return;
            }

            qemu::run(&cfg, workdir, datadir);

            println!("Alright, so far so good!");

            unimplemented!();
        }
    }
}

static KERNEL_MODULES: &'static str = "vfio vfio_iommu_type1 vfio_pci vfio_virqfd";

fn is_iommu_enabled() -> bool {
    read_dir("/sys/devices/virtual/iommu/").ok().and_then(|mut x| x.next()).is_some()
}

fn sudo_write_file<P: AsRef<Path>, F: FnOnce(&mut ChildStdin) -> IoResult<()>>(path: P, write: F) -> IoResult<bool> {
    let mut writer_child = Command::new("/usr/bin/sudo").env("SUDO_EDITOR", "/usr/bin/tee")
        .arg("-e").arg(path.as_ref().to_str().unwrap()).stdin(Stdio::piped()).stdout(Stdio::null()).spawn()?;
    {
        let mut writer = writer_child.stdin.as_mut().unwrap();
        write(writer)?;
    }
    Ok(writer_child.wait()?.success())
}

fn ask_yesno(stdin: &mut StdinLock, question: &str) -> bool {
    ask_anything(stdin, question, "y/n", |line| {
        if line == "y" { Some(true) }
        else if line == "n" { Some(false) }
        else { None }
    })
}

fn ask_numeric<T : PartialOrd<T> + FromStr + Copy + Display>(stdin: &mut StdinLock, question: &str, range: Range<T>) -> T {
    // bug: the options string is off by one as the end index is exclusive
    ask_anything(stdin, question, &format!("{}..{}", range.start, range.end), |line| {
        line.parse().ok().and_then(|x| if (range.start <= x) && (range.end > x) { Some(x) } else { None })
    })
}

fn ask_anything<T, F: Fn(&str) -> Option<T>>(stdin: &mut StdinLock, question: &str, options: &str, parse_answer: F) -> T {
    let mut line = String::new();
    loop {
        print!("{} [{}] ", question, options);
        io::stdout().flush().ok().expect("Could not flush stdout");
        stdin.read_line(&mut line).unwrap();

        if let Some(t) = parse_answer(&line[..(line.len() - 1)]) {
            return t;
        }

        line.clear();
    }
}


pub fn run(target: &Path, workdir: &Path, datadir: &Path) {
    let stdin = io::stdin();
    Wizard {
        stdin: stdin.lock(),
        udev: Context::new().expect("Failed to create udev context"),
    }.run(target, workdir, datadir);
}