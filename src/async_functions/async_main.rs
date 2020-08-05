use crate::async_functions::{
    async_cpu::async_cpu,
    async_device::async_device,
    async_dewm::async_dewm,
    async_distro::async_distro,
    async_env::{async_editor, async_shell, async_user},
    async_hostname::async_hostname,
    async_kernel::async_kernel,
    async_memory::async_memory,
    async_music::async_music,
    async_packages::async_packages,
    async_uptime::async_uptime,
};
use clap::ArgMatches;

pub async fn async_main(matches: ArgMatches<'_>) {
    let f1 = async_cpu(&matches);
    let f2 = async_device(&matches);
    let f3 = async_dewm(&matches);
    let f4 = async_distro(&matches);
    let f5 = async_editor(&matches);
    let f6 = async_hostname(&matches);
    let f7 = async_kernel(&matches);
    let f8 = async_memory(&matches);
    let f9 = async_packages(&matches);
    let f10 = async_shell(&matches);
    let f11 = async_uptime(&matches);
    let f12 = async_user(&matches);
    let f13 = async_music(&matches);
    futures::join!(f1, f2, f3, f4, f5, f6, f7, f8, f9, f10, f11, f12, f13);
}
