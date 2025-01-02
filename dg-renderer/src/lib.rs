use std::sync::Arc;

use vulkano::{
    device::{
        physical::PhysicalDevice, Device, DeviceCreateInfo, Queue, QueueCreateInfo, QueueFlags,
    }, instance::{Instance, InstanceCreateInfo}, memory::allocator::StandardMemoryAllocator, VulkanLibrary
};

pub mod immediate_draw;

struct State {
    library: Arc<VulkanLibrary>,
    instance: Arc<Instance>,
    device: Arc<Device>,
    queue: Arc<Queue>,
    memory_allocator: Arc<StandardMemoryAllocator>,
}

impl State {
    pub fn initialize() -> Self {
        let library = VulkanLibrary::new().expect("vulkan library / dll not found");
        let instance = Instance::new(library.clone(), InstanceCreateInfo::default())
            .expect("failed to create instance");

        let physical_device: Arc<PhysicalDevice> = instance
            .enumerate_physical_devices()
            .expect("could not enumerate devices")
            .next()
            .expect("no devices available");

        let queue_family_index = physical_device
            .queue_family_properties()
            .iter()
            .enumerate()
            .position(|(_index, properties)| properties.queue_flags.contains(QueueFlags::GRAPHICS))
            .expect("couldn't find a graphical queue family")
            as u32;

        let (device, mut queues) = Device::new(
            physical_device,
            DeviceCreateInfo {
                queue_create_infos: vec![QueueCreateInfo {
                    queue_family_index,
                    ..Default::default()
                }],
                ..Default::default()
            },
        )
        .expect("failed to create device");

        let queue = queues.next().unwrap();

        let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));

        Self {
            library,
            instance,
            device,
            queue,
            memory_allocator,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::State;

    #[test]
    fn initialization() {
        let _state = State::initialize();
    }
}
