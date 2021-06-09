#![allow(unused_qualifications)]

use crate::models;
#[cfg(any(feature = "client", feature = "server"))]
use crate::header;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CloudAccount {
    #[serde(rename = "cloudAccountId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cloud_account_id: Option<usize>,

    #[serde(rename = "cloudAccountSecificConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cloud_account_secific_configuration: Option<serde_json::Value>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "project")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub project: Option<String>,

    #[serde(rename = "token")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub token: Option<String>,

}

impl CloudAccount {
    pub fn new() -> CloudAccount {
        CloudAccount {
            cloud_account_id: None,
            cloud_account_secific_configuration: None,
            description: None,
            name: None,
            project: None,
            token: None,
        }
    }
}

/// Converts the CloudAccount value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CloudAccount {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref cloud_account_id) = self.cloud_account_id {
            params.push("cloudAccountId".to_string());
            params.push(cloud_account_id.to_string());
        }

        // Skipping cloudAccountSecificConfiguration in query parameter serialization


        if let Some(ref description) = self.description {
            params.push("description".to_string());
            params.push(description.to_string());
        }


        if let Some(ref name) = self.name {
            params.push("name".to_string());
            params.push(name.to_string());
        }


        if let Some(ref project) = self.project {
            params.push("project".to_string());
            params.push(project.to_string());
        }


        if let Some(ref token) = self.token {
            params.push("token".to_string());
            params.push(token.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CloudAccount value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CloudAccount {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub cloud_account_id: Vec<usize>,
            pub cloud_account_secific_configuration: Vec<serde_json::Value>,
            pub description: Vec<String>,
            pub name: Vec<String>,
            pub project: Vec<String>,
            pub token: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing CloudAccount".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "cloudAccountId" => intermediate_rep.cloud_account_id.push(<usize as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "cloudAccountSecificConfiguration" => intermediate_rep.cloud_account_secific_configuration.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "description" => intermediate_rep.description.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "project" => intermediate_rep.project.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "token" => intermediate_rep.token.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing CloudAccount".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CloudAccount {
            cloud_account_id: intermediate_rep.cloud_account_id.into_iter().next(),
            cloud_account_secific_configuration: intermediate_rep.cloud_account_secific_configuration.into_iter().next(),
            description: intermediate_rep.description.into_iter().next(),
            name: intermediate_rep.name.into_iter().next(),
            project: intermediate_rep.project.into_iter().next(),
            token: intermediate_rep.token.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CloudAccount> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CloudAccount>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CloudAccount>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for CloudAccount - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<CloudAccount> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CloudAccount as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into CloudAccount - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CloudAccountAllOf {
    #[serde(rename = "cloudAccountId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cloud_account_id: Option<usize>,

}

impl CloudAccountAllOf {
    pub fn new() -> CloudAccountAllOf {
        CloudAccountAllOf {
            cloud_account_id: None,
        }
    }
}

/// Converts the CloudAccountAllOf value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CloudAccountAllOf {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref cloud_account_id) = self.cloud_account_id {
            params.push("cloudAccountId".to_string());
            params.push(cloud_account_id.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CloudAccountAllOf value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CloudAccountAllOf {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub cloud_account_id: Vec<usize>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing CloudAccountAllOf".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "cloudAccountId" => intermediate_rep.cloud_account_id.push(<usize as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing CloudAccountAllOf".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CloudAccountAllOf {
            cloud_account_id: intermediate_rep.cloud_account_id.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CloudAccountAllOf> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CloudAccountAllOf>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CloudAccountAllOf>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for CloudAccountAllOf - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<CloudAccountAllOf> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CloudAccountAllOf as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into CloudAccountAllOf - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CloudAccountContent {
    #[serde(rename = "cloudAccountSecificConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cloud_account_secific_configuration: Option<serde_json::Value>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "project")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub project: Option<String>,

    #[serde(rename = "token")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub token: Option<String>,

}

impl CloudAccountContent {
    pub fn new() -> CloudAccountContent {
        CloudAccountContent {
            cloud_account_secific_configuration: None,
            description: None,
            name: None,
            project: None,
            token: None,
        }
    }
}

/// Converts the CloudAccountContent value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CloudAccountContent {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping cloudAccountSecificConfiguration in query parameter serialization


        if let Some(ref description) = self.description {
            params.push("description".to_string());
            params.push(description.to_string());
        }


        if let Some(ref name) = self.name {
            params.push("name".to_string());
            params.push(name.to_string());
        }


        if let Some(ref project) = self.project {
            params.push("project".to_string());
            params.push(project.to_string());
        }


        if let Some(ref token) = self.token {
            params.push("token".to_string());
            params.push(token.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CloudAccountContent value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CloudAccountContent {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub cloud_account_secific_configuration: Vec<serde_json::Value>,
            pub description: Vec<String>,
            pub name: Vec<String>,
            pub project: Vec<String>,
            pub token: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing CloudAccountContent".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "cloudAccountSecificConfiguration" => intermediate_rep.cloud_account_secific_configuration.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "description" => intermediate_rep.description.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "project" => intermediate_rep.project.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "token" => intermediate_rep.token.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing CloudAccountContent".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CloudAccountContent {
            cloud_account_secific_configuration: intermediate_rep.cloud_account_secific_configuration.into_iter().next(),
            description: intermediate_rep.description.into_iter().next(),
            name: intermediate_rep.name.into_iter().next(),
            project: intermediate_rep.project.into_iter().next(),
            token: intermediate_rep.token.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CloudAccountContent> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CloudAccountContent>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CloudAccountContent>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for CloudAccountContent - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<CloudAccountContent> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CloudAccountContent as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into CloudAccountContent - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CloudAccounts(
    Vec<CloudAccount>
);

impl std::convert::From<Vec<CloudAccount>> for CloudAccounts {
    fn from(x: Vec<CloudAccount>) -> Self {
        CloudAccounts(x)
    }
}

impl std::convert::From<CloudAccounts> for Vec<CloudAccount> {
    fn from(x: CloudAccounts) -> Self {
        x.0
    }
}

impl std::iter::FromIterator<CloudAccount> for CloudAccounts {
    fn from_iter<U: IntoIterator<Item=CloudAccount>>(u: U) -> Self {
        CloudAccounts(Vec::<CloudAccount>::from_iter(u))
    }
}

impl std::iter::IntoIterator for CloudAccounts {
    type Item = CloudAccount;
    type IntoIter = std::vec::IntoIter<CloudAccount>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> std::iter::IntoIterator for &'a CloudAccounts {
    type Item = &'a CloudAccount;
    type IntoIter = std::slice::Iter<'a, CloudAccount>;

    fn into_iter(self) -> Self::IntoIter {
        (&self.0).into_iter()
    }
}

impl<'a> std::iter::IntoIterator for &'a mut CloudAccounts {
    type Item = &'a mut CloudAccount;
    type IntoIter = std::slice::IterMut<'a, CloudAccount>;

    fn into_iter(self) -> Self::IntoIter {
        (&mut self.0).into_iter()
    }
}

impl std::ops::Deref for CloudAccounts {
    type Target = Vec<CloudAccount>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for CloudAccounts {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Converts the CloudAccounts value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CloudAccounts {
    fn to_string(&self) -> String {
        self.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CloudAccounts value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CloudAccounts {
    type Err = <CloudAccount as std::str::FromStr>::Err;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut items = vec![];
        for item in s.split(',')
        {
            items.push(item.parse()?);
        }
        std::result::Result::Ok(CloudAccounts(items))
    }
}


// Methods for converting between header::IntoHeaderValue<CloudAccounts> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CloudAccounts>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CloudAccounts>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for CloudAccounts - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<CloudAccounts> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CloudAccounts as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into CloudAccounts - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CloudModule {
    #[serde(rename = "capabilities")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub capabilities: Option<models::CloudModuleAllOfCapabilities>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "enabled")]
    pub enabled: bool,

    #[serde(rename = "error")]
    #[serde(deserialize_with = "swagger::nullable_format::deserialize_optional_nullable")]
    #[serde(default = "swagger::nullable_format::default_optional_nullable")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error: Option<swagger::Nullable<models::Error>>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,

    #[serde(rename = "uuid")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub uuid: Option<uuid::Uuid>,

    #[serde(rename = "cloudModuleConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cloud_module_configuration: Option<serde_json::Value>,

    #[serde(rename = "schemas")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schemas: Option<models::CloudModuleAllOfSchemas>,

}

impl CloudModule {
    pub fn new(enabled: bool, ) -> CloudModule {
        CloudModule {
            capabilities: None,
            description: None,
            enabled: enabled,
            error: None,
            name: None,
            state: None,
            uuid: None,
            cloud_module_configuration: None,
            schemas: None,
        }
    }
}

/// Converts the CloudModule value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CloudModule {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping capabilities in query parameter serialization


        if let Some(ref description) = self.description {
            params.push("description".to_string());
            params.push(description.to_string());
        }


        params.push("enabled".to_string());
        params.push(self.enabled.to_string());

        // Skipping error in query parameter serialization


        if let Some(ref name) = self.name {
            params.push("name".to_string());
            params.push(name.to_string());
        }


        if let Some(ref state) = self.state {
            params.push("state".to_string());
            params.push(state.to_string());
        }

        // Skipping uuid in query parameter serialization

        // Skipping cloudModuleConfiguration in query parameter serialization

        // Skipping schemas in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CloudModule value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CloudModule {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub capabilities: Vec<models::CloudModuleAllOfCapabilities>,
            pub description: Vec<String>,
            pub enabled: Vec<bool>,
            pub error: Vec<models::Error>,
            pub name: Vec<String>,
            pub state: Vec<String>,
            pub uuid: Vec<uuid::Uuid>,
            pub cloud_module_configuration: Vec<serde_json::Value>,
            pub schemas: Vec<models::CloudModuleAllOfSchemas>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing CloudModule".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "capabilities" => intermediate_rep.capabilities.push(<models::CloudModuleAllOfCapabilities as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "description" => intermediate_rep.description.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "enabled" => intermediate_rep.enabled.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "error" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in CloudModule".to_string()),
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "state" => intermediate_rep.state.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "uuid" => intermediate_rep.uuid.push(<uuid::Uuid as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "cloudModuleConfiguration" => intermediate_rep.cloud_module_configuration.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "schemas" => intermediate_rep.schemas.push(<models::CloudModuleAllOfSchemas as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing CloudModule".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CloudModule {
            capabilities: intermediate_rep.capabilities.into_iter().next(),
            description: intermediate_rep.description.into_iter().next(),
            enabled: intermediate_rep.enabled.into_iter().next().ok_or("enabled missing in CloudModule".to_string())?,
            error: std::result::Result::Err("Nullable types not supported in CloudModule".to_string())?,
            name: intermediate_rep.name.into_iter().next(),
            state: intermediate_rep.state.into_iter().next(),
            uuid: intermediate_rep.uuid.into_iter().next(),
            cloud_module_configuration: intermediate_rep.cloud_module_configuration.into_iter().next(),
            schemas: intermediate_rep.schemas.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CloudModule> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CloudModule>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CloudModule>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for CloudModule - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<CloudModule> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CloudModule as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into CloudModule - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CloudModuleAllOf {
    #[serde(rename = "capabilities")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub capabilities: Option<models::CloudModuleAllOfCapabilities>,

    #[serde(rename = "cloudModuleConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cloud_module_configuration: Option<serde_json::Value>,

    #[serde(rename = "schemas")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schemas: Option<models::CloudModuleAllOfSchemas>,

}

impl CloudModuleAllOf {
    pub fn new() -> CloudModuleAllOf {
        CloudModuleAllOf {
            capabilities: None,
            cloud_module_configuration: None,
            schemas: None,
        }
    }
}

/// Converts the CloudModuleAllOf value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CloudModuleAllOf {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping capabilities in query parameter serialization

        // Skipping cloudModuleConfiguration in query parameter serialization

        // Skipping schemas in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CloudModuleAllOf value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CloudModuleAllOf {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub capabilities: Vec<models::CloudModuleAllOfCapabilities>,
            pub cloud_module_configuration: Vec<serde_json::Value>,
            pub schemas: Vec<models::CloudModuleAllOfSchemas>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing CloudModuleAllOf".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "capabilities" => intermediate_rep.capabilities.push(<models::CloudModuleAllOfCapabilities as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "cloudModuleConfiguration" => intermediate_rep.cloud_module_configuration.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "schemas" => intermediate_rep.schemas.push(<models::CloudModuleAllOfSchemas as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing CloudModuleAllOf".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CloudModuleAllOf {
            capabilities: intermediate_rep.capabilities.into_iter().next(),
            cloud_module_configuration: intermediate_rep.cloud_module_configuration.into_iter().next(),
            schemas: intermediate_rep.schemas.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CloudModuleAllOf> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CloudModuleAllOf>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CloudModuleAllOf>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for CloudModuleAllOf - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<CloudModuleAllOf> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CloudModuleAllOf as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into CloudModuleAllOf - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CloudModuleAllOfCapabilities {
    #[serde(rename = "canCreateNewServers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub can_create_new_servers: Option<bool>,

    #[serde(rename = "canDeleteServers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub can_delete_servers: Option<bool>,

    #[serde(rename = "canQuery")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub can_query: Option<models::CloudModuleAllOfCapabilitiesCanQuery>,

    #[serde(rename = "canReconfigureOffline")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub can_reconfigure_offline: Option<models::CloudModuleReconfigurationProperties>,

    #[serde(rename = "canReconfigureOnline")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub can_reconfigure_online: Option<models::CloudModuleReconfigurationProperties>,

    #[serde(rename = "supportConfigurationValidation")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub support_configuration_validation: Option<bool>,

    #[serde(rename = "supportPartialConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub support_partial_configuration: Option<bool>,

}

impl CloudModuleAllOfCapabilities {
    pub fn new() -> CloudModuleAllOfCapabilities {
        CloudModuleAllOfCapabilities {
            can_create_new_servers: None,
            can_delete_servers: None,
            can_query: None,
            can_reconfigure_offline: None,
            can_reconfigure_online: None,
            support_configuration_validation: None,
            support_partial_configuration: None,
        }
    }
}

/// Converts the CloudModuleAllOfCapabilities value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CloudModuleAllOfCapabilities {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref can_create_new_servers) = self.can_create_new_servers {
            params.push("canCreateNewServers".to_string());
            params.push(can_create_new_servers.to_string());
        }


        if let Some(ref can_delete_servers) = self.can_delete_servers {
            params.push("canDeleteServers".to_string());
            params.push(can_delete_servers.to_string());
        }

        // Skipping canQuery in query parameter serialization

        // Skipping canReconfigureOffline in query parameter serialization

        // Skipping canReconfigureOnline in query parameter serialization


        if let Some(ref support_configuration_validation) = self.support_configuration_validation {
            params.push("supportConfigurationValidation".to_string());
            params.push(support_configuration_validation.to_string());
        }


        if let Some(ref support_partial_configuration) = self.support_partial_configuration {
            params.push("supportPartialConfiguration".to_string());
            params.push(support_partial_configuration.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CloudModuleAllOfCapabilities value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CloudModuleAllOfCapabilities {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub can_create_new_servers: Vec<bool>,
            pub can_delete_servers: Vec<bool>,
            pub can_query: Vec<models::CloudModuleAllOfCapabilitiesCanQuery>,
            pub can_reconfigure_offline: Vec<models::CloudModuleReconfigurationProperties>,
            pub can_reconfigure_online: Vec<models::CloudModuleReconfigurationProperties>,
            pub support_configuration_validation: Vec<bool>,
            pub support_partial_configuration: Vec<bool>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing CloudModuleAllOfCapabilities".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "canCreateNewServers" => intermediate_rep.can_create_new_servers.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "canDeleteServers" => intermediate_rep.can_delete_servers.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "canQuery" => intermediate_rep.can_query.push(<models::CloudModuleAllOfCapabilitiesCanQuery as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "canReconfigureOffline" => intermediate_rep.can_reconfigure_offline.push(<models::CloudModuleReconfigurationProperties as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "canReconfigureOnline" => intermediate_rep.can_reconfigure_online.push(<models::CloudModuleReconfigurationProperties as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "supportConfigurationValidation" => intermediate_rep.support_configuration_validation.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "supportPartialConfiguration" => intermediate_rep.support_partial_configuration.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing CloudModuleAllOfCapabilities".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CloudModuleAllOfCapabilities {
            can_create_new_servers: intermediate_rep.can_create_new_servers.into_iter().next(),
            can_delete_servers: intermediate_rep.can_delete_servers.into_iter().next(),
            can_query: intermediate_rep.can_query.into_iter().next(),
            can_reconfigure_offline: intermediate_rep.can_reconfigure_offline.into_iter().next(),
            can_reconfigure_online: intermediate_rep.can_reconfigure_online.into_iter().next(),
            support_configuration_validation: intermediate_rep.support_configuration_validation.into_iter().next(),
            support_partial_configuration: intermediate_rep.support_partial_configuration.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CloudModuleAllOfCapabilities> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CloudModuleAllOfCapabilities>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CloudModuleAllOfCapabilities>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for CloudModuleAllOfCapabilities - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<CloudModuleAllOfCapabilities> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CloudModuleAllOfCapabilities as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into CloudModuleAllOfCapabilities - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CloudModuleAllOfCapabilitiesCanQuery {
    #[serde(rename = "cpuCores")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cpu_cores: Option<bool>,

    #[serde(rename = "diskBounds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub disk_bounds: Option<bool>,

    #[serde(rename = "machineTypes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub machine_types: Option<bool>,

    #[serde(rename = "operatingSystems")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub operating_systems: Option<bool>,

    #[serde(rename = "orientations")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub orientations: Option<bool>,

    #[serde(rename = "ramBounds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ram_bounds: Option<bool>,

}

impl CloudModuleAllOfCapabilitiesCanQuery {
    pub fn new() -> CloudModuleAllOfCapabilitiesCanQuery {
        CloudModuleAllOfCapabilitiesCanQuery {
            cpu_cores: None,
            disk_bounds: None,
            machine_types: None,
            operating_systems: None,
            orientations: None,
            ram_bounds: None,
        }
    }
}

/// Converts the CloudModuleAllOfCapabilitiesCanQuery value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CloudModuleAllOfCapabilitiesCanQuery {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref cpu_cores) = self.cpu_cores {
            params.push("cpuCores".to_string());
            params.push(cpu_cores.to_string());
        }


        if let Some(ref disk_bounds) = self.disk_bounds {
            params.push("diskBounds".to_string());
            params.push(disk_bounds.to_string());
        }


        if let Some(ref machine_types) = self.machine_types {
            params.push("machineTypes".to_string());
            params.push(machine_types.to_string());
        }


        if let Some(ref operating_systems) = self.operating_systems {
            params.push("operatingSystems".to_string());
            params.push(operating_systems.to_string());
        }


        if let Some(ref orientations) = self.orientations {
            params.push("orientations".to_string());
            params.push(orientations.to_string());
        }


        if let Some(ref ram_bounds) = self.ram_bounds {
            params.push("ramBounds".to_string());
            params.push(ram_bounds.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CloudModuleAllOfCapabilitiesCanQuery value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CloudModuleAllOfCapabilitiesCanQuery {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub cpu_cores: Vec<bool>,
            pub disk_bounds: Vec<bool>,
            pub machine_types: Vec<bool>,
            pub operating_systems: Vec<bool>,
            pub orientations: Vec<bool>,
            pub ram_bounds: Vec<bool>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing CloudModuleAllOfCapabilitiesCanQuery".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "cpuCores" => intermediate_rep.cpu_cores.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "diskBounds" => intermediate_rep.disk_bounds.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "machineTypes" => intermediate_rep.machine_types.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "operatingSystems" => intermediate_rep.operating_systems.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "orientations" => intermediate_rep.orientations.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "ramBounds" => intermediate_rep.ram_bounds.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing CloudModuleAllOfCapabilitiesCanQuery".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CloudModuleAllOfCapabilitiesCanQuery {
            cpu_cores: intermediate_rep.cpu_cores.into_iter().next(),
            disk_bounds: intermediate_rep.disk_bounds.into_iter().next(),
            machine_types: intermediate_rep.machine_types.into_iter().next(),
            operating_systems: intermediate_rep.operating_systems.into_iter().next(),
            orientations: intermediate_rep.orientations.into_iter().next(),
            ram_bounds: intermediate_rep.ram_bounds.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CloudModuleAllOfCapabilitiesCanQuery> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CloudModuleAllOfCapabilitiesCanQuery>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CloudModuleAllOfCapabilitiesCanQuery>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for CloudModuleAllOfCapabilitiesCanQuery - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<CloudModuleAllOfCapabilitiesCanQuery> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CloudModuleAllOfCapabilitiesCanQuery as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into CloudModuleAllOfCapabilitiesCanQuery - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CloudModuleAllOfSchemas {
    #[serde(rename = "cloudAccountSpecificConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cloud_account_specific_configuration: Option<serde_json::Value>,

    #[serde(rename = "cloudModuleConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cloud_module_configuration: Option<serde_json::Value>,

    #[serde(rename = "serverSpecifiConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub server_specifi_configuration: Option<serde_json::Value>,

}

impl CloudModuleAllOfSchemas {
    pub fn new() -> CloudModuleAllOfSchemas {
        CloudModuleAllOfSchemas {
            cloud_account_specific_configuration: None,
            cloud_module_configuration: None,
            server_specifi_configuration: None,
        }
    }
}

/// Converts the CloudModuleAllOfSchemas value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CloudModuleAllOfSchemas {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping cloudAccountSpecificConfiguration in query parameter serialization

        // Skipping cloudModuleConfiguration in query parameter serialization

        // Skipping serverSpecifiConfiguration in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CloudModuleAllOfSchemas value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CloudModuleAllOfSchemas {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub cloud_account_specific_configuration: Vec<serde_json::Value>,
            pub cloud_module_configuration: Vec<serde_json::Value>,
            pub server_specifi_configuration: Vec<serde_json::Value>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing CloudModuleAllOfSchemas".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "cloudAccountSpecificConfiguration" => intermediate_rep.cloud_account_specific_configuration.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "cloudModuleConfiguration" => intermediate_rep.cloud_module_configuration.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "serverSpecifiConfiguration" => intermediate_rep.server_specifi_configuration.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing CloudModuleAllOfSchemas".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CloudModuleAllOfSchemas {
            cloud_account_specific_configuration: intermediate_rep.cloud_account_specific_configuration.into_iter().next(),
            cloud_module_configuration: intermediate_rep.cloud_module_configuration.into_iter().next(),
            server_specifi_configuration: intermediate_rep.server_specifi_configuration.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CloudModuleAllOfSchemas> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CloudModuleAllOfSchemas>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CloudModuleAllOfSchemas>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for CloudModuleAllOfSchemas - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<CloudModuleAllOfSchemas> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CloudModuleAllOfSchemas as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into CloudModuleAllOfSchemas - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CloudModuleReconfigurationProperties {
    #[serde(rename = "disk")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub disk: Option<bool>,

    #[serde(rename = "firewall")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub firewall: Option<bool>,

    #[serde(rename = "machine")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub machine: Option<bool>,

    #[serde(rename = "network")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub network: Option<bool>,

}

impl CloudModuleReconfigurationProperties {
    pub fn new() -> CloudModuleReconfigurationProperties {
        CloudModuleReconfigurationProperties {
            disk: None,
            firewall: None,
            machine: None,
            network: None,
        }
    }
}

/// Converts the CloudModuleReconfigurationProperties value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CloudModuleReconfigurationProperties {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref disk) = self.disk {
            params.push("disk".to_string());
            params.push(disk.to_string());
        }


        if let Some(ref firewall) = self.firewall {
            params.push("firewall".to_string());
            params.push(firewall.to_string());
        }


        if let Some(ref machine) = self.machine {
            params.push("machine".to_string());
            params.push(machine.to_string());
        }


        if let Some(ref network) = self.network {
            params.push("network".to_string());
            params.push(network.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CloudModuleReconfigurationProperties value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CloudModuleReconfigurationProperties {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub disk: Vec<bool>,
            pub firewall: Vec<bool>,
            pub machine: Vec<bool>,
            pub network: Vec<bool>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing CloudModuleReconfigurationProperties".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "disk" => intermediate_rep.disk.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "firewall" => intermediate_rep.firewall.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "machine" => intermediate_rep.machine.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "network" => intermediate_rep.network.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing CloudModuleReconfigurationProperties".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CloudModuleReconfigurationProperties {
            disk: intermediate_rep.disk.into_iter().next(),
            firewall: intermediate_rep.firewall.into_iter().next(),
            machine: intermediate_rep.machine.into_iter().next(),
            network: intermediate_rep.network.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CloudModuleReconfigurationProperties> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CloudModuleReconfigurationProperties>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CloudModuleReconfigurationProperties>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for CloudModuleReconfigurationProperties - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<CloudModuleReconfigurationProperties> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CloudModuleReconfigurationProperties as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into CloudModuleReconfigurationProperties - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CloudModules(
    Vec<CloudModule>
);

impl std::convert::From<Vec<CloudModule>> for CloudModules {
    fn from(x: Vec<CloudModule>) -> Self {
        CloudModules(x)
    }
}

impl std::convert::From<CloudModules> for Vec<CloudModule> {
    fn from(x: CloudModules) -> Self {
        x.0
    }
}

impl std::iter::FromIterator<CloudModule> for CloudModules {
    fn from_iter<U: IntoIterator<Item=CloudModule>>(u: U) -> Self {
        CloudModules(Vec::<CloudModule>::from_iter(u))
    }
}

impl std::iter::IntoIterator for CloudModules {
    type Item = CloudModule;
    type IntoIter = std::vec::IntoIter<CloudModule>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> std::iter::IntoIterator for &'a CloudModules {
    type Item = &'a CloudModule;
    type IntoIter = std::slice::Iter<'a, CloudModule>;

    fn into_iter(self) -> Self::IntoIter {
        (&self.0).into_iter()
    }
}

impl<'a> std::iter::IntoIterator for &'a mut CloudModules {
    type Item = &'a mut CloudModule;
    type IntoIter = std::slice::IterMut<'a, CloudModule>;

    fn into_iter(self) -> Self::IntoIter {
        (&mut self.0).into_iter()
    }
}

impl std::ops::Deref for CloudModules {
    type Target = Vec<CloudModule>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for CloudModules {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Converts the CloudModules value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CloudModules {
    fn to_string(&self) -> String {
        self.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CloudModules value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CloudModules {
    type Err = <CloudModule as std::str::FromStr>::Err;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut items = vec![];
        for item in s.split(',')
        {
            items.push(item.parse()?);
        }
        std::result::Result::Ok(CloudModules(items))
    }
}


// Methods for converting between header::IntoHeaderValue<CloudModules> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CloudModules>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CloudModules>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for CloudModules - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<CloudModules> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CloudModules as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into CloudModules - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ConsoleSettings {
    #[serde(rename = "backgroundProcessSchedule")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub background_process_schedule: Option<models::ConsoleSettingsBackgroundProcessSchedule>,

    #[serde(rename = "consolePort")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub console_port: Option<u16>,

    #[serde(rename = "sql")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sql: Option<models::ConsoleSettingsSql>,

    #[serde(rename = "timeouts")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub timeouts: Option<models::ConsoleSettingsTimeouts>,

}

impl ConsoleSettings {
    pub fn new() -> ConsoleSettings {
        ConsoleSettings {
            background_process_schedule: None,
            console_port: None,
            sql: None,
            timeouts: None,
        }
    }
}

/// Converts the ConsoleSettings value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ConsoleSettings {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping backgroundProcessSchedule in query parameter serialization


        if let Some(ref console_port) = self.console_port {
            params.push("consolePort".to_string());
            params.push(console_port.to_string());
        }

        // Skipping sql in query parameter serialization

        // Skipping timeouts in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ConsoleSettings value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ConsoleSettings {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub background_process_schedule: Vec<models::ConsoleSettingsBackgroundProcessSchedule>,
            pub console_port: Vec<u16>,
            pub sql: Vec<models::ConsoleSettingsSql>,
            pub timeouts: Vec<models::ConsoleSettingsTimeouts>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ConsoleSettings".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "backgroundProcessSchedule" => intermediate_rep.background_process_schedule.push(<models::ConsoleSettingsBackgroundProcessSchedule as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "consolePort" => intermediate_rep.console_port.push(<u16 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "sql" => intermediate_rep.sql.push(<models::ConsoleSettingsSql as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "timeouts" => intermediate_rep.timeouts.push(<models::ConsoleSettingsTimeouts as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ConsoleSettings".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ConsoleSettings {
            background_process_schedule: intermediate_rep.background_process_schedule.into_iter().next(),
            console_port: intermediate_rep.console_port.into_iter().next(),
            sql: intermediate_rep.sql.into_iter().next(),
            timeouts: intermediate_rep.timeouts.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ConsoleSettings> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ConsoleSettings>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ConsoleSettings>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ConsoleSettings - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ConsoleSettings> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ConsoleSettings as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ConsoleSettings - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ConsoleSettingsBackgroundProcessSchedule {
    #[serde(rename = "moduleHeartbeats")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub module_heartbeats: Option<usize>,

    #[serde(rename = "serviceStatusPoll")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub service_status_poll: Option<usize>,

}

impl ConsoleSettingsBackgroundProcessSchedule {
    pub fn new() -> ConsoleSettingsBackgroundProcessSchedule {
        ConsoleSettingsBackgroundProcessSchedule {
            module_heartbeats: None,
            service_status_poll: None,
        }
    }
}

/// Converts the ConsoleSettingsBackgroundProcessSchedule value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ConsoleSettingsBackgroundProcessSchedule {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref module_heartbeats) = self.module_heartbeats {
            params.push("moduleHeartbeats".to_string());
            params.push(module_heartbeats.to_string());
        }


        if let Some(ref service_status_poll) = self.service_status_poll {
            params.push("serviceStatusPoll".to_string());
            params.push(service_status_poll.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ConsoleSettingsBackgroundProcessSchedule value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ConsoleSettingsBackgroundProcessSchedule {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub module_heartbeats: Vec<usize>,
            pub service_status_poll: Vec<usize>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ConsoleSettingsBackgroundProcessSchedule".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "moduleHeartbeats" => intermediate_rep.module_heartbeats.push(<usize as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "serviceStatusPoll" => intermediate_rep.service_status_poll.push(<usize as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ConsoleSettingsBackgroundProcessSchedule".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ConsoleSettingsBackgroundProcessSchedule {
            module_heartbeats: intermediate_rep.module_heartbeats.into_iter().next(),
            service_status_poll: intermediate_rep.service_status_poll.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ConsoleSettingsBackgroundProcessSchedule> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ConsoleSettingsBackgroundProcessSchedule>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ConsoleSettingsBackgroundProcessSchedule>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ConsoleSettingsBackgroundProcessSchedule - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ConsoleSettingsBackgroundProcessSchedule> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ConsoleSettingsBackgroundProcessSchedule as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ConsoleSettingsBackgroundProcessSchedule - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ConsoleSettingsSql {
    #[serde(rename = "address")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub address: Option<String>,

    #[serde(rename = "password")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub password: Option<String>,

    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub type_: Option<String>,

    #[serde(rename = "username")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub username: Option<String>,

}

impl ConsoleSettingsSql {
    pub fn new() -> ConsoleSettingsSql {
        ConsoleSettingsSql {
            address: None,
            password: None,
            type_: None,
            username: None,
        }
    }
}

/// Converts the ConsoleSettingsSql value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ConsoleSettingsSql {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref address) = self.address {
            params.push("address".to_string());
            params.push(address.to_string());
        }


        if let Some(ref password) = self.password {
            params.push("password".to_string());
            params.push(password.to_string());
        }


        if let Some(ref type_) = self.type_ {
            params.push("type".to_string());
            params.push(type_.to_string());
        }


        if let Some(ref username) = self.username {
            params.push("username".to_string());
            params.push(username.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ConsoleSettingsSql value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ConsoleSettingsSql {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub address: Vec<String>,
            pub password: Vec<String>,
            pub type_: Vec<String>,
            pub username: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ConsoleSettingsSql".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "address" => intermediate_rep.address.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "password" => intermediate_rep.password.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "type" => intermediate_rep.type_.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "username" => intermediate_rep.username.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ConsoleSettingsSql".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ConsoleSettingsSql {
            address: intermediate_rep.address.into_iter().next(),
            password: intermediate_rep.password.into_iter().next(),
            type_: intermediate_rep.type_.into_iter().next(),
            username: intermediate_rep.username.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ConsoleSettingsSql> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ConsoleSettingsSql>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ConsoleSettingsSql>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ConsoleSettingsSql - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ConsoleSettingsSql> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ConsoleSettingsSql as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ConsoleSettingsSql - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ConsoleSettingsTimeouts {
    #[serde(rename = "moduleCalls")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub module_calls: Option<usize>,

    #[serde(rename = "moduleHeartbeats")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub module_heartbeats: Option<usize>,

}

impl ConsoleSettingsTimeouts {
    pub fn new() -> ConsoleSettingsTimeouts {
        ConsoleSettingsTimeouts {
            module_calls: None,
            module_heartbeats: None,
        }
    }
}

/// Converts the ConsoleSettingsTimeouts value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ConsoleSettingsTimeouts {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref module_calls) = self.module_calls {
            params.push("moduleCalls".to_string());
            params.push(module_calls.to_string());
        }


        if let Some(ref module_heartbeats) = self.module_heartbeats {
            params.push("moduleHeartbeats".to_string());
            params.push(module_heartbeats.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ConsoleSettingsTimeouts value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ConsoleSettingsTimeouts {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub module_calls: Vec<usize>,
            pub module_heartbeats: Vec<usize>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ConsoleSettingsTimeouts".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "moduleCalls" => intermediate_rep.module_calls.push(<usize as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "moduleHeartbeats" => intermediate_rep.module_heartbeats.push(<usize as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ConsoleSettingsTimeouts".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ConsoleSettingsTimeouts {
            module_calls: intermediate_rep.module_calls.into_iter().next(),
            module_heartbeats: intermediate_rep.module_heartbeats.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ConsoleSettingsTimeouts> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ConsoleSettingsTimeouts>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ConsoleSettingsTimeouts>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ConsoleSettingsTimeouts - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ConsoleSettingsTimeouts> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ConsoleSettingsTimeouts as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ConsoleSettingsTimeouts - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum DesiredServerStates {
    #[serde(rename = "started")]
    STARTED,
    #[serde(rename = "stopped")]
    STOPPED,
}

impl std::fmt::Display for DesiredServerStates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            DesiredServerStates::STARTED => write!(f, "{}", "started"),
            DesiredServerStates::STOPPED => write!(f, "{}", "stopped"),
        }
    }
}

impl std::str::FromStr for DesiredServerStates {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "started" => std::result::Result::Ok(DesiredServerStates::STARTED),
            "stopped" => std::result::Result::Ok(DesiredServerStates::STOPPED),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum DesiredServiceStates {
    #[serde(rename = "started")]
    STARTED,
    #[serde(rename = "stopped")]
    STOPPED,
}

impl std::fmt::Display for DesiredServiceStates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            DesiredServiceStates::STARTED => write!(f, "{}", "started"),
            DesiredServiceStates::STOPPED => write!(f, "{}", "stopped"),
        }
    }
}

impl std::str::FromStr for DesiredServiceStates {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "started" => std::result::Result::Ok(DesiredServiceStates::STARTED),
            "stopped" => std::result::Result::Ok(DesiredServiceStates::STOPPED),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Error {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub code: Option<isize>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "title")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,

}

impl Error {
    pub fn new() -> Error {
        Error {
            code: None,
            description: None,
            title: None,
        }
    }
}

/// Converts the Error value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Error {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref code) = self.code {
            params.push("code".to_string());
            params.push(code.to_string());
        }


        if let Some(ref description) = self.description {
            params.push("description".to_string());
            params.push(description.to_string());
        }


        if let Some(ref title) = self.title {
            params.push("title".to_string());
            params.push(title.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Error value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Error {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub code: Vec<isize>,
            pub description: Vec<String>,
            pub title: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Error".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "code" => intermediate_rep.code.push(<isize as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "description" => intermediate_rep.description.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "title" => intermediate_rep.title.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Error".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Error {
            code: intermediate_rep.code.into_iter().next(),
            description: intermediate_rep.description.into_iter().next(),
            title: intermediate_rep.title.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Error> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Error>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Error>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Error - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Error> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Error as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Error - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ExistingUserCredentials {
    #[serde(rename = "twoFactorKey")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub two_factor_key: Option<u8>,

    #[serde(rename = "password")]
    pub password: String,

    #[serde(rename = "username")]
    pub username: String,

}

impl ExistingUserCredentials {
    pub fn new(password: String, username: String, ) -> ExistingUserCredentials {
        ExistingUserCredentials {
            two_factor_key: None,
            password: password,
            username: username,
        }
    }
}

/// Converts the ExistingUserCredentials value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ExistingUserCredentials {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref two_factor_key) = self.two_factor_key {
            params.push("twoFactorKey".to_string());
            params.push(two_factor_key.to_string());
        }


        params.push("password".to_string());
        params.push(self.password.to_string());


        params.push("username".to_string());
        params.push(self.username.to_string());

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ExistingUserCredentials value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ExistingUserCredentials {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub two_factor_key: Vec<u8>,
            pub password: Vec<String>,
            pub username: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ExistingUserCredentials".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "twoFactorKey" => intermediate_rep.two_factor_key.push(<u8 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "password" => intermediate_rep.password.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "username" => intermediate_rep.username.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ExistingUserCredentials".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ExistingUserCredentials {
            two_factor_key: intermediate_rep.two_factor_key.into_iter().next(),
            password: intermediate_rep.password.into_iter().next().ok_or("password missing in ExistingUserCredentials".to_string())?,
            username: intermediate_rep.username.into_iter().next().ok_or("username missing in ExistingUserCredentials".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ExistingUserCredentials> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ExistingUserCredentials>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ExistingUserCredentials>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ExistingUserCredentials - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ExistingUserCredentials> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ExistingUserCredentials as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ExistingUserCredentials - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ExistingUserCredentialsAllOf {
    #[serde(rename = "twoFactorKey")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub two_factor_key: Option<u8>,

}

impl ExistingUserCredentialsAllOf {
    pub fn new() -> ExistingUserCredentialsAllOf {
        ExistingUserCredentialsAllOf {
            two_factor_key: None,
        }
    }
}

/// Converts the ExistingUserCredentialsAllOf value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ExistingUserCredentialsAllOf {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref two_factor_key) = self.two_factor_key {
            params.push("twoFactorKey".to_string());
            params.push(two_factor_key.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ExistingUserCredentialsAllOf value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ExistingUserCredentialsAllOf {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub two_factor_key: Vec<u8>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ExistingUserCredentialsAllOf".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "twoFactorKey" => intermediate_rep.two_factor_key.push(<u8 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ExistingUserCredentialsAllOf".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ExistingUserCredentialsAllOf {
            two_factor_key: intermediate_rep.two_factor_key.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ExistingUserCredentialsAllOf> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ExistingUserCredentialsAllOf>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ExistingUserCredentialsAllOf>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ExistingUserCredentialsAllOf - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ExistingUserCredentialsAllOf> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ExistingUserCredentialsAllOf as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ExistingUserCredentialsAllOf - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Group {
    #[serde(rename = "groupId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub group_id: Option<usize>,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "ownerUserId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub owner_user_id: Option<usize>,

    #[serde(rename = "users")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub users: Option<Vec<i32>>,

}

impl Group {
    pub fn new(name: String, ) -> Group {
        Group {
            group_id: None,
            name: name,
            owner_user_id: None,
            users: None,
        }
    }
}

/// Converts the Group value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Group {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref group_id) = self.group_id {
            params.push("groupId".to_string());
            params.push(group_id.to_string());
        }


        params.push("name".to_string());
        params.push(self.name.to_string());


        if let Some(ref owner_user_id) = self.owner_user_id {
            params.push("ownerUserId".to_string());
            params.push(owner_user_id.to_string());
        }


        if let Some(ref users) = self.users {
            params.push("users".to_string());
            params.push(users.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",").to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Group value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Group {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub group_id: Vec<usize>,
            pub name: Vec<String>,
            pub owner_user_id: Vec<usize>,
            pub users: Vec<Vec<i32>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Group".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "groupId" => intermediate_rep.group_id.push(<usize as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "ownerUserId" => intermediate_rep.owner_user_id.push(<usize as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "users" => return std::result::Result::Err("Parsing a container in this style is not supported in Group".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing Group".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Group {
            group_id: intermediate_rep.group_id.into_iter().next(),
            name: intermediate_rep.name.into_iter().next().ok_or("name missing in Group".to_string())?,
            owner_user_id: intermediate_rep.owner_user_id.into_iter().next(),
            users: intermediate_rep.users.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Group> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Group>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Group>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Group - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Group> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Group as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Group - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GroupAllOf {
    #[serde(rename = "groupId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub group_id: Option<usize>,

}

impl GroupAllOf {
    pub fn new() -> GroupAllOf {
        GroupAllOf {
            group_id: None,
        }
    }
}

/// Converts the GroupAllOf value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for GroupAllOf {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref group_id) = self.group_id {
            params.push("groupId".to_string());
            params.push(group_id.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GroupAllOf value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GroupAllOf {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub group_id: Vec<usize>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing GroupAllOf".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "groupId" => intermediate_rep.group_id.push(<usize as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing GroupAllOf".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GroupAllOf {
            group_id: intermediate_rep.group_id.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<GroupAllOf> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<GroupAllOf>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<GroupAllOf>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for GroupAllOf - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<GroupAllOf> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <GroupAllOf as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into GroupAllOf - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GroupContent {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "ownerUserId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub owner_user_id: Option<usize>,

    #[serde(rename = "users")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub users: Option<Vec<i32>>,

}

impl GroupContent {
    pub fn new(name: String, ) -> GroupContent {
        GroupContent {
            name: name,
            owner_user_id: None,
            users: None,
        }
    }
}

/// Converts the GroupContent value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for GroupContent {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("name".to_string());
        params.push(self.name.to_string());


        if let Some(ref owner_user_id) = self.owner_user_id {
            params.push("ownerUserId".to_string());
            params.push(owner_user_id.to_string());
        }


        if let Some(ref users) = self.users {
            params.push("users".to_string());
            params.push(users.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",").to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GroupContent value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GroupContent {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub name: Vec<String>,
            pub owner_user_id: Vec<usize>,
            pub users: Vec<Vec<i32>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing GroupContent".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "ownerUserId" => intermediate_rep.owner_user_id.push(<usize as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "users" => return std::result::Result::Err("Parsing a container in this style is not supported in GroupContent".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing GroupContent".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GroupContent {
            name: intermediate_rep.name.into_iter().next().ok_or("name missing in GroupContent".to_string())?,
            owner_user_id: intermediate_rep.owner_user_id.into_iter().next(),
            users: intermediate_rep.users.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<GroupContent> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<GroupContent>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<GroupContent>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for GroupContent - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<GroupContent> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <GroupContent as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into GroupContent - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Groups(
    Vec<Group>
);

impl std::convert::From<Vec<Group>> for Groups {
    fn from(x: Vec<Group>) -> Self {
        Groups(x)
    }
}

impl std::convert::From<Groups> for Vec<Group> {
    fn from(x: Groups) -> Self {
        x.0
    }
}

impl std::iter::FromIterator<Group> for Groups {
    fn from_iter<U: IntoIterator<Item=Group>>(u: U) -> Self {
        Groups(Vec::<Group>::from_iter(u))
    }
}

impl std::iter::IntoIterator for Groups {
    type Item = Group;
    type IntoIter = std::vec::IntoIter<Group>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> std::iter::IntoIterator for &'a Groups {
    type Item = &'a Group;
    type IntoIter = std::slice::Iter<'a, Group>;

    fn into_iter(self) -> Self::IntoIter {
        (&self.0).into_iter()
    }
}

impl<'a> std::iter::IntoIterator for &'a mut Groups {
    type Item = &'a mut Group;
    type IntoIter = std::slice::IterMut<'a, Group>;

    fn into_iter(self) -> Self::IntoIter {
        (&mut self.0).into_iter()
    }
}

impl std::ops::Deref for Groups {
    type Target = Vec<Group>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Groups {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Converts the Groups value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Groups {
    fn to_string(&self) -> String {
        self.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Groups value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Groups {
    type Err = <Group as std::str::FromStr>::Err;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut items = vec![];
        for item in s.split(',')
        {
            items.push(item.parse()?);
        }
        std::result::Result::Ok(Groups(items))
    }
}


// Methods for converting between header::IntoHeaderValue<Groups> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Groups>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Groups>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Groups - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Groups> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Groups as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Groups - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InitialServerStates {
}

impl InitialServerStates {
    pub fn new() -> InitialServerStates {
        InitialServerStates {
        }
    }
}

/// Converts the InitialServerStates value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for InitialServerStates {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a InitialServerStates value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for InitialServerStates {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing InitialServerStates".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    _ => return std::result::Result::Err("Unexpected key while parsing InitialServerStates".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(InitialServerStates {
        })
    }
}

// Methods for converting between header::IntoHeaderValue<InitialServerStates> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<InitialServerStates>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<InitialServerStates>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for InitialServerStates - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<InitialServerStates> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <InitialServerStates as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into InitialServerStates - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum InitialServerStatesAllOf {
    #[serde(rename = "uninitialized")]
    UNINITIALIZED,
}

impl std::fmt::Display for InitialServerStatesAllOf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            InitialServerStatesAllOf::UNINITIALIZED => write!(f, "{}", "uninitialized"),
        }
    }
}

impl std::str::FromStr for InitialServerStatesAllOf {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "uninitialized" => std::result::Result::Ok(InitialServerStatesAllOf::UNINITIALIZED),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InitialServiceStates {
}

impl InitialServiceStates {
    pub fn new() -> InitialServiceStates {
        InitialServiceStates {
        }
    }
}

/// Converts the InitialServiceStates value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for InitialServiceStates {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a InitialServiceStates value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for InitialServiceStates {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing InitialServiceStates".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    _ => return std::result::Result::Err("Unexpected key while parsing InitialServiceStates".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(InitialServiceStates {
        })
    }
}

// Methods for converting between header::IntoHeaderValue<InitialServiceStates> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<InitialServiceStates>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<InitialServiceStates>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for InitialServiceStates - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<InitialServiceStates> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <InitialServiceStates as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into InitialServiceStates - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InlineResponse200 {
    #[serde(rename = "maximum")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub maximum: Option<isize>,

    #[serde(rename = "minimum")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub minimum: Option<isize>,

}

impl InlineResponse200 {
    pub fn new() -> InlineResponse200 {
        InlineResponse200 {
            maximum: None,
            minimum: None,
        }
    }
}

/// Converts the InlineResponse200 value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for InlineResponse200 {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref maximum) = self.maximum {
            params.push("maximum".to_string());
            params.push(maximum.to_string());
        }


        if let Some(ref minimum) = self.minimum {
            params.push("minimum".to_string());
            params.push(minimum.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a InlineResponse200 value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for InlineResponse200 {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub maximum: Vec<isize>,
            pub minimum: Vec<isize>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing InlineResponse200".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "maximum" => intermediate_rep.maximum.push(<isize as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "minimum" => intermediate_rep.minimum.push(<isize as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing InlineResponse200".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(InlineResponse200 {
            maximum: intermediate_rep.maximum.into_iter().next(),
            minimum: intermediate_rep.minimum.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<InlineResponse200> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<InlineResponse200>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<InlineResponse200>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for InlineResponse200 - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<InlineResponse200> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <InlineResponse200 as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into InlineResponse200 - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InlineResponse2001 {
    #[serde(rename = "maximum")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub maximum: Option<isize>,

    #[serde(rename = "minimum")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub minimum: Option<isize>,

}

impl InlineResponse2001 {
    pub fn new() -> InlineResponse2001 {
        InlineResponse2001 {
            maximum: None,
            minimum: None,
        }
    }
}

/// Converts the InlineResponse2001 value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for InlineResponse2001 {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref maximum) = self.maximum {
            params.push("maximum".to_string());
            params.push(maximum.to_string());
        }


        if let Some(ref minimum) = self.minimum {
            params.push("minimum".to_string());
            params.push(minimum.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a InlineResponse2001 value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for InlineResponse2001 {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub maximum: Vec<isize>,
            pub minimum: Vec<isize>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing InlineResponse2001".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "maximum" => intermediate_rep.maximum.push(<isize as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "minimum" => intermediate_rep.minimum.push(<isize as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing InlineResponse2001".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(InlineResponse2001 {
            maximum: intermediate_rep.maximum.into_iter().next(),
            minimum: intermediate_rep.minimum.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<InlineResponse2001> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<InlineResponse2001>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<InlineResponse2001>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for InlineResponse2001 - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<InlineResponse2001> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <InlineResponse2001 as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into InlineResponse2001 - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InlineResponse2002 {
    #[serde(rename = "token")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub token: Option<String>,

}

impl InlineResponse2002 {
    pub fn new() -> InlineResponse2002 {
        InlineResponse2002 {
            token: None,
        }
    }
}

/// Converts the InlineResponse2002 value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for InlineResponse2002 {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref token) = self.token {
            params.push("token".to_string());
            params.push(token.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a InlineResponse2002 value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for InlineResponse2002 {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub token: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing InlineResponse2002".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "token" => intermediate_rep.token.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing InlineResponse2002".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(InlineResponse2002 {
            token: intermediate_rep.token.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<InlineResponse2002> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<InlineResponse2002>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<InlineResponse2002>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for InlineResponse2002 - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<InlineResponse2002> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <InlineResponse2002 as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into InlineResponse2002 - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ModuleShared {
    #[serde(rename = "capabilities")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub capabilities: Option<models::ModuleSharedCapabilities>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "enabled")]
    pub enabled: bool,

    #[serde(rename = "error")]
    #[serde(deserialize_with = "swagger::nullable_format::deserialize_optional_nullable")]
    #[serde(default = "swagger::nullable_format::default_optional_nullable")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error: Option<swagger::Nullable<models::Error>>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,

    #[serde(rename = "uuid")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub uuid: Option<uuid::Uuid>,

}

impl ModuleShared {
    pub fn new(enabled: bool, ) -> ModuleShared {
        ModuleShared {
            capabilities: None,
            description: None,
            enabled: enabled,
            error: None,
            name: None,
            state: None,
            uuid: None,
        }
    }
}

/// Converts the ModuleShared value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ModuleShared {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping capabilities in query parameter serialization


        if let Some(ref description) = self.description {
            params.push("description".to_string());
            params.push(description.to_string());
        }


        params.push("enabled".to_string());
        params.push(self.enabled.to_string());

        // Skipping error in query parameter serialization


        if let Some(ref name) = self.name {
            params.push("name".to_string());
            params.push(name.to_string());
        }


        if let Some(ref state) = self.state {
            params.push("state".to_string());
            params.push(state.to_string());
        }

        // Skipping uuid in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ModuleShared value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ModuleShared {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub capabilities: Vec<models::ModuleSharedCapabilities>,
            pub description: Vec<String>,
            pub enabled: Vec<bool>,
            pub error: Vec<models::Error>,
            pub name: Vec<String>,
            pub state: Vec<String>,
            pub uuid: Vec<uuid::Uuid>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ModuleShared".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "capabilities" => intermediate_rep.capabilities.push(<models::ModuleSharedCapabilities as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "description" => intermediate_rep.description.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "enabled" => intermediate_rep.enabled.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "error" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in ModuleShared".to_string()),
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "state" => intermediate_rep.state.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "uuid" => intermediate_rep.uuid.push(<uuid::Uuid as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ModuleShared".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ModuleShared {
            capabilities: intermediate_rep.capabilities.into_iter().next(),
            description: intermediate_rep.description.into_iter().next(),
            enabled: intermediate_rep.enabled.into_iter().next().ok_or("enabled missing in ModuleShared".to_string())?,
            error: std::result::Result::Err("Nullable types not supported in ModuleShared".to_string())?,
            name: intermediate_rep.name.into_iter().next(),
            state: intermediate_rep.state.into_iter().next(),
            uuid: intermediate_rep.uuid.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ModuleShared> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ModuleShared>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ModuleShared>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ModuleShared - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ModuleShared> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ModuleShared as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ModuleShared - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ModuleSharedCapabilities {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "moduleType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub module_type: Option<String>,

}

impl ModuleSharedCapabilities {
    pub fn new() -> ModuleSharedCapabilities {
        ModuleSharedCapabilities {
            module_type: None,
        }
    }
}

/// Converts the ModuleSharedCapabilities value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ModuleSharedCapabilities {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref module_type) = self.module_type {
            params.push("moduleType".to_string());
            params.push(module_type.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ModuleSharedCapabilities value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ModuleSharedCapabilities {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub module_type: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ModuleSharedCapabilities".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "moduleType" => intermediate_rep.module_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ModuleSharedCapabilities".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ModuleSharedCapabilities {
            module_type: intermediate_rep.module_type.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ModuleSharedCapabilities> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ModuleSharedCapabilities>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ModuleSharedCapabilities>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ModuleSharedCapabilities - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ModuleSharedCapabilities> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ModuleSharedCapabilities as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ModuleSharedCapabilities - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct NewUserCredentials {
    #[serde(rename = "twoFactorSecret")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub two_factor_secret: Option<String>,

    #[serde(rename = "password")]
    pub password: String,

    #[serde(rename = "username")]
    pub username: String,

}

impl NewUserCredentials {
    pub fn new(password: String, username: String, ) -> NewUserCredentials {
        NewUserCredentials {
            two_factor_secret: Some("".to_string()),
            password: password,
            username: username,
        }
    }
}

/// Converts the NewUserCredentials value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for NewUserCredentials {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref two_factor_secret) = self.two_factor_secret {
            params.push("twoFactorSecret".to_string());
            params.push(two_factor_secret.to_string());
        }


        params.push("password".to_string());
        params.push(self.password.to_string());


        params.push("username".to_string());
        params.push(self.username.to_string());

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a NewUserCredentials value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for NewUserCredentials {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub two_factor_secret: Vec<String>,
            pub password: Vec<String>,
            pub username: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing NewUserCredentials".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "twoFactorSecret" => intermediate_rep.two_factor_secret.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "password" => intermediate_rep.password.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "username" => intermediate_rep.username.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing NewUserCredentials".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(NewUserCredentials {
            two_factor_secret: intermediate_rep.two_factor_secret.into_iter().next(),
            password: intermediate_rep.password.into_iter().next().ok_or("password missing in NewUserCredentials".to_string())?,
            username: intermediate_rep.username.into_iter().next().ok_or("username missing in NewUserCredentials".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<NewUserCredentials> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<NewUserCredentials>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<NewUserCredentials>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for NewUserCredentials - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<NewUserCredentials> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <NewUserCredentials as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into NewUserCredentials - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct NewUserCredentialsAllOf {
    #[serde(rename = "twoFactorSecret")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub two_factor_secret: Option<String>,

}

impl NewUserCredentialsAllOf {
    pub fn new() -> NewUserCredentialsAllOf {
        NewUserCredentialsAllOf {
            two_factor_secret: Some("".to_string()),
        }
    }
}

/// Converts the NewUserCredentialsAllOf value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for NewUserCredentialsAllOf {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref two_factor_secret) = self.two_factor_secret {
            params.push("twoFactorSecret".to_string());
            params.push(two_factor_secret.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a NewUserCredentialsAllOf value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for NewUserCredentialsAllOf {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub two_factor_secret: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing NewUserCredentialsAllOf".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "twoFactorSecret" => intermediate_rep.two_factor_secret.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing NewUserCredentialsAllOf".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(NewUserCredentialsAllOf {
            two_factor_secret: intermediate_rep.two_factor_secret.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<NewUserCredentialsAllOf> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<NewUserCredentialsAllOf>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<NewUserCredentialsAllOf>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for NewUserCredentialsAllOf - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<NewUserCredentialsAllOf> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <NewUserCredentialsAllOf as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into NewUserCredentialsAllOf - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Permission {
    #[serde(rename = "groupId")]
    pub group_id: usize,

    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub permissions: Option<models::PermissionPermissions>,

}

impl Permission {
    pub fn new(group_id: usize, ) -> Permission {
        Permission {
            group_id: group_id,
            permissions: None,
        }
    }
}

/// Converts the Permission value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Permission {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("groupId".to_string());
        params.push(self.group_id.to_string());

        // Skipping permissions in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Permission value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Permission {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub group_id: Vec<usize>,
            pub permissions: Vec<models::PermissionPermissions>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Permission".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "groupId" => intermediate_rep.group_id.push(<usize as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "permissions" => intermediate_rep.permissions.push(<models::PermissionPermissions as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Permission".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Permission {
            group_id: intermediate_rep.group_id.into_iter().next().ok_or("groupId missing in Permission".to_string())?,
            permissions: intermediate_rep.permissions.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Permission> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Permission>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Permission>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Permission - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Permission> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Permission as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Permission - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct PermissionPermissions {
    #[serde(rename = "alterPermissions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub alter_permissions: Option<bool>,

    #[serde(rename = "create")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub create: Option<bool>,

    #[serde(rename = "delete")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub delete: Option<bool>,

    #[serde(rename = "read")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub read: Option<bool>,

    #[serde(rename = "write")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub write: Option<bool>,

}

impl PermissionPermissions {
    pub fn new() -> PermissionPermissions {
        PermissionPermissions {
            alter_permissions: Some(false),
            create: Some(false),
            delete: Some(false),
            read: Some(false),
            write: Some(false),
        }
    }
}

/// Converts the PermissionPermissions value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for PermissionPermissions {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref alter_permissions) = self.alter_permissions {
            params.push("alterPermissions".to_string());
            params.push(alter_permissions.to_string());
        }


        if let Some(ref create) = self.create {
            params.push("create".to_string());
            params.push(create.to_string());
        }


        if let Some(ref delete) = self.delete {
            params.push("delete".to_string());
            params.push(delete.to_string());
        }


        if let Some(ref read) = self.read {
            params.push("read".to_string());
            params.push(read.to_string());
        }


        if let Some(ref write) = self.write {
            params.push("write".to_string());
            params.push(write.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a PermissionPermissions value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for PermissionPermissions {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub alter_permissions: Vec<bool>,
            pub create: Vec<bool>,
            pub delete: Vec<bool>,
            pub read: Vec<bool>,
            pub write: Vec<bool>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing PermissionPermissions".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "alterPermissions" => intermediate_rep.alter_permissions.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "create" => intermediate_rep.create.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "delete" => intermediate_rep.delete.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "read" => intermediate_rep.read.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "write" => intermediate_rep.write.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing PermissionPermissions".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(PermissionPermissions {
            alter_permissions: intermediate_rep.alter_permissions.into_iter().next(),
            create: intermediate_rep.create.into_iter().next(),
            delete: intermediate_rep.delete.into_iter().next(),
            read: intermediate_rep.read.into_iter().next(),
            write: intermediate_rep.write.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<PermissionPermissions> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<PermissionPermissions>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<PermissionPermissions>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for PermissionPermissions - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<PermissionPermissions> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <PermissionPermissions as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into PermissionPermissions - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Permissions(
    Vec<Permission>
);

impl std::convert::From<Vec<Permission>> for Permissions {
    fn from(x: Vec<Permission>) -> Self {
        Permissions(x)
    }
}

impl std::convert::From<Permissions> for Vec<Permission> {
    fn from(x: Permissions) -> Self {
        x.0
    }
}

impl std::iter::FromIterator<Permission> for Permissions {
    fn from_iter<U: IntoIterator<Item=Permission>>(u: U) -> Self {
        Permissions(Vec::<Permission>::from_iter(u))
    }
}

impl std::iter::IntoIterator for Permissions {
    type Item = Permission;
    type IntoIter = std::vec::IntoIter<Permission>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> std::iter::IntoIterator for &'a Permissions {
    type Item = &'a Permission;
    type IntoIter = std::slice::Iter<'a, Permission>;

    fn into_iter(self) -> Self::IntoIter {
        (&self.0).into_iter()
    }
}

impl<'a> std::iter::IntoIterator for &'a mut Permissions {
    type Item = &'a mut Permission;
    type IntoIter = std::slice::IterMut<'a, Permission>;

    fn into_iter(self) -> Self::IntoIter {
        (&mut self.0).into_iter()
    }
}

impl std::ops::Deref for Permissions {
    type Target = Vec<Permission>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Permissions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Converts the Permissions value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Permissions {
    fn to_string(&self) -> String {
        self.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Permissions value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Permissions {
    type Err = <Permission as std::str::FromStr>::Err;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut items = vec![];
        for item in s.split(',')
        {
            items.push(item.parse()?);
        }
        std::result::Result::Ok(Permissions(items))
    }
}


// Methods for converting between header::IntoHeaderValue<Permissions> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Permissions>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Permissions>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Permissions - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Permissions> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Permissions as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Permissions - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct PostNewServer {
    #[serde(rename = "state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<models::InitialServerStates>,

    #[serde(rename = "cloudModule")]
    pub cloud_module: models::ServerContentCloudModule,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "machine")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub machine: Option<models::ServerContentMachine>,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "network")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub network: Option<models::ServerContentNetwork>,

    #[serde(rename = "primaryServer")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub primary_server: Option<bool>,

    #[serde(rename = "serverSpecificConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub server_specific_configuration: Option<serde_json::Value>,

}

impl PostNewServer {
    pub fn new(cloud_module: models::ServerContentCloudModule, name: String, ) -> PostNewServer {
        PostNewServer {
            state: None,
            cloud_module: cloud_module,
            description: None,
            machine: None,
            name: name,
            network: None,
            primary_server: Some(false),
            server_specific_configuration: None,
        }
    }
}

/// Converts the PostNewServer value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for PostNewServer {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping state in query parameter serialization

        // Skipping cloudModule in query parameter serialization


        if let Some(ref description) = self.description {
            params.push("description".to_string());
            params.push(description.to_string());
        }

        // Skipping machine in query parameter serialization


        params.push("name".to_string());
        params.push(self.name.to_string());

        // Skipping network in query parameter serialization


        if let Some(ref primary_server) = self.primary_server {
            params.push("primaryServer".to_string());
            params.push(primary_server.to_string());
        }

        // Skipping serverSpecificConfiguration in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a PostNewServer value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for PostNewServer {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub state: Vec<models::InitialServerStates>,
            pub cloud_module: Vec<models::ServerContentCloudModule>,
            pub description: Vec<String>,
            pub machine: Vec<models::ServerContentMachine>,
            pub name: Vec<String>,
            pub network: Vec<models::ServerContentNetwork>,
            pub primary_server: Vec<bool>,
            pub server_specific_configuration: Vec<serde_json::Value>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing PostNewServer".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "state" => intermediate_rep.state.push(<models::InitialServerStates as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "cloudModule" => intermediate_rep.cloud_module.push(<models::ServerContentCloudModule as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "description" => intermediate_rep.description.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "machine" => intermediate_rep.machine.push(<models::ServerContentMachine as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "network" => intermediate_rep.network.push(<models::ServerContentNetwork as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "primaryServer" => intermediate_rep.primary_server.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "serverSpecificConfiguration" => intermediate_rep.server_specific_configuration.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing PostNewServer".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(PostNewServer {
            state: intermediate_rep.state.into_iter().next(),
            cloud_module: intermediate_rep.cloud_module.into_iter().next().ok_or("cloudModule missing in PostNewServer".to_string())?,
            description: intermediate_rep.description.into_iter().next(),
            machine: intermediate_rep.machine.into_iter().next(),
            name: intermediate_rep.name.into_iter().next().ok_or("name missing in PostNewServer".to_string())?,
            network: intermediate_rep.network.into_iter().next(),
            primary_server: intermediate_rep.primary_server.into_iter().next(),
            server_specific_configuration: intermediate_rep.server_specific_configuration.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<PostNewServer> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<PostNewServer>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<PostNewServer>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for PostNewServer - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<PostNewServer> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <PostNewServer as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into PostNewServer - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct PostNewServerAllOf {
    #[serde(rename = "state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<models::InitialServerStates>,

}

impl PostNewServerAllOf {
    pub fn new() -> PostNewServerAllOf {
        PostNewServerAllOf {
            state: None,
        }
    }
}

/// Converts the PostNewServerAllOf value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for PostNewServerAllOf {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping state in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a PostNewServerAllOf value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for PostNewServerAllOf {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub state: Vec<models::InitialServerStates>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing PostNewServerAllOf".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "state" => intermediate_rep.state.push(<models::InitialServerStates as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing PostNewServerAllOf".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(PostNewServerAllOf {
            state: intermediate_rep.state.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<PostNewServerAllOf> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<PostNewServerAllOf>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<PostNewServerAllOf>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for PostNewServerAllOf - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<PostNewServerAllOf> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <PostNewServerAllOf as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into PostNewServerAllOf - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct PostNewService {
    #[serde(rename = "state")]
    pub state: models::InitialServiceStates,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "dns")]
    pub dns: String,

    #[serde(rename = "flags")]
    pub flags: models::ServiceContentFlags,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "ports")]
    pub ports: Vec<i32>,

    #[serde(rename = "serviceSpecificConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub service_specific_configuration: Option<serde_json::Value>,

    #[serde(rename = "shutdownAfterTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub shutdown_after_time: Option<usize>,

    #[serde(rename = "serviceModuleUuid")]
    pub service_module_uuid: uuid::Uuid,

}

impl PostNewService {
    pub fn new(state: models::InitialServiceStates, dns: String, flags: models::ServiceContentFlags, name: String, ports: Vec<i32>, service_module_uuid: uuid::Uuid, ) -> PostNewService {
        PostNewService {
            state: state,
            description: Some("".to_string()),
            dns: dns,
            flags: flags,
            name: name,
            ports: ports,
            service_specific_configuration: None,
            shutdown_after_time: None,
            service_module_uuid: service_module_uuid,
        }
    }
}

/// Converts the PostNewService value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for PostNewService {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping state in query parameter serialization


        if let Some(ref description) = self.description {
            params.push("description".to_string());
            params.push(description.to_string());
        }


        params.push("dns".to_string());
        params.push(self.dns.to_string());

        // Skipping flags in query parameter serialization


        params.push("name".to_string());
        params.push(self.name.to_string());


        params.push("ports".to_string());
        params.push(self.ports.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",").to_string());

        // Skipping serviceSpecificConfiguration in query parameter serialization


        if let Some(ref shutdown_after_time) = self.shutdown_after_time {
            params.push("shutdownAfterTime".to_string());
            params.push(shutdown_after_time.to_string());
        }

        // Skipping serviceModuleUuid in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a PostNewService value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for PostNewService {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub state: Vec<models::InitialServiceStates>,
            pub description: Vec<String>,
            pub dns: Vec<String>,
            pub flags: Vec<models::ServiceContentFlags>,
            pub name: Vec<String>,
            pub ports: Vec<Vec<i32>>,
            pub service_specific_configuration: Vec<serde_json::Value>,
            pub shutdown_after_time: Vec<usize>,
            pub service_module_uuid: Vec<uuid::Uuid>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing PostNewService".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "state" => intermediate_rep.state.push(<models::InitialServiceStates as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "description" => intermediate_rep.description.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "dns" => intermediate_rep.dns.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "flags" => intermediate_rep.flags.push(<models::ServiceContentFlags as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "ports" => return std::result::Result::Err("Parsing a container in this style is not supported in PostNewService".to_string()),
                    "serviceSpecificConfiguration" => intermediate_rep.service_specific_configuration.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "shutdownAfterTime" => intermediate_rep.shutdown_after_time.push(<usize as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "serviceModuleUuid" => intermediate_rep.service_module_uuid.push(<uuid::Uuid as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing PostNewService".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(PostNewService {
            state: intermediate_rep.state.into_iter().next().ok_or("state missing in PostNewService".to_string())?,
            description: intermediate_rep.description.into_iter().next(),
            dns: intermediate_rep.dns.into_iter().next().ok_or("dns missing in PostNewService".to_string())?,
            flags: intermediate_rep.flags.into_iter().next().ok_or("flags missing in PostNewService".to_string())?,
            name: intermediate_rep.name.into_iter().next().ok_or("name missing in PostNewService".to_string())?,
            ports: intermediate_rep.ports.into_iter().next().ok_or("ports missing in PostNewService".to_string())?,
            service_specific_configuration: intermediate_rep.service_specific_configuration.into_iter().next(),
            shutdown_after_time: intermediate_rep.shutdown_after_time.into_iter().next(),
            service_module_uuid: intermediate_rep.service_module_uuid.into_iter().next().ok_or("serviceModuleUuid missing in PostNewService".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<PostNewService> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<PostNewService>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<PostNewService>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for PostNewService - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<PostNewService> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <PostNewService as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into PostNewService - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct PostNewServiceAllOf {
    #[serde(rename = "state")]
    pub state: models::InitialServiceStates,

}

impl PostNewServiceAllOf {
    pub fn new(state: models::InitialServiceStates, ) -> PostNewServiceAllOf {
        PostNewServiceAllOf {
            state: state,
        }
    }
}

/// Converts the PostNewServiceAllOf value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for PostNewServiceAllOf {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping state in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a PostNewServiceAllOf value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for PostNewServiceAllOf {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub state: Vec<models::InitialServiceStates>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing PostNewServiceAllOf".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "state" => intermediate_rep.state.push(<models::InitialServiceStates as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing PostNewServiceAllOf".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(PostNewServiceAllOf {
            state: intermediate_rep.state.into_iter().next().ok_or("state missing in PostNewServiceAllOf".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<PostNewServiceAllOf> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<PostNewServiceAllOf>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<PostNewServiceAllOf>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for PostNewServiceAllOf - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<PostNewServiceAllOf> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <PostNewServiceAllOf as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into PostNewServiceAllOf - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct PostNewServiceAllOf1 {
    #[serde(rename = "serviceModuleUuid")]
    pub service_module_uuid: uuid::Uuid,

}

impl PostNewServiceAllOf1 {
    pub fn new(service_module_uuid: uuid::Uuid, ) -> PostNewServiceAllOf1 {
        PostNewServiceAllOf1 {
            service_module_uuid: service_module_uuid,
        }
    }
}

/// Converts the PostNewServiceAllOf1 value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for PostNewServiceAllOf1 {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping serviceModuleUuid in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a PostNewServiceAllOf1 value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for PostNewServiceAllOf1 {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub service_module_uuid: Vec<uuid::Uuid>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing PostNewServiceAllOf1".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "serviceModuleUuid" => intermediate_rep.service_module_uuid.push(<uuid::Uuid as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing PostNewServiceAllOf1".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(PostNewServiceAllOf1 {
            service_module_uuid: intermediate_rep.service_module_uuid.into_iter().next().ok_or("serviceModuleUuid missing in PostNewServiceAllOf1".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<PostNewServiceAllOf1> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<PostNewServiceAllOf1>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<PostNewServiceAllOf1>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for PostNewServiceAllOf1 - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<PostNewServiceAllOf1> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <PostNewServiceAllOf1 as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into PostNewServiceAllOf1 - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Server {
    #[serde(rename = "state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<serde_json::Value>,

    #[serde(rename = "desiredState")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub desired_state: Option<serde_json::Value>,

    #[serde(rename = "error")]
    #[serde(deserialize_with = "swagger::nullable_format::deserialize_optional_nullable")]
    #[serde(default = "swagger::nullable_format::default_optional_nullable")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error: Option<swagger::Nullable<models::Error>>,

    #[serde(rename = "newServer")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub new_server: Option<bool>,

    #[serde(rename = "serverId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub server_id: Option<usize>,

    #[serde(rename = "serverSpecificIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub server_specific_ids: Option<models::ServerAllOf4ServerSpecificIds>,

    #[serde(rename = "cloudModule")]
    pub cloud_module: models::ServerContentCloudModule,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "machine")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub machine: Option<models::ServerContentMachine>,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "network")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub network: Option<models::ServerContentNetwork>,

    #[serde(rename = "primaryServer")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub primary_server: Option<bool>,

    #[serde(rename = "serverSpecificConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub server_specific_configuration: Option<serde_json::Value>,

}

impl Server {
    pub fn new(cloud_module: models::ServerContentCloudModule, name: String, ) -> Server {
        Server {
            state: None,
            desired_state: None,
            error: None,
            new_server: None,
            server_id: None,
            server_specific_ids: None,
            cloud_module: cloud_module,
            description: None,
            machine: None,
            name: name,
            network: None,
            primary_server: Some(false),
            server_specific_configuration: None,
        }
    }
}

/// Converts the Server value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Server {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping state in query parameter serialization

        // Skipping desiredState in query parameter serialization

        // Skipping error in query parameter serialization


        if let Some(ref new_server) = self.new_server {
            params.push("newServer".to_string());
            params.push(new_server.to_string());
        }


        if let Some(ref server_id) = self.server_id {
            params.push("serverId".to_string());
            params.push(server_id.to_string());
        }

        // Skipping serverSpecificIds in query parameter serialization

        // Skipping cloudModule in query parameter serialization


        if let Some(ref description) = self.description {
            params.push("description".to_string());
            params.push(description.to_string());
        }

        // Skipping machine in query parameter serialization


        params.push("name".to_string());
        params.push(self.name.to_string());

        // Skipping network in query parameter serialization


        if let Some(ref primary_server) = self.primary_server {
            params.push("primaryServer".to_string());
            params.push(primary_server.to_string());
        }

        // Skipping serverSpecificConfiguration in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Server value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Server {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub state: Vec<serde_json::Value>,
            pub desired_state: Vec<serde_json::Value>,
            pub error: Vec<models::Error>,
            pub new_server: Vec<bool>,
            pub server_id: Vec<usize>,
            pub server_specific_ids: Vec<models::ServerAllOf4ServerSpecificIds>,
            pub cloud_module: Vec<models::ServerContentCloudModule>,
            pub description: Vec<String>,
            pub machine: Vec<models::ServerContentMachine>,
            pub name: Vec<String>,
            pub network: Vec<models::ServerContentNetwork>,
            pub primary_server: Vec<bool>,
            pub server_specific_configuration: Vec<serde_json::Value>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Server".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "state" => intermediate_rep.state.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "desiredState" => intermediate_rep.desired_state.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "error" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in Server".to_string()),
                    "newServer" => intermediate_rep.new_server.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "serverId" => intermediate_rep.server_id.push(<usize as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "serverSpecificIds" => intermediate_rep.server_specific_ids.push(<models::ServerAllOf4ServerSpecificIds as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "cloudModule" => intermediate_rep.cloud_module.push(<models::ServerContentCloudModule as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "description" => intermediate_rep.description.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "machine" => intermediate_rep.machine.push(<models::ServerContentMachine as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "network" => intermediate_rep.network.push(<models::ServerContentNetwork as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "primaryServer" => intermediate_rep.primary_server.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "serverSpecificConfiguration" => intermediate_rep.server_specific_configuration.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Server".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Server {
            state: intermediate_rep.state.into_iter().next(),
            desired_state: intermediate_rep.desired_state.into_iter().next(),
            error: std::result::Result::Err("Nullable types not supported in Server".to_string())?,
            new_server: intermediate_rep.new_server.into_iter().next(),
            server_id: intermediate_rep.server_id.into_iter().next(),
            server_specific_ids: intermediate_rep.server_specific_ids.into_iter().next(),
            cloud_module: intermediate_rep.cloud_module.into_iter().next().ok_or("cloudModule missing in Server".to_string())?,
            description: intermediate_rep.description.into_iter().next(),
            machine: intermediate_rep.machine.into_iter().next(),
            name: intermediate_rep.name.into_iter().next().ok_or("name missing in Server".to_string())?,
            network: intermediate_rep.network.into_iter().next(),
            primary_server: intermediate_rep.primary_server.into_iter().next(),
            server_specific_configuration: intermediate_rep.server_specific_configuration.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Server> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Server>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Server>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Server - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Server> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Server as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Server - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ServerAllOf {
    #[serde(rename = "state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<models::ServerStates>,

}

impl ServerAllOf {
    pub fn new() -> ServerAllOf {
        ServerAllOf {
            state: None,
        }
    }
}

/// Converts the ServerAllOf value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ServerAllOf {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping state in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ServerAllOf value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ServerAllOf {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub state: Vec<models::ServerStates>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ServerAllOf".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "state" => intermediate_rep.state.push(<models::ServerStates as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ServerAllOf".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ServerAllOf {
            state: intermediate_rep.state.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ServerAllOf> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ServerAllOf>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ServerAllOf>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ServerAllOf - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ServerAllOf> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ServerAllOf as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ServerAllOf - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ServerAllOf1 {
    #[serde(rename = "state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<serde_json::Value>,

}

impl ServerAllOf1 {
    pub fn new() -> ServerAllOf1 {
        ServerAllOf1 {
            state: None,
        }
    }
}

/// Converts the ServerAllOf1 value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ServerAllOf1 {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping state in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ServerAllOf1 value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ServerAllOf1 {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub state: Vec<serde_json::Value>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ServerAllOf1".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "state" => intermediate_rep.state.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ServerAllOf1".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ServerAllOf1 {
            state: intermediate_rep.state.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ServerAllOf1> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ServerAllOf1>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ServerAllOf1>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ServerAllOf1 - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ServerAllOf1> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ServerAllOf1 as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ServerAllOf1 - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ServerAllOf2 {
    #[serde(rename = "desiredState")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub desired_state: Option<models::ServerStates>,

}

impl ServerAllOf2 {
    pub fn new() -> ServerAllOf2 {
        ServerAllOf2 {
            desired_state: None,
        }
    }
}

/// Converts the ServerAllOf2 value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ServerAllOf2 {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping desiredState in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ServerAllOf2 value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ServerAllOf2 {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub desired_state: Vec<models::ServerStates>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ServerAllOf2".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "desiredState" => intermediate_rep.desired_state.push(<models::ServerStates as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ServerAllOf2".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ServerAllOf2 {
            desired_state: intermediate_rep.desired_state.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ServerAllOf2> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ServerAllOf2>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ServerAllOf2>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ServerAllOf2 - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ServerAllOf2> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ServerAllOf2 as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ServerAllOf2 - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ServerAllOf3 {
    #[serde(rename = "desiredState")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub desired_state: Option<serde_json::Value>,

}

impl ServerAllOf3 {
    pub fn new() -> ServerAllOf3 {
        ServerAllOf3 {
            desired_state: None,
        }
    }
}

/// Converts the ServerAllOf3 value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ServerAllOf3 {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping desiredState in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ServerAllOf3 value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ServerAllOf3 {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub desired_state: Vec<serde_json::Value>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ServerAllOf3".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "desiredState" => intermediate_rep.desired_state.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ServerAllOf3".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ServerAllOf3 {
            desired_state: intermediate_rep.desired_state.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ServerAllOf3> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ServerAllOf3>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ServerAllOf3>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ServerAllOf3 - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ServerAllOf3> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ServerAllOf3 as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ServerAllOf3 - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ServerAllOf4 {
    #[serde(rename = "error")]
    #[serde(deserialize_with = "swagger::nullable_format::deserialize_optional_nullable")]
    #[serde(default = "swagger::nullable_format::default_optional_nullable")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error: Option<swagger::Nullable<models::Error>>,

    #[serde(rename = "newServer")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub new_server: Option<bool>,

    #[serde(rename = "serverId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub server_id: Option<usize>,

    #[serde(rename = "serverSpecificIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub server_specific_ids: Option<models::ServerAllOf4ServerSpecificIds>,

}

impl ServerAllOf4 {
    pub fn new() -> ServerAllOf4 {
        ServerAllOf4 {
            error: None,
            new_server: None,
            server_id: None,
            server_specific_ids: None,
        }
    }
}

/// Converts the ServerAllOf4 value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ServerAllOf4 {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping error in query parameter serialization


        if let Some(ref new_server) = self.new_server {
            params.push("newServer".to_string());
            params.push(new_server.to_string());
        }


        if let Some(ref server_id) = self.server_id {
            params.push("serverId".to_string());
            params.push(server_id.to_string());
        }

        // Skipping serverSpecificIds in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ServerAllOf4 value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ServerAllOf4 {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub error: Vec<models::Error>,
            pub new_server: Vec<bool>,
            pub server_id: Vec<usize>,
            pub server_specific_ids: Vec<models::ServerAllOf4ServerSpecificIds>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ServerAllOf4".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "error" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in ServerAllOf4".to_string()),
                    "newServer" => intermediate_rep.new_server.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "serverId" => intermediate_rep.server_id.push(<usize as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "serverSpecificIds" => intermediate_rep.server_specific_ids.push(<models::ServerAllOf4ServerSpecificIds as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ServerAllOf4".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ServerAllOf4 {
            error: std::result::Result::Err("Nullable types not supported in ServerAllOf4".to_string())?,
            new_server: intermediate_rep.new_server.into_iter().next(),
            server_id: intermediate_rep.server_id.into_iter().next(),
            server_specific_ids: intermediate_rep.server_specific_ids.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ServerAllOf4> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ServerAllOf4>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ServerAllOf4>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ServerAllOf4 - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ServerAllOf4> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ServerAllOf4 as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ServerAllOf4 - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ServerAllOf4ServerSpecificIds {
    #[serde(rename = "diskId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub disk_id: Option<String>,

    #[serde(rename = "firewallId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub firewall_id: Option<String>,

    #[serde(rename = "machineId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub machine_id: Option<String>,

    #[serde(rename = "networkId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub network_id: Option<String>,

    #[serde(rename = "publicIpv4Id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub public_ipv4_id: Option<String>,

    #[serde(rename = "publicIpv6Id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub public_ipv6_id: Option<String>,

}

impl ServerAllOf4ServerSpecificIds {
    pub fn new() -> ServerAllOf4ServerSpecificIds {
        ServerAllOf4ServerSpecificIds {
            disk_id: None,
            firewall_id: None,
            machine_id: None,
            network_id: None,
            public_ipv4_id: None,
            public_ipv6_id: None,
        }
    }
}

/// Converts the ServerAllOf4ServerSpecificIds value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ServerAllOf4ServerSpecificIds {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref disk_id) = self.disk_id {
            params.push("diskId".to_string());
            params.push(disk_id.to_string());
        }


        if let Some(ref firewall_id) = self.firewall_id {
            params.push("firewallId".to_string());
            params.push(firewall_id.to_string());
        }


        if let Some(ref machine_id) = self.machine_id {
            params.push("machineId".to_string());
            params.push(machine_id.to_string());
        }


        if let Some(ref network_id) = self.network_id {
            params.push("networkId".to_string());
            params.push(network_id.to_string());
        }


        if let Some(ref public_ipv4_id) = self.public_ipv4_id {
            params.push("publicIpv4Id".to_string());
            params.push(public_ipv4_id.to_string());
        }


        if let Some(ref public_ipv6_id) = self.public_ipv6_id {
            params.push("publicIpv6Id".to_string());
            params.push(public_ipv6_id.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ServerAllOf4ServerSpecificIds value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ServerAllOf4ServerSpecificIds {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub disk_id: Vec<String>,
            pub firewall_id: Vec<String>,
            pub machine_id: Vec<String>,
            pub network_id: Vec<String>,
            pub public_ipv4_id: Vec<String>,
            pub public_ipv6_id: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ServerAllOf4ServerSpecificIds".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "diskId" => intermediate_rep.disk_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "firewallId" => intermediate_rep.firewall_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "machineId" => intermediate_rep.machine_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "networkId" => intermediate_rep.network_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "publicIpv4Id" => intermediate_rep.public_ipv4_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "publicIpv6Id" => intermediate_rep.public_ipv6_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ServerAllOf4ServerSpecificIds".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ServerAllOf4ServerSpecificIds {
            disk_id: intermediate_rep.disk_id.into_iter().next(),
            firewall_id: intermediate_rep.firewall_id.into_iter().next(),
            machine_id: intermediate_rep.machine_id.into_iter().next(),
            network_id: intermediate_rep.network_id.into_iter().next(),
            public_ipv4_id: intermediate_rep.public_ipv4_id.into_iter().next(),
            public_ipv6_id: intermediate_rep.public_ipv6_id.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ServerAllOf4ServerSpecificIds> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ServerAllOf4ServerSpecificIds>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ServerAllOf4ServerSpecificIds>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ServerAllOf4ServerSpecificIds - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ServerAllOf4ServerSpecificIds> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ServerAllOf4ServerSpecificIds as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ServerAllOf4ServerSpecificIds - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ServerContent {
    #[serde(rename = "cloudModule")]
    pub cloud_module: models::ServerContentCloudModule,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "machine")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub machine: Option<models::ServerContentMachine>,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "network")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub network: Option<models::ServerContentNetwork>,

    #[serde(rename = "primaryServer")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub primary_server: Option<bool>,

    #[serde(rename = "serverSpecificConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub server_specific_configuration: Option<serde_json::Value>,

}

impl ServerContent {
    pub fn new(cloud_module: models::ServerContentCloudModule, name: String, ) -> ServerContent {
        ServerContent {
            cloud_module: cloud_module,
            description: None,
            machine: None,
            name: name,
            network: None,
            primary_server: Some(false),
            server_specific_configuration: None,
        }
    }
}

/// Converts the ServerContent value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ServerContent {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping cloudModule in query parameter serialization


        if let Some(ref description) = self.description {
            params.push("description".to_string());
            params.push(description.to_string());
        }

        // Skipping machine in query parameter serialization


        params.push("name".to_string());
        params.push(self.name.to_string());

        // Skipping network in query parameter serialization


        if let Some(ref primary_server) = self.primary_server {
            params.push("primaryServer".to_string());
            params.push(primary_server.to_string());
        }

        // Skipping serverSpecificConfiguration in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ServerContent value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ServerContent {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub cloud_module: Vec<models::ServerContentCloudModule>,
            pub description: Vec<String>,
            pub machine: Vec<models::ServerContentMachine>,
            pub name: Vec<String>,
            pub network: Vec<models::ServerContentNetwork>,
            pub primary_server: Vec<bool>,
            pub server_specific_configuration: Vec<serde_json::Value>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ServerContent".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "cloudModule" => intermediate_rep.cloud_module.push(<models::ServerContentCloudModule as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "description" => intermediate_rep.description.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "machine" => intermediate_rep.machine.push(<models::ServerContentMachine as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "network" => intermediate_rep.network.push(<models::ServerContentNetwork as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "primaryServer" => intermediate_rep.primary_server.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "serverSpecificConfiguration" => intermediate_rep.server_specific_configuration.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ServerContent".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ServerContent {
            cloud_module: intermediate_rep.cloud_module.into_iter().next().ok_or("cloudModule missing in ServerContent".to_string())?,
            description: intermediate_rep.description.into_iter().next(),
            machine: intermediate_rep.machine.into_iter().next(),
            name: intermediate_rep.name.into_iter().next().ok_or("name missing in ServerContent".to_string())?,
            network: intermediate_rep.network.into_iter().next(),
            primary_server: intermediate_rep.primary_server.into_iter().next(),
            server_specific_configuration: intermediate_rep.server_specific_configuration.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ServerContent> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ServerContent>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ServerContent>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ServerContent - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ServerContent> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ServerContent as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ServerContent - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ServerContentCloudModule {
    #[serde(rename = "cloudAccountId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cloud_account_id: Option<usize>,

    #[serde(rename = "cloudModuleUuid")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cloud_module_uuid: Option<uuid::Uuid>,

}

impl ServerContentCloudModule {
    pub fn new() -> ServerContentCloudModule {
        ServerContentCloudModule {
            cloud_account_id: None,
            cloud_module_uuid: None,
        }
    }
}

/// Converts the ServerContentCloudModule value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ServerContentCloudModule {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref cloud_account_id) = self.cloud_account_id {
            params.push("cloudAccountId".to_string());
            params.push(cloud_account_id.to_string());
        }

        // Skipping cloudModuleUuid in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ServerContentCloudModule value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ServerContentCloudModule {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub cloud_account_id: Vec<usize>,
            pub cloud_module_uuid: Vec<uuid::Uuid>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ServerContentCloudModule".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "cloudAccountId" => intermediate_rep.cloud_account_id.push(<usize as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "cloudModuleUuid" => intermediate_rep.cloud_module_uuid.push(<uuid::Uuid as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ServerContentCloudModule".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ServerContentCloudModule {
            cloud_account_id: intermediate_rep.cloud_account_id.into_iter().next(),
            cloud_module_uuid: intermediate_rep.cloud_module_uuid.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ServerContentCloudModule> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ServerContentCloudModule>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ServerContentCloudModule>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ServerContentCloudModule - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ServerContentCloudModule> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ServerContentCloudModule as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ServerContentCloudModule - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ServerContentMachine {
    #[serde(rename = "cpus")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cpus: Option<f64>,

    #[serde(rename = "disk")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub disk: Option<usize>,

    #[serde(rename = "operatingSystem")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub operating_system: Option<String>,

    #[serde(rename = "orientation")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub orientation: Option<String>,

    #[serde(rename = "ram")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ram: Option<usize>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub type_: Option<String>,

}

impl ServerContentMachine {
    pub fn new() -> ServerContentMachine {
        ServerContentMachine {
            cpus: None,
            disk: None,
            operating_system: Some("".to_string()),
            orientation: Some("".to_string()),
            ram: None,
            type_: Some("".to_string()),
        }
    }
}

/// Converts the ServerContentMachine value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ServerContentMachine {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref cpus) = self.cpus {
            params.push("cpus".to_string());
            params.push(cpus.to_string());
        }


        if let Some(ref disk) = self.disk {
            params.push("disk".to_string());
            params.push(disk.to_string());
        }


        if let Some(ref operating_system) = self.operating_system {
            params.push("operatingSystem".to_string());
            params.push(operating_system.to_string());
        }


        if let Some(ref orientation) = self.orientation {
            params.push("orientation".to_string());
            params.push(orientation.to_string());
        }


        if let Some(ref ram) = self.ram {
            params.push("ram".to_string());
            params.push(ram.to_string());
        }


        if let Some(ref type_) = self.type_ {
            params.push("type".to_string());
            params.push(type_.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ServerContentMachine value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ServerContentMachine {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub cpus: Vec<f64>,
            pub disk: Vec<usize>,
            pub operating_system: Vec<String>,
            pub orientation: Vec<String>,
            pub ram: Vec<usize>,
            pub type_: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ServerContentMachine".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "cpus" => intermediate_rep.cpus.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "disk" => intermediate_rep.disk.push(<usize as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "operatingSystem" => intermediate_rep.operating_system.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "orientation" => intermediate_rep.orientation.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "ram" => intermediate_rep.ram.push(<usize as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "type" => intermediate_rep.type_.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ServerContentMachine".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ServerContentMachine {
            cpus: intermediate_rep.cpus.into_iter().next(),
            disk: intermediate_rep.disk.into_iter().next(),
            operating_system: intermediate_rep.operating_system.into_iter().next(),
            orientation: intermediate_rep.orientation.into_iter().next(),
            ram: intermediate_rep.ram.into_iter().next(),
            type_: intermediate_rep.type_.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ServerContentMachine> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ServerContentMachine>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ServerContentMachine>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ServerContentMachine - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ServerContentMachine> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ServerContentMachine as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ServerContentMachine - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ServerContentNetwork {
    #[serde(rename = "additionalPorts")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub additional_ports: Option<Vec<i32>>,

    #[serde(rename = "dns")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub dns: Option<String>,

    #[serde(rename = "ipv4")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ipv4: Option<String>,

    #[serde(rename = "ipv6")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ipv6: Option<String>,

}

impl ServerContentNetwork {
    pub fn new() -> ServerContentNetwork {
        ServerContentNetwork {
            additional_ports: None,
            dns: None,
            ipv4: None,
            ipv6: None,
        }
    }
}

/// Converts the ServerContentNetwork value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ServerContentNetwork {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref additional_ports) = self.additional_ports {
            params.push("additionalPorts".to_string());
            params.push(additional_ports.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",").to_string());
        }


        if let Some(ref dns) = self.dns {
            params.push("dns".to_string());
            params.push(dns.to_string());
        }


        if let Some(ref ipv4) = self.ipv4 {
            params.push("ipv4".to_string());
            params.push(ipv4.to_string());
        }


        if let Some(ref ipv6) = self.ipv6 {
            params.push("ipv6".to_string());
            params.push(ipv6.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ServerContentNetwork value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ServerContentNetwork {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub additional_ports: Vec<Vec<i32>>,
            pub dns: Vec<String>,
            pub ipv4: Vec<String>,
            pub ipv6: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ServerContentNetwork".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "additionalPorts" => return std::result::Result::Err("Parsing a container in this style is not supported in ServerContentNetwork".to_string()),
                    "dns" => intermediate_rep.dns.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "ipv4" => intermediate_rep.ipv4.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "ipv6" => intermediate_rep.ipv6.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ServerContentNetwork".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ServerContentNetwork {
            additional_ports: intermediate_rep.additional_ports.into_iter().next(),
            dns: intermediate_rep.dns.into_iter().next(),
            ipv4: intermediate_rep.ipv4.into_iter().next(),
            ipv6: intermediate_rep.ipv6.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ServerContentNetwork> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ServerContentNetwork>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ServerContentNetwork>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ServerContentNetwork - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ServerContentNetwork> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ServerContentNetwork as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ServerContentNetwork - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ServerCredentials {
    #[serde(rename = "password")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub password: Option<String>,

    #[serde(rename = "privateKey")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub private_key: Option<String>,

    #[serde(rename = "username")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub username: Option<String>,

}

impl ServerCredentials {
    pub fn new() -> ServerCredentials {
        ServerCredentials {
            password: Some("".to_string()),
            private_key: Some("".to_string()),
            username: Some("".to_string()),
        }
    }
}

/// Converts the ServerCredentials value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ServerCredentials {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref password) = self.password {
            params.push("password".to_string());
            params.push(password.to_string());
        }


        if let Some(ref private_key) = self.private_key {
            params.push("privateKey".to_string());
            params.push(private_key.to_string());
        }


        if let Some(ref username) = self.username {
            params.push("username".to_string());
            params.push(username.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ServerCredentials value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ServerCredentials {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub password: Vec<String>,
            pub private_key: Vec<String>,
            pub username: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ServerCredentials".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "password" => intermediate_rep.password.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "privateKey" => intermediate_rep.private_key.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "username" => intermediate_rep.username.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ServerCredentials".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ServerCredentials {
            password: intermediate_rep.password.into_iter().next(),
            private_key: intermediate_rep.private_key.into_iter().next(),
            username: intermediate_rep.username.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ServerCredentials> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ServerCredentials>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ServerCredentials>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ServerCredentials - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ServerCredentials> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ServerCredentials as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ServerCredentials - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ServerStates {
}

impl ServerStates {
    pub fn new() -> ServerStates {
        ServerStates {
        }
    }
}

/// Converts the ServerStates value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ServerStates {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ServerStates value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ServerStates {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ServerStates".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    _ => return std::result::Result::Err("Unexpected key while parsing ServerStates".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ServerStates {
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ServerStates> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ServerStates>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ServerStates>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ServerStates - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ServerStates> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ServerStates as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ServerStates - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum ServerStatesAllOf {
    #[serde(rename = "configuring network")]
    NETWORK,
    #[serde(rename = "configuring firewall")]
    FIREWALL,
    #[serde(rename = "configuring disk")]
    DISK,
    #[serde(rename = "configuring machine")]
    MACHINE,
}

impl std::fmt::Display for ServerStatesAllOf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ServerStatesAllOf::NETWORK => write!(f, "{}", "configuring network"),
            ServerStatesAllOf::FIREWALL => write!(f, "{}", "configuring firewall"),
            ServerStatesAllOf::DISK => write!(f, "{}", "configuring disk"),
            ServerStatesAllOf::MACHINE => write!(f, "{}", "configuring machine"),
        }
    }
}

impl std::str::FromStr for ServerStatesAllOf {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "configuring network" => std::result::Result::Ok(ServerStatesAllOf::NETWORK),
            "configuring firewall" => std::result::Result::Ok(ServerStatesAllOf::FIREWALL),
            "configuring disk" => std::result::Result::Ok(ServerStatesAllOf::DISK),
            "configuring machine" => std::result::Result::Ok(ServerStatesAllOf::MACHINE),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Servers(
    Vec<Server>
);

impl std::convert::From<Vec<Server>> for Servers {
    fn from(x: Vec<Server>) -> Self {
        Servers(x)
    }
}

impl std::convert::From<Servers> for Vec<Server> {
    fn from(x: Servers) -> Self {
        x.0
    }
}

impl std::iter::FromIterator<Server> for Servers {
    fn from_iter<U: IntoIterator<Item=Server>>(u: U) -> Self {
        Servers(Vec::<Server>::from_iter(u))
    }
}

impl std::iter::IntoIterator for Servers {
    type Item = Server;
    type IntoIter = std::vec::IntoIter<Server>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> std::iter::IntoIterator for &'a Servers {
    type Item = &'a Server;
    type IntoIter = std::slice::Iter<'a, Server>;

    fn into_iter(self) -> Self::IntoIter {
        (&self.0).into_iter()
    }
}

impl<'a> std::iter::IntoIterator for &'a mut Servers {
    type Item = &'a mut Server;
    type IntoIter = std::slice::IterMut<'a, Server>;

    fn into_iter(self) -> Self::IntoIter {
        (&mut self.0).into_iter()
    }
}

impl std::ops::Deref for Servers {
    type Target = Vec<Server>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Servers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Converts the Servers value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Servers {
    fn to_string(&self) -> String {
        self.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Servers value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Servers {
    type Err = <Server as std::str::FromStr>::Err;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut items = vec![];
        for item in s.split(',')
        {
            items.push(item.parse()?);
        }
        std::result::Result::Ok(Servers(items))
    }
}


// Methods for converting between header::IntoHeaderValue<Servers> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Servers>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Servers>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Servers - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Servers> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Servers as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Servers - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Service {
    #[serde(rename = "state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<serde_json::Value>,

    #[serde(rename = "desiredState")]
    pub desired_state: models::DesiredServiceStates,

    #[serde(rename = "error")]
    #[serde(deserialize_with = "swagger::nullable_format::deserialize_optional_nullable")]
    #[serde(default = "swagger::nullable_format::default_optional_nullable")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error: Option<swagger::Nullable<models::Error>>,

    #[serde(rename = "flags")]
    pub flags: models::ServiceContentFlags,

    #[serde(rename = "loggedInUsers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub logged_in_users: Option<usize>,

    #[serde(rename = "serviceId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub service_id: Option<usize>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "dns")]
    pub dns: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "ports")]
    pub ports: Vec<i32>,

    #[serde(rename = "serviceSpecificConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub service_specific_configuration: Option<serde_json::Value>,

    #[serde(rename = "shutdownAfterTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub shutdown_after_time: Option<usize>,

}

impl Service {
    pub fn new(desired_state: models::DesiredServiceStates, flags: models::ServiceContentFlags, dns: String, name: String, ports: Vec<i32>, ) -> Service {
        Service {
            state: None,
            desired_state: desired_state,
            error: None,
            flags: flags,
            logged_in_users: None,
            service_id: None,
            description: Some("".to_string()),
            dns: dns,
            name: name,
            ports: ports,
            service_specific_configuration: None,
            shutdown_after_time: None,
        }
    }
}

/// Converts the Service value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Service {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping state in query parameter serialization

        // Skipping desiredState in query parameter serialization

        // Skipping error in query parameter serialization

        // Skipping flags in query parameter serialization


        if let Some(ref logged_in_users) = self.logged_in_users {
            params.push("loggedInUsers".to_string());
            params.push(logged_in_users.to_string());
        }


        if let Some(ref service_id) = self.service_id {
            params.push("serviceId".to_string());
            params.push(service_id.to_string());
        }


        if let Some(ref description) = self.description {
            params.push("description".to_string());
            params.push(description.to_string());
        }


        params.push("dns".to_string());
        params.push(self.dns.to_string());


        params.push("name".to_string());
        params.push(self.name.to_string());


        params.push("ports".to_string());
        params.push(self.ports.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",").to_string());

        // Skipping serviceSpecificConfiguration in query parameter serialization


        if let Some(ref shutdown_after_time) = self.shutdown_after_time {
            params.push("shutdownAfterTime".to_string());
            params.push(shutdown_after_time.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Service value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Service {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub state: Vec<serde_json::Value>,
            pub desired_state: Vec<models::DesiredServiceStates>,
            pub error: Vec<models::Error>,
            pub flags: Vec<models::ServiceContentFlags>,
            pub logged_in_users: Vec<usize>,
            pub service_id: Vec<usize>,
            pub description: Vec<String>,
            pub dns: Vec<String>,
            pub name: Vec<String>,
            pub ports: Vec<Vec<i32>>,
            pub service_specific_configuration: Vec<serde_json::Value>,
            pub shutdown_after_time: Vec<usize>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Service".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "state" => intermediate_rep.state.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "desiredState" => intermediate_rep.desired_state.push(<models::DesiredServiceStates as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "error" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in Service".to_string()),
                    "flags" => intermediate_rep.flags.push(<models::ServiceContentFlags as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "loggedInUsers" => intermediate_rep.logged_in_users.push(<usize as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "serviceId" => intermediate_rep.service_id.push(<usize as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "description" => intermediate_rep.description.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "dns" => intermediate_rep.dns.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "ports" => return std::result::Result::Err("Parsing a container in this style is not supported in Service".to_string()),
                    "serviceSpecificConfiguration" => intermediate_rep.service_specific_configuration.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "shutdownAfterTime" => intermediate_rep.shutdown_after_time.push(<usize as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Service".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Service {
            state: intermediate_rep.state.into_iter().next(),
            desired_state: intermediate_rep.desired_state.into_iter().next().ok_or("desiredState missing in Service".to_string())?,
            error: std::result::Result::Err("Nullable types not supported in Service".to_string())?,
            flags: intermediate_rep.flags.into_iter().next().ok_or("flags missing in Service".to_string())?,
            logged_in_users: intermediate_rep.logged_in_users.into_iter().next(),
            service_id: intermediate_rep.service_id.into_iter().next(),
            description: intermediate_rep.description.into_iter().next(),
            dns: intermediate_rep.dns.into_iter().next().ok_or("dns missing in Service".to_string())?,
            name: intermediate_rep.name.into_iter().next().ok_or("name missing in Service".to_string())?,
            ports: intermediate_rep.ports.into_iter().next().ok_or("ports missing in Service".to_string())?,
            service_specific_configuration: intermediate_rep.service_specific_configuration.into_iter().next(),
            shutdown_after_time: intermediate_rep.shutdown_after_time.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Service> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Service>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Service>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Service - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Service> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Service as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Service - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ServiceAllOf {
    #[serde(rename = "state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<models::ServiceStates>,

}

impl ServiceAllOf {
    pub fn new() -> ServiceAllOf {
        ServiceAllOf {
            state: None,
        }
    }
}

/// Converts the ServiceAllOf value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ServiceAllOf {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping state in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ServiceAllOf value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ServiceAllOf {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub state: Vec<models::ServiceStates>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ServiceAllOf".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "state" => intermediate_rep.state.push(<models::ServiceStates as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ServiceAllOf".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ServiceAllOf {
            state: intermediate_rep.state.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ServiceAllOf> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ServiceAllOf>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ServiceAllOf>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ServiceAllOf - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ServiceAllOf> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ServiceAllOf as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ServiceAllOf - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ServiceAllOf1 {
    #[serde(rename = "desiredState")]
    pub desired_state: models::DesiredServiceStates,

    #[serde(rename = "error")]
    #[serde(deserialize_with = "swagger::nullable_format::deserialize_optional_nullable")]
    #[serde(default = "swagger::nullable_format::default_optional_nullable")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error: Option<swagger::Nullable<models::Error>>,

    #[serde(rename = "flags")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub flags: Option<models::ServiceAllOf1Flags>,

    #[serde(rename = "loggedInUsers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub logged_in_users: Option<usize>,

    #[serde(rename = "serviceId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub service_id: Option<usize>,

}

impl ServiceAllOf1 {
    pub fn new(desired_state: models::DesiredServiceStates, ) -> ServiceAllOf1 {
        ServiceAllOf1 {
            desired_state: desired_state,
            error: None,
            flags: None,
            logged_in_users: None,
            service_id: None,
        }
    }
}

/// Converts the ServiceAllOf1 value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ServiceAllOf1 {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping desiredState in query parameter serialization

        // Skipping error in query parameter serialization

        // Skipping flags in query parameter serialization


        if let Some(ref logged_in_users) = self.logged_in_users {
            params.push("loggedInUsers".to_string());
            params.push(logged_in_users.to_string());
        }


        if let Some(ref service_id) = self.service_id {
            params.push("serviceId".to_string());
            params.push(service_id.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ServiceAllOf1 value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ServiceAllOf1 {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub desired_state: Vec<models::DesiredServiceStates>,
            pub error: Vec<models::Error>,
            pub flags: Vec<models::ServiceAllOf1Flags>,
            pub logged_in_users: Vec<usize>,
            pub service_id: Vec<usize>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ServiceAllOf1".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "desiredState" => intermediate_rep.desired_state.push(<models::DesiredServiceStates as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "error" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in ServiceAllOf1".to_string()),
                    "flags" => intermediate_rep.flags.push(<models::ServiceAllOf1Flags as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "loggedInUsers" => intermediate_rep.logged_in_users.push(<usize as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "serviceId" => intermediate_rep.service_id.push(<usize as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ServiceAllOf1".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ServiceAllOf1 {
            desired_state: intermediate_rep.desired_state.into_iter().next().ok_or("desiredState missing in ServiceAllOf1".to_string())?,
            error: std::result::Result::Err("Nullable types not supported in ServiceAllOf1".to_string())?,
            flags: intermediate_rep.flags.into_iter().next(),
            logged_in_users: intermediate_rep.logged_in_users.into_iter().next(),
            service_id: intermediate_rep.service_id.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ServiceAllOf1> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ServiceAllOf1>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ServiceAllOf1>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ServiceAllOf1 - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ServiceAllOf1> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ServiceAllOf1 as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ServiceAllOf1 - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ServiceAllOf1Flags {
    #[serde(rename = "manuallyConfigured")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub manually_configured: Option<bool>,

    #[serde(rename = "newService")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub new_service: Option<bool>,

}

impl ServiceAllOf1Flags {
    pub fn new() -> ServiceAllOf1Flags {
        ServiceAllOf1Flags {
            manually_configured: None,
            new_service: None,
        }
    }
}

/// Converts the ServiceAllOf1Flags value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ServiceAllOf1Flags {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref manually_configured) = self.manually_configured {
            params.push("manuallyConfigured".to_string());
            params.push(manually_configured.to_string());
        }


        if let Some(ref new_service) = self.new_service {
            params.push("newService".to_string());
            params.push(new_service.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ServiceAllOf1Flags value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ServiceAllOf1Flags {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub manually_configured: Vec<bool>,
            pub new_service: Vec<bool>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ServiceAllOf1Flags".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "manuallyConfigured" => intermediate_rep.manually_configured.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "newService" => intermediate_rep.new_service.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ServiceAllOf1Flags".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ServiceAllOf1Flags {
            manually_configured: intermediate_rep.manually_configured.into_iter().next(),
            new_service: intermediate_rep.new_service.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ServiceAllOf1Flags> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ServiceAllOf1Flags>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ServiceAllOf1Flags>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ServiceAllOf1Flags - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ServiceAllOf1Flags> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ServiceAllOf1Flags as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ServiceAllOf1Flags - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ServiceContent {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "dns")]
    pub dns: String,

    #[serde(rename = "flags")]
    pub flags: models::ServiceContentFlags,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "ports")]
    pub ports: Vec<i32>,

    #[serde(rename = "serviceSpecificConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub service_specific_configuration: Option<serde_json::Value>,

    #[serde(rename = "shutdownAfterTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub shutdown_after_time: Option<usize>,

}

impl ServiceContent {
    pub fn new(dns: String, flags: models::ServiceContentFlags, name: String, ports: Vec<i32>, ) -> ServiceContent {
        ServiceContent {
            description: Some("".to_string()),
            dns: dns,
            flags: flags,
            name: name,
            ports: ports,
            service_specific_configuration: None,
            shutdown_after_time: None,
        }
    }
}

/// Converts the ServiceContent value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ServiceContent {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref description) = self.description {
            params.push("description".to_string());
            params.push(description.to_string());
        }


        params.push("dns".to_string());
        params.push(self.dns.to_string());

        // Skipping flags in query parameter serialization


        params.push("name".to_string());
        params.push(self.name.to_string());


        params.push("ports".to_string());
        params.push(self.ports.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",").to_string());

        // Skipping serviceSpecificConfiguration in query parameter serialization


        if let Some(ref shutdown_after_time) = self.shutdown_after_time {
            params.push("shutdownAfterTime".to_string());
            params.push(shutdown_after_time.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ServiceContent value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ServiceContent {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub description: Vec<String>,
            pub dns: Vec<String>,
            pub flags: Vec<models::ServiceContentFlags>,
            pub name: Vec<String>,
            pub ports: Vec<Vec<i32>>,
            pub service_specific_configuration: Vec<serde_json::Value>,
            pub shutdown_after_time: Vec<usize>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ServiceContent".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "description" => intermediate_rep.description.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "dns" => intermediate_rep.dns.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "flags" => intermediate_rep.flags.push(<models::ServiceContentFlags as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "ports" => return std::result::Result::Err("Parsing a container in this style is not supported in ServiceContent".to_string()),
                    "serviceSpecificConfiguration" => intermediate_rep.service_specific_configuration.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "shutdownAfterTime" => intermediate_rep.shutdown_after_time.push(<usize as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ServiceContent".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ServiceContent {
            description: intermediate_rep.description.into_iter().next(),
            dns: intermediate_rep.dns.into_iter().next().ok_or("dns missing in ServiceContent".to_string())?,
            flags: intermediate_rep.flags.into_iter().next().ok_or("flags missing in ServiceContent".to_string())?,
            name: intermediate_rep.name.into_iter().next().ok_or("name missing in ServiceContent".to_string())?,
            ports: intermediate_rep.ports.into_iter().next().ok_or("ports missing in ServiceContent".to_string())?,
            service_specific_configuration: intermediate_rep.service_specific_configuration.into_iter().next(),
            shutdown_after_time: intermediate_rep.shutdown_after_time.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ServiceContent> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ServiceContent>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ServiceContent>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ServiceContent - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ServiceContent> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ServiceContent as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ServiceContent - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ServiceContentFlags {
    #[serde(rename = "sharedNetwork")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub shared_network: Option<bool>,

}

impl ServiceContentFlags {
    pub fn new() -> ServiceContentFlags {
        ServiceContentFlags {
            shared_network: Some(false),
        }
    }
}

/// Converts the ServiceContentFlags value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ServiceContentFlags {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref shared_network) = self.shared_network {
            params.push("sharedNetwork".to_string());
            params.push(shared_network.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ServiceContentFlags value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ServiceContentFlags {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub shared_network: Vec<bool>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ServiceContentFlags".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "sharedNetwork" => intermediate_rep.shared_network.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ServiceContentFlags".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ServiceContentFlags {
            shared_network: intermediate_rep.shared_network.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ServiceContentFlags> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ServiceContentFlags>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ServiceContentFlags>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ServiceContentFlags - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ServiceContentFlags> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ServiceContentFlags as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ServiceContentFlags - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ServiceModule {
    #[serde(rename = "capabilities")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub capabilities: Option<models::ServiceModuleAllOfCapabilities>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "enabled")]
    pub enabled: bool,

    #[serde(rename = "error")]
    #[serde(deserialize_with = "swagger::nullable_format::deserialize_optional_nullable")]
    #[serde(default = "swagger::nullable_format::default_optional_nullable")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error: Option<swagger::Nullable<models::Error>>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,

    #[serde(rename = "uuid")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub uuid: Option<uuid::Uuid>,

    #[serde(rename = "schemas")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schemas: Option<models::ServiceModuleAllOfSchemas>,

    #[serde(rename = "serviceModuleConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub service_module_configuration: Option<serde_json::Value>,

}

impl ServiceModule {
    pub fn new(enabled: bool, ) -> ServiceModule {
        ServiceModule {
            capabilities: None,
            description: None,
            enabled: enabled,
            error: None,
            name: None,
            state: None,
            uuid: None,
            schemas: None,
            service_module_configuration: None,
        }
    }
}

/// Converts the ServiceModule value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ServiceModule {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping capabilities in query parameter serialization


        if let Some(ref description) = self.description {
            params.push("description".to_string());
            params.push(description.to_string());
        }


        params.push("enabled".to_string());
        params.push(self.enabled.to_string());

        // Skipping error in query parameter serialization


        if let Some(ref name) = self.name {
            params.push("name".to_string());
            params.push(name.to_string());
        }


        if let Some(ref state) = self.state {
            params.push("state".to_string());
            params.push(state.to_string());
        }

        // Skipping uuid in query parameter serialization

        // Skipping schemas in query parameter serialization

        // Skipping serviceModuleConfiguration in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ServiceModule value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ServiceModule {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub capabilities: Vec<models::ServiceModuleAllOfCapabilities>,
            pub description: Vec<String>,
            pub enabled: Vec<bool>,
            pub error: Vec<models::Error>,
            pub name: Vec<String>,
            pub state: Vec<String>,
            pub uuid: Vec<uuid::Uuid>,
            pub schemas: Vec<models::ServiceModuleAllOfSchemas>,
            pub service_module_configuration: Vec<serde_json::Value>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ServiceModule".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "capabilities" => intermediate_rep.capabilities.push(<models::ServiceModuleAllOfCapabilities as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "description" => intermediate_rep.description.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "enabled" => intermediate_rep.enabled.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "error" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in ServiceModule".to_string()),
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "state" => intermediate_rep.state.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "uuid" => intermediate_rep.uuid.push(<uuid::Uuid as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "schemas" => intermediate_rep.schemas.push(<models::ServiceModuleAllOfSchemas as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "serviceModuleConfiguration" => intermediate_rep.service_module_configuration.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ServiceModule".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ServiceModule {
            capabilities: intermediate_rep.capabilities.into_iter().next(),
            description: intermediate_rep.description.into_iter().next(),
            enabled: intermediate_rep.enabled.into_iter().next().ok_or("enabled missing in ServiceModule".to_string())?,
            error: std::result::Result::Err("Nullable types not supported in ServiceModule".to_string())?,
            name: intermediate_rep.name.into_iter().next(),
            state: intermediate_rep.state.into_iter().next(),
            uuid: intermediate_rep.uuid.into_iter().next(),
            schemas: intermediate_rep.schemas.into_iter().next(),
            service_module_configuration: intermediate_rep.service_module_configuration.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ServiceModule> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ServiceModule>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ServiceModule>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ServiceModule - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ServiceModule> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ServiceModule as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ServiceModule - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ServiceModuleAllOf {
    #[serde(rename = "capabilities")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub capabilities: Option<models::ServiceModuleAllOfCapabilities>,

    #[serde(rename = "schemas")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schemas: Option<models::ServiceModuleAllOfSchemas>,

    #[serde(rename = "serviceModuleConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub service_module_configuration: Option<serde_json::Value>,

}

impl ServiceModuleAllOf {
    pub fn new() -> ServiceModuleAllOf {
        ServiceModuleAllOf {
            capabilities: None,
            schemas: None,
            service_module_configuration: None,
        }
    }
}

/// Converts the ServiceModuleAllOf value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ServiceModuleAllOf {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping capabilities in query parameter serialization

        // Skipping schemas in query parameter serialization

        // Skipping serviceModuleConfiguration in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ServiceModuleAllOf value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ServiceModuleAllOf {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub capabilities: Vec<models::ServiceModuleAllOfCapabilities>,
            pub schemas: Vec<models::ServiceModuleAllOfSchemas>,
            pub service_module_configuration: Vec<serde_json::Value>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ServiceModuleAllOf".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "capabilities" => intermediate_rep.capabilities.push(<models::ServiceModuleAllOfCapabilities as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "schemas" => intermediate_rep.schemas.push(<models::ServiceModuleAllOfSchemas as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "serviceModuleConfiguration" => intermediate_rep.service_module_configuration.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ServiceModuleAllOf".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ServiceModuleAllOf {
            capabilities: intermediate_rep.capabilities.into_iter().next(),
            schemas: intermediate_rep.schemas.into_iter().next(),
            service_module_configuration: intermediate_rep.service_module_configuration.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ServiceModuleAllOf> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ServiceModuleAllOf>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ServiceModuleAllOf>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ServiceModuleAllOf - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ServiceModuleAllOf> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ServiceModuleAllOf as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ServiceModuleAllOf - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ServiceModuleAllOfCapabilities {
    #[serde(rename = "canBeUsedAsTrigger")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub can_be_used_as_trigger: Option<bool>,

    #[serde(rename = "canConfigureNewService")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub can_configure_new_service: Option<bool>,

    #[serde(rename = "canInformUserCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub can_inform_user_count: Option<bool>,

    #[serde(rename = "canReconfigureService")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub can_reconfigure_service: Option<bool>,

    #[serde(rename = "supportMultipleServers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub support_multiple_servers: Option<bool>,

    #[serde(rename = "supportServerAmountChanges")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub support_server_amount_changes: Option<bool>,

}

impl ServiceModuleAllOfCapabilities {
    pub fn new() -> ServiceModuleAllOfCapabilities {
        ServiceModuleAllOfCapabilities {
            can_be_used_as_trigger: None,
            can_configure_new_service: None,
            can_inform_user_count: None,
            can_reconfigure_service: None,
            support_multiple_servers: None,
            support_server_amount_changes: None,
        }
    }
}

/// Converts the ServiceModuleAllOfCapabilities value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ServiceModuleAllOfCapabilities {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref can_be_used_as_trigger) = self.can_be_used_as_trigger {
            params.push("canBeUsedAsTrigger".to_string());
            params.push(can_be_used_as_trigger.to_string());
        }


        if let Some(ref can_configure_new_service) = self.can_configure_new_service {
            params.push("canConfigureNewService".to_string());
            params.push(can_configure_new_service.to_string());
        }


        if let Some(ref can_inform_user_count) = self.can_inform_user_count {
            params.push("canInformUserCount".to_string());
            params.push(can_inform_user_count.to_string());
        }


        if let Some(ref can_reconfigure_service) = self.can_reconfigure_service {
            params.push("canReconfigureService".to_string());
            params.push(can_reconfigure_service.to_string());
        }


        if let Some(ref support_multiple_servers) = self.support_multiple_servers {
            params.push("supportMultipleServers".to_string());
            params.push(support_multiple_servers.to_string());
        }


        if let Some(ref support_server_amount_changes) = self.support_server_amount_changes {
            params.push("supportServerAmountChanges".to_string());
            params.push(support_server_amount_changes.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ServiceModuleAllOfCapabilities value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ServiceModuleAllOfCapabilities {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub can_be_used_as_trigger: Vec<bool>,
            pub can_configure_new_service: Vec<bool>,
            pub can_inform_user_count: Vec<bool>,
            pub can_reconfigure_service: Vec<bool>,
            pub support_multiple_servers: Vec<bool>,
            pub support_server_amount_changes: Vec<bool>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ServiceModuleAllOfCapabilities".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "canBeUsedAsTrigger" => intermediate_rep.can_be_used_as_trigger.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "canConfigureNewService" => intermediate_rep.can_configure_new_service.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "canInformUserCount" => intermediate_rep.can_inform_user_count.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "canReconfigureService" => intermediate_rep.can_reconfigure_service.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "supportMultipleServers" => intermediate_rep.support_multiple_servers.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "supportServerAmountChanges" => intermediate_rep.support_server_amount_changes.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ServiceModuleAllOfCapabilities".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ServiceModuleAllOfCapabilities {
            can_be_used_as_trigger: intermediate_rep.can_be_used_as_trigger.into_iter().next(),
            can_configure_new_service: intermediate_rep.can_configure_new_service.into_iter().next(),
            can_inform_user_count: intermediate_rep.can_inform_user_count.into_iter().next(),
            can_reconfigure_service: intermediate_rep.can_reconfigure_service.into_iter().next(),
            support_multiple_servers: intermediate_rep.support_multiple_servers.into_iter().next(),
            support_server_amount_changes: intermediate_rep.support_server_amount_changes.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ServiceModuleAllOfCapabilities> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ServiceModuleAllOfCapabilities>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ServiceModuleAllOfCapabilities>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ServiceModuleAllOfCapabilities - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ServiceModuleAllOfCapabilities> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ServiceModuleAllOfCapabilities as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ServiceModuleAllOfCapabilities - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ServiceModuleAllOfSchemas {
    #[serde(rename = "serviceModuleConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub service_module_configuration: Option<serde_json::Value>,

    #[serde(rename = "serviceSpecifigConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub service_specifig_configuration: Option<serde_json::Value>,

}

impl ServiceModuleAllOfSchemas {
    pub fn new() -> ServiceModuleAllOfSchemas {
        ServiceModuleAllOfSchemas {
            service_module_configuration: None,
            service_specifig_configuration: None,
        }
    }
}

/// Converts the ServiceModuleAllOfSchemas value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ServiceModuleAllOfSchemas {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping serviceModuleConfiguration in query parameter serialization

        // Skipping serviceSpecifigConfiguration in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ServiceModuleAllOfSchemas value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ServiceModuleAllOfSchemas {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub service_module_configuration: Vec<serde_json::Value>,
            pub service_specifig_configuration: Vec<serde_json::Value>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ServiceModuleAllOfSchemas".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "serviceModuleConfiguration" => intermediate_rep.service_module_configuration.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "serviceSpecifigConfiguration" => intermediate_rep.service_specifig_configuration.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ServiceModuleAllOfSchemas".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ServiceModuleAllOfSchemas {
            service_module_configuration: intermediate_rep.service_module_configuration.into_iter().next(),
            service_specifig_configuration: intermediate_rep.service_specifig_configuration.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ServiceModuleAllOfSchemas> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ServiceModuleAllOfSchemas>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ServiceModuleAllOfSchemas>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ServiceModuleAllOfSchemas - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ServiceModuleAllOfSchemas> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ServiceModuleAllOfSchemas as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ServiceModuleAllOfSchemas - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ServiceModules(
    Vec<ServiceModule>
);

impl std::convert::From<Vec<ServiceModule>> for ServiceModules {
    fn from(x: Vec<ServiceModule>) -> Self {
        ServiceModules(x)
    }
}

impl std::convert::From<ServiceModules> for Vec<ServiceModule> {
    fn from(x: ServiceModules) -> Self {
        x.0
    }
}

impl std::iter::FromIterator<ServiceModule> for ServiceModules {
    fn from_iter<U: IntoIterator<Item=ServiceModule>>(u: U) -> Self {
        ServiceModules(Vec::<ServiceModule>::from_iter(u))
    }
}

impl std::iter::IntoIterator for ServiceModules {
    type Item = ServiceModule;
    type IntoIter = std::vec::IntoIter<ServiceModule>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> std::iter::IntoIterator for &'a ServiceModules {
    type Item = &'a ServiceModule;
    type IntoIter = std::slice::Iter<'a, ServiceModule>;

    fn into_iter(self) -> Self::IntoIter {
        (&self.0).into_iter()
    }
}

impl<'a> std::iter::IntoIterator for &'a mut ServiceModules {
    type Item = &'a mut ServiceModule;
    type IntoIter = std::slice::IterMut<'a, ServiceModule>;

    fn into_iter(self) -> Self::IntoIter {
        (&mut self.0).into_iter()
    }
}

impl std::ops::Deref for ServiceModules {
    type Target = Vec<ServiceModule>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ServiceModules {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Converts the ServiceModules value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ServiceModules {
    fn to_string(&self) -> String {
        self.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ServiceModules value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ServiceModules {
    type Err = <ServiceModule as std::str::FromStr>::Err;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut items = vec![];
        for item in s.split(',')
        {
            items.push(item.parse()?);
        }
        std::result::Result::Ok(ServiceModules(items))
    }
}


// Methods for converting between header::IntoHeaderValue<ServiceModules> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ServiceModules>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ServiceModules>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ServiceModules - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ServiceModules> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ServiceModules as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ServiceModules - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ServiceStates {
}

impl ServiceStates {
    pub fn new() -> ServiceStates {
        ServiceStates {
        }
    }
}

/// Converts the ServiceStates value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ServiceStates {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ServiceStates value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ServiceStates {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ServiceStates".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    _ => return std::result::Result::Err("Unexpected key while parsing ServiceStates".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ServiceStates {
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ServiceStates> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ServiceStates>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ServiceStates>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ServiceStates - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ServiceStates> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ServiceStates as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ServiceStates - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum ServiceStatesAllOf {
    #[serde(rename = "server starting")]
    SERVER_STARTING,
    #[serde(rename = "dns configuring")]
    DNS_CONFIGURING,
    #[serde(rename = "service configuring")]
    SERVICE_CONFIGURING,
    #[serde(rename = "service stopping")]
    SERVICE_STOPPING,
    #[serde(rename = "server stopping")]
    SERVER_STOPPING,
    #[serde(rename = "server reconfiguring")]
    SERVER_RECONFIGURING,
    #[serde(rename = "terminating server")]
    TERMINATING_SERVER,
}

impl std::fmt::Display for ServiceStatesAllOf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ServiceStatesAllOf::SERVER_STARTING => write!(f, "{}", "server starting"),
            ServiceStatesAllOf::DNS_CONFIGURING => write!(f, "{}", "dns configuring"),
            ServiceStatesAllOf::SERVICE_CONFIGURING => write!(f, "{}", "service configuring"),
            ServiceStatesAllOf::SERVICE_STOPPING => write!(f, "{}", "service stopping"),
            ServiceStatesAllOf::SERVER_STOPPING => write!(f, "{}", "server stopping"),
            ServiceStatesAllOf::SERVER_RECONFIGURING => write!(f, "{}", "server reconfiguring"),
            ServiceStatesAllOf::TERMINATING_SERVER => write!(f, "{}", "terminating server"),
        }
    }
}

impl std::str::FromStr for ServiceStatesAllOf {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "server starting" => std::result::Result::Ok(ServiceStatesAllOf::SERVER_STARTING),
            "dns configuring" => std::result::Result::Ok(ServiceStatesAllOf::DNS_CONFIGURING),
            "service configuring" => std::result::Result::Ok(ServiceStatesAllOf::SERVICE_CONFIGURING),
            "service stopping" => std::result::Result::Ok(ServiceStatesAllOf::SERVICE_STOPPING),
            "server stopping" => std::result::Result::Ok(ServiceStatesAllOf::SERVER_STOPPING),
            "server reconfiguring" => std::result::Result::Ok(ServiceStatesAllOf::SERVER_RECONFIGURING),
            "terminating server" => std::result::Result::Ok(ServiceStatesAllOf::TERMINATING_SERVER),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Services(
    Vec<serde_json::Value>
);

impl std::convert::From<Vec<serde_json::Value>> for Services {
    fn from(x: Vec<serde_json::Value>) -> Self {
        Services(x)
    }
}

impl std::convert::From<Services> for Vec<serde_json::Value> {
    fn from(x: Services) -> Self {
        x.0
    }
}

impl std::iter::FromIterator<serde_json::Value> for Services {
    fn from_iter<U: IntoIterator<Item=serde_json::Value>>(u: U) -> Self {
        Services(Vec::<serde_json::Value>::from_iter(u))
    }
}

impl std::iter::IntoIterator for Services {
    type Item = serde_json::Value;
    type IntoIter = std::vec::IntoIter<serde_json::Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> std::iter::IntoIterator for &'a Services {
    type Item = &'a serde_json::Value;
    type IntoIter = std::slice::Iter<'a, serde_json::Value>;

    fn into_iter(self) -> Self::IntoIter {
        (&self.0).into_iter()
    }
}

impl<'a> std::iter::IntoIterator for &'a mut Services {
    type Item = &'a mut serde_json::Value;
    type IntoIter = std::slice::IterMut<'a, serde_json::Value>;

    fn into_iter(self) -> Self::IntoIter {
        (&mut self.0).into_iter()
    }
}

impl std::ops::Deref for Services {
    type Target = Vec<serde_json::Value>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Services {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Converts the Services value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Services {
    fn to_string(&self) -> String {
        self.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Services value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Services {
    type Err = <serde_json::Value as std::str::FromStr>::Err;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut items = vec![];
        for item in s.split(',')
        {
            items.push(item.parse()?);
        }
        std::result::Result::Ok(Services(items))
    }
}


// Methods for converting between header::IntoHeaderValue<Services> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Services>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Services>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Services - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Services> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Services as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Services - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct User {
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_id: Option<usize>,

    #[serde(rename = "administrator")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub administrator: Option<bool>,

    #[serde(rename = "email")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<String>,

    #[serde(rename = "twoFactorSecret")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub two_factor_secret: Option<String>,

    #[serde(rename = "password")]
    pub password: String,

    #[serde(rename = "username")]
    pub username: String,

}

impl User {
    pub fn new(password: String, username: String, ) -> User {
        User {
            user_id: None,
            administrator: Some(false),
            email: None,
            two_factor_secret: Some("".to_string()),
            password: password,
            username: username,
        }
    }
}

/// Converts the User value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for User {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref user_id) = self.user_id {
            params.push("userId".to_string());
            params.push(user_id.to_string());
        }


        if let Some(ref administrator) = self.administrator {
            params.push("administrator".to_string());
            params.push(administrator.to_string());
        }


        if let Some(ref email) = self.email {
            params.push("email".to_string());
            params.push(email.to_string());
        }


        if let Some(ref two_factor_secret) = self.two_factor_secret {
            params.push("twoFactorSecret".to_string());
            params.push(two_factor_secret.to_string());
        }


        params.push("password".to_string());
        params.push(self.password.to_string());


        params.push("username".to_string());
        params.push(self.username.to_string());

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a User value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for User {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub user_id: Vec<usize>,
            pub administrator: Vec<bool>,
            pub email: Vec<String>,
            pub two_factor_secret: Vec<String>,
            pub password: Vec<String>,
            pub username: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing User".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "userId" => intermediate_rep.user_id.push(<usize as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "administrator" => intermediate_rep.administrator.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "email" => intermediate_rep.email.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "twoFactorSecret" => intermediate_rep.two_factor_secret.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "password" => intermediate_rep.password.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "username" => intermediate_rep.username.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing User".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(User {
            user_id: intermediate_rep.user_id.into_iter().next(),
            administrator: intermediate_rep.administrator.into_iter().next(),
            email: intermediate_rep.email.into_iter().next(),
            two_factor_secret: intermediate_rep.two_factor_secret.into_iter().next(),
            password: intermediate_rep.password.into_iter().next().ok_or("password missing in User".to_string())?,
            username: intermediate_rep.username.into_iter().next().ok_or("username missing in User".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<User> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<User>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<User>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for User - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<User> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <User as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into User - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UserAllOf {
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_id: Option<usize>,

}

impl UserAllOf {
    pub fn new() -> UserAllOf {
        UserAllOf {
            user_id: None,
        }
    }
}

/// Converts the UserAllOf value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for UserAllOf {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref user_id) = self.user_id {
            params.push("userId".to_string());
            params.push(user_id.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UserAllOf value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UserAllOf {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub user_id: Vec<usize>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing UserAllOf".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "userId" => intermediate_rep.user_id.push(<usize as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing UserAllOf".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UserAllOf {
            user_id: intermediate_rep.user_id.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UserAllOf> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<UserAllOf>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<UserAllOf>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for UserAllOf - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<UserAllOf> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <UserAllOf as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into UserAllOf - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UserContent {
    #[serde(rename = "administrator")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub administrator: Option<bool>,

    #[serde(rename = "email")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<String>,

    #[serde(rename = "twoFactorSecret")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub two_factor_secret: Option<String>,

    #[serde(rename = "password")]
    pub password: String,

    #[serde(rename = "username")]
    pub username: String,

}

impl UserContent {
    pub fn new(password: String, username: String, ) -> UserContent {
        UserContent {
            administrator: Some(false),
            email: None,
            two_factor_secret: Some("".to_string()),
            password: password,
            username: username,
        }
    }
}

/// Converts the UserContent value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for UserContent {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref administrator) = self.administrator {
            params.push("administrator".to_string());
            params.push(administrator.to_string());
        }


        if let Some(ref email) = self.email {
            params.push("email".to_string());
            params.push(email.to_string());
        }


        if let Some(ref two_factor_secret) = self.two_factor_secret {
            params.push("twoFactorSecret".to_string());
            params.push(two_factor_secret.to_string());
        }


        params.push("password".to_string());
        params.push(self.password.to_string());


        params.push("username".to_string());
        params.push(self.username.to_string());

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UserContent value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UserContent {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub administrator: Vec<bool>,
            pub email: Vec<String>,
            pub two_factor_secret: Vec<String>,
            pub password: Vec<String>,
            pub username: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing UserContent".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "administrator" => intermediate_rep.administrator.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "email" => intermediate_rep.email.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "twoFactorSecret" => intermediate_rep.two_factor_secret.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "password" => intermediate_rep.password.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "username" => intermediate_rep.username.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing UserContent".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UserContent {
            administrator: intermediate_rep.administrator.into_iter().next(),
            email: intermediate_rep.email.into_iter().next(),
            two_factor_secret: intermediate_rep.two_factor_secret.into_iter().next(),
            password: intermediate_rep.password.into_iter().next().ok_or("password missing in UserContent".to_string())?,
            username: intermediate_rep.username.into_iter().next().ok_or("username missing in UserContent".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UserContent> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<UserContent>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<UserContent>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for UserContent - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<UserContent> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <UserContent as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into UserContent - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UserContentAllOf {
    #[serde(rename = "administrator")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub administrator: Option<bool>,

    #[serde(rename = "email")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<String>,

}

impl UserContentAllOf {
    pub fn new() -> UserContentAllOf {
        UserContentAllOf {
            administrator: Some(false),
            email: None,
        }
    }
}

/// Converts the UserContentAllOf value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for UserContentAllOf {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref administrator) = self.administrator {
            params.push("administrator".to_string());
            params.push(administrator.to_string());
        }


        if let Some(ref email) = self.email {
            params.push("email".to_string());
            params.push(email.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UserContentAllOf value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UserContentAllOf {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub administrator: Vec<bool>,
            pub email: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing UserContentAllOf".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "administrator" => intermediate_rep.administrator.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "email" => intermediate_rep.email.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing UserContentAllOf".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UserContentAllOf {
            administrator: intermediate_rep.administrator.into_iter().next(),
            email: intermediate_rep.email.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UserContentAllOf> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<UserContentAllOf>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<UserContentAllOf>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for UserContentAllOf - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<UserContentAllOf> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <UserContentAllOf as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into UserContentAllOf - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UserCredentials {
    #[serde(rename = "password")]
    pub password: String,

    #[serde(rename = "username")]
    pub username: String,

}

impl UserCredentials {
    pub fn new(password: String, username: String, ) -> UserCredentials {
        UserCredentials {
            password: password,
            username: username,
        }
    }
}

/// Converts the UserCredentials value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for UserCredentials {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("password".to_string());
        params.push(self.password.to_string());


        params.push("username".to_string());
        params.push(self.username.to_string());

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UserCredentials value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UserCredentials {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub password: Vec<String>,
            pub username: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing UserCredentials".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "password" => intermediate_rep.password.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "username" => intermediate_rep.username.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing UserCredentials".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UserCredentials {
            password: intermediate_rep.password.into_iter().next().ok_or("password missing in UserCredentials".to_string())?,
            username: intermediate_rep.username.into_iter().next().ok_or("username missing in UserCredentials".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UserCredentials> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<UserCredentials>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<UserCredentials>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for UserCredentials - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<UserCredentials> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <UserCredentials as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into UserCredentials - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Users(
    Vec<User>
);

impl std::convert::From<Vec<User>> for Users {
    fn from(x: Vec<User>) -> Self {
        Users(x)
    }
}

impl std::convert::From<Users> for Vec<User> {
    fn from(x: Users) -> Self {
        x.0
    }
}

impl std::iter::FromIterator<User> for Users {
    fn from_iter<U: IntoIterator<Item=User>>(u: U) -> Self {
        Users(Vec::<User>::from_iter(u))
    }
}

impl std::iter::IntoIterator for Users {
    type Item = User;
    type IntoIter = std::vec::IntoIter<User>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> std::iter::IntoIterator for &'a Users {
    type Item = &'a User;
    type IntoIter = std::slice::Iter<'a, User>;

    fn into_iter(self) -> Self::IntoIter {
        (&self.0).into_iter()
    }
}

impl<'a> std::iter::IntoIterator for &'a mut Users {
    type Item = &'a mut User;
    type IntoIter = std::slice::IterMut<'a, User>;

    fn into_iter(self) -> Self::IntoIter {
        (&mut self.0).into_iter()
    }
}

impl std::ops::Deref for Users {
    type Target = Vec<User>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Users {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Converts the Users value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Users {
    fn to_string(&self) -> String {
        self.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Users value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Users {
    type Err = <User as std::str::FromStr>::Err;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut items = vec![];
        for item in s.split(',')
        {
            items.push(item.parse()?);
        }
        std::result::Result::Ok(Users(items))
    }
}


// Methods for converting between header::IntoHeaderValue<Users> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Users>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Users>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Users - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Users> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Users as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Users - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}

