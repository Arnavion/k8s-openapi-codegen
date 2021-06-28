// Generated from definition io.k8s.api.storage.v1.CSIDriverSpec

/// CSIDriverSpec is the specification of a CSIDriver.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CSIDriverSpec {
    /// attachRequired indicates this CSI volume driver requires an attach operation (because it implements the CSI ControllerPublishVolume() method), and that the Kubernetes attach detach controller should call the attach volume interface which checks the volumeattachment status and waits until the volume is attached before proceeding to mounting. The CSI external-attacher coordinates with CSI volume driver and updates the volumeattachment status when the attach operation is complete. If the CSIDriverRegistry feature gate is enabled and the value is specified to false, the attach operation will be skipped. Otherwise the attach operation will be called.
    pub attach_required: Option<bool>,

    /// Defines if the underlying volume supports changing ownership and permission of the volume before being mounted. Refer to the specific FSGroupPolicy values for additional details. This field is alpha-level, and is only honored by servers that enable the CSIVolumeFSGroupPolicy feature gate.
    pub fs_group_policy: Option<String>,

    /// If set to true, podInfoOnMount indicates this CSI volume driver requires additional pod information (like podName, podUID, etc.) during mount operations. If set to false, pod information will not be passed on mount. Default is false. The CSI driver specifies podInfoOnMount as part of driver deployment. If true, Kubelet will pass pod information as VolumeContext in the CSI NodePublishVolume() calls. The CSI driver is responsible for parsing and validating the information passed in as VolumeContext. The following VolumeConext will be passed if podInfoOnMount is set to true. This list might grow, but the prefix will be used. "csi.storage.k8s.io/pod.name": pod.Name "csi.storage.k8s.io/pod.namespace": pod.Namespace "csi.storage.k8s.io/pod.uid": string(pod.UID) "csi.storage.k8s.io/ephemeral": "true" iff the volume is an ephemeral inline volume
    ///                                 defined by a CSIVolumeSource, otherwise "false"
    ///
    /// "csi.storage.k8s.io/ephemeral" is a new feature in Kubernetes 1.16. It is only required for drivers which support both the "Persistent" and "Ephemeral" VolumeLifecycleMode. Other drivers can leave pod info disabled and/or ignore this field. As Kubernetes 1.15 doesn't support this field, drivers can only support one mode when deployed on such a cluster and the deployment determines which mode that is, for example via a command line parameter of the driver.
    pub pod_info_on_mount: Option<bool>,

    /// RequiresRepublish indicates the CSI driver wants `NodePublishVolume` being periodically called to reflect any possible change in the mounted volume. This field defaults to false.
    ///
    /// Note: After a successful initial NodePublishVolume call, subsequent calls to NodePublishVolume should only update the contents of the volume. New mount points will not be seen by a running container.
    ///
    /// This is an alpha feature and only available when the CSIServiceAccountToken feature is enabled.
    pub requires_republish: Option<bool>,

    /// If set to true, storageCapacity indicates that the CSI volume driver wants pod scheduling to consider the storage capacity that the driver deployment will report by creating CSIStorageCapacity objects with capacity information.
    ///
    /// The check can be enabled immediately when deploying a driver. In that case, provisioning new volumes with late binding will pause until the driver deployment has published some suitable CSIStorageCapacity object.
    ///
    /// Alternatively, the driver can be deployed with the field unset or false and it can be flipped later when storage capacity information has been published.
    ///
    /// This is an alpha field and only available when the CSIStorageCapacity feature is enabled. The default is false.
    pub storage_capacity: Option<bool>,

    /// TokenRequests indicates the CSI driver needs pods' service account tokens it is mounting volume for to do necessary authentication. Kubelet will pass the tokens in VolumeContext in the CSI NodePublishVolume calls. The CSI driver should parse and validate the following VolumeContext: "csi.storage.k8s.io/serviceAccount.tokens": {
    ///   "\<audience\>": {
    ///     "token": \<token\>,
    ///     "expirationTimestamp": \<expiration timestamp in RFC3339\>,
    ///   },
    ///   ...
    /// }
    ///
    /// Note: Audience in each TokenRequest should be different and at most one token is empty string. To receive a new token after expiry, RequiresRepublish can be used to trigger NodePublishVolume periodically.
    ///
    /// This is an alpha feature and only available when the CSIServiceAccountToken feature is enabled.
    pub token_requests: Vec<crate::api::storage::v1::TokenRequest>,

    /// volumeLifecycleModes defines what kind of volumes this CSI volume driver supports. The default if the list is empty is "Persistent", which is the usage defined by the CSI specification and implemented in Kubernetes via the usual PV/PVC mechanism. The other mode is "Ephemeral". In this mode, volumes are defined inline inside the pod spec with CSIVolumeSource and their lifecycle is tied to the lifecycle of that pod. A driver has to be aware of this because it is only going to get a NodePublishVolume call for such a volume. For more information about implementing this mode, see https://kubernetes-csi.github.io/docs/ephemeral-local-volumes.html A driver can support one or more of these modes and more modes may be added in the future. This field is beta.
    pub volume_lifecycle_modes: Vec<String>,
}

impl<'de> crate::serde::Deserialize<'de> for CSIDriverSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_attach_required,
            Key_fs_group_policy,
            Key_pod_info_on_mount,
            Key_requires_republish,
            Key_storage_capacity,
            Key_token_requests,
            Key_volume_lifecycle_modes,
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
                            "attachRequired" => Field::Key_attach_required,
                            "fsGroupPolicy" => Field::Key_fs_group_policy,
                            "podInfoOnMount" => Field::Key_pod_info_on_mount,
                            "requiresRepublish" => Field::Key_requires_republish,
                            "storageCapacity" => Field::Key_storage_capacity,
                            "tokenRequests" => Field::Key_token_requests,
                            "volumeLifecycleModes" => Field::Key_volume_lifecycle_modes,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = CSIDriverSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("CSIDriverSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_attach_required: Option<bool> = None;
                let mut value_fs_group_policy: Option<String> = None;
                let mut value_pod_info_on_mount: Option<bool> = None;
                let mut value_requires_republish: Option<bool> = None;
                let mut value_storage_capacity: Option<bool> = None;
                let mut value_token_requests: Option<Vec<crate::api::storage::v1::TokenRequest>> = None;
                let mut value_volume_lifecycle_modes: Option<Vec<String>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_attach_required => value_attach_required = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_fs_group_policy => value_fs_group_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pod_info_on_mount => value_pod_info_on_mount = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_requires_republish => value_requires_republish = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_storage_capacity => value_storage_capacity = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_token_requests => value_token_requests = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volume_lifecycle_modes => value_volume_lifecycle_modes = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CSIDriverSpec {
                    attach_required: value_attach_required,
                    fs_group_policy: value_fs_group_policy,
                    pod_info_on_mount: value_pod_info_on_mount,
                    requires_republish: value_requires_republish,
                    storage_capacity: value_storage_capacity,
                    token_requests: value_token_requests.unwrap_or_default(),
                    volume_lifecycle_modes: value_volume_lifecycle_modes.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "CSIDriverSpec",
            &[
                "attachRequired",
                "fsGroupPolicy",
                "podInfoOnMount",
                "requiresRepublish",
                "storageCapacity",
                "tokenRequests",
                "volumeLifecycleModes",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for CSIDriverSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CSIDriverSpec",
            self.attach_required.as_ref().map_or(0, |_| 1) +
            self.fs_group_policy.as_ref().map_or(0, |_| 1) +
            self.pod_info_on_mount.as_ref().map_or(0, |_| 1) +
            self.requires_republish.as_ref().map_or(0, |_| 1) +
            self.storage_capacity.as_ref().map_or(0, |_| 1) +
            usize::from(!self.token_requests.is_empty()) +
            usize::from(!self.volume_lifecycle_modes.is_empty()),
        )?;
        if let Some(value) = &self.attach_required {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "attachRequired", value)?;
        }
        if let Some(value) = &self.fs_group_policy {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "fsGroupPolicy", value)?;
        }
        if let Some(value) = &self.pod_info_on_mount {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "podInfoOnMount", value)?;
        }
        if let Some(value) = &self.requires_republish {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "requiresRepublish", value)?;
        }
        if let Some(value) = &self.storage_capacity {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "storageCapacity", value)?;
        }
        if !self.token_requests.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "tokenRequests", &self.token_requests)?;
        }
        if !self.volume_lifecycle_modes.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "volumeLifecycleModes", &self.volume_lifecycle_modes)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for CSIDriverSpec {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "CSIDriverSpec is the specification of a CSIDriver.",
          "properties": {
            "attachRequired": {
              "description": "attachRequired indicates this CSI volume driver requires an attach operation (because it implements the CSI ControllerPublishVolume() method), and that the Kubernetes attach detach controller should call the attach volume interface which checks the volumeattachment status and waits until the volume is attached before proceeding to mounting. The CSI external-attacher coordinates with CSI volume driver and updates the volumeattachment status when the attach operation is complete. If the CSIDriverRegistry feature gate is enabled and the value is specified to false, the attach operation will be skipped. Otherwise the attach operation will be called.",
              "type": "boolean"
            },
            "fsGroupPolicy": {
              "description": "Defines if the underlying volume supports changing ownership and permission of the volume before being mounted. Refer to the specific FSGroupPolicy values for additional details. This field is alpha-level, and is only honored by servers that enable the CSIVolumeFSGroupPolicy feature gate.",
              "type": "string"
            },
            "podInfoOnMount": {
              "description": "If set to true, podInfoOnMount indicates this CSI volume driver requires additional pod information (like podName, podUID, etc.) during mount operations. If set to false, pod information will not be passed on mount. Default is false. The CSI driver specifies podInfoOnMount as part of driver deployment. If true, Kubelet will pass pod information as VolumeContext in the CSI NodePublishVolume() calls. The CSI driver is responsible for parsing and validating the information passed in as VolumeContext. The following VolumeConext will be passed if podInfoOnMount is set to true. This list might grow, but the prefix will be used. \"csi.storage.k8s.io/pod.name\": pod.Name \"csi.storage.k8s.io/pod.namespace\": pod.Namespace \"csi.storage.k8s.io/pod.uid\": string(pod.UID) \"csi.storage.k8s.io/ephemeral\": \"true\" iff the volume is an ephemeral inline volume\n                                defined by a CSIVolumeSource, otherwise \"false\"\n\n\"csi.storage.k8s.io/ephemeral\" is a new feature in Kubernetes 1.16. It is only required for drivers which support both the \"Persistent\" and \"Ephemeral\" VolumeLifecycleMode. Other drivers can leave pod info disabled and/or ignore this field. As Kubernetes 1.15 doesn't support this field, drivers can only support one mode when deployed on such a cluster and the deployment determines which mode that is, for example via a command line parameter of the driver.",
              "type": "boolean"
            },
            "requiresRepublish": {
              "description": "RequiresRepublish indicates the CSI driver wants `NodePublishVolume` being periodically called to reflect any possible change in the mounted volume. This field defaults to false.\n\nNote: After a successful initial NodePublishVolume call, subsequent calls to NodePublishVolume should only update the contents of the volume. New mount points will not be seen by a running container.\n\nThis is an alpha feature and only available when the CSIServiceAccountToken feature is enabled.",
              "type": "boolean"
            },
            "storageCapacity": {
              "description": "If set to true, storageCapacity indicates that the CSI volume driver wants pod scheduling to consider the storage capacity that the driver deployment will report by creating CSIStorageCapacity objects with capacity information.\n\nThe check can be enabled immediately when deploying a driver. In that case, provisioning new volumes with late binding will pause until the driver deployment has published some suitable CSIStorageCapacity object.\n\nAlternatively, the driver can be deployed with the field unset or false and it can be flipped later when storage capacity information has been published.\n\nThis is an alpha field and only available when the CSIStorageCapacity feature is enabled. The default is false.",
              "type": "boolean"
            },
            "tokenRequests": {
              "description": "TokenRequests indicates the CSI driver needs pods' service account tokens it is mounting volume for to do necessary authentication. Kubelet will pass the tokens in VolumeContext in the CSI NodePublishVolume calls. The CSI driver should parse and validate the following VolumeContext: \"csi.storage.k8s.io/serviceAccount.tokens\": {\n  \"<audience>\": {\n    \"token\": <token>,\n    \"expirationTimestamp\": <expiration timestamp in RFC3339>,\n  },\n  ...\n}\n\nNote: Audience in each TokenRequest should be different and at most one token is empty string. To receive a new token after expiry, RequiresRepublish can be used to trigger NodePublishVolume periodically.\n\nThis is an alpha feature and only available when the CSIServiceAccountToken feature is enabled.",
              "items": crate::api::storage::v1::TokenRequest::schema(),
              "type": "array"
            },
            "volumeLifecycleModes": {
              "description": "volumeLifecycleModes defines what kind of volumes this CSI volume driver supports. The default if the list is empty is \"Persistent\", which is the usage defined by the CSI specification and implemented in Kubernetes via the usual PV/PVC mechanism. The other mode is \"Ephemeral\". In this mode, volumes are defined inline inside the pod spec with CSIVolumeSource and their lifecycle is tied to the lifecycle of that pod. A driver has to be aware of this because it is only going to get a NodePublishVolume call for such a volume. For more information about implementing this mode, see https://kubernetes-csi.github.io/docs/ephemeral-local-volumes.html A driver can support one or more of these modes and more modes may be added in the future. This field is beta.",
              "items": {
                "type": "string"
              },
              "type": "array"
            }
          },
          "type": "object"
        })
    }
}
