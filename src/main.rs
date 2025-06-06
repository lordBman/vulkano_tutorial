use std::sync::Arc;

use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage};
use vulkano::device::{Device, DeviceCreateInfo, QueueCreateInfo, QueueFlags};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator};
use vulkano::VulkanLibrary;
use vulkano::instance::{  Instance, InstanceCreateFlags, InstanceCreateInfo };

fn main() {
    let library = VulkanLibrary::new().expect("no local Vulkan library/DLL");
    let instance = Instance::new(library, InstanceCreateInfo{
        flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
        ..Default::default()
    }).expect("failed to create vulkan instance");

    let mut physical_devices = instance.enumerate_physical_devices().expect("could not enumerate devices");
    println!("Found {} devices compatible with Vulkan", physical_devices.len());
    
    let physical_device = physical_devices.next().expect("no devices available");
    println!("API Version:{}", physical_device.api_version());

    if let Ok(display_properties) = physical_device.display_properties(){
        for property in display_properties{
            println!("Display dimentions: {:?}", property.physical_dimensions());
            println!("Display resolution: {:?}", property.physical_resolution());
        }
    }

    for family in physical_device.queue_family_properties() {
        println!("Found a queue family with {:?} queue(s) with flags: {:?}", family.queue_count, family.queue_flags);
    }

    let queue_family_index = physical_device
        .queue_family_properties()
        .iter()
        .enumerate()
        .position(|(_queue_family_index, queue_family_properties)| {
            queue_family_properties.queue_flags.contains(QueueFlags::GRAPHICS)
        })
        .expect("couldn't find a graphical queue family") as u32;

    let (device, mut queues) = Device::new(physical_device,
        DeviceCreateInfo {
            // here we pass the desired queue family to use by index
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index,
                ..Default::default()
            }],
            ..Default::default()
        }).expect("failed to create device");

    let queue = queues.next().unwrap();

    let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));

    let data: i32 = 12;
    let buffer = Buffer::from_data(
        memory_allocator.clone(),
        BufferCreateInfo {
            usage: BufferUsage::UNIFORM_BUFFER,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..Default::default()
        },
        data,
    ).expect("failed to create buffer");
}
