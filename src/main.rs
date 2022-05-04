use clap::Parser;
use daemonize::Daemonize;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, multiple_values = true)]
    name: Vec<String>,
}

struct EnvInfo {
    uname: String,
    working_dir: String,
    group: String,
}

fn init_daemon(cmd: &String, env: &EnvInfo) {
    std::process::Command::new(cmd)
        .spawn()
        .expect(format!("{} failed to start", &cmd).as_str());

    let daemonize = Daemonize::new()
        .chown_pid_file(true)
        .working_directory(&env.working_dir)
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
    };

    let args = Args::parse();
    for i in args.name {
        init_daemon(&i, &envinfo);
    }
}
