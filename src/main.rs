extern crate clap;
use clap::Parser;
use daemonize::Daemonize;
use libc::getuid;

const USRBIN: &str = "/usr/bin/";
static RUNUSR: &str = "/run/user/";
// static UID: u32 = unsafe { getuid() };
// static RUNDIR: String = format!("{RUNUSR}{UID}");

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, multiple_values = true)]
    name: Vec<String>,
}

struct EnvInfo {
    uid: u32,
    uname: String,
    working_dir: String,
    group: String,
    rundir: String,
}

fn init_daemon(cmd: &String, env: &EnvInfo) {
    std::process::Command::new(cmd)
        .spawn()
        .expect(format!("{} failed to start", &cmd).as_str());

    let daemonize = Daemonize::new()
        //.pid_file(format!("{}/{cmd}.pid", env.rundir))
        .chown_pid_file(true)
        //.working_directory(std::env::current_dir().into())
        .user(env.uname.as_str())
        .group(env.group.as_str())
        .exit_action(|| println!("Executed before master process exits"))
        .privileged_action(|| "Executed before drop privileges");

    match daemonize.start() {
        Ok(_) => println!("asdasd"),
        Err(e) => eprintln!("Error, {e} did not daemonize correctly."),
    }
}
fn main() {
    let envinfo = EnvInfo {
        uid: unsafe { getuid() },
        uname: users::get_current_username()
            .unwrap()
            .into_string()
            .unwrap(),
        working_dir: std::env::current_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap(),
        group: "daemon-starter".into(),
        rundir: format!("{RUNUSR}{}/daemon-starter", unsafe { getuid() }),
    };
    // println!("{}", &envinfo.rundir.as_str());
    match std::fs::create_dir(&envinfo.rundir) {
        Ok(x) => {}
        Err(x) => {eprintln!("Creating directory in /run/usr/{}/ failed", &envinfo.uid);}
    };

    let args = Args::parse();
    for i in args.name {
        // println!("{}{}{}", &envinfo.rundir, envinfo.uid, i);
        init_daemon(&i, &envinfo);
    }
}
