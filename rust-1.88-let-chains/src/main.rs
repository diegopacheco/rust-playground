#[allow(dead_code)]
struct Semver {
    major: u32,
    minor: u32,
    patch: u32,
}

#[allow(dead_code)]
enum Channel {
    Stable(Semver),
    Beta(Semver),
    Nightly(Semver),
}

fn release_info() -> Channel {
    Channel::Stable(Semver {
        major: 1,
        minor: 88,
        patch: 0,
    })
}

fn main() {
    if let Channel::Stable(v) = release_info()
        && let Semver { major, minor, .. } = v
        && major == 1
        && minor == 88
    {
        println!("`let_chains` was stabilized in this version");
    }else {
        println!("`let_chains` was not stabilized in this version");
    }
}