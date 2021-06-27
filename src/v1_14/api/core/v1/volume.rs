// Generated from definition io.k8s.api.core.v1.Volume

/// Volume represents a named volume in a pod that may be accessed by any container in the pod.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Volume {
    /// AWSElasticBlockStore represents an AWS Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
    pub aws_elastic_block_store: Option<crate::api::core::v1::AWSElasticBlockStoreVolumeSource>,

    /// AzureDisk represents an Azure Data Disk mount on the host and bind mount to the pod.
    pub azure_disk: Option<crate::api::core::v1::AzureDiskVolumeSource>,

    /// AzureFile represents an Azure File Service mount on the host and bind mount to the pod.
    pub azure_file: Option<crate::api::core::v1::AzureFileVolumeSource>,

    /// CephFS represents a Ceph FS mount on the host that shares a pod's lifetime
    pub cephfs: Option<crate::api::core::v1::CephFSVolumeSource>,

    /// Cinder represents a cinder volume attached and mounted on kubelets host machine More info: https://releases.k8s.io/HEAD/examples/mysql-cinder-pd/README.md
    pub cinder: Option<crate::api::core::v1::CinderVolumeSource>,

    /// ConfigMap represents a configMap that should populate this volume
    pub config_map: Option<crate::api::core::v1::ConfigMapVolumeSource>,

    /// CSI (Container Storage Interface) represents storage that is handled by an external CSI driver (Alpha feature).
    pub csi: Option<crate::api::core::v1::CSIVolumeSource>,

    /// DownwardAPI represents downward API about the pod that should populate this volume
    pub downward_api: Option<crate::api::core::v1::DownwardAPIVolumeSource>,

    /// EmptyDir represents a temporary directory that shares a pod's lifetime. More info: https://kubernetes.io/docs/concepts/storage/volumes#emptydir
    pub empty_dir: Option<crate::api::core::v1::EmptyDirVolumeSource>,

    /// FC represents a Fibre Channel resource that is attached to a kubelet's host machine and then exposed to the pod.
    pub fc: Option<crate::api::core::v1::FCVolumeSource>,

    /// FlexVolume represents a generic volume resource that is provisioned/attached using an exec based plugin.
    pub flex_volume: Option<crate::api::core::v1::FlexVolumeSource>,

    /// Flocker represents a Flocker volume attached to a kubelet's host machine. This depends on the Flocker control service being running
    pub flocker: Option<crate::api::core::v1::FlockerVolumeSource>,

    /// GCEPersistentDisk represents a GCE Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
    pub gce_persistent_disk: Option<crate::api::core::v1::GCEPersistentDiskVolumeSource>,

    /// GitRepo represents a git repository at a particular revision. DEPRECATED: GitRepo is deprecated. To provision a container with a git repo, mount an EmptyDir into an InitContainer that clones the repo using git, then mount the EmptyDir into the Pod's container.
    pub git_repo: Option<crate::api::core::v1::GitRepoVolumeSource>,

    /// Glusterfs represents a Glusterfs mount on the host that shares a pod's lifetime. More info: https://releases.k8s.io/HEAD/examples/volumes/glusterfs/README.md
    pub glusterfs: Option<crate::api::core::v1::GlusterfsVolumeSource>,

    /// HostPath represents a pre-existing file or directory on the host machine that is directly exposed to the container. This is generally used for system agents or other privileged things that are allowed to see the host machine. Most containers will NOT need this. More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath
    pub host_path: Option<crate::api::core::v1::HostPathVolumeSource>,

    /// ISCSI represents an ISCSI Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://releases.k8s.io/HEAD/examples/volumes/iscsi/README.md
    pub iscsi: Option<crate::api::core::v1::ISCSIVolumeSource>,

    /// Volume's name. Must be a DNS_LABEL and unique within the pod. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    pub name: String,

    /// NFS represents an NFS mount on the host that shares a pod's lifetime More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
    pub nfs: Option<crate::api::core::v1::NFSVolumeSource>,

    /// PersistentVolumeClaimVolumeSource represents a reference to a PersistentVolumeClaim in the same namespace. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistentvolumeclaims
    pub persistent_volume_claim: Option<crate::api::core::v1::PersistentVolumeClaimVolumeSource>,

    /// PhotonPersistentDisk represents a PhotonController persistent disk attached and mounted on kubelets host machine
    pub photon_persistent_disk: Option<crate::api::core::v1::PhotonPersistentDiskVolumeSource>,

    /// PortworxVolume represents a portworx volume attached and mounted on kubelets host machine
    pub portworx_volume: Option<crate::api::core::v1::PortworxVolumeSource>,

    /// Items for all in one resources secrets, configmaps, and downward API
    pub projected: Option<crate::api::core::v1::ProjectedVolumeSource>,

    /// Quobyte represents a Quobyte mount on the host that shares a pod's lifetime
    pub quobyte: Option<crate::api::core::v1::QuobyteVolumeSource>,

    /// RBD represents a Rados Block Device mount on the host that shares a pod's lifetime. More info: https://releases.k8s.io/HEAD/examples/volumes/rbd/README.md
    pub rbd: Option<crate::api::core::v1::RBDVolumeSource>,

    /// ScaleIO represents a ScaleIO persistent volume attached and mounted on Kubernetes nodes.
    pub scale_io: Option<crate::api::core::v1::ScaleIOVolumeSource>,

    /// Secret represents a secret that should populate this volume. More info: https://kubernetes.io/docs/concepts/storage/volumes#secret
    pub secret: Option<crate::api::core::v1::SecretVolumeSource>,

    /// StorageOS represents a StorageOS volume attached and mounted on Kubernetes nodes.
    pub storageos: Option<crate::api::core::v1::StorageOSVolumeSource>,

    /// VsphereVolume represents a vSphere volume attached and mounted on kubelets host machine
    pub vsphere_volume: Option<crate::api::core::v1::VsphereVirtualDiskVolumeSource>,
}

impl<'de> crate::serde::Deserialize<'de> for Volume {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_aws_elastic_block_store,
            Key_azure_disk,
            Key_azure_file,
            Key_cephfs,
            Key_cinder,
            Key_config_map,
            Key_csi,
            Key_downward_api,
            Key_empty_dir,
            Key_fc,
            Key_flex_volume,
            Key_flocker,
            Key_gce_persistent_disk,
            Key_git_repo,
            Key_glusterfs,
            Key_host_path,
            Key_iscsi,
            Key_name,
            Key_nfs,
            Key_persistent_volume_claim,
            Key_photon_persistent_disk,
            Key_portworx_volume,
            Key_projected,
            Key_quobyte,
            Key_rbd,
            Key_scale_io,
            Key_secret,
            Key_storageos,
            Key_vsphere_volume,
            Other,
        }

        impl<'de> crate::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> crate::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: crate::serde::de::Error {
                        Ok(match v {
                            "awsElasticBlockStore" => Field::Key_aws_elastic_block_store,
                            "azureDisk" => Field::Key_azure_disk,
                            "azureFile" => Field::Key_azure_file,
                            "cephfs" => Field::Key_cephfs,
                            "cinder" => Field::Key_cinder,
                            "configMap" => Field::Key_config_map,
                            "csi" => Field::Key_csi,
                            "downwardAPI" => Field::Key_downward_api,
                            "emptyDir" => Field::Key_empty_dir,
                            "fc" => Field::Key_fc,
                            "flexVolume" => Field::Key_flex_volume,
                            "flocker" => Field::Key_flocker,
                            "gcePersistentDisk" => Field::Key_gce_persistent_disk,
                            "gitRepo" => Field::Key_git_repo,
                            "glusterfs" => Field::Key_glusterfs,
                            "hostPath" => Field::Key_host_path,
                            "iscsi" => Field::Key_iscsi,
                            "name" => Field::Key_name,
                            "nfs" => Field::Key_nfs,
                            "persistentVolumeClaim" => Field::Key_persistent_volume_claim,
                            "photonPersistentDisk" => Field::Key_photon_persistent_disk,
                            "portworxVolume" => Field::Key_portworx_volume,
                            "projected" => Field::Key_projected,
                            "quobyte" => Field::Key_quobyte,
                            "rbd" => Field::Key_rbd,
                            "scaleIO" => Field::Key_scale_io,
                            "secret" => Field::Key_secret,
                            "storageos" => Field::Key_storageos,
                            "vsphereVolume" => Field::Key_vsphere_volume,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = Volume;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("Volume")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_aws_elastic_block_store: Option<crate::api::core::v1::AWSElasticBlockStoreVolumeSource> = None;
                let mut value_azure_disk: Option<crate::api::core::v1::AzureDiskVolumeSource> = None;
                let mut value_azure_file: Option<crate::api::core::v1::AzureFileVolumeSource> = None;
                let mut value_cephfs: Option<crate::api::core::v1::CephFSVolumeSource> = None;
                let mut value_cinder: Option<crate::api::core::v1::CinderVolumeSource> = None;
                let mut value_config_map: Option<crate::api::core::v1::ConfigMapVolumeSource> = None;
                let mut value_csi: Option<crate::api::core::v1::CSIVolumeSource> = None;
                let mut value_downward_api: Option<crate::api::core::v1::DownwardAPIVolumeSource> = None;
                let mut value_empty_dir: Option<crate::api::core::v1::EmptyDirVolumeSource> = None;
                let mut value_fc: Option<crate::api::core::v1::FCVolumeSource> = None;
                let mut value_flex_volume: Option<crate::api::core::v1::FlexVolumeSource> = None;
                let mut value_flocker: Option<crate::api::core::v1::FlockerVolumeSource> = None;
                let mut value_gce_persistent_disk: Option<crate::api::core::v1::GCEPersistentDiskVolumeSource> = None;
                let mut value_git_repo: Option<crate::api::core::v1::GitRepoVolumeSource> = None;
                let mut value_glusterfs: Option<crate::api::core::v1::GlusterfsVolumeSource> = None;
                let mut value_host_path: Option<crate::api::core::v1::HostPathVolumeSource> = None;
                let mut value_iscsi: Option<crate::api::core::v1::ISCSIVolumeSource> = None;
                let mut value_name: Option<String> = None;
                let mut value_nfs: Option<crate::api::core::v1::NFSVolumeSource> = None;
                let mut value_persistent_volume_claim: Option<crate::api::core::v1::PersistentVolumeClaimVolumeSource> = None;
                let mut value_photon_persistent_disk: Option<crate::api::core::v1::PhotonPersistentDiskVolumeSource> = None;
                let mut value_portworx_volume: Option<crate::api::core::v1::PortworxVolumeSource> = None;
                let mut value_projected: Option<crate::api::core::v1::ProjectedVolumeSource> = None;
                let mut value_quobyte: Option<crate::api::core::v1::QuobyteVolumeSource> = None;
                let mut value_rbd: Option<crate::api::core::v1::RBDVolumeSource> = None;
                let mut value_scale_io: Option<crate::api::core::v1::ScaleIOVolumeSource> = None;
                let mut value_secret: Option<crate::api::core::v1::SecretVolumeSource> = None;
                let mut value_storageos: Option<crate::api::core::v1::StorageOSVolumeSource> = None;
                let mut value_vsphere_volume: Option<crate::api::core::v1::VsphereVirtualDiskVolumeSource> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_aws_elastic_block_store => value_aws_elastic_block_store = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_azure_disk => value_azure_disk = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_azure_file => value_azure_file = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_cephfs => value_cephfs = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_cinder => value_cinder = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_config_map => value_config_map = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_csi => value_csi = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_downward_api => value_downward_api = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_empty_dir => value_empty_dir = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_fc => value_fc = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_flex_volume => value_flex_volume = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_flocker => value_flocker = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_gce_persistent_disk => value_gce_persistent_disk = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_git_repo => value_git_repo = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_glusterfs => value_glusterfs = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_host_path => value_host_path = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_iscsi => value_iscsi = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_nfs => value_nfs = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_persistent_volume_claim => value_persistent_volume_claim = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_photon_persistent_disk => value_photon_persistent_disk = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_portworx_volume => value_portworx_volume = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_projected => value_projected = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_quobyte => value_quobyte = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_rbd => value_rbd = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_scale_io => value_scale_io = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_secret => value_secret = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_storageos => value_storageos = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_vsphere_volume => value_vsphere_volume = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Volume {
                    aws_elastic_block_store: value_aws_elastic_block_store,
                    azure_disk: value_azure_disk,
                    azure_file: value_azure_file,
                    cephfs: value_cephfs,
                    cinder: value_cinder,
                    config_map: value_config_map,
                    csi: value_csi,
                    downward_api: value_downward_api,
                    empty_dir: value_empty_dir,
                    fc: value_fc,
                    flex_volume: value_flex_volume,
                    flocker: value_flocker,
                    gce_persistent_disk: value_gce_persistent_disk,
                    git_repo: value_git_repo,
                    glusterfs: value_glusterfs,
                    host_path: value_host_path,
                    iscsi: value_iscsi,
                    name: value_name.ok_or_else(|| crate::serde::de::Error::missing_field("name"))?,
                    nfs: value_nfs,
                    persistent_volume_claim: value_persistent_volume_claim,
                    photon_persistent_disk: value_photon_persistent_disk,
                    portworx_volume: value_portworx_volume,
                    projected: value_projected,
                    quobyte: value_quobyte,
                    rbd: value_rbd,
                    scale_io: value_scale_io,
                    secret: value_secret,
                    storageos: value_storageos,
                    vsphere_volume: value_vsphere_volume,
                })
            }
        }

        deserializer.deserialize_struct(
            "Volume",
            &[
                "awsElasticBlockStore",
                "azureDisk",
                "azureFile",
                "cephfs",
                "cinder",
                "configMap",
                "csi",
                "downwardAPI",
                "emptyDir",
                "fc",
                "flexVolume",
                "flocker",
                "gcePersistentDisk",
                "gitRepo",
                "glusterfs",
                "hostPath",
                "iscsi",
                "name",
                "nfs",
                "persistentVolumeClaim",
                "photonPersistentDisk",
                "portworxVolume",
                "projected",
                "quobyte",
                "rbd",
                "scaleIO",
                "secret",
                "storageos",
                "vsphereVolume",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for Volume {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Volume",
            1 +
            self.aws_elastic_block_store.as_ref().map_or(0, |_| 1) +
            self.azure_disk.as_ref().map_or(0, |_| 1) +
            self.azure_file.as_ref().map_or(0, |_| 1) +
            self.cephfs.as_ref().map_or(0, |_| 1) +
            self.cinder.as_ref().map_or(0, |_| 1) +
            self.config_map.as_ref().map_or(0, |_| 1) +
            self.csi.as_ref().map_or(0, |_| 1) +
            self.downward_api.as_ref().map_or(0, |_| 1) +
            self.empty_dir.as_ref().map_or(0, |_| 1) +
            self.fc.as_ref().map_or(0, |_| 1) +
            self.flex_volume.as_ref().map_or(0, |_| 1) +
            self.flocker.as_ref().map_or(0, |_| 1) +
            self.gce_persistent_disk.as_ref().map_or(0, |_| 1) +
            self.git_repo.as_ref().map_or(0, |_| 1) +
            self.glusterfs.as_ref().map_or(0, |_| 1) +
            self.host_path.as_ref().map_or(0, |_| 1) +
            self.iscsi.as_ref().map_or(0, |_| 1) +
            self.nfs.as_ref().map_or(0, |_| 1) +
            self.persistent_volume_claim.as_ref().map_or(0, |_| 1) +
            self.photon_persistent_disk.as_ref().map_or(0, |_| 1) +
            self.portworx_volume.as_ref().map_or(0, |_| 1) +
            self.projected.as_ref().map_or(0, |_| 1) +
            self.quobyte.as_ref().map_or(0, |_| 1) +
            self.rbd.as_ref().map_or(0, |_| 1) +
            self.scale_io.as_ref().map_or(0, |_| 1) +
            self.secret.as_ref().map_or(0, |_| 1) +
            self.storageos.as_ref().map_or(0, |_| 1) +
            self.vsphere_volume.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.aws_elastic_block_store {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "awsElasticBlockStore", value)?;
        }
        if let Some(value) = &self.azure_disk {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "azureDisk", value)?;
        }
        if let Some(value) = &self.azure_file {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "azureFile", value)?;
        }
        if let Some(value) = &self.cephfs {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "cephfs", value)?;
        }
        if let Some(value) = &self.cinder {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "cinder", value)?;
        }
        if let Some(value) = &self.config_map {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "configMap", value)?;
        }
        if let Some(value) = &self.csi {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "csi", value)?;
        }
        if let Some(value) = &self.downward_api {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "downwardAPI", value)?;
        }
        if let Some(value) = &self.empty_dir {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "emptyDir", value)?;
        }
        if let Some(value) = &self.fc {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "fc", value)?;
        }
        if let Some(value) = &self.flex_volume {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "flexVolume", value)?;
        }
        if let Some(value) = &self.flocker {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "flocker", value)?;
        }
        if let Some(value) = &self.gce_persistent_disk {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "gcePersistentDisk", value)?;
        }
        if let Some(value) = &self.git_repo {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "gitRepo", value)?;
        }
        if let Some(value) = &self.glusterfs {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "glusterfs", value)?;
        }
        if let Some(value) = &self.host_path {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "hostPath", value)?;
        }
        if let Some(value) = &self.iscsi {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "iscsi", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        if let Some(value) = &self.nfs {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nfs", value)?;
        }
        if let Some(value) = &self.persistent_volume_claim {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "persistentVolumeClaim", value)?;
        }
        if let Some(value) = &self.photon_persistent_disk {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "photonPersistentDisk", value)?;
        }
        if let Some(value) = &self.portworx_volume {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "portworxVolume", value)?;
        }
        if let Some(value) = &self.projected {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "projected", value)?;
        }
        if let Some(value) = &self.quobyte {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "quobyte", value)?;
        }
        if let Some(value) = &self.rbd {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "rbd", value)?;
        }
        if let Some(value) = &self.scale_io {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "scaleIO", value)?;
        }
        if let Some(value) = &self.secret {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "secret", value)?;
        }
        if let Some(value) = &self.storageos {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "storageos", value)?;
        }
        if let Some(value) = &self.vsphere_volume {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "vsphereVolume", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl Volume {
    pub fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "Volume represents a named volume in a pod that may be accessed by any container in the pod.",
          "properties": {
            "awsElasticBlockStore": crate::schema_ref_with_description(crate::api::core::v1::AWSElasticBlockStoreVolumeSource::schema(), "AWSElasticBlockStore represents an AWS Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore"),
            "azureDisk": crate::schema_ref_with_description(crate::api::core::v1::AzureDiskVolumeSource::schema(), "AzureDisk represents an Azure Data Disk mount on the host and bind mount to the pod."),
            "azureFile": crate::schema_ref_with_description(crate::api::core::v1::AzureFileVolumeSource::schema(), "AzureFile represents an Azure File Service mount on the host and bind mount to the pod."),
            "cephfs": crate::schema_ref_with_description(crate::api::core::v1::CephFSVolumeSource::schema(), "CephFS represents a Ceph FS mount on the host that shares a pod's lifetime"),
            "cinder": crate::schema_ref_with_description(crate::api::core::v1::CinderVolumeSource::schema(), "Cinder represents a cinder volume attached and mounted on kubelets host machine More info: https://releases.k8s.io/HEAD/examples/mysql-cinder-pd/README.md"),
            "configMap": crate::schema_ref_with_description(crate::api::core::v1::ConfigMapVolumeSource::schema(), "ConfigMap represents a configMap that should populate this volume"),
            "csi": crate::schema_ref_with_description(crate::api::core::v1::CSIVolumeSource::schema(), "CSI (Container Storage Interface) represents storage that is handled by an external CSI driver (Alpha feature)."),
            "downwardAPI": crate::schema_ref_with_description(crate::api::core::v1::DownwardAPIVolumeSource::schema(), "DownwardAPI represents downward API about the pod that should populate this volume"),
            "emptyDir": crate::schema_ref_with_description(crate::api::core::v1::EmptyDirVolumeSource::schema(), "EmptyDir represents a temporary directory that shares a pod's lifetime. More info: https://kubernetes.io/docs/concepts/storage/volumes#emptydir"),
            "fc": crate::schema_ref_with_description(crate::api::core::v1::FCVolumeSource::schema(), "FC represents a Fibre Channel resource that is attached to a kubelet's host machine and then exposed to the pod."),
            "flexVolume": crate::schema_ref_with_description(crate::api::core::v1::FlexVolumeSource::schema(), "FlexVolume represents a generic volume resource that is provisioned/attached using an exec based plugin."),
            "flocker": crate::schema_ref_with_description(crate::api::core::v1::FlockerVolumeSource::schema(), "Flocker represents a Flocker volume attached to a kubelet's host machine. This depends on the Flocker control service being running"),
            "gcePersistentDisk": crate::schema_ref_with_description(crate::api::core::v1::GCEPersistentDiskVolumeSource::schema(), "GCEPersistentDisk represents a GCE Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk"),
            "gitRepo": crate::schema_ref_with_description(crate::api::core::v1::GitRepoVolumeSource::schema(), "GitRepo represents a git repository at a particular revision. DEPRECATED: GitRepo is deprecated. To provision a container with a git repo, mount an EmptyDir into an InitContainer that clones the repo using git, then mount the EmptyDir into the Pod's container."),
            "glusterfs": crate::schema_ref_with_description(crate::api::core::v1::GlusterfsVolumeSource::schema(), "Glusterfs represents a Glusterfs mount on the host that shares a pod's lifetime. More info: https://releases.k8s.io/HEAD/examples/volumes/glusterfs/README.md"),
            "hostPath": crate::schema_ref_with_description(crate::api::core::v1::HostPathVolumeSource::schema(), "HostPath represents a pre-existing file or directory on the host machine that is directly exposed to the container. This is generally used for system agents or other privileged things that are allowed to see the host machine. Most containers will NOT need this. More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath"),
            "iscsi": crate::schema_ref_with_description(crate::api::core::v1::ISCSIVolumeSource::schema(), "ISCSI represents an ISCSI Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://releases.k8s.io/HEAD/examples/volumes/iscsi/README.md"),
            "name": {
              "description": "Volume's name. Must be a DNS_LABEL and unique within the pod. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names",
              "type": "string"
            },
            "nfs": crate::schema_ref_with_description(crate::api::core::v1::NFSVolumeSource::schema(), "NFS represents an NFS mount on the host that shares a pod's lifetime More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs"),
            "persistentVolumeClaim": crate::schema_ref_with_description(crate::api::core::v1::PersistentVolumeClaimVolumeSource::schema(), "PersistentVolumeClaimVolumeSource represents a reference to a PersistentVolumeClaim in the same namespace. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistentvolumeclaims"),
            "photonPersistentDisk": crate::schema_ref_with_description(crate::api::core::v1::PhotonPersistentDiskVolumeSource::schema(), "PhotonPersistentDisk represents a PhotonController persistent disk attached and mounted on kubelets host machine"),
            "portworxVolume": crate::schema_ref_with_description(crate::api::core::v1::PortworxVolumeSource::schema(), "PortworxVolume represents a portworx volume attached and mounted on kubelets host machine"),
            "projected": crate::schema_ref_with_description(crate::api::core::v1::ProjectedVolumeSource::schema(), "Items for all in one resources secrets, configmaps, and downward API"),
            "quobyte": crate::schema_ref_with_description(crate::api::core::v1::QuobyteVolumeSource::schema(), "Quobyte represents a Quobyte mount on the host that shares a pod's lifetime"),
            "rbd": crate::schema_ref_with_description(crate::api::core::v1::RBDVolumeSource::schema(), "RBD represents a Rados Block Device mount on the host that shares a pod's lifetime. More info: https://releases.k8s.io/HEAD/examples/volumes/rbd/README.md"),
            "scaleIO": crate::schema_ref_with_description(crate::api::core::v1::ScaleIOVolumeSource::schema(), "ScaleIO represents a ScaleIO persistent volume attached and mounted on Kubernetes nodes."),
            "secret": crate::schema_ref_with_description(crate::api::core::v1::SecretVolumeSource::schema(), "Secret represents a secret that should populate this volume. More info: https://kubernetes.io/docs/concepts/storage/volumes#secret"),
            "storageos": crate::schema_ref_with_description(crate::api::core::v1::StorageOSVolumeSource::schema(), "StorageOS represents a StorageOS volume attached and mounted on Kubernetes nodes."),
            "vsphereVolume": crate::schema_ref_with_description(crate::api::core::v1::VsphereVirtualDiskVolumeSource::schema(), "VsphereVolume represents a vSphere volume attached and mounted on kubelets host machine")
          },
          "required": [
            "name"
          ],
          "type": "object"
        })
    }
}
