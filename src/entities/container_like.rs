use k8s_openapi::api::core::v1::Container;

pub trait ContainerLike {
    fn into_container(&self) -> anyhow::Result<Container>;
}