use vulkano::VulkanLibrary;
use vulkano::instance::{  Instance, InstanceCreateFlags, InstanceCreateInfo, InstanceExtensions };

fn main() {
    let library = VulkanLibrary::new().expect("no local Vulkan library/DLL");
    let instance = Instance::new(library, InstanceCreateInfo{
        flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
        ..Default::default()
    }).expect("failed to create vulkan instance");

    let physical_device = instance.enumerate_physical_devices().expect("could not enumerate devices").next().expect("no devices available");

    println!("Hello, world!");
}
