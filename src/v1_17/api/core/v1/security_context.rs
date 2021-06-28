// Generated from definition io.k8s.api.core.v1.SecurityContext

/// SecurityContext holds security configuration that will be applied to a container. Some fields are present in both SecurityContext and PodSecurityContext.  When both are set, the values in SecurityContext take precedence.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SecurityContext {
    /// AllowPrivilegeEscalation controls whether a process can gain more privileges than its parent process. This bool directly controls if the no_new_privs flag will be set on the container process. AllowPrivilegeEscalation is true always when the container is: 1) run as Privileged 2) has CAP_SYS_ADMIN
    pub allow_privilege_escalation: Option<bool>,

    /// The capabilities to add/drop when running containers. Defaults to the default set of capabilities granted by the container runtime.
    pub capabilities: Option<crate::api::core::v1::Capabilities>,

    /// Run container in privileged mode. Processes in privileged containers are essentially equivalent to root on the host. Defaults to false.
    pub privileged: Option<bool>,

    /// procMount denotes the type of proc mount to use for the containers. The default is DefaultProcMount which uses the container runtime defaults for readonly paths and masked paths. This requires the ProcMountType feature flag to be enabled.
    pub proc_mount: Option<String>,

    /// Whether this container has a read-only root filesystem. Default is false.
    pub read_only_root_filesystem: Option<bool>,

    /// The GID to run the entrypoint of the container process. Uses runtime default if unset. May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
    pub run_as_group: Option<i64>,

    /// Indicates that the container must run as a non-root user. If true, the Kubelet will validate the image at runtime to ensure that it does not run as UID 0 (root) and fail to start the container if it does. If unset or false, no such validation will be performed. May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
    pub run_as_non_root: Option<bool>,

    /// The UID to run the entrypoint of the container process. Defaults to user specified in image metadata if unspecified. May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
    pub run_as_user: Option<i64>,

    /// The SELinux context to be applied to the container. If unspecified, the container runtime will allocate a random SELinux context for each container.  May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
    pub se_linux_options: Option<crate::api::core::v1::SELinuxOptions>,

    /// The Windows specific settings applied to all containers. If unspecified, the options from the PodSecurityContext will be used. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
    pub windows_options: Option<crate::api::core::v1::WindowsSecurityContextOptions>,
}

impl<'de> crate::serde::Deserialize<'de> for SecurityContext {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_allow_privilege_escalation,
            Key_capabilities,
            Key_privileged,
            Key_proc_mount,
            Key_read_only_root_filesystem,
            Key_run_as_group,
            Key_run_as_non_root,
            Key_run_as_user,
            Key_se_linux_options,
            Key_windows_options,
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
                            "allowPrivilegeEscalation" => Field::Key_allow_privilege_escalation,
                            "capabilities" => Field::Key_capabilities,
                            "privileged" => Field::Key_privileged,
                            "procMount" => Field::Key_proc_mount,
                            "readOnlyRootFilesystem" => Field::Key_read_only_root_filesystem,
                            "runAsGroup" => Field::Key_run_as_group,
                            "runAsNonRoot" => Field::Key_run_as_non_root,
                            "runAsUser" => Field::Key_run_as_user,
                            "seLinuxOptions" => Field::Key_se_linux_options,
                            "windowsOptions" => Field::Key_windows_options,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = SecurityContext;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("SecurityContext")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_allow_privilege_escalation: Option<bool> = None;
                let mut value_capabilities: Option<crate::api::core::v1::Capabilities> = None;
                let mut value_privileged: Option<bool> = None;
                let mut value_proc_mount: Option<String> = None;
                let mut value_read_only_root_filesystem: Option<bool> = None;
                let mut value_run_as_group: Option<i64> = None;
                let mut value_run_as_non_root: Option<bool> = None;
                let mut value_run_as_user: Option<i64> = None;
                let mut value_se_linux_options: Option<crate::api::core::v1::SELinuxOptions> = None;
                let mut value_windows_options: Option<crate::api::core::v1::WindowsSecurityContextOptions> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_allow_privilege_escalation => value_allow_privilege_escalation = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_capabilities => value_capabilities = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_privileged => value_privileged = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_proc_mount => value_proc_mount = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_read_only_root_filesystem => value_read_only_root_filesystem = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_run_as_group => value_run_as_group = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_run_as_non_root => value_run_as_non_root = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_run_as_user => value_run_as_user = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_se_linux_options => value_se_linux_options = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_windows_options => value_windows_options = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(SecurityContext {
                    allow_privilege_escalation: value_allow_privilege_escalation,
                    capabilities: value_capabilities,
                    privileged: value_privileged,
                    proc_mount: value_proc_mount,
                    read_only_root_filesystem: value_read_only_root_filesystem,
                    run_as_group: value_run_as_group,
                    run_as_non_root: value_run_as_non_root,
                    run_as_user: value_run_as_user,
                    se_linux_options: value_se_linux_options,
                    windows_options: value_windows_options,
                })
            }
        }

        deserializer.deserialize_struct(
            "SecurityContext",
            &[
                "allowPrivilegeEscalation",
                "capabilities",
                "privileged",
                "procMount",
                "readOnlyRootFilesystem",
                "runAsGroup",
                "runAsNonRoot",
                "runAsUser",
                "seLinuxOptions",
                "windowsOptions",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for SecurityContext {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "SecurityContext",
            self.allow_privilege_escalation.as_ref().map_or(0, |_| 1) +
            self.capabilities.as_ref().map_or(0, |_| 1) +
            self.privileged.as_ref().map_or(0, |_| 1) +
            self.proc_mount.as_ref().map_or(0, |_| 1) +
            self.read_only_root_filesystem.as_ref().map_or(0, |_| 1) +
            self.run_as_group.as_ref().map_or(0, |_| 1) +
            self.run_as_non_root.as_ref().map_or(0, |_| 1) +
            self.run_as_user.as_ref().map_or(0, |_| 1) +
            self.se_linux_options.as_ref().map_or(0, |_| 1) +
            self.windows_options.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.allow_privilege_escalation {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "allowPrivilegeEscalation", value)?;
        }
        if let Some(value) = &self.capabilities {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "capabilities", value)?;
        }
        if let Some(value) = &self.privileged {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "privileged", value)?;
        }
        if let Some(value) = &self.proc_mount {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "procMount", value)?;
        }
        if let Some(value) = &self.read_only_root_filesystem {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "readOnlyRootFilesystem", value)?;
        }
        if let Some(value) = &self.run_as_group {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "runAsGroup", value)?;
        }
        if let Some(value) = &self.run_as_non_root {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "runAsNonRoot", value)?;
        }
        if let Some(value) = &self.run_as_user {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "runAsUser", value)?;
        }
        if let Some(value) = &self.se_linux_options {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "seLinuxOptions", value)?;
        }
        if let Some(value) = &self.windows_options {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "windowsOptions", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for SecurityContext {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "SecurityContext holds security configuration that will be applied to a container. Some fields are present in both SecurityContext and PodSecurityContext.  When both are set, the values in SecurityContext take precedence.",
          "properties": {
            "allowPrivilegeEscalation": {
              "description": "AllowPrivilegeEscalation controls whether a process can gain more privileges than its parent process. This bool directly controls if the no_new_privs flag will be set on the container process. AllowPrivilegeEscalation is true always when the container is: 1) run as Privileged 2) has CAP_SYS_ADMIN",
              "type": "boolean"
            },
            "capabilities": crate::schema_ref_with_description(crate::api::core::v1::Capabilities::schema(), "The capabilities to add/drop when running containers. Defaults to the default set of capabilities granted by the container runtime."),
            "privileged": {
              "description": "Run container in privileged mode. Processes in privileged containers are essentially equivalent to root on the host. Defaults to false.",
              "type": "boolean"
            },
            "procMount": {
              "description": "procMount denotes the type of proc mount to use for the containers. The default is DefaultProcMount which uses the container runtime defaults for readonly paths and masked paths. This requires the ProcMountType feature flag to be enabled.",
              "type": "string"
            },
            "readOnlyRootFilesystem": {
              "description": "Whether this container has a read-only root filesystem. Default is false.",
              "type": "boolean"
            },
            "runAsGroup": {
              "description": "The GID to run the entrypoint of the container process. Uses runtime default if unset. May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.",
              "format": "int64",
              "type": "integer"
            },
            "runAsNonRoot": {
              "description": "Indicates that the container must run as a non-root user. If true, the Kubelet will validate the image at runtime to ensure that it does not run as UID 0 (root) and fail to start the container if it does. If unset or false, no such validation will be performed. May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.",
              "type": "boolean"
            },
            "runAsUser": {
              "description": "The UID to run the entrypoint of the container process. Defaults to user specified in image metadata if unspecified. May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.",
              "format": "int64",
              "type": "integer"
            },
            "seLinuxOptions": crate::schema_ref_with_description(crate::api::core::v1::SELinuxOptions::schema(), "The SELinux context to be applied to the container. If unspecified, the container runtime will allocate a random SELinux context for each container.  May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence."),
            "windowsOptions": crate::schema_ref_with_description(crate::api::core::v1::WindowsSecurityContextOptions::schema(), "The Windows specific settings applied to all containers. If unspecified, the options from the PodSecurityContext will be used. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.")
          },
          "type": "object"
        })
    }
}
