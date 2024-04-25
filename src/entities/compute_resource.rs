#[derive(Debug, Clone, PartialEq, PartialOrd, Eq)]
pub enum ComputeResource {Cpu, Memory, Disk }

impl Ord for ComputeResource {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (ComputeResource::Cpu, ComputeResource::Cpu) => std::cmp::Ordering::Equal,
            (ComputeResource::Cpu, ComputeResource::Memory) => std::cmp::Ordering::Greater,
            (ComputeResource::Cpu, ComputeResource::Disk) => std::cmp::Ordering::Greater,
            (ComputeResource::Memory, ComputeResource::Cpu) => std::cmp::Ordering::Less,
            (ComputeResource::Memory, ComputeResource::Memory) => std::cmp::Ordering::Equal,
            (ComputeResource::Memory, ComputeResource::Disk) => std::cmp::Ordering::Greater,
            (ComputeResource::Disk, ComputeResource::Cpu) => std::cmp::Ordering::Less,
            (ComputeResource::Disk, ComputeResource::Memory) => std::cmp::Ordering::Less,
            (ComputeResource::Disk, ComputeResource::Disk) => std::cmp::Ordering::Equal,
        }
    }
}


impl ToString for ComputeResource {
    fn to_string(&self) -> String {
        let string_slice_name = 
            match *self {
                ComputeResource::Cpu => "cpu",
                ComputeResource::Memory => "memory",
                ComputeResource::Disk => "ephemeral-storage",
            };

        string_slice_name.to_string()
    }
}