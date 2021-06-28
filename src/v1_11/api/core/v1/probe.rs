// Generated from definition io.k8s.api.core.v1.Probe

/// Probe describes a health check to be performed against a container to determine whether it is alive or ready to receive traffic.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Probe {
    /// One and only one of the following should be specified. Exec specifies the action to take.
    pub exec: Option<crate::api::core::v1::ExecAction>,

    /// Minimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1.
    pub failure_threshold: Option<i32>,

    /// HTTPGet specifies the http request to perform.
    pub http_get: Option<crate::api::core::v1::HTTPGetAction>,

    /// Number of seconds after the container has started before liveness probes are initiated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
    pub initial_delay_seconds: Option<i32>,

    /// How often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1.
    pub period_seconds: Option<i32>,

    /// Minimum consecutive successes for the probe to be considered successful after having failed. Defaults to 1. Must be 1 for liveness. Minimum value is 1.
    pub success_threshold: Option<i32>,

    /// TCPSocket specifies an action involving a TCP port. TCP hooks not yet supported
    pub tcp_socket: Option<crate::api::core::v1::TCPSocketAction>,

    /// Number of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
    pub timeout_seconds: Option<i32>,
}

impl<'de> crate::serde::Deserialize<'de> for Probe {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_exec,
            Key_failure_threshold,
            Key_http_get,
            Key_initial_delay_seconds,
            Key_period_seconds,
            Key_success_threshold,
            Key_tcp_socket,
            Key_timeout_seconds,
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
                            "exec" => Field::Key_exec,
                            "failureThreshold" => Field::Key_failure_threshold,
                            "httpGet" => Field::Key_http_get,
                            "initialDelaySeconds" => Field::Key_initial_delay_seconds,
                            "periodSeconds" => Field::Key_period_seconds,
                            "successThreshold" => Field::Key_success_threshold,
                            "tcpSocket" => Field::Key_tcp_socket,
                            "timeoutSeconds" => Field::Key_timeout_seconds,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = Probe;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("Probe")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_exec: Option<crate::api::core::v1::ExecAction> = None;
                let mut value_failure_threshold: Option<i32> = None;
                let mut value_http_get: Option<crate::api::core::v1::HTTPGetAction> = None;
                let mut value_initial_delay_seconds: Option<i32> = None;
                let mut value_period_seconds: Option<i32> = None;
                let mut value_success_threshold: Option<i32> = None;
                let mut value_tcp_socket: Option<crate::api::core::v1::TCPSocketAction> = None;
                let mut value_timeout_seconds: Option<i32> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_exec => value_exec = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_failure_threshold => value_failure_threshold = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_http_get => value_http_get = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_initial_delay_seconds => value_initial_delay_seconds = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_period_seconds => value_period_seconds = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_success_threshold => value_success_threshold = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_tcp_socket => value_tcp_socket = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_timeout_seconds => value_timeout_seconds = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Probe {
                    exec: value_exec,
                    failure_threshold: value_failure_threshold,
                    http_get: value_http_get,
                    initial_delay_seconds: value_initial_delay_seconds,
                    period_seconds: value_period_seconds,
                    success_threshold: value_success_threshold,
                    tcp_socket: value_tcp_socket,
                    timeout_seconds: value_timeout_seconds,
                })
            }
        }

        deserializer.deserialize_struct(
            "Probe",
            &[
                "exec",
                "failureThreshold",
                "httpGet",
                "initialDelaySeconds",
                "periodSeconds",
                "successThreshold",
                "tcpSocket",
                "timeoutSeconds",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for Probe {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Probe",
            self.exec.as_ref().map_or(0, |_| 1) +
            self.failure_threshold.as_ref().map_or(0, |_| 1) +
            self.http_get.as_ref().map_or(0, |_| 1) +
            self.initial_delay_seconds.as_ref().map_or(0, |_| 1) +
            self.period_seconds.as_ref().map_or(0, |_| 1) +
            self.success_threshold.as_ref().map_or(0, |_| 1) +
            self.tcp_socket.as_ref().map_or(0, |_| 1) +
            self.timeout_seconds.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.exec {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "exec", value)?;
        }
        if let Some(value) = &self.failure_threshold {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "failureThreshold", value)?;
        }
        if let Some(value) = &self.http_get {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "httpGet", value)?;
        }
        if let Some(value) = &self.initial_delay_seconds {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "initialDelaySeconds", value)?;
        }
        if let Some(value) = &self.period_seconds {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "periodSeconds", value)?;
        }
        if let Some(value) = &self.success_threshold {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "successThreshold", value)?;
        }
        if let Some(value) = &self.tcp_socket {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "tcpSocket", value)?;
        }
        if let Some(value) = &self.timeout_seconds {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "timeoutSeconds", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for Probe {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "Probe describes a health check to be performed against a container to determine whether it is alive or ready to receive traffic.",
          "properties": {
            "exec": crate::schema_ref_with_description(crate::api::core::v1::ExecAction::schema(), "One and only one of the following should be specified. Exec specifies the action to take."),
            "failureThreshold": {
              "description": "Minimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1.",
              "format": "int32",
              "type": "integer"
            },
            "httpGet": crate::schema_ref_with_description(crate::api::core::v1::HTTPGetAction::schema(), "HTTPGet specifies the http request to perform."),
            "initialDelaySeconds": {
              "description": "Number of seconds after the container has started before liveness probes are initiated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes",
              "format": "int32",
              "type": "integer"
            },
            "periodSeconds": {
              "description": "How often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1.",
              "format": "int32",
              "type": "integer"
            },
            "successThreshold": {
              "description": "Minimum consecutive successes for the probe to be considered successful after having failed. Defaults to 1. Must be 1 for liveness. Minimum value is 1.",
              "format": "int32",
              "type": "integer"
            },
            "tcpSocket": crate::schema_ref_with_description(crate::api::core::v1::TCPSocketAction::schema(), "TCPSocket specifies an action involving a TCP port. TCP hooks not yet supported"),
            "timeoutSeconds": {
              "description": "Number of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes",
              "format": "int32",
              "type": "integer"
            }
          },
          "type": "object"
        })
    }
}
