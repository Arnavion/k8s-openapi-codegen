// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.CustomResourceConversion

/// CustomResourceConversion describes how to convert different versions of a CR.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CustomResourceConversion {
    /// conversionReviewVersions is an ordered list of preferred `ConversionReview` versions the Webhook expects. The API server will use the first version in the list which it supports. If none of the versions specified in this list are supported by API server, conversion will fail for the custom resource. If a persisted Webhook configuration specifies allowed versions and does not include any versions known to the API Server, calls to the webhook will fail. Defaults to `\["v1beta1"\]`.
    pub conversion_review_versions: Vec<String>,

    /// strategy specifies how custom resources are converted between versions. Allowed values are: - `None`: The converter only change the apiVersion and would not touch any other field in the custom resource. - `Webhook`: API Server will call to an external webhook to do the conversion. Additional information
    ///   is needed for this option. This requires spec.preserveUnknownFields to be false, and spec.conversion.webhookClientConfig to be set.
    pub strategy: String,

    /// webhookClientConfig is the instructions for how to call the webhook if strategy is `Webhook`. Required when `strategy` is set to `Webhook`.
    pub webhook_client_config: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::WebhookClientConfig>,
}

impl<'de> crate::serde::Deserialize<'de> for CustomResourceConversion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_conversion_review_versions,
            Key_strategy,
            Key_webhook_client_config,
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
                            "conversionReviewVersions" => Field::Key_conversion_review_versions,
                            "strategy" => Field::Key_strategy,
                            "webhookClientConfig" => Field::Key_webhook_client_config,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = CustomResourceConversion;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("CustomResourceConversion")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_conversion_review_versions: Option<Vec<String>> = None;
                let mut value_strategy: Option<String> = None;
                let mut value_webhook_client_config: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::WebhookClientConfig> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_conversion_review_versions => value_conversion_review_versions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_strategy => value_strategy = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_webhook_client_config => value_webhook_client_config = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CustomResourceConversion {
                    conversion_review_versions: value_conversion_review_versions.unwrap_or_default(),
                    strategy: value_strategy.ok_or_else(|| crate::serde::de::Error::missing_field("strategy"))?,
                    webhook_client_config: value_webhook_client_config,
                })
            }
        }

        deserializer.deserialize_struct(
            "CustomResourceConversion",
            &[
                "conversionReviewVersions",
                "strategy",
                "webhookClientConfig",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for CustomResourceConversion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CustomResourceConversion",
            1 +
            usize::from(!self.conversion_review_versions.is_empty()) +
            self.webhook_client_config.as_ref().map_or(0, |_| 1),
        )?;
        if !self.conversion_review_versions.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "conversionReviewVersions", &self.conversion_review_versions)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "strategy", &self.strategy)?;
        if let Some(value) = &self.webhook_client_config {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "webhookClientConfig", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for CustomResourceConversion {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "CustomResourceConversion describes how to convert different versions of a CR.",
          "properties": {
            "conversionReviewVersions": {
              "description": "conversionReviewVersions is an ordered list of preferred `ConversionReview` versions the Webhook expects. The API server will use the first version in the list which it supports. If none of the versions specified in this list are supported by API server, conversion will fail for the custom resource. If a persisted Webhook configuration specifies allowed versions and does not include any versions known to the API Server, calls to the webhook will fail. Defaults to `[\"v1beta1\"]`.",
              "items": {
                "type": "string"
              },
              "type": "array"
            },
            "strategy": {
              "description": "strategy specifies how custom resources are converted between versions. Allowed values are: - `None`: The converter only change the apiVersion and would not touch any other field in the custom resource. - `Webhook`: API Server will call to an external webhook to do the conversion. Additional information\n  is needed for this option. This requires spec.preserveUnknownFields to be false, and spec.conversion.webhookClientConfig to be set.",
              "type": "string"
            },
            "webhookClientConfig": crate::schema_ref_with_description(crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::WebhookClientConfig::schema(), "webhookClientConfig is the instructions for how to call the webhook if strategy is `Webhook`. Required when `strategy` is set to `Webhook`.")
          },
          "required": [
            "strategy"
          ],
          "type": "object"
        })
    }
}
