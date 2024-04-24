#[derive(Debug, Clone)]
pub enum ComputeResource { Cpu, Memory, Disk }


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