use futures::{future, future::BoxFuture, Stream, stream, future::FutureExt, stream::TryStreamExt};
use hyper::{Request, Response, StatusCode, Body, HeaderMap};
use hyper::header::{HeaderName, HeaderValue, CONTENT_TYPE};
use log::warn;
#[allow(unused_imports)]
use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::future::Future;
use std::marker::PhantomData;
use std::task::{Context, Poll};
use swagger::{ApiError, BodyExt, Has, RequestParser, XSpanIdString};
pub use swagger::auth::Authorization;
use swagger::auth::Scopes;
use url::form_urlencoded;

#[allow(unused_imports)]
use crate::models;
use crate::header;

pub use crate::context;

type ServiceFuture = BoxFuture<'static, Result<Response<Body>, crate::ServiceError>>;

use crate::{Api,
     DeleteCloudModuleResponse,
     DeleteCloudModuleAccountResponse,
     GetCloudModuleResponse,
     GetCloudModuleAccountResponse,
     GetCloudModuleAccountPermissionsResponse,
     GetCloudModuleAccountsResponse,
     GetCloudModulePermissionsResponse,
     GetCloudModulesResponse,
     PostCloudModuleResponse,
     PostCloudModuleAccountsResponse,
     PostCloudModulesResponse,
     PutCloudModuleAccountResponse,
     PutCloudModuleAccountPermissionsResponse,
     PutCloudModulePermissionsResponse,
     GetCpuCoresResponse,
     GetDiskBoundsResponse,
     GetMachineTypesResponse,
     GetOperatingSystemsResponse,
     GetOrientationsResponse,
     GetRamBoundsResponse,
     DeleteGroupResponse,
     GetGroupResponse,
     GetGroupsResponse,
     PostGroupResponse,
     PutGroupResponse,
     GetConsoleManagementResponse,
     PostConsoleManagementResponse,
     GetServiceServerResponse,
     GetServiceServerPermissionsResponse,
     GetServiceServerServerCredentialsResponse,
     GetServiceServerServerCredentialsPermissionsResponse,
     GetServiceServersResponse,
     PostServiceServerValidateResponse,
     PostServiceServersResponse,
     PutServiceServerResponse,
     PutServiceServerPermissionsResponse,
     PutServiceServerServerCredentialsResponse,
     PutServiceServerServerCredentialsPermissionsResponse,
     DeleteServiceModuleResponse,
     GetServiceModuleResponse,
     GetServiceModulePermissionsResponse,
     GetServiceModulesResponse,
     PostServiceModuleResponse,
     PostServiceModulesResponse,
     PutServiceModulePermissionsResponse,
     CreateServiceResponse,
     DeleteServiceResponse,
     GetServiceResponse,
     GetServicePermissionsResponse,
     GetServicesResponse,
     GetServicesPermissionsResponse,
     PostServiceResponse,
     PutServicePermissionsResponse,
     PutServicesPermissionsResponse,
     DeleteUserResponse,
     GetUserResponse,
     GetUsersResponse,
     PostUserResponse,
     PostUserLoginResponse,
     PutUserResponse
};

mod paths {
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref GLOBAL_REGEX_SET: regex::RegexSet = regex::RegexSet::new(vec![
            r"^/v1/consoleManagement$",
            r"^/v1/groups$",
            r"^/v1/groups/(?P<groupId>[^/?#]*)$",
            r"^/v1/modules/cloudModules$",
            r"^/v1/modules/cloudModules/(?P<cloudModuleUuid>[^/?#]*)$",
            r"^/v1/modules/cloudModules/(?P<cloudModuleUuid>[^/?#]*)/accounts$",
            r"^/v1/modules/cloudModules/(?P<cloudModuleUuid>[^/?#]*)/accounts/(?P<cloudAccountId>[^/?#]*)$",
            r"^/v1/modules/cloudModules/(?P<cloudModuleUuid>[^/?#]*)/accounts/(?P<cloudAccountId>[^/?#]*)/permissions$",
            r"^/v1/modules/cloudModules/(?P<cloudModuleUuid>[^/?#]*)/permissions$",
            r"^/v1/modules/cloudModules/(?P<cloudModuleUuid>[^/?#]*)/queries/cpuCores$",
            r"^/v1/modules/cloudModules/(?P<cloudModuleUuid>[^/?#]*)/queries/diskBounds$",
            r"^/v1/modules/cloudModules/(?P<cloudModuleUuid>[^/?#]*)/queries/machineTypes$",
            r"^/v1/modules/cloudModules/(?P<cloudModuleUuid>[^/?#]*)/queries/operatingSystems$",
            r"^/v1/modules/cloudModules/(?P<cloudModuleUuid>[^/?#]*)/queries/orientations$",
            r"^/v1/modules/cloudModules/(?P<cloudModuleUuid>[^/?#]*)/queries/ramBounds$",
            r"^/v1/modules/serviceModules$",
            r"^/v1/modules/serviceModules/(?P<serviceModuleUuid>[^/?#]*)$",
            r"^/v1/modules/serviceModules/(?P<serviceModuleUuid>[^/?#]*)/permissions$",
            r"^/v1/services$",
            r"^/v1/services/permissions$",
            r"^/v1/services/(?P<serviceId>[^/?#]*)$",
            r"^/v1/services/(?P<serviceId>[^/?#]*)/permissions$",
            r"^/v1/services/(?P<serviceId>[^/?#]*)/servers$",
            r"^/v1/services/(?P<serviceId>[^/?#]*)/servers/(?P<serverId>[^/?#]*)$",
            r"^/v1/services/(?P<serviceId>[^/?#]*)/servers/(?P<serverId>[^/?#]*)/permissions$",
            r"^/v1/services/(?P<serviceId>[^/?#]*)/servers/(?P<serverId>[^/?#]*)/serverCredentials$",
            r"^/v1/services/(?P<serviceId>[^/?#]*)/servers/(?P<serverId>[^/?#]*)/serverCredentials/permissions$",
            r"^/v1/services/(?P<serviceId>[^/?#]*)/servers/(?P<serverId>[^/?#]*)/validate$",
            r"^/v1/users$",
            r"^/v1/users/login$",
            r"^/v1/users/(?P<userId>[^/?#]*)$"
        ])
        .expect("Unable to create global regex set");
    }
    pub(crate) static ID_CONSOLEMANAGEMENT: usize = 0;
    pub(crate) static ID_GROUPS: usize = 1;
    pub(crate) static ID_GROUPS_GROUPID: usize = 2;
    lazy_static! {
        pub static ref REGEX_GROUPS_GROUPID: regex::Regex =
            regex::Regex::new(r"^/v1/groups/(?P<groupId>[^/?#]*)$")
                .expect("Unable to create regex for GROUPS_GROUPID");
    }
    pub(crate) static ID_MODULES_CLOUDMODULES: usize = 3;
    pub(crate) static ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID: usize = 4;
    lazy_static! {
        pub static ref REGEX_MODULES_CLOUDMODULES_CLOUDMODULEUUID: regex::Regex =
            regex::Regex::new(r"^/v1/modules/cloudModules/(?P<cloudModuleUuid>[^/?#]*)$")
                .expect("Unable to create regex for MODULES_CLOUDMODULES_CLOUDMODULEUUID");
    }
    pub(crate) static ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID_ACCOUNTS: usize = 5;
    lazy_static! {
        pub static ref REGEX_MODULES_CLOUDMODULES_CLOUDMODULEUUID_ACCOUNTS: regex::Regex =
            regex::Regex::new(r"^/v1/modules/cloudModules/(?P<cloudModuleUuid>[^/?#]*)/accounts$")
                .expect("Unable to create regex for MODULES_CLOUDMODULES_CLOUDMODULEUUID_ACCOUNTS");
    }
    pub(crate) static ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID_ACCOUNTS_CLOUDACCOUNTID: usize = 6;
    lazy_static! {
        pub static ref REGEX_MODULES_CLOUDMODULES_CLOUDMODULEUUID_ACCOUNTS_CLOUDACCOUNTID: regex::Regex =
            regex::Regex::new(r"^/v1/modules/cloudModules/(?P<cloudModuleUuid>[^/?#]*)/accounts/(?P<cloudAccountId>[^/?#]*)$")
                .expect("Unable to create regex for MODULES_CLOUDMODULES_CLOUDMODULEUUID_ACCOUNTS_CLOUDACCOUNTID");
    }
    pub(crate) static ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID_ACCOUNTS_CLOUDACCOUNTID_PERMISSIONS: usize = 7;
    lazy_static! {
        pub static ref REGEX_MODULES_CLOUDMODULES_CLOUDMODULEUUID_ACCOUNTS_CLOUDACCOUNTID_PERMISSIONS: regex::Regex =
            regex::Regex::new(r"^/v1/modules/cloudModules/(?P<cloudModuleUuid>[^/?#]*)/accounts/(?P<cloudAccountId>[^/?#]*)/permissions$")
                .expect("Unable to create regex for MODULES_CLOUDMODULES_CLOUDMODULEUUID_ACCOUNTS_CLOUDACCOUNTID_PERMISSIONS");
    }
    pub(crate) static ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID_PERMISSIONS: usize = 8;
    lazy_static! {
        pub static ref REGEX_MODULES_CLOUDMODULES_CLOUDMODULEUUID_PERMISSIONS: regex::Regex =
            regex::Regex::new(r"^/v1/modules/cloudModules/(?P<cloudModuleUuid>[^/?#]*)/permissions$")
                .expect("Unable to create regex for MODULES_CLOUDMODULES_CLOUDMODULEUUID_PERMISSIONS");
    }
    pub(crate) static ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID_QUERIES_CPUCORES: usize = 9;
    lazy_static! {
        pub static ref REGEX_MODULES_CLOUDMODULES_CLOUDMODULEUUID_QUERIES_CPUCORES: regex::Regex =
            regex::Regex::new(r"^/v1/modules/cloudModules/(?P<cloudModuleUuid>[^/?#]*)/queries/cpuCores$")
                .expect("Unable to create regex for MODULES_CLOUDMODULES_CLOUDMODULEUUID_QUERIES_CPUCORES");
    }
    pub(crate) static ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID_QUERIES_DISKBOUNDS: usize = 10;
    lazy_static! {
        pub static ref REGEX_MODULES_CLOUDMODULES_CLOUDMODULEUUID_QUERIES_DISKBOUNDS: regex::Regex =
            regex::Regex::new(r"^/v1/modules/cloudModules/(?P<cloudModuleUuid>[^/?#]*)/queries/diskBounds$")
                .expect("Unable to create regex for MODULES_CLOUDMODULES_CLOUDMODULEUUID_QUERIES_DISKBOUNDS");
    }
    pub(crate) static ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID_QUERIES_MACHINETYPES: usize = 11;
    lazy_static! {
        pub static ref REGEX_MODULES_CLOUDMODULES_CLOUDMODULEUUID_QUERIES_MACHINETYPES: regex::Regex =
            regex::Regex::new(r"^/v1/modules/cloudModules/(?P<cloudModuleUuid>[^/?#]*)/queries/machineTypes$")
                .expect("Unable to create regex for MODULES_CLOUDMODULES_CLOUDMODULEUUID_QUERIES_MACHINETYPES");
    }
    pub(crate) static ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID_QUERIES_OPERATINGSYSTEMS: usize = 12;
    lazy_static! {
        pub static ref REGEX_MODULES_CLOUDMODULES_CLOUDMODULEUUID_QUERIES_OPERATINGSYSTEMS: regex::Regex =
            regex::Regex::new(r"^/v1/modules/cloudModules/(?P<cloudModuleUuid>[^/?#]*)/queries/operatingSystems$")
                .expect("Unable to create regex for MODULES_CLOUDMODULES_CLOUDMODULEUUID_QUERIES_OPERATINGSYSTEMS");
    }
    pub(crate) static ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID_QUERIES_ORIENTATIONS: usize = 13;
    lazy_static! {
        pub static ref REGEX_MODULES_CLOUDMODULES_CLOUDMODULEUUID_QUERIES_ORIENTATIONS: regex::Regex =
            regex::Regex::new(r"^/v1/modules/cloudModules/(?P<cloudModuleUuid>[^/?#]*)/queries/orientations$")
                .expect("Unable to create regex for MODULES_CLOUDMODULES_CLOUDMODULEUUID_QUERIES_ORIENTATIONS");
    }
    pub(crate) static ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID_QUERIES_RAMBOUNDS: usize = 14;
    lazy_static! {
        pub static ref REGEX_MODULES_CLOUDMODULES_CLOUDMODULEUUID_QUERIES_RAMBOUNDS: regex::Regex =
            regex::Regex::new(r"^/v1/modules/cloudModules/(?P<cloudModuleUuid>[^/?#]*)/queries/ramBounds$")
                .expect("Unable to create regex for MODULES_CLOUDMODULES_CLOUDMODULEUUID_QUERIES_RAMBOUNDS");
    }
    pub(crate) static ID_MODULES_SERVICEMODULES: usize = 15;
    pub(crate) static ID_MODULES_SERVICEMODULES_SERVICEMODULEUUID: usize = 16;
    lazy_static! {
        pub static ref REGEX_MODULES_SERVICEMODULES_SERVICEMODULEUUID: regex::Regex =
            regex::Regex::new(r"^/v1/modules/serviceModules/(?P<serviceModuleUuid>[^/?#]*)$")
                .expect("Unable to create regex for MODULES_SERVICEMODULES_SERVICEMODULEUUID");
    }
    pub(crate) static ID_MODULES_SERVICEMODULES_SERVICEMODULEUUID_PERMISSIONS: usize = 17;
    lazy_static! {
        pub static ref REGEX_MODULES_SERVICEMODULES_SERVICEMODULEUUID_PERMISSIONS: regex::Regex =
            regex::Regex::new(r"^/v1/modules/serviceModules/(?P<serviceModuleUuid>[^/?#]*)/permissions$")
                .expect("Unable to create regex for MODULES_SERVICEMODULES_SERVICEMODULEUUID_PERMISSIONS");
    }
    pub(crate) static ID_SERVICES: usize = 18;
    pub(crate) static ID_SERVICES_PERMISSIONS: usize = 19;
    pub(crate) static ID_SERVICES_SERVICEID: usize = 20;
    lazy_static! {
        pub static ref REGEX_SERVICES_SERVICEID: regex::Regex =
            regex::Regex::new(r"^/v1/services/(?P<serviceId>[^/?#]*)$")
                .expect("Unable to create regex for SERVICES_SERVICEID");
    }
    pub(crate) static ID_SERVICES_SERVICEID_PERMISSIONS: usize = 21;
    lazy_static! {
        pub static ref REGEX_SERVICES_SERVICEID_PERMISSIONS: regex::Regex =
            regex::Regex::new(r"^/v1/services/(?P<serviceId>[^/?#]*)/permissions$")
                .expect("Unable to create regex for SERVICES_SERVICEID_PERMISSIONS");
    }
    pub(crate) static ID_SERVICES_SERVICEID_SERVERS: usize = 22;
    lazy_static! {
        pub static ref REGEX_SERVICES_SERVICEID_SERVERS: regex::Regex =
            regex::Regex::new(r"^/v1/services/(?P<serviceId>[^/?#]*)/servers$")
                .expect("Unable to create regex for SERVICES_SERVICEID_SERVERS");
    }
    pub(crate) static ID_SERVICES_SERVICEID_SERVERS_SERVERID: usize = 23;
    lazy_static! {
        pub static ref REGEX_SERVICES_SERVICEID_SERVERS_SERVERID: regex::Regex =
            regex::Regex::new(r"^/v1/services/(?P<serviceId>[^/?#]*)/servers/(?P<serverId>[^/?#]*)$")
                .expect("Unable to create regex for SERVICES_SERVICEID_SERVERS_SERVERID");
    }
    pub(crate) static ID_SERVICES_SERVICEID_SERVERS_SERVERID_PERMISSIONS: usize = 24;
    lazy_static! {
        pub static ref REGEX_SERVICES_SERVICEID_SERVERS_SERVERID_PERMISSIONS: regex::Regex =
            regex::Regex::new(r"^/v1/services/(?P<serviceId>[^/?#]*)/servers/(?P<serverId>[^/?#]*)/permissions$")
                .expect("Unable to create regex for SERVICES_SERVICEID_SERVERS_SERVERID_PERMISSIONS");
    }
    pub(crate) static ID_SERVICES_SERVICEID_SERVERS_SERVERID_SERVERCREDENTIALS: usize = 25;
    lazy_static! {
        pub static ref REGEX_SERVICES_SERVICEID_SERVERS_SERVERID_SERVERCREDENTIALS: regex::Regex =
            regex::Regex::new(r"^/v1/services/(?P<serviceId>[^/?#]*)/servers/(?P<serverId>[^/?#]*)/serverCredentials$")
                .expect("Unable to create regex for SERVICES_SERVICEID_SERVERS_SERVERID_SERVERCREDENTIALS");
    }
    pub(crate) static ID_SERVICES_SERVICEID_SERVERS_SERVERID_SERVERCREDENTIALS_PERMISSIONS: usize = 26;
    lazy_static! {
        pub static ref REGEX_SERVICES_SERVICEID_SERVERS_SERVERID_SERVERCREDENTIALS_PERMISSIONS: regex::Regex =
            regex::Regex::new(r"^/v1/services/(?P<serviceId>[^/?#]*)/servers/(?P<serverId>[^/?#]*)/serverCredentials/permissions$")
                .expect("Unable to create regex for SERVICES_SERVICEID_SERVERS_SERVERID_SERVERCREDENTIALS_PERMISSIONS");
    }
    pub(crate) static ID_SERVICES_SERVICEID_SERVERS_SERVERID_VALIDATE: usize = 27;
    lazy_static! {
        pub static ref REGEX_SERVICES_SERVICEID_SERVERS_SERVERID_VALIDATE: regex::Regex =
            regex::Regex::new(r"^/v1/services/(?P<serviceId>[^/?#]*)/servers/(?P<serverId>[^/?#]*)/validate$")
                .expect("Unable to create regex for SERVICES_SERVICEID_SERVERS_SERVERID_VALIDATE");
    }
    pub(crate) static ID_USERS: usize = 28;
    pub(crate) static ID_USERS_LOGIN: usize = 29;
    pub(crate) static ID_USERS_USERID: usize = 30;
    lazy_static! {
        pub static ref REGEX_USERS_USERID: regex::Regex =
            regex::Regex::new(r"^/v1/users/(?P<userId>[^/?#]*)$")
                .expect("Unable to create regex for USERS_USERID");
    }
}

pub struct MakeService<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static
{
    api_impl: T,
    marker: PhantomData<C>,
}

impl<T, C> MakeService<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static
{
    pub fn new(api_impl: T) -> Self {
        MakeService {
            api_impl,
            marker: PhantomData
        }
    }
}

impl<T, C, Target> hyper::service::Service<Target> for MakeService<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static
{
    type Response = Service<T, C>;
    type Error = crate::ServiceError;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, target: Target) -> Self::Future {
        futures::future::ok(Service::new(
            self.api_impl.clone(),
        ))
    }
}

fn method_not_allowed() -> Result<Response<Body>, crate::ServiceError> {
    Ok(
        Response::builder().status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::empty())
            .expect("Unable to create Method Not Allowed response")
    )
}

pub struct Service<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static
{
    api_impl: T,
    marker: PhantomData<C>,
}

impl<T, C> Service<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static
{
    pub fn new(api_impl: T) -> Self {
        Service {
            api_impl: api_impl,
            marker: PhantomData
        }
    }
}

impl<T, C> Clone for Service<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static
{
    fn clone(&self) -> Self {
        Service {
            api_impl: self.api_impl.clone(),
            marker: self.marker.clone(),
        }
    }
}

impl<T, C> hyper::service::Service<(Request<Body>, C)> for Service<T, C> where
    T: Api<C> + Clone + Send + Sync + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static
{
    type Response = Response<Body>;
    type Error = crate::ServiceError;
    type Future = ServiceFuture;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.api_impl.poll_ready(cx)
    }

    fn call(&mut self, req: (Request<Body>, C)) -> Self::Future { async fn run<T, C>(mut api_impl: T, req: (Request<Body>, C)) -> Result<Response<Body>, crate::ServiceError> where
        T: Api<C> + Clone + Send + 'static,
        C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static
    {
        let (request, context) = req;
        let (parts, body) = request.into_parts();
        let (method, uri, headers) = (parts.method, parts.uri, parts.headers);
        let path = paths::GLOBAL_REGEX_SET.matches(uri.path());

        match &method {

            // DeleteCloudModule - DELETE /modules/cloudModules/{cloudModuleUuid}
            &hyper::Method::DELETE if path.matched(paths::ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_MODULES_CLOUDMODULES_CLOUDMODULEUUID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MODULES_CLOUDMODULES_CLOUDMODULEUUID in set but failed match against \"{}\"", path, paths::REGEX_MODULES_CLOUDMODULES_CLOUDMODULEUUID.as_str())
                    );

                let param_cloud_module_uuid = match percent_encoding::percent_decode(path_params["cloudModuleUuid"].as_bytes()).decode_utf8() {
                    Ok(param_cloud_module_uuid) => match param_cloud_module_uuid.parse::<uuid::Uuid>() {
                        Ok(param_cloud_module_uuid) => param_cloud_module_uuid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter cloudModuleUuid: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["cloudModuleUuid"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.delete_cloud_module(
                                            param_cloud_module_uuid,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                DeleteCloudModuleResponse::CloudModuleDeleted
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                DeleteCloudModuleResponse::YouDoNotHaveSufficientPermissions
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DELETE_CLOUD_MODULE_YOU_DO_NOT_HAVE_SUFFICIENT_PERMISSIONS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // DeleteCloudModuleAccount - DELETE /modules/cloudModules/{cloudModuleUuid}/accounts/{cloudAccountId}
            &hyper::Method::DELETE if path.matched(paths::ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID_ACCOUNTS_CLOUDACCOUNTID) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_MODULES_CLOUDMODULES_CLOUDMODULEUUID_ACCOUNTS_CLOUDACCOUNTID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MODULES_CLOUDMODULES_CLOUDMODULEUUID_ACCOUNTS_CLOUDACCOUNTID in set but failed match against \"{}\"", path, paths::REGEX_MODULES_CLOUDMODULES_CLOUDMODULEUUID_ACCOUNTS_CLOUDACCOUNTID.as_str())
                    );

                let param_cloud_module_uuid = match percent_encoding::percent_decode(path_params["cloudModuleUuid"].as_bytes()).decode_utf8() {
                    Ok(param_cloud_module_uuid) => match param_cloud_module_uuid.parse::<uuid::Uuid>() {
                        Ok(param_cloud_module_uuid) => param_cloud_module_uuid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter cloudModuleUuid: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["cloudModuleUuid"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_cloud_account_id = match percent_encoding::percent_decode(path_params["cloudAccountId"].as_bytes()).decode_utf8() {
                    Ok(param_cloud_account_id) => match param_cloud_account_id.parse::<String>() {
                        Ok(param_cloud_account_id) => param_cloud_account_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter cloudAccountId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["cloudAccountId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.delete_cloud_module_account(
                                            param_cloud_module_uuid,
                                            param_cloud_account_id,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                DeleteCloudModuleAccountResponse::CloudModuleAccountDeleted
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                DeleteCloudModuleAccountResponse::YouDoNotHaveSufficientPermissions
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DELETE_CLOUD_MODULE_ACCOUNT_YOU_DO_NOT_HAVE_SUFFICIENT_PERMISSIONS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetCloudModule - GET /modules/cloudModules/{cloudModuleUuid}
            &hyper::Method::GET if path.matched(paths::ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_MODULES_CLOUDMODULES_CLOUDMODULEUUID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MODULES_CLOUDMODULES_CLOUDMODULEUUID in set but failed match against \"{}\"", path, paths::REGEX_MODULES_CLOUDMODULES_CLOUDMODULEUUID.as_str())
                    );

                let param_cloud_module_uuid = match percent_encoding::percent_decode(path_params["cloudModuleUuid"].as_bytes()).decode_utf8() {
                    Ok(param_cloud_module_uuid) => match param_cloud_module_uuid.parse::<uuid::Uuid>() {
                        Ok(param_cloud_module_uuid) => param_cloud_module_uuid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter cloudModuleUuid: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["cloudModuleUuid"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_cloud_module(
                                            param_cloud_module_uuid,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetCloudModuleResponse::SpecificCloudModuleDetails
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_CLOUD_MODULE_SPECIFIC_CLOUD_MODULE_DETAILS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetCloudModuleResponse::YouDoNotHaveSufficientPermissions
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_CLOUD_MODULE_YOU_DO_NOT_HAVE_SUFFICIENT_PERMISSIONS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetCloudModuleAccount - GET /modules/cloudModules/{cloudModuleUuid}/accounts/{cloudAccountId}
            &hyper::Method::GET if path.matched(paths::ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID_ACCOUNTS_CLOUDACCOUNTID) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_MODULES_CLOUDMODULES_CLOUDMODULEUUID_ACCOUNTS_CLOUDACCOUNTID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MODULES_CLOUDMODULES_CLOUDMODULEUUID_ACCOUNTS_CLOUDACCOUNTID in set but failed match against \"{}\"", path, paths::REGEX_MODULES_CLOUDMODULES_CLOUDMODULEUUID_ACCOUNTS_CLOUDACCOUNTID.as_str())
                    );

                let param_cloud_module_uuid = match percent_encoding::percent_decode(path_params["cloudModuleUuid"].as_bytes()).decode_utf8() {
                    Ok(param_cloud_module_uuid) => match param_cloud_module_uuid.parse::<uuid::Uuid>() {
                        Ok(param_cloud_module_uuid) => param_cloud_module_uuid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter cloudModuleUuid: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["cloudModuleUuid"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_cloud_account_id = match percent_encoding::percent_decode(path_params["cloudAccountId"].as_bytes()).decode_utf8() {
                    Ok(param_cloud_account_id) => match param_cloud_account_id.parse::<String>() {
                        Ok(param_cloud_account_id) => param_cloud_account_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter cloudAccountId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["cloudAccountId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_cloud_module_account(
                                            param_cloud_module_uuid,
                                            param_cloud_account_id,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetCloudModuleAccountResponse::CloudModuleAccountDetails
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_CLOUD_MODULE_ACCOUNT_CLOUD_MODULE_ACCOUNT_DETAILS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetCloudModuleAccountResponse::YouDoNotHaveSufficientPermissions
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_CLOUD_MODULE_ACCOUNT_YOU_DO_NOT_HAVE_SUFFICIENT_PERMISSIONS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetCloudModuleAccountPermissions - GET /modules/cloudModules/{cloudModuleUuid}/accounts/{cloudAccountId}/permissions
            &hyper::Method::GET if path.matched(paths::ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID_ACCOUNTS_CLOUDACCOUNTID_PERMISSIONS) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_MODULES_CLOUDMODULES_CLOUDMODULEUUID_ACCOUNTS_CLOUDACCOUNTID_PERMISSIONS
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MODULES_CLOUDMODULES_CLOUDMODULEUUID_ACCOUNTS_CLOUDACCOUNTID_PERMISSIONS in set but failed match against \"{}\"", path, paths::REGEX_MODULES_CLOUDMODULES_CLOUDMODULEUUID_ACCOUNTS_CLOUDACCOUNTID_PERMISSIONS.as_str())
                    );

                let param_cloud_module_uuid = match percent_encoding::percent_decode(path_params["cloudModuleUuid"].as_bytes()).decode_utf8() {
                    Ok(param_cloud_module_uuid) => match param_cloud_module_uuid.parse::<uuid::Uuid>() {
                        Ok(param_cloud_module_uuid) => param_cloud_module_uuid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter cloudModuleUuid: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["cloudModuleUuid"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_cloud_account_id = match percent_encoding::percent_decode(path_params["cloudAccountId"].as_bytes()).decode_utf8() {
                    Ok(param_cloud_account_id) => match param_cloud_account_id.parse::<String>() {
                        Ok(param_cloud_account_id) => param_cloud_account_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter cloudAccountId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["cloudAccountId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_cloud_module_account_permissions(
                                            param_cloud_module_uuid,
                                            param_cloud_account_id,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetCloudModuleAccountPermissionsResponse::PermissionsWillBeProvided
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_CLOUD_MODULE_ACCOUNT_PERMISSIONS_PERMISSIONS_WILL_BE_PROVIDED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetCloudModuleAccountPermissionsResponse::YouDoNotHaveSufficientPermissions
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_CLOUD_MODULE_ACCOUNT_PERMISSIONS_YOU_DO_NOT_HAVE_SUFFICIENT_PERMISSIONS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetCloudModuleAccounts - GET /modules/cloudModules/{cloudModuleUuid}/accounts
            &hyper::Method::GET if path.matched(paths::ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID_ACCOUNTS) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_MODULES_CLOUDMODULES_CLOUDMODULEUUID_ACCOUNTS
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MODULES_CLOUDMODULES_CLOUDMODULEUUID_ACCOUNTS in set but failed match against \"{}\"", path, paths::REGEX_MODULES_CLOUDMODULES_CLOUDMODULEUUID_ACCOUNTS.as_str())
                    );

                let param_cloud_module_uuid = match percent_encoding::percent_decode(path_params["cloudModuleUuid"].as_bytes()).decode_utf8() {
                    Ok(param_cloud_module_uuid) => match param_cloud_module_uuid.parse::<uuid::Uuid>() {
                        Ok(param_cloud_module_uuid) => param_cloud_module_uuid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter cloudModuleUuid: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["cloudModuleUuid"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_cloud_module_accounts(
                                            param_cloud_module_uuid,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetCloudModuleAccountsResponse::ListOfAllCloudCredentials
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_CLOUD_MODULE_ACCOUNTS_LIST_OF_ALL_CLOUD_CREDENTIALS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetCloudModuleAccountsResponse::YouDoNotHaveSufficientPermissions
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_CLOUD_MODULE_ACCOUNTS_YOU_DO_NOT_HAVE_SUFFICIENT_PERMISSIONS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetCloudModulePermissions - GET /modules/cloudModules/{cloudModuleUuid}/permissions
            &hyper::Method::GET if path.matched(paths::ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID_PERMISSIONS) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_MODULES_CLOUDMODULES_CLOUDMODULEUUID_PERMISSIONS
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MODULES_CLOUDMODULES_CLOUDMODULEUUID_PERMISSIONS in set but failed match against \"{}\"", path, paths::REGEX_MODULES_CLOUDMODULES_CLOUDMODULEUUID_PERMISSIONS.as_str())
                    );

                let param_cloud_module_uuid = match percent_encoding::percent_decode(path_params["cloudModuleUuid"].as_bytes()).decode_utf8() {
                    Ok(param_cloud_module_uuid) => match param_cloud_module_uuid.parse::<uuid::Uuid>() {
                        Ok(param_cloud_module_uuid) => param_cloud_module_uuid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter cloudModuleUuid: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["cloudModuleUuid"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_cloud_module_permissions(
                                            param_cloud_module_uuid,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetCloudModulePermissionsResponse::PermissionsWillBeProvided
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_CLOUD_MODULE_PERMISSIONS_PERMISSIONS_WILL_BE_PROVIDED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetCloudModulePermissionsResponse::YouDoNotHaveSufficientPermissions
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_CLOUD_MODULE_PERMISSIONS_YOU_DO_NOT_HAVE_SUFFICIENT_PERMISSIONS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetCloudModules - GET /modules/cloudModules
            &hyper::Method::GET if path.matched(paths::ID_MODULES_CLOUDMODULES) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                                let result = api_impl.get_cloud_modules(
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetCloudModulesResponse::ListOfAllCloudModules
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_CLOUD_MODULES_LIST_OF_ALL_CLOUD_MODULES"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetCloudModulesResponse::YouDoNotHaveSufficientPermissions
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_CLOUD_MODULES_YOU_DO_NOT_HAVE_SUFFICIENT_PERMISSIONS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // PostCloudModule - POST /modules/cloudModules/{cloudModuleUuid}
            &hyper::Method::POST if path.matched(paths::ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_MODULES_CLOUDMODULES_CLOUDMODULEUUID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MODULES_CLOUDMODULES_CLOUDMODULEUUID in set but failed match against \"{}\"", path, paths::REGEX_MODULES_CLOUDMODULES_CLOUDMODULEUUID.as_str())
                    );

                let param_cloud_module_uuid = match percent_encoding::percent_decode(path_params["cloudModuleUuid"].as_bytes()).decode_utf8() {
                    Ok(param_cloud_module_uuid) => match param_cloud_module_uuid.parse::<uuid::Uuid>() {
                        Ok(param_cloud_module_uuid) => param_cloud_module_uuid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter cloudModuleUuid: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["cloudModuleUuid"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.to_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_body: Option<models::ServiceModule> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_body) => param_body,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter body - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter body due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_body = match param_body {
                                    Some(param_body) => param_body,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter body"))
                                                        .expect("Unable to create Bad Request response for missing body parameter body")),
                                };

                                let result = api_impl.post_cloud_module(
                                            param_cloud_module_uuid,
                                            param_body,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PostCloudModuleResponse::CloudModuleAltered
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                PostCloudModuleResponse::YouDoNotHaveSufficientPermissions
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_CLOUD_MODULE_YOU_DO_NOT_HAVE_SUFFICIENT_PERMISSIONS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter body: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter body")),
                        }
            },

            // PostCloudModuleAccounts - POST /modules/cloudModules/{cloudModuleUuid}/accounts
            &hyper::Method::POST if path.matched(paths::ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID_ACCOUNTS) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_MODULES_CLOUDMODULES_CLOUDMODULEUUID_ACCOUNTS
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MODULES_CLOUDMODULES_CLOUDMODULEUUID_ACCOUNTS in set but failed match against \"{}\"", path, paths::REGEX_MODULES_CLOUDMODULES_CLOUDMODULEUUID_ACCOUNTS.as_str())
                    );

                let param_cloud_module_uuid = match percent_encoding::percent_decode(path_params["cloudModuleUuid"].as_bytes()).decode_utf8() {
                    Ok(param_cloud_module_uuid) => match param_cloud_module_uuid.parse::<uuid::Uuid>() {
                        Ok(param_cloud_module_uuid) => param_cloud_module_uuid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter cloudModuleUuid: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["cloudModuleUuid"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.to_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_body: Option<models::CloudAccount> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_body) => param_body,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter body - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter body due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_body = match param_body {
                                    Some(param_body) => param_body,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter body"))
                                                        .expect("Unable to create Bad Request response for missing body parameter body")),
                                };

                                let result = api_impl.post_cloud_module_accounts(
                                            param_cloud_module_uuid,
                                            param_body,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PostCloudModuleAccountsResponse::CloudModuleAccountCreated
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(201).expect("Unable to turn 201 into a StatusCode");
                                                },
                                                PostCloudModuleAccountsResponse::YouDoNotHaveSufficientPermissions
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_CLOUD_MODULE_ACCOUNTS_YOU_DO_NOT_HAVE_SUFFICIENT_PERMISSIONS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter body: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter body")),
                        }
            },

            // PostCloudModules - POST /modules/cloudModules
            &hyper::Method::POST if path.matched(paths::ID_MODULES_CLOUDMODULES) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.to_raw().await;
                match result {
                            Ok(body) => {
                                let param_body: Option<swagger::ByteArray> = if !body.is_empty() {
                                } else {
                                    None
                                };
                                let param_body = match param_body {
                                    Some(param_body) => param_body,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter body"))
                                                        .expect("Unable to create Bad Request response for missing body parameter body")),
                                };

                                let result = api_impl.post_cloud_modules(
                                            param_body,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PostCloudModulesResponse::SpecificCloudModuleDetails
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(201).expect("Unable to turn 201 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_CLOUD_MODULES_SPECIFIC_CLOUD_MODULE_DETAILS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                PostCloudModulesResponse::YouDoNotHaveSufficientPermissions
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_CLOUD_MODULES_YOU_DO_NOT_HAVE_SUFFICIENT_PERMISSIONS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter body: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter body")),
                        }
            },

            // PutCloudModuleAccount - PUT /modules/cloudModules/{cloudModuleUuid}/accounts/{cloudAccountId}
            &hyper::Method::PUT if path.matched(paths::ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID_ACCOUNTS_CLOUDACCOUNTID) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_MODULES_CLOUDMODULES_CLOUDMODULEUUID_ACCOUNTS_CLOUDACCOUNTID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MODULES_CLOUDMODULES_CLOUDMODULEUUID_ACCOUNTS_CLOUDACCOUNTID in set but failed match against \"{}\"", path, paths::REGEX_MODULES_CLOUDMODULES_CLOUDMODULEUUID_ACCOUNTS_CLOUDACCOUNTID.as_str())
                    );

                let param_cloud_module_uuid = match percent_encoding::percent_decode(path_params["cloudModuleUuid"].as_bytes()).decode_utf8() {
                    Ok(param_cloud_module_uuid) => match param_cloud_module_uuid.parse::<uuid::Uuid>() {
                        Ok(param_cloud_module_uuid) => param_cloud_module_uuid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter cloudModuleUuid: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["cloudModuleUuid"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_cloud_account_id = match percent_encoding::percent_decode(path_params["cloudAccountId"].as_bytes()).decode_utf8() {
                    Ok(param_cloud_account_id) => match param_cloud_account_id.parse::<String>() {
                        Ok(param_cloud_account_id) => param_cloud_account_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter cloudAccountId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["cloudAccountId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.to_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_body: Option<models::CloudAccount> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_body) => param_body,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter body - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter body due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_body = match param_body {
                                    Some(param_body) => param_body,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter body"))
                                                        .expect("Unable to create Bad Request response for missing body parameter body")),
                                };

                                let result = api_impl.put_cloud_module_account(
                                            param_cloud_module_uuid,
                                            param_cloud_account_id,
                                            param_body,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PutCloudModuleAccountResponse::CloudModuleAccountAltered
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                PutCloudModuleAccountResponse::YouDoNotHaveSufficientPermissions
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for PUT_CLOUD_MODULE_ACCOUNT_YOU_DO_NOT_HAVE_SUFFICIENT_PERMISSIONS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter body: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter body")),
                        }
            },

            // PutCloudModuleAccountPermissions - PUT /modules/cloudModules/{cloudModuleUuid}/accounts/{cloudAccountId}/permissions
            &hyper::Method::PUT if path.matched(paths::ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID_ACCOUNTS_CLOUDACCOUNTID_PERMISSIONS) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_MODULES_CLOUDMODULES_CLOUDMODULEUUID_ACCOUNTS_CLOUDACCOUNTID_PERMISSIONS
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MODULES_CLOUDMODULES_CLOUDMODULEUUID_ACCOUNTS_CLOUDACCOUNTID_PERMISSIONS in set but failed match against \"{}\"", path, paths::REGEX_MODULES_CLOUDMODULES_CLOUDMODULEUUID_ACCOUNTS_CLOUDACCOUNTID_PERMISSIONS.as_str())
                    );

                let param_cloud_module_uuid = match percent_encoding::percent_decode(path_params["cloudModuleUuid"].as_bytes()).decode_utf8() {
                    Ok(param_cloud_module_uuid) => match param_cloud_module_uuid.parse::<uuid::Uuid>() {
                        Ok(param_cloud_module_uuid) => param_cloud_module_uuid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter cloudModuleUuid: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["cloudModuleUuid"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_cloud_account_id = match percent_encoding::percent_decode(path_params["cloudAccountId"].as_bytes()).decode_utf8() {
                    Ok(param_cloud_account_id) => match param_cloud_account_id.parse::<String>() {
                        Ok(param_cloud_account_id) => param_cloud_account_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter cloudAccountId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["cloudAccountId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.to_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_body: Option<models::Permissions> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_body) => param_body,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter body - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter body due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_body = match param_body {
                                    Some(param_body) => param_body,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter body"))
                                                        .expect("Unable to create Bad Request response for missing body parameter body")),
                                };

                                let result = api_impl.put_cloud_module_account_permissions(
                                            param_cloud_module_uuid,
                                            param_cloud_account_id,
                                            param_body,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PutCloudModuleAccountPermissionsResponse::PermissionsAlteredSuccessfully
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                PutCloudModuleAccountPermissionsResponse::YouDoNotHaveSufficientPermissions
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for PUT_CLOUD_MODULE_ACCOUNT_PERMISSIONS_YOU_DO_NOT_HAVE_SUFFICIENT_PERMISSIONS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter body: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter body")),
                        }
            },

            // PutCloudModulePermissions - PUT /modules/cloudModules/{cloudModuleUuid}/permissions
            &hyper::Method::PUT if path.matched(paths::ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID_PERMISSIONS) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_MODULES_CLOUDMODULES_CLOUDMODULEUUID_PERMISSIONS
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MODULES_CLOUDMODULES_CLOUDMODULEUUID_PERMISSIONS in set but failed match against \"{}\"", path, paths::REGEX_MODULES_CLOUDMODULES_CLOUDMODULEUUID_PERMISSIONS.as_str())
                    );

                let param_cloud_module_uuid = match percent_encoding::percent_decode(path_params["cloudModuleUuid"].as_bytes()).decode_utf8() {
                    Ok(param_cloud_module_uuid) => match param_cloud_module_uuid.parse::<uuid::Uuid>() {
                        Ok(param_cloud_module_uuid) => param_cloud_module_uuid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter cloudModuleUuid: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["cloudModuleUuid"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.to_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_body: Option<models::Permissions> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_body) => param_body,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter body - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter body due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_body = match param_body {
                                    Some(param_body) => param_body,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter body"))
                                                        .expect("Unable to create Bad Request response for missing body parameter body")),
                                };

                                let result = api_impl.put_cloud_module_permissions(
                                            param_cloud_module_uuid,
                                            param_body,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PutCloudModulePermissionsResponse::PermissionsAlteredSuccessfully
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                PutCloudModulePermissionsResponse::YouDoNotHaveSufficientPermissions
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for PUT_CLOUD_MODULE_PERMISSIONS_YOU_DO_NOT_HAVE_SUFFICIENT_PERMISSIONS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter body: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter body")),
                        }
            },

            // GetCpuCores - GET /modules/cloudModules/{cloudModuleUuid}/queries/cpuCores
            &hyper::Method::GET if path.matched(paths::ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID_QUERIES_CPUCORES) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_MODULES_CLOUDMODULES_CLOUDMODULEUUID_QUERIES_CPUCORES
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MODULES_CLOUDMODULES_CLOUDMODULEUUID_QUERIES_CPUCORES in set but failed match against \"{}\"", path, paths::REGEX_MODULES_CLOUDMODULES_CLOUDMODULEUUID_QUERIES_CPUCORES.as_str())
                    );

                let param_cloud_module_uuid = match percent_encoding::percent_decode(path_params["cloudModuleUuid"].as_bytes()).decode_utf8() {
                    Ok(param_cloud_module_uuid) => match param_cloud_module_uuid.parse::<uuid::Uuid>() {
                        Ok(param_cloud_module_uuid) => param_cloud_module_uuid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter cloudModuleUuid: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["cloudModuleUuid"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_cpu_cores(
                                            param_cloud_module_uuid,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetCpuCoresResponse::AvailableCpuAmountProfiles
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_CPU_CORES_AVAILABLE_CPU_AMOUNT_PROFILES"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetCpuCoresResponse::SomethingHasGoneWrongInTheModuleThatWasProcessingRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_CPU_CORES_SOMETHING_HAS_GONE_WRONG_IN_THE_MODULE_THAT_WAS_PROCESSING_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetCpuCoresResponse::ModuleDoesNotSupportThisFunctionality
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(501).expect("Unable to turn 501 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_CPU_CORES_MODULE_DOES_NOT_SUPPORT_THIS_FUNCTIONALITY"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetDiskBounds - GET /modules/cloudModules/{cloudModuleUuid}/queries/diskBounds
            &hyper::Method::GET if path.matched(paths::ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID_QUERIES_DISKBOUNDS) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_MODULES_CLOUDMODULES_CLOUDMODULEUUID_QUERIES_DISKBOUNDS
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MODULES_CLOUDMODULES_CLOUDMODULEUUID_QUERIES_DISKBOUNDS in set but failed match against \"{}\"", path, paths::REGEX_MODULES_CLOUDMODULES_CLOUDMODULEUUID_QUERIES_DISKBOUNDS.as_str())
                    );

                let param_cloud_module_uuid = match percent_encoding::percent_decode(path_params["cloudModuleUuid"].as_bytes()).decode_utf8() {
                    Ok(param_cloud_module_uuid) => match param_cloud_module_uuid.parse::<uuid::Uuid>() {
                        Ok(param_cloud_module_uuid) => param_cloud_module_uuid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter cloudModuleUuid: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["cloudModuleUuid"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_disk_bounds(
                                            param_cloud_module_uuid,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetDiskBoundsResponse::AvailableMinimumAndMaximumDisk
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_DISK_BOUNDS_AVAILABLE_MINIMUM_AND_MAXIMUM_DISK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetDiskBoundsResponse::SomethingHasGoneWrongInTheModuleThatWasProcessingRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_DISK_BOUNDS_SOMETHING_HAS_GONE_WRONG_IN_THE_MODULE_THAT_WAS_PROCESSING_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetDiskBoundsResponse::ModuleDoesNotSupportThisFunctionality
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(501).expect("Unable to turn 501 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_DISK_BOUNDS_MODULE_DOES_NOT_SUPPORT_THIS_FUNCTIONALITY"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetMachineTypes - GET /modules/cloudModules/{cloudModuleUuid}/queries/machineTypes
            &hyper::Method::GET if path.matched(paths::ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID_QUERIES_MACHINETYPES) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_MODULES_CLOUDMODULES_CLOUDMODULEUUID_QUERIES_MACHINETYPES
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MODULES_CLOUDMODULES_CLOUDMODULEUUID_QUERIES_MACHINETYPES in set but failed match against \"{}\"", path, paths::REGEX_MODULES_CLOUDMODULES_CLOUDMODULEUUID_QUERIES_MACHINETYPES.as_str())
                    );

                let param_cloud_module_uuid = match percent_encoding::percent_decode(path_params["cloudModuleUuid"].as_bytes()).decode_utf8() {
                    Ok(param_cloud_module_uuid) => match param_cloud_module_uuid.parse::<uuid::Uuid>() {
                        Ok(param_cloud_module_uuid) => param_cloud_module_uuid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter cloudModuleUuid: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["cloudModuleUuid"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_machine_types(
                                            param_cloud_module_uuid,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetMachineTypesResponse::AvailableMachineTypes
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_MACHINE_TYPES_AVAILABLE_MACHINE_TYPES"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetMachineTypesResponse::SomethingHasGoneWrongInTheModuleThatWasProcessingRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_MACHINE_TYPES_SOMETHING_HAS_GONE_WRONG_IN_THE_MODULE_THAT_WAS_PROCESSING_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetMachineTypesResponse::ModuleDoesNotSupportThisFunctionality
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(501).expect("Unable to turn 501 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_MACHINE_TYPES_MODULE_DOES_NOT_SUPPORT_THIS_FUNCTIONALITY"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetOperatingSystems - GET /modules/cloudModules/{cloudModuleUuid}/queries/operatingSystems
            &hyper::Method::GET if path.matched(paths::ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID_QUERIES_OPERATINGSYSTEMS) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_MODULES_CLOUDMODULES_CLOUDMODULEUUID_QUERIES_OPERATINGSYSTEMS
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MODULES_CLOUDMODULES_CLOUDMODULEUUID_QUERIES_OPERATINGSYSTEMS in set but failed match against \"{}\"", path, paths::REGEX_MODULES_CLOUDMODULES_CLOUDMODULEUUID_QUERIES_OPERATINGSYSTEMS.as_str())
                    );

                let param_cloud_module_uuid = match percent_encoding::percent_decode(path_params["cloudModuleUuid"].as_bytes()).decode_utf8() {
                    Ok(param_cloud_module_uuid) => match param_cloud_module_uuid.parse::<uuid::Uuid>() {
                        Ok(param_cloud_module_uuid) => param_cloud_module_uuid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter cloudModuleUuid: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["cloudModuleUuid"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_operating_systems(
                                            param_cloud_module_uuid,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetOperatingSystemsResponse::AvailableOperatingSystemTypes
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_OPERATING_SYSTEMS_AVAILABLE_OPERATING_SYSTEM_TYPES"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetOperatingSystemsResponse::SomethingHasGoneWrongInTheModuleThatWasProcessingRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_OPERATING_SYSTEMS_SOMETHING_HAS_GONE_WRONG_IN_THE_MODULE_THAT_WAS_PROCESSING_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetOperatingSystemsResponse::ModuleDoesNotSupportThisFunctionality
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(501).expect("Unable to turn 501 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_OPERATING_SYSTEMS_MODULE_DOES_NOT_SUPPORT_THIS_FUNCTIONALITY"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetOrientations - GET /modules/cloudModules/{cloudModuleUuid}/queries/orientations
            &hyper::Method::GET if path.matched(paths::ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID_QUERIES_ORIENTATIONS) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_MODULES_CLOUDMODULES_CLOUDMODULEUUID_QUERIES_ORIENTATIONS
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MODULES_CLOUDMODULES_CLOUDMODULEUUID_QUERIES_ORIENTATIONS in set but failed match against \"{}\"", path, paths::REGEX_MODULES_CLOUDMODULES_CLOUDMODULEUUID_QUERIES_ORIENTATIONS.as_str())
                    );

                let param_cloud_module_uuid = match percent_encoding::percent_decode(path_params["cloudModuleUuid"].as_bytes()).decode_utf8() {
                    Ok(param_cloud_module_uuid) => match param_cloud_module_uuid.parse::<uuid::Uuid>() {
                        Ok(param_cloud_module_uuid) => param_cloud_module_uuid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter cloudModuleUuid: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["cloudModuleUuid"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_orientations(
                                            param_cloud_module_uuid,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetOrientationsResponse::AvailableComputeOrientationTypes
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ORIENTATIONS_AVAILABLE_COMPUTE_ORIENTATION_TYPES"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetOrientationsResponse::SomethingHasGoneWrongInTheModuleThatWasProcessingRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ORIENTATIONS_SOMETHING_HAS_GONE_WRONG_IN_THE_MODULE_THAT_WAS_PROCESSING_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetOrientationsResponse::ModuleDoesNotSupportThisFunctionality
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(501).expect("Unable to turn 501 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ORIENTATIONS_MODULE_DOES_NOT_SUPPORT_THIS_FUNCTIONALITY"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetRamBounds - GET /modules/cloudModules/{cloudModuleUuid}/queries/ramBounds
            &hyper::Method::GET if path.matched(paths::ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID_QUERIES_RAMBOUNDS) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_MODULES_CLOUDMODULES_CLOUDMODULEUUID_QUERIES_RAMBOUNDS
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MODULES_CLOUDMODULES_CLOUDMODULEUUID_QUERIES_RAMBOUNDS in set but failed match against \"{}\"", path, paths::REGEX_MODULES_CLOUDMODULES_CLOUDMODULEUUID_QUERIES_RAMBOUNDS.as_str())
                    );

                let param_cloud_module_uuid = match percent_encoding::percent_decode(path_params["cloudModuleUuid"].as_bytes()).decode_utf8() {
                    Ok(param_cloud_module_uuid) => match param_cloud_module_uuid.parse::<uuid::Uuid>() {
                        Ok(param_cloud_module_uuid) => param_cloud_module_uuid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter cloudModuleUuid: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["cloudModuleUuid"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_ram_bounds(
                                            param_cloud_module_uuid,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetRamBoundsResponse::AvailableMinimumAndMaximumRam
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_RAM_BOUNDS_AVAILABLE_MINIMUM_AND_MAXIMUM_RAM"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetRamBoundsResponse::SomethingHasGoneWrongInTheModuleThatWasProcessingRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_RAM_BOUNDS_SOMETHING_HAS_GONE_WRONG_IN_THE_MODULE_THAT_WAS_PROCESSING_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetRamBoundsResponse::ModuleDoesNotSupportThisFunctionality
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(501).expect("Unable to turn 501 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_RAM_BOUNDS_MODULE_DOES_NOT_SUPPORT_THIS_FUNCTIONALITY"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // DeleteGroup - DELETE /groups/{groupId}
            &hyper::Method::DELETE if path.matched(paths::ID_GROUPS_GROUPID) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_GROUPS_GROUPID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE GROUPS_GROUPID in set but failed match against \"{}\"", path, paths::REGEX_GROUPS_GROUPID.as_str())
                    );

                let param_group_id = match percent_encoding::percent_decode(path_params["groupId"].as_bytes()).decode_utf8() {
                    Ok(param_group_id) => match param_group_id.parse::<i32>() {
                        Ok(param_group_id) => param_group_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter groupId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["groupId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.delete_group(
                                            param_group_id,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                DeleteGroupResponse::GroupDeleted
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                DeleteGroupResponse::YouDoNotHaveSufficientPermissions
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DELETE_GROUP_YOU_DO_NOT_HAVE_SUFFICIENT_PERMISSIONS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetGroup - GET /groups/{groupId}
            &hyper::Method::GET if path.matched(paths::ID_GROUPS_GROUPID) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_GROUPS_GROUPID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE GROUPS_GROUPID in set but failed match against \"{}\"", path, paths::REGEX_GROUPS_GROUPID.as_str())
                    );

                let param_group_id = match percent_encoding::percent_decode(path_params["groupId"].as_bytes()).decode_utf8() {
                    Ok(param_group_id) => match param_group_id.parse::<i32>() {
                        Ok(param_group_id) => param_group_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter groupId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["groupId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_group(
                                            param_group_id,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetGroupResponse::DetailsOfThePermissionGroup
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_GROUP_DETAILS_OF_THE_PERMISSION_GROUP"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetGroupResponse::YouDoNotHaveSufficientPermissions
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_GROUP_YOU_DO_NOT_HAVE_SUFFICIENT_PERMISSIONS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetGroups - GET /groups
            &hyper::Method::GET if path.matched(paths::ID_GROUPS) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                                let result = api_impl.get_groups(
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetGroupsResponse::ListOfAllPermissionGroups
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_GROUPS_LIST_OF_ALL_PERMISSION_GROUPS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetGroupsResponse::YouDoNotHaveSufficientPermissions
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_GROUPS_YOU_DO_NOT_HAVE_SUFFICIENT_PERMISSIONS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // PostGroup - POST /groups
            &hyper::Method::POST if path.matched(paths::ID_GROUPS) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.to_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_body: Option<models::GroupContent> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_body) => param_body,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter body - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter body due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_body = match param_body {
                                    Some(param_body) => param_body,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter body"))
                                                        .expect("Unable to create Bad Request response for missing body parameter body")),
                                };

                                let result = api_impl.post_group(
                                            param_body,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PostGroupResponse::GroupCreatedSuccessfully
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(201).expect("Unable to turn 201 into a StatusCode");
                                                },
                                                PostGroupResponse::YouDoNotHaveSufficientPermissions
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_GROUP_YOU_DO_NOT_HAVE_SUFFICIENT_PERMISSIONS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter body: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter body")),
                        }
            },

            // PutGroup - PUT /groups/{groupId}
            &hyper::Method::PUT if path.matched(paths::ID_GROUPS_GROUPID) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_GROUPS_GROUPID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE GROUPS_GROUPID in set but failed match against \"{}\"", path, paths::REGEX_GROUPS_GROUPID.as_str())
                    );

                let param_group_id = match percent_encoding::percent_decode(path_params["groupId"].as_bytes()).decode_utf8() {
                    Ok(param_group_id) => match param_group_id.parse::<i32>() {
                        Ok(param_group_id) => param_group_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter groupId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["groupId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.to_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_body: Option<models::Group> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_body) => param_body,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter body - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter body due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_body = match param_body {
                                    Some(param_body) => param_body,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter body"))
                                                        .expect("Unable to create Bad Request response for missing body parameter body")),
                                };

                                let result = api_impl.put_group(
                                            param_group_id,
                                            param_body,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PutGroupResponse::GroupWasAlteredSuccessfully
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                PutGroupResponse::YouDoNotHaveSufficientPermissions
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for PUT_GROUP_YOU_DO_NOT_HAVE_SUFFICIENT_PERMISSIONS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter body: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter body")),
                        }
            },

            // GetConsoleManagement - GET /consoleManagement
            &hyper::Method::GET if path.matched(paths::ID_CONSOLEMANAGEMENT) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                                let result = api_impl.get_console_management(
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetConsoleManagementResponse::CurrentPotkuConsoleMainSettings
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_CONSOLE_MANAGEMENT_CURRENT_POTKU_CONSOLE_MAIN_SETTINGS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetConsoleManagementResponse::YouDoNotHaveSufficientPermissions
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_CONSOLE_MANAGEMENT_YOU_DO_NOT_HAVE_SUFFICIENT_PERMISSIONS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // PostConsoleManagement - POST /consoleManagement
            &hyper::Method::POST if path.matched(paths::ID_CONSOLEMANAGEMENT) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.to_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_body: Option<models::ConsoleSettings> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_body) => param_body,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter body - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter body due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_body = match param_body {
                                    Some(param_body) => param_body,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter body"))
                                                        .expect("Unable to create Bad Request response for missing body parameter body")),
                                };

                                let result = api_impl.post_console_management(
                                            param_body,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PostConsoleManagementResponse::SpecificCloudModuleDetails
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(201).expect("Unable to turn 201 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_CONSOLE_MANAGEMENT_SPECIFIC_CLOUD_MODULE_DETAILS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                PostConsoleManagementResponse::YouDoNotHaveSufficientPermissions
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_CONSOLE_MANAGEMENT_YOU_DO_NOT_HAVE_SUFFICIENT_PERMISSIONS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter body: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter body")),
                        }
            },

            // GetServiceServer - GET /services/{serviceId}/servers/{serverId}
            &hyper::Method::GET if path.matched(paths::ID_SERVICES_SERVICEID_SERVERS_SERVERID) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_SERVICES_SERVICEID_SERVERS_SERVERID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE SERVICES_SERVICEID_SERVERS_SERVERID in set but failed match against \"{}\"", path, paths::REGEX_SERVICES_SERVICEID_SERVERS_SERVERID.as_str())
                    );

                let param_service_id = match percent_encoding::percent_decode(path_params["serviceId"].as_bytes()).decode_utf8() {
                    Ok(param_service_id) => match param_service_id.parse::<i32>() {
                        Ok(param_service_id) => param_service_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter serviceId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["serviceId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_server_id = match percent_encoding::percent_decode(path_params["serverId"].as_bytes()).decode_utf8() {
                    Ok(param_server_id) => match param_server_id.parse::<i32>() {
                        Ok(param_server_id) => param_server_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter serverId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["serverId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_service_server(
                                            param_service_id,
                                            param_server_id,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetServiceServerResponse::Service
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_SERVICE_SERVER_SERVICE"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetServiceServerResponse::YouDoNotHaveSufficientPermissions
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_SERVICE_SERVER_YOU_DO_NOT_HAVE_SUFFICIENT_PERMISSIONS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetServiceServerPermissions - GET /services/{serviceId}/servers/{serverId}/permissions
            &hyper::Method::GET if path.matched(paths::ID_SERVICES_SERVICEID_SERVERS_SERVERID_PERMISSIONS) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_SERVICES_SERVICEID_SERVERS_SERVERID_PERMISSIONS
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE SERVICES_SERVICEID_SERVERS_SERVERID_PERMISSIONS in set but failed match against \"{}\"", path, paths::REGEX_SERVICES_SERVICEID_SERVERS_SERVERID_PERMISSIONS.as_str())
                    );

                let param_service_id = match percent_encoding::percent_decode(path_params["serviceId"].as_bytes()).decode_utf8() {
                    Ok(param_service_id) => match param_service_id.parse::<i32>() {
                        Ok(param_service_id) => param_service_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter serviceId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["serviceId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_server_id = match percent_encoding::percent_decode(path_params["serverId"].as_bytes()).decode_utf8() {
                    Ok(param_server_id) => match param_server_id.parse::<i32>() {
                        Ok(param_server_id) => param_server_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter serverId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["serverId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_service_server_permissions(
                                            param_service_id,
                                            param_server_id,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetServiceServerPermissionsResponse::PermissionsWillBeProvided
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_SERVICE_SERVER_PERMISSIONS_PERMISSIONS_WILL_BE_PROVIDED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetServiceServerPermissionsResponse::YouDoNotHaveSufficientPermissions
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_SERVICE_SERVER_PERMISSIONS_YOU_DO_NOT_HAVE_SUFFICIENT_PERMISSIONS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetServiceServerServerCredentials - GET /services/{serviceId}/servers/{serverId}/serverCredentials
            &hyper::Method::GET if path.matched(paths::ID_SERVICES_SERVICEID_SERVERS_SERVERID_SERVERCREDENTIALS) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_SERVICES_SERVICEID_SERVERS_SERVERID_SERVERCREDENTIALS
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE SERVICES_SERVICEID_SERVERS_SERVERID_SERVERCREDENTIALS in set but failed match against \"{}\"", path, paths::REGEX_SERVICES_SERVICEID_SERVERS_SERVERID_SERVERCREDENTIALS.as_str())
                    );

                let param_service_id = match percent_encoding::percent_decode(path_params["serviceId"].as_bytes()).decode_utf8() {
                    Ok(param_service_id) => match param_service_id.parse::<i32>() {
                        Ok(param_service_id) => param_service_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter serviceId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["serviceId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_server_id = match percent_encoding::percent_decode(path_params["serverId"].as_bytes()).decode_utf8() {
                    Ok(param_server_id) => match param_server_id.parse::<i32>() {
                        Ok(param_server_id) => param_server_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter serverId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["serverId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_service_server_server_credentials(
                                            param_service_id,
                                            param_server_id,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetServiceServerServerCredentialsResponse::ServerCredentials
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_SERVICE_SERVER_SERVER_CREDENTIALS_SERVER_CREDENTIALS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetServiceServerServerCredentialsResponse::YouDoNotHaveSufficientPermissions
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_SERVICE_SERVER_SERVER_CREDENTIALS_YOU_DO_NOT_HAVE_SUFFICIENT_PERMISSIONS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetServiceServerServerCredentialsPermissions - GET /services/{serviceId}/servers/{serverId}/serverCredentials/permissions
            &hyper::Method::GET if path.matched(paths::ID_SERVICES_SERVICEID_SERVERS_SERVERID_SERVERCREDENTIALS_PERMISSIONS) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_SERVICES_SERVICEID_SERVERS_SERVERID_SERVERCREDENTIALS_PERMISSIONS
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE SERVICES_SERVICEID_SERVERS_SERVERID_SERVERCREDENTIALS_PERMISSIONS in set but failed match against \"{}\"", path, paths::REGEX_SERVICES_SERVICEID_SERVERS_SERVERID_SERVERCREDENTIALS_PERMISSIONS.as_str())
                    );

                let param_service_id = match percent_encoding::percent_decode(path_params["serviceId"].as_bytes()).decode_utf8() {
                    Ok(param_service_id) => match param_service_id.parse::<i32>() {
                        Ok(param_service_id) => param_service_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter serviceId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["serviceId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_server_id = match percent_encoding::percent_decode(path_params["serverId"].as_bytes()).decode_utf8() {
                    Ok(param_server_id) => match param_server_id.parse::<i32>() {
                        Ok(param_server_id) => param_server_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter serverId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["serverId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_service_server_server_credentials_permissions(
                                            param_service_id,
                                            param_server_id,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetServiceServerServerCredentialsPermissionsResponse::PermissionsWillBeProvided
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_SERVICE_SERVER_SERVER_CREDENTIALS_PERMISSIONS_PERMISSIONS_WILL_BE_PROVIDED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetServiceServerServerCredentialsPermissionsResponse::YouDoNotHaveSufficientPermissions
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_SERVICE_SERVER_SERVER_CREDENTIALS_PERMISSIONS_YOU_DO_NOT_HAVE_SUFFICIENT_PERMISSIONS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetServiceServers - GET /services/{serviceId}/servers
            &hyper::Method::GET if path.matched(paths::ID_SERVICES_SERVICEID_SERVERS) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_SERVICES_SERVICEID_SERVERS
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE SERVICES_SERVICEID_SERVERS in set but failed match against \"{}\"", path, paths::REGEX_SERVICES_SERVICEID_SERVERS.as_str())
                    );

                let param_service_id = match percent_encoding::percent_decode(path_params["serviceId"].as_bytes()).decode_utf8() {
                    Ok(param_service_id) => match param_service_id.parse::<i32>() {
                        Ok(param_service_id) => param_service_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter serviceId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["serviceId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_service_servers(
                                            param_service_id,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetServiceServersResponse::ListOfServersUnderAService
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_SERVICE_SERVERS_LIST_OF_SERVERS_UNDER_A_SERVICE"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetServiceServersResponse::YouDoNotHaveSufficientPermissions
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_SERVICE_SERVERS_YOU_DO_NOT_HAVE_SUFFICIENT_PERMISSIONS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // PostServiceServerValidate - POST /services/{serviceId}/servers/{serverId}/validate
            &hyper::Method::POST if path.matched(paths::ID_SERVICES_SERVICEID_SERVERS_SERVERID_VALIDATE) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_SERVICES_SERVICEID_SERVERS_SERVERID_VALIDATE
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE SERVICES_SERVICEID_SERVERS_SERVERID_VALIDATE in set but failed match against \"{}\"", path, paths::REGEX_SERVICES_SERVICEID_SERVERS_SERVERID_VALIDATE.as_str())
                    );

                let param_service_id = match percent_encoding::percent_decode(path_params["serviceId"].as_bytes()).decode_utf8() {
                    Ok(param_service_id) => match param_service_id.parse::<i32>() {
                        Ok(param_service_id) => param_service_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter serviceId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["serviceId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_server_id = match percent_encoding::percent_decode(path_params["serverId"].as_bytes()).decode_utf8() {
                    Ok(param_server_id) => match param_server_id.parse::<i32>() {
                        Ok(param_server_id) => param_server_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter serverId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["serverId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.post_service_server_validate(
                                            param_service_id,
                                            param_server_id,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PostServiceServerValidateResponse::ValidationWentSuccessfully
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                PostServiceServerValidateResponse::ThereWereSomethingWrongWithTheRequestOrConfiguration
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_SERVICE_SERVER_VALIDATE_THERE_WERE_SOMETHING_WRONG_WITH_THE_REQUEST_OR_CONFIGURATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                PostServiceServerValidateResponse::YouDoNotHaveSufficientPermissions
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_SERVICE_SERVER_VALIDATE_YOU_DO_NOT_HAVE_SUFFICIENT_PERMISSIONS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // PostServiceServers - POST /services/{serviceId}/servers
            &hyper::Method::POST if path.matched(paths::ID_SERVICES_SERVICEID_SERVERS) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_SERVICES_SERVICEID_SERVERS
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE SERVICES_SERVICEID_SERVERS in set but failed match against \"{}\"", path, paths::REGEX_SERVICES_SERVICEID_SERVERS.as_str())
                    );

                let param_service_id = match percent_encoding::percent_decode(path_params["serviceId"].as_bytes()).decode_utf8() {
                    Ok(param_service_id) => match param_service_id.parse::<i32>() {
                        Ok(param_service_id) => param_service_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter serviceId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["serviceId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.to_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_body: Option<models::PostNewServer> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_body) => param_body,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter body - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter body due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_body = match param_body {
                                    Some(param_body) => param_body,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter body"))
                                                        .expect("Unable to create Bad Request response for missing body parameter body")),
                                };

                                let result = api_impl.post_service_servers(
                                            param_service_id,
                                            param_body,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PostServiceServersResponse::ServerDetails
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(201).expect("Unable to turn 201 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_SERVICE_SERVERS_SERVER_DETAILS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                PostServiceServersResponse::YouDoNotHaveSufficientPermissions
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_SERVICE_SERVERS_YOU_DO_NOT_HAVE_SUFFICIENT_PERMISSIONS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter body: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter body")),
                        }
            },

            // PutServiceServer - PUT /services/{serviceId}/servers/{serverId}
            &hyper::Method::PUT if path.matched(paths::ID_SERVICES_SERVICEID_SERVERS_SERVERID) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_SERVICES_SERVICEID_SERVERS_SERVERID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE SERVICES_SERVICEID_SERVERS_SERVERID in set but failed match against \"{}\"", path, paths::REGEX_SERVICES_SERVICEID_SERVERS_SERVERID.as_str())
                    );

                let param_service_id = match percent_encoding::percent_decode(path_params["serviceId"].as_bytes()).decode_utf8() {
                    Ok(param_service_id) => match param_service_id.parse::<i32>() {
                        Ok(param_service_id) => param_service_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter serviceId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["serviceId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_server_id = match percent_encoding::percent_decode(path_params["serverId"].as_bytes()).decode_utf8() {
                    Ok(param_server_id) => match param_server_id.parse::<i32>() {
                        Ok(param_server_id) => param_server_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter serverId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["serverId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.to_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_body: Option<models::Server> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_body) => param_body,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter body - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter body due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_body = match param_body {
                                    Some(param_body) => param_body,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter body"))
                                                        .expect("Unable to create Bad Request response for missing body parameter body")),
                                };

                                let result = api_impl.put_service_server(
                                            param_service_id,
                                            param_server_id,
                                            param_body,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PutServiceServerResponse::ServerWasUpdatedSuccessfully
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                PutServiceServerResponse::ServerCan
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for PUT_SERVICE_SERVER_SERVER_CAN"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                PutServiceServerResponse::YouDoNotHaveSufficientPermissions
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for PUT_SERVICE_SERVER_YOU_DO_NOT_HAVE_SUFFICIENT_PERMISSIONS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter body: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter body")),
                        }
            },

            // PutServiceServerPermissions - PUT /services/{serviceId}/servers/{serverId}/permissions
            &hyper::Method::PUT if path.matched(paths::ID_SERVICES_SERVICEID_SERVERS_SERVERID_PERMISSIONS) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_SERVICES_SERVICEID_SERVERS_SERVERID_PERMISSIONS
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE SERVICES_SERVICEID_SERVERS_SERVERID_PERMISSIONS in set but failed match against \"{}\"", path, paths::REGEX_SERVICES_SERVICEID_SERVERS_SERVERID_PERMISSIONS.as_str())
                    );

                let param_service_id = match percent_encoding::percent_decode(path_params["serviceId"].as_bytes()).decode_utf8() {
                    Ok(param_service_id) => match param_service_id.parse::<i32>() {
                        Ok(param_service_id) => param_service_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter serviceId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["serviceId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_server_id = match percent_encoding::percent_decode(path_params["serverId"].as_bytes()).decode_utf8() {
                    Ok(param_server_id) => match param_server_id.parse::<i32>() {
                        Ok(param_server_id) => param_server_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter serverId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["serverId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.to_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_body: Option<models::Permissions> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_body) => param_body,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter body - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter body due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_body = match param_body {
                                    Some(param_body) => param_body,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter body"))
                                                        .expect("Unable to create Bad Request response for missing body parameter body")),
                                };

                                let result = api_impl.put_service_server_permissions(
                                            param_service_id,
                                            param_server_id,
                                            param_body,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PutServiceServerPermissionsResponse::PermissionsAlteredSuccessfully
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                PutServiceServerPermissionsResponse::YouDoNotHaveSufficientPermissions
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for PUT_SERVICE_SERVER_PERMISSIONS_YOU_DO_NOT_HAVE_SUFFICIENT_PERMISSIONS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter body: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter body")),
                        }
            },

            // PutServiceServerServerCredentials - PUT /services/{serviceId}/servers/{serverId}/serverCredentials
            &hyper::Method::PUT if path.matched(paths::ID_SERVICES_SERVICEID_SERVERS_SERVERID_SERVERCREDENTIALS) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_SERVICES_SERVICEID_SERVERS_SERVERID_SERVERCREDENTIALS
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE SERVICES_SERVICEID_SERVERS_SERVERID_SERVERCREDENTIALS in set but failed match against \"{}\"", path, paths::REGEX_SERVICES_SERVICEID_SERVERS_SERVERID_SERVERCREDENTIALS.as_str())
                    );

                let param_service_id = match percent_encoding::percent_decode(path_params["serviceId"].as_bytes()).decode_utf8() {
                    Ok(param_service_id) => match param_service_id.parse::<i32>() {
                        Ok(param_service_id) => param_service_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter serviceId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["serviceId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_server_id = match percent_encoding::percent_decode(path_params["serverId"].as_bytes()).decode_utf8() {
                    Ok(param_server_id) => match param_server_id.parse::<i32>() {
                        Ok(param_server_id) => param_server_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter serverId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["serverId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.to_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_body: Option<models::ServerCredentials> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_body) => param_body,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter body - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter body due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_body = match param_body {
                                    Some(param_body) => param_body,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter body"))
                                                        .expect("Unable to create Bad Request response for missing body parameter body")),
                                };

                                let result = api_impl.put_service_server_server_credentials(
                                            param_service_id,
                                            param_server_id,
                                            param_body,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PutServiceServerServerCredentialsResponse::ServerCredentialsAlteredSuccessfully
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                PutServiceServerServerCredentialsResponse::YouDoNotHaveSufficientPermissions
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for PUT_SERVICE_SERVER_SERVER_CREDENTIALS_YOU_DO_NOT_HAVE_SUFFICIENT_PERMISSIONS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter body: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter body")),
                        }
            },

            // PutServiceServerServerCredentialsPermissions - PUT /services/{serviceId}/servers/{serverId}/serverCredentials/permissions
            &hyper::Method::PUT if path.matched(paths::ID_SERVICES_SERVICEID_SERVERS_SERVERID_SERVERCREDENTIALS_PERMISSIONS) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_SERVICES_SERVICEID_SERVERS_SERVERID_SERVERCREDENTIALS_PERMISSIONS
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE SERVICES_SERVICEID_SERVERS_SERVERID_SERVERCREDENTIALS_PERMISSIONS in set but failed match against \"{}\"", path, paths::REGEX_SERVICES_SERVICEID_SERVERS_SERVERID_SERVERCREDENTIALS_PERMISSIONS.as_str())
                    );

                let param_service_id = match percent_encoding::percent_decode(path_params["serviceId"].as_bytes()).decode_utf8() {
                    Ok(param_service_id) => match param_service_id.parse::<i32>() {
                        Ok(param_service_id) => param_service_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter serviceId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["serviceId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_server_id = match percent_encoding::percent_decode(path_params["serverId"].as_bytes()).decode_utf8() {
                    Ok(param_server_id) => match param_server_id.parse::<i32>() {
                        Ok(param_server_id) => param_server_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter serverId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["serverId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.to_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_body: Option<models::Permissions> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_body) => param_body,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter body - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter body due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_body = match param_body {
                                    Some(param_body) => param_body,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter body"))
                                                        .expect("Unable to create Bad Request response for missing body parameter body")),
                                };

                                let result = api_impl.put_service_server_server_credentials_permissions(
                                            param_service_id,
                                            param_server_id,
                                            param_body,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PutServiceServerServerCredentialsPermissionsResponse::PermissionsAlteredSuccessfully
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                PutServiceServerServerCredentialsPermissionsResponse::YouDoNotHaveSufficientPermissions
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for PUT_SERVICE_SERVER_SERVER_CREDENTIALS_PERMISSIONS_YOU_DO_NOT_HAVE_SUFFICIENT_PERMISSIONS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter body: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter body")),
                        }
            },

            // DeleteServiceModule - DELETE /modules/serviceModules/{serviceModuleUuid}
            &hyper::Method::DELETE if path.matched(paths::ID_MODULES_SERVICEMODULES_SERVICEMODULEUUID) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_MODULES_SERVICEMODULES_SERVICEMODULEUUID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MODULES_SERVICEMODULES_SERVICEMODULEUUID in set but failed match against \"{}\"", path, paths::REGEX_MODULES_SERVICEMODULES_SERVICEMODULEUUID.as_str())
                    );

                let param_service_module_uuid = match percent_encoding::percent_decode(path_params["serviceModuleUuid"].as_bytes()).decode_utf8() {
                    Ok(param_service_module_uuid) => match param_service_module_uuid.parse::<uuid::Uuid>() {
                        Ok(param_service_module_uuid) => param_service_module_uuid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter serviceModuleUuid: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["serviceModuleUuid"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.delete_service_module(
                                            param_service_module_uuid,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                DeleteServiceModuleResponse::ServiceModuleDeleted
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                DeleteServiceModuleResponse::YouDoNotHaveSufficientPermissions
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DELETE_SERVICE_MODULE_YOU_DO_NOT_HAVE_SUFFICIENT_PERMISSIONS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetServiceModule - GET /modules/serviceModules/{serviceModuleUuid}
            &hyper::Method::GET if path.matched(paths::ID_MODULES_SERVICEMODULES_SERVICEMODULEUUID) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_MODULES_SERVICEMODULES_SERVICEMODULEUUID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MODULES_SERVICEMODULES_SERVICEMODULEUUID in set but failed match against \"{}\"", path, paths::REGEX_MODULES_SERVICEMODULES_SERVICEMODULEUUID.as_str())
                    );

                let param_service_module_uuid = match percent_encoding::percent_decode(path_params["serviceModuleUuid"].as_bytes()).decode_utf8() {
                    Ok(param_service_module_uuid) => match param_service_module_uuid.parse::<uuid::Uuid>() {
                        Ok(param_service_module_uuid) => param_service_module_uuid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter serviceModuleUuid: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["serviceModuleUuid"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_service_module(
                                            param_service_module_uuid,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetServiceModuleResponse::SpecificServiceModuleDetails
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_SERVICE_MODULE_SPECIFIC_SERVICE_MODULE_DETAILS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetServiceModuleResponse::YouDoNotHaveSufficientPermissions
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_SERVICE_MODULE_YOU_DO_NOT_HAVE_SUFFICIENT_PERMISSIONS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetServiceModulePermissions - GET /modules/serviceModules/{serviceModuleUuid}/permissions
            &hyper::Method::GET if path.matched(paths::ID_MODULES_SERVICEMODULES_SERVICEMODULEUUID_PERMISSIONS) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_MODULES_SERVICEMODULES_SERVICEMODULEUUID_PERMISSIONS
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MODULES_SERVICEMODULES_SERVICEMODULEUUID_PERMISSIONS in set but failed match against \"{}\"", path, paths::REGEX_MODULES_SERVICEMODULES_SERVICEMODULEUUID_PERMISSIONS.as_str())
                    );

                let param_service_module_uuid = match percent_encoding::percent_decode(path_params["serviceModuleUuid"].as_bytes()).decode_utf8() {
                    Ok(param_service_module_uuid) => match param_service_module_uuid.parse::<uuid::Uuid>() {
                        Ok(param_service_module_uuid) => param_service_module_uuid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter serviceModuleUuid: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["serviceModuleUuid"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_service_module_permissions(
                                            param_service_module_uuid,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetServiceModulePermissionsResponse::PermissionsWillBeProvided
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_SERVICE_MODULE_PERMISSIONS_PERMISSIONS_WILL_BE_PROVIDED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetServiceModulePermissionsResponse::YouDoNotHaveSufficientPermissions
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_SERVICE_MODULE_PERMISSIONS_YOU_DO_NOT_HAVE_SUFFICIENT_PERMISSIONS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetServiceModules - GET /modules/serviceModules
            &hyper::Method::GET if path.matched(paths::ID_MODULES_SERVICEMODULES) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                                let result = api_impl.get_service_modules(
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetServiceModulesResponse::ListOfAllServiceModules
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_SERVICE_MODULES_LIST_OF_ALL_SERVICE_MODULES"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetServiceModulesResponse::YouDoNotHaveSufficientPermissions
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_SERVICE_MODULES_YOU_DO_NOT_HAVE_SUFFICIENT_PERMISSIONS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // PostServiceModule - POST /modules/serviceModules/{serviceModuleUuid}
            &hyper::Method::POST if path.matched(paths::ID_MODULES_SERVICEMODULES_SERVICEMODULEUUID) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_MODULES_SERVICEMODULES_SERVICEMODULEUUID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MODULES_SERVICEMODULES_SERVICEMODULEUUID in set but failed match against \"{}\"", path, paths::REGEX_MODULES_SERVICEMODULES_SERVICEMODULEUUID.as_str())
                    );

                let param_service_module_uuid = match percent_encoding::percent_decode(path_params["serviceModuleUuid"].as_bytes()).decode_utf8() {
                    Ok(param_service_module_uuid) => match param_service_module_uuid.parse::<uuid::Uuid>() {
                        Ok(param_service_module_uuid) => param_service_module_uuid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter serviceModuleUuid: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["serviceModuleUuid"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.to_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_body: Option<models::ServiceModule> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_body) => param_body,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter body - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter body due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_body = match param_body {
                                    Some(param_body) => param_body,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter body"))
                                                        .expect("Unable to create Bad Request response for missing body parameter body")),
                                };

                                let result = api_impl.post_service_module(
                                            param_service_module_uuid,
                                            param_body,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PostServiceModuleResponse::ServiceModuleAltered
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                PostServiceModuleResponse::YouDoNotHaveSufficientPermissions
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_SERVICE_MODULE_YOU_DO_NOT_HAVE_SUFFICIENT_PERMISSIONS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter body: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter body")),
                        }
            },

            // PostServiceModules - POST /modules/serviceModules
            &hyper::Method::POST if path.matched(paths::ID_MODULES_SERVICEMODULES) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.to_raw().await;
                match result {
                            Ok(body) => {
                                let param_body: Option<swagger::ByteArray> = if !body.is_empty() {
                                } else {
                                    None
                                };
                                let param_body = match param_body {
                                    Some(param_body) => param_body,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter body"))
                                                        .expect("Unable to create Bad Request response for missing body parameter body")),
                                };

                                let result = api_impl.post_service_modules(
                                            param_body,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PostServiceModulesResponse::SpecificServiceModuleDetails
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(201).expect("Unable to turn 201 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_SERVICE_MODULES_SPECIFIC_SERVICE_MODULE_DETAILS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                PostServiceModulesResponse::YouDoNotHaveSufficientPermissions
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_SERVICE_MODULES_YOU_DO_NOT_HAVE_SUFFICIENT_PERMISSIONS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter body: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter body")),
                        }
            },

            // PutServiceModulePermissions - PUT /modules/serviceModules/{serviceModuleUuid}/permissions
            &hyper::Method::PUT if path.matched(paths::ID_MODULES_SERVICEMODULES_SERVICEMODULEUUID_PERMISSIONS) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_MODULES_SERVICEMODULES_SERVICEMODULEUUID_PERMISSIONS
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MODULES_SERVICEMODULES_SERVICEMODULEUUID_PERMISSIONS in set but failed match against \"{}\"", path, paths::REGEX_MODULES_SERVICEMODULES_SERVICEMODULEUUID_PERMISSIONS.as_str())
                    );

                let param_service_module_uuid = match percent_encoding::percent_decode(path_params["serviceModuleUuid"].as_bytes()).decode_utf8() {
                    Ok(param_service_module_uuid) => match param_service_module_uuid.parse::<uuid::Uuid>() {
                        Ok(param_service_module_uuid) => param_service_module_uuid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter serviceModuleUuid: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["serviceModuleUuid"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.to_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_body: Option<models::Permissions> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_body) => param_body,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter body - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter body due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_body = match param_body {
                                    Some(param_body) => param_body,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter body"))
                                                        .expect("Unable to create Bad Request response for missing body parameter body")),
                                };

                                let result = api_impl.put_service_module_permissions(
                                            param_service_module_uuid,
                                            param_body,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PutServiceModulePermissionsResponse::PermissionsAlteredSuccessfully
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                PutServiceModulePermissionsResponse::YouDoNotHaveSufficientPermissions
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for PUT_SERVICE_MODULE_PERMISSIONS_YOU_DO_NOT_HAVE_SUFFICIENT_PERMISSIONS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter body: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter body")),
                        }
            },

            // CreateService - POST /services
            &hyper::Method::POST if path.matched(paths::ID_SERVICES) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.to_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_body: Option<models::PostNewService> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_body) => param_body,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter body - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter body due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_body = match param_body {
                                    Some(param_body) => param_body,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter body"))
                                                        .expect("Unable to create Bad Request response for missing body parameter body")),
                                };

                                let result = api_impl.create_service(
                                            param_body,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                CreateServiceResponse::ServiceDetails
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(201).expect("Unable to turn 201 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for CREATE_SERVICE_SERVICE_DETAILS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                CreateServiceResponse::YouDoNotHaveSufficientPermissions
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for CREATE_SERVICE_YOU_DO_NOT_HAVE_SUFFICIENT_PERMISSIONS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter body: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter body")),
                        }
            },

            // DeleteService - DELETE /services/{serviceId}
            &hyper::Method::DELETE if path.matched(paths::ID_SERVICES_SERVICEID) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_SERVICES_SERVICEID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE SERVICES_SERVICEID in set but failed match against \"{}\"", path, paths::REGEX_SERVICES_SERVICEID.as_str())
                    );

                let param_service_id = match percent_encoding::percent_decode(path_params["serviceId"].as_bytes()).decode_utf8() {
                    Ok(param_service_id) => match param_service_id.parse::<i32>() {
                        Ok(param_service_id) => param_service_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter serviceId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["serviceId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.delete_service(
                                            param_service_id,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                DeleteServiceResponse::DeleteOperationWasSuccessful
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                DeleteServiceResponse::ServiceCan
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DELETE_SERVICE_SERVICE_CAN"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                DeleteServiceResponse::YouDoNotHaveSufficientPermissions
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DELETE_SERVICE_YOU_DO_NOT_HAVE_SUFFICIENT_PERMISSIONS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetService - GET /services/{serviceId}
            &hyper::Method::GET if path.matched(paths::ID_SERVICES_SERVICEID) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_SERVICES_SERVICEID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE SERVICES_SERVICEID in set but failed match against \"{}\"", path, paths::REGEX_SERVICES_SERVICEID.as_str())
                    );

                let param_service_id = match percent_encoding::percent_decode(path_params["serviceId"].as_bytes()).decode_utf8() {
                    Ok(param_service_id) => match param_service_id.parse::<i32>() {
                        Ok(param_service_id) => param_service_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter serviceId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["serviceId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_service(
                                            param_service_id,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetServiceResponse::ServiceDetails
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_SERVICE_SERVICE_DETAILS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetServiceResponse::YouDoNotHaveSufficientPermissions
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_SERVICE_YOU_DO_NOT_HAVE_SUFFICIENT_PERMISSIONS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetServicePermissions - GET /services/{serviceId}/permissions
            &hyper::Method::GET if path.matched(paths::ID_SERVICES_SERVICEID_PERMISSIONS) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_SERVICES_SERVICEID_PERMISSIONS
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE SERVICES_SERVICEID_PERMISSIONS in set but failed match against \"{}\"", path, paths::REGEX_SERVICES_SERVICEID_PERMISSIONS.as_str())
                    );

                let param_service_id = match percent_encoding::percent_decode(path_params["serviceId"].as_bytes()).decode_utf8() {
                    Ok(param_service_id) => match param_service_id.parse::<i32>() {
                        Ok(param_service_id) => param_service_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter serviceId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["serviceId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_service_permissions(
                                            param_service_id,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetServicePermissionsResponse::PermissionsWillBeProvided
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_SERVICE_PERMISSIONS_PERMISSIONS_WILL_BE_PROVIDED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetServicePermissionsResponse::YouDoNotHaveSufficientPermissions
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_SERVICE_PERMISSIONS_YOU_DO_NOT_HAVE_SUFFICIENT_PERMISSIONS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetServices - GET /services
            &hyper::Method::GET if path.matched(paths::ID_SERVICES) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_port = query_params.iter().filter(|e| e.0 == "port").map(|e| e.1.to_owned())
                    .nth(0);
                let param_port = match param_port {
                    Some(param_port) => {
                        let param_port =
                            <i32 as std::str::FromStr>::from_str
                                (&param_port);
                        match param_port {
                            Ok(param_port) => Some(param_port),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter port - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter port")),
                        }
                    },
                    None => None,
                };
                let param_dns = query_params.iter().filter(|e| e.0 == "dns").map(|e| e.1.to_owned())
                    .nth(0);
                let param_dns = match param_dns {
                    Some(param_dns) => {
                        let param_dns =
                            <String as std::str::FromStr>::from_str
                                (&param_dns);
                        match param_dns {
                            Ok(param_dns) => Some(param_dns),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter dns - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter dns")),
                        }
                    },
                    None => None,
                };

                                let result = api_impl.get_services(
                                            param_port,
                                            param_dns,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetServicesResponse::ValidRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_SERVICES_VALID_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetServicesResponse::YouDoNotHaveSufficientPermissions
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_SERVICES_YOU_DO_NOT_HAVE_SUFFICIENT_PERMISSIONS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetServicesPermissions - GET /services/permissions
            &hyper::Method::GET if path.matched(paths::ID_SERVICES_PERMISSIONS) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                                let result = api_impl.get_services_permissions(
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetServicesPermissionsResponse::PermissionsWillBeProvided
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_SERVICES_PERMISSIONS_PERMISSIONS_WILL_BE_PROVIDED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetServicesPermissionsResponse::YouDoNotHaveSufficientPermissions
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_SERVICES_PERMISSIONS_YOU_DO_NOT_HAVE_SUFFICIENT_PERMISSIONS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // PostService - POST /services/{serviceId}
            &hyper::Method::POST if path.matched(paths::ID_SERVICES_SERVICEID) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_SERVICES_SERVICEID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE SERVICES_SERVICEID in set but failed match against \"{}\"", path, paths::REGEX_SERVICES_SERVICEID.as_str())
                    );

                let param_service_id = match percent_encoding::percent_decode(path_params["serviceId"].as_bytes()).decode_utf8() {
                    Ok(param_service_id) => match param_service_id.parse::<i32>() {
                        Ok(param_service_id) => param_service_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter serviceId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["serviceId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.to_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_body: Option<models::Service> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_body) => param_body,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter body - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter body due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_body = match param_body {
                                    Some(param_body) => param_body,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter body"))
                                                        .expect("Unable to create Bad Request response for missing body parameter body")),
                                };

                                let result = api_impl.post_service(
                                            param_service_id,
                                            param_body,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PostServiceResponse::ServiceDetails
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_SERVICE_SERVICE_DETAILS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                PostServiceResponse::ServiceIDOrServiceModuleIDCan
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_SERVICE_SERVICE_IDOR_SERVICE_MODULE_ID_CAN"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                PostServiceResponse::YouDoNotHaveSufficientPermissions
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_SERVICE_YOU_DO_NOT_HAVE_SUFFICIENT_PERMISSIONS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter body: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter body")),
                        }
            },

            // PutServicePermissions - PUT /services/{serviceId}/permissions
            &hyper::Method::PUT if path.matched(paths::ID_SERVICES_SERVICEID_PERMISSIONS) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_SERVICES_SERVICEID_PERMISSIONS
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE SERVICES_SERVICEID_PERMISSIONS in set but failed match against \"{}\"", path, paths::REGEX_SERVICES_SERVICEID_PERMISSIONS.as_str())
                    );

                let param_service_id = match percent_encoding::percent_decode(path_params["serviceId"].as_bytes()).decode_utf8() {
                    Ok(param_service_id) => match param_service_id.parse::<i32>() {
                        Ok(param_service_id) => param_service_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter serviceId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["serviceId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.to_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_body: Option<models::Permissions> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_body) => param_body,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter body - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter body due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_body = match param_body {
                                    Some(param_body) => param_body,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter body"))
                                                        .expect("Unable to create Bad Request response for missing body parameter body")),
                                };

                                let result = api_impl.put_service_permissions(
                                            param_service_id,
                                            param_body,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PutServicePermissionsResponse::PermissionsAlteredSuccessfully
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                PutServicePermissionsResponse::YouDoNotHaveSufficientPermissions
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for PUT_SERVICE_PERMISSIONS_YOU_DO_NOT_HAVE_SUFFICIENT_PERMISSIONS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter body: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter body")),
                        }
            },

            // PutServicesPermissions - PUT /services/permissions
            &hyper::Method::PUT if path.matched(paths::ID_SERVICES_PERMISSIONS) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.to_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_body: Option<models::Permissions> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_body) => param_body,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter body - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter body due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_body = match param_body {
                                    Some(param_body) => param_body,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter body"))
                                                        .expect("Unable to create Bad Request response for missing body parameter body")),
                                };

                                let result = api_impl.put_services_permissions(
                                            param_body,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PutServicesPermissionsResponse::PermissionsAlteredSuccessfully
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                PutServicesPermissionsResponse::YouDoNotHaveSufficientPermissions
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for PUT_SERVICES_PERMISSIONS_YOU_DO_NOT_HAVE_SUFFICIENT_PERMISSIONS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter body: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter body")),
                        }
            },

            // DeleteUser - DELETE /users/{userId}
            &hyper::Method::DELETE if path.matched(paths::ID_USERS_USERID) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_USERS_USERID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE USERS_USERID in set but failed match against \"{}\"", path, paths::REGEX_USERS_USERID.as_str())
                    );

                let param_user_id = match percent_encoding::percent_decode(path_params["userId"].as_bytes()).decode_utf8() {
                    Ok(param_user_id) => match param_user_id.parse::<i32>() {
                        Ok(param_user_id) => param_user_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter userId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["userId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.delete_user(
                                            param_user_id,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                DeleteUserResponse::UserDeleted
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                DeleteUserResponse::YouDoNotHaveSufficientPermissions
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DELETE_USER_YOU_DO_NOT_HAVE_SUFFICIENT_PERMISSIONS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetUser - GET /users/{userId}
            &hyper::Method::GET if path.matched(paths::ID_USERS_USERID) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_USERS_USERID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE USERS_USERID in set but failed match against \"{}\"", path, paths::REGEX_USERS_USERID.as_str())
                    );

                let param_user_id = match percent_encoding::percent_decode(path_params["userId"].as_bytes()).decode_utf8() {
                    Ok(param_user_id) => match param_user_id.parse::<i32>() {
                        Ok(param_user_id) => param_user_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter userId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["userId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_user(
                                            param_user_id,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetUserResponse::DetailsOfTheUser
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_USER_DETAILS_OF_THE_USER"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetUserResponse::YouDoNotHaveSufficientPermissions
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_USER_YOU_DO_NOT_HAVE_SUFFICIENT_PERMISSIONS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetUsers - GET /users
            &hyper::Method::GET if path.matched(paths::ID_USERS) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                                let result = api_impl.get_users(
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetUsersResponse::ListOfAllUsers
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_USERS_LIST_OF_ALL_USERS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetUsersResponse::YouDoNotHaveSufficientPermissions
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_USERS_YOU_DO_NOT_HAVE_SUFFICIENT_PERMISSIONS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // PostUser - POST /users
            &hyper::Method::POST if path.matched(paths::ID_USERS) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.to_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_body: Option<models::UserContent> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_body) => param_body,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter body - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter body due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_body = match param_body {
                                    Some(param_body) => param_body,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter body"))
                                                        .expect("Unable to create Bad Request response for missing body parameter body")),
                                };

                                let result = api_impl.post_user(
                                            param_body,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PostUserResponse::UserCreatedSuccessfully
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(201).expect("Unable to turn 201 into a StatusCode");
                                                },
                                                PostUserResponse::YouDoNotHaveSufficientPermissions
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_USER_YOU_DO_NOT_HAVE_SUFFICIENT_PERMISSIONS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter body: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter body")),
                        }
            },

            // PostUserLogin - POST /users/login
            &hyper::Method::POST if path.matched(paths::ID_USERS_LOGIN) => {
                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.to_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_body: Option<models::ExistingUserCredentials> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_body) => param_body,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter body - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter body due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_body = match param_body {
                                    Some(param_body) => param_body,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter body"))
                                                        .expect("Unable to create Bad Request response for missing body parameter body")),
                                };

                                let result = api_impl.post_user_login(
                                            param_body,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PostUserLoginResponse::UserAuthenticatedSuccessfully
                                                    {
                                                        body,
                                                        set_cookie
                                                    }
                                                => {
                                                    if let Some(set_cookie) = set_cookie {
                                                    let set_cookie = match header::IntoHeaderValue(set_cookie).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Ok(Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(Body::from(format!("An internal server error occurred handling set_cookie header - {}", e)))
                                                                    .expect("Unable to create Internal Server Error for invalid response header"))
                                                        }
                                                    };

                                                    response.headers_mut().insert(
                                                        HeaderName::from_static("set-cookie"),
                                                        set_cookie
                                                    );
                                                    }
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_USER_LOGIN_USER_AUTHENTICATED_SUCCESSFULLY"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                PostUserLoginResponse::LoginRequiredTwoFactorAuthentication
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_USER_LOGIN_LOGIN_REQUIRED_TWO_FACTOR_AUTHENTICATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter body: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter body")),
                        }
            },

            // PutUser - PUT /users/{userId}
            &hyper::Method::PUT if path.matched(paths::ID_USERS_USERID) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_USERS_USERID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE USERS_USERID in set but failed match against \"{}\"", path, paths::REGEX_USERS_USERID.as_str())
                    );

                let param_user_id = match percent_encoding::percent_decode(path_params["userId"].as_bytes()).decode_utf8() {
                    Ok(param_user_id) => match param_user_id.parse::<i32>() {
                        Ok(param_user_id) => param_user_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter userId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["userId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.to_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_body: Option<models::User> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_body) => param_body,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter body - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter body due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_body = match param_body {
                                    Some(param_body) => param_body,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter body"))
                                                        .expect("Unable to create Bad Request response for missing body parameter body")),
                                };

                                let result = api_impl.put_user(
                                            param_user_id,
                                            param_body,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PutUserResponse::UserWasAlteredSuccessfully
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                PutUserResponse::YouDoNotHaveSufficientPermissions
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for PUT_USER_YOU_DO_NOT_HAVE_SUFFICIENT_PERMISSIONS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter body: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter body")),
                        }
            },

            _ if path.matched(paths::ID_CONSOLEMANAGEMENT) => method_not_allowed(),
            _ if path.matched(paths::ID_GROUPS) => method_not_allowed(),
            _ if path.matched(paths::ID_GROUPS_GROUPID) => method_not_allowed(),
            _ if path.matched(paths::ID_MODULES_CLOUDMODULES) => method_not_allowed(),
            _ if path.matched(paths::ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID) => method_not_allowed(),
            _ if path.matched(paths::ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID_ACCOUNTS) => method_not_allowed(),
            _ if path.matched(paths::ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID_ACCOUNTS_CLOUDACCOUNTID) => method_not_allowed(),
            _ if path.matched(paths::ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID_ACCOUNTS_CLOUDACCOUNTID_PERMISSIONS) => method_not_allowed(),
            _ if path.matched(paths::ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID_PERMISSIONS) => method_not_allowed(),
            _ if path.matched(paths::ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID_QUERIES_CPUCORES) => method_not_allowed(),
            _ if path.matched(paths::ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID_QUERIES_DISKBOUNDS) => method_not_allowed(),
            _ if path.matched(paths::ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID_QUERIES_MACHINETYPES) => method_not_allowed(),
            _ if path.matched(paths::ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID_QUERIES_OPERATINGSYSTEMS) => method_not_allowed(),
            _ if path.matched(paths::ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID_QUERIES_ORIENTATIONS) => method_not_allowed(),
            _ if path.matched(paths::ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID_QUERIES_RAMBOUNDS) => method_not_allowed(),
            _ if path.matched(paths::ID_MODULES_SERVICEMODULES) => method_not_allowed(),
            _ if path.matched(paths::ID_MODULES_SERVICEMODULES_SERVICEMODULEUUID) => method_not_allowed(),
            _ if path.matched(paths::ID_MODULES_SERVICEMODULES_SERVICEMODULEUUID_PERMISSIONS) => method_not_allowed(),
            _ if path.matched(paths::ID_SERVICES) => method_not_allowed(),
            _ if path.matched(paths::ID_SERVICES_PERMISSIONS) => method_not_allowed(),
            _ if path.matched(paths::ID_SERVICES_SERVICEID) => method_not_allowed(),
            _ if path.matched(paths::ID_SERVICES_SERVICEID_PERMISSIONS) => method_not_allowed(),
            _ if path.matched(paths::ID_SERVICES_SERVICEID_SERVERS) => method_not_allowed(),
            _ if path.matched(paths::ID_SERVICES_SERVICEID_SERVERS_SERVERID) => method_not_allowed(),
            _ if path.matched(paths::ID_SERVICES_SERVICEID_SERVERS_SERVERID_PERMISSIONS) => method_not_allowed(),
            _ if path.matched(paths::ID_SERVICES_SERVICEID_SERVERS_SERVERID_SERVERCREDENTIALS) => method_not_allowed(),
            _ if path.matched(paths::ID_SERVICES_SERVICEID_SERVERS_SERVERID_SERVERCREDENTIALS_PERMISSIONS) => method_not_allowed(),
            _ if path.matched(paths::ID_SERVICES_SERVICEID_SERVERS_SERVERID_VALIDATE) => method_not_allowed(),
            _ if path.matched(paths::ID_USERS) => method_not_allowed(),
            _ if path.matched(paths::ID_USERS_LOGIN) => method_not_allowed(),
            _ if path.matched(paths::ID_USERS_USERID) => method_not_allowed(),
            _ => Ok(Response::builder().status(StatusCode::NOT_FOUND)
                    .body(Body::empty())
                    .expect("Unable to create Not Found response"))
        }
    } Box::pin(run(self.api_impl.clone(), req)) }
}

/// Request parser for `Api`.
pub struct ApiRequestParser;
impl<T> RequestParser<T> for ApiRequestParser {
    fn parse_operation_id(request: &Request<T>) -> Result<&'static str, ()> {
        let path = paths::GLOBAL_REGEX_SET.matches(request.uri().path());
        match request.method() {
            // DeleteCloudModule - DELETE /modules/cloudModules/{cloudModuleUuid}
            &hyper::Method::DELETE if path.matched(paths::ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID) => Ok("DeleteCloudModule"),
            // DeleteCloudModuleAccount - DELETE /modules/cloudModules/{cloudModuleUuid}/accounts/{cloudAccountId}
            &hyper::Method::DELETE if path.matched(paths::ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID_ACCOUNTS_CLOUDACCOUNTID) => Ok("DeleteCloudModuleAccount"),
            // GetCloudModule - GET /modules/cloudModules/{cloudModuleUuid}
            &hyper::Method::GET if path.matched(paths::ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID) => Ok("GetCloudModule"),
            // GetCloudModuleAccount - GET /modules/cloudModules/{cloudModuleUuid}/accounts/{cloudAccountId}
            &hyper::Method::GET if path.matched(paths::ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID_ACCOUNTS_CLOUDACCOUNTID) => Ok("GetCloudModuleAccount"),
            // GetCloudModuleAccountPermissions - GET /modules/cloudModules/{cloudModuleUuid}/accounts/{cloudAccountId}/permissions
            &hyper::Method::GET if path.matched(paths::ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID_ACCOUNTS_CLOUDACCOUNTID_PERMISSIONS) => Ok("GetCloudModuleAccountPermissions"),
            // GetCloudModuleAccounts - GET /modules/cloudModules/{cloudModuleUuid}/accounts
            &hyper::Method::GET if path.matched(paths::ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID_ACCOUNTS) => Ok("GetCloudModuleAccounts"),
            // GetCloudModulePermissions - GET /modules/cloudModules/{cloudModuleUuid}/permissions
            &hyper::Method::GET if path.matched(paths::ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID_PERMISSIONS) => Ok("GetCloudModulePermissions"),
            // GetCloudModules - GET /modules/cloudModules
            &hyper::Method::GET if path.matched(paths::ID_MODULES_CLOUDMODULES) => Ok("GetCloudModules"),
            // PostCloudModule - POST /modules/cloudModules/{cloudModuleUuid}
            &hyper::Method::POST if path.matched(paths::ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID) => Ok("PostCloudModule"),
            // PostCloudModuleAccounts - POST /modules/cloudModules/{cloudModuleUuid}/accounts
            &hyper::Method::POST if path.matched(paths::ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID_ACCOUNTS) => Ok("PostCloudModuleAccounts"),
            // PostCloudModules - POST /modules/cloudModules
            &hyper::Method::POST if path.matched(paths::ID_MODULES_CLOUDMODULES) => Ok("PostCloudModules"),
            // PutCloudModuleAccount - PUT /modules/cloudModules/{cloudModuleUuid}/accounts/{cloudAccountId}
            &hyper::Method::PUT if path.matched(paths::ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID_ACCOUNTS_CLOUDACCOUNTID) => Ok("PutCloudModuleAccount"),
            // PutCloudModuleAccountPermissions - PUT /modules/cloudModules/{cloudModuleUuid}/accounts/{cloudAccountId}/permissions
            &hyper::Method::PUT if path.matched(paths::ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID_ACCOUNTS_CLOUDACCOUNTID_PERMISSIONS) => Ok("PutCloudModuleAccountPermissions"),
            // PutCloudModulePermissions - PUT /modules/cloudModules/{cloudModuleUuid}/permissions
            &hyper::Method::PUT if path.matched(paths::ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID_PERMISSIONS) => Ok("PutCloudModulePermissions"),
            // GetCpuCores - GET /modules/cloudModules/{cloudModuleUuid}/queries/cpuCores
            &hyper::Method::GET if path.matched(paths::ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID_QUERIES_CPUCORES) => Ok("GetCpuCores"),
            // GetDiskBounds - GET /modules/cloudModules/{cloudModuleUuid}/queries/diskBounds
            &hyper::Method::GET if path.matched(paths::ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID_QUERIES_DISKBOUNDS) => Ok("GetDiskBounds"),
            // GetMachineTypes - GET /modules/cloudModules/{cloudModuleUuid}/queries/machineTypes
            &hyper::Method::GET if path.matched(paths::ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID_QUERIES_MACHINETYPES) => Ok("GetMachineTypes"),
            // GetOperatingSystems - GET /modules/cloudModules/{cloudModuleUuid}/queries/operatingSystems
            &hyper::Method::GET if path.matched(paths::ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID_QUERIES_OPERATINGSYSTEMS) => Ok("GetOperatingSystems"),
            // GetOrientations - GET /modules/cloudModules/{cloudModuleUuid}/queries/orientations
            &hyper::Method::GET if path.matched(paths::ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID_QUERIES_ORIENTATIONS) => Ok("GetOrientations"),
            // GetRamBounds - GET /modules/cloudModules/{cloudModuleUuid}/queries/ramBounds
            &hyper::Method::GET if path.matched(paths::ID_MODULES_CLOUDMODULES_CLOUDMODULEUUID_QUERIES_RAMBOUNDS) => Ok("GetRamBounds"),
            // DeleteGroup - DELETE /groups/{groupId}
            &hyper::Method::DELETE if path.matched(paths::ID_GROUPS_GROUPID) => Ok("DeleteGroup"),
            // GetGroup - GET /groups/{groupId}
            &hyper::Method::GET if path.matched(paths::ID_GROUPS_GROUPID) => Ok("GetGroup"),
            // GetGroups - GET /groups
            &hyper::Method::GET if path.matched(paths::ID_GROUPS) => Ok("GetGroups"),
            // PostGroup - POST /groups
            &hyper::Method::POST if path.matched(paths::ID_GROUPS) => Ok("PostGroup"),
            // PutGroup - PUT /groups/{groupId}
            &hyper::Method::PUT if path.matched(paths::ID_GROUPS_GROUPID) => Ok("PutGroup"),
            // GetConsoleManagement - GET /consoleManagement
            &hyper::Method::GET if path.matched(paths::ID_CONSOLEMANAGEMENT) => Ok("GetConsoleManagement"),
            // PostConsoleManagement - POST /consoleManagement
            &hyper::Method::POST if path.matched(paths::ID_CONSOLEMANAGEMENT) => Ok("PostConsoleManagement"),
            // GetServiceServer - GET /services/{serviceId}/servers/{serverId}
            &hyper::Method::GET if path.matched(paths::ID_SERVICES_SERVICEID_SERVERS_SERVERID) => Ok("GetServiceServer"),
            // GetServiceServerPermissions - GET /services/{serviceId}/servers/{serverId}/permissions
            &hyper::Method::GET if path.matched(paths::ID_SERVICES_SERVICEID_SERVERS_SERVERID_PERMISSIONS) => Ok("GetServiceServerPermissions"),
            // GetServiceServerServerCredentials - GET /services/{serviceId}/servers/{serverId}/serverCredentials
            &hyper::Method::GET if path.matched(paths::ID_SERVICES_SERVICEID_SERVERS_SERVERID_SERVERCREDENTIALS) => Ok("GetServiceServerServerCredentials"),
            // GetServiceServerServerCredentialsPermissions - GET /services/{serviceId}/servers/{serverId}/serverCredentials/permissions
            &hyper::Method::GET if path.matched(paths::ID_SERVICES_SERVICEID_SERVERS_SERVERID_SERVERCREDENTIALS_PERMISSIONS) => Ok("GetServiceServerServerCredentialsPermissions"),
            // GetServiceServers - GET /services/{serviceId}/servers
            &hyper::Method::GET if path.matched(paths::ID_SERVICES_SERVICEID_SERVERS) => Ok("GetServiceServers"),
            // PostServiceServerValidate - POST /services/{serviceId}/servers/{serverId}/validate
            &hyper::Method::POST if path.matched(paths::ID_SERVICES_SERVICEID_SERVERS_SERVERID_VALIDATE) => Ok("PostServiceServerValidate"),
            // PostServiceServers - POST /services/{serviceId}/servers
            &hyper::Method::POST if path.matched(paths::ID_SERVICES_SERVICEID_SERVERS) => Ok("PostServiceServers"),
            // PutServiceServer - PUT /services/{serviceId}/servers/{serverId}
            &hyper::Method::PUT if path.matched(paths::ID_SERVICES_SERVICEID_SERVERS_SERVERID) => Ok("PutServiceServer"),
            // PutServiceServerPermissions - PUT /services/{serviceId}/servers/{serverId}/permissions
            &hyper::Method::PUT if path.matched(paths::ID_SERVICES_SERVICEID_SERVERS_SERVERID_PERMISSIONS) => Ok("PutServiceServerPermissions"),
            // PutServiceServerServerCredentials - PUT /services/{serviceId}/servers/{serverId}/serverCredentials
            &hyper::Method::PUT if path.matched(paths::ID_SERVICES_SERVICEID_SERVERS_SERVERID_SERVERCREDENTIALS) => Ok("PutServiceServerServerCredentials"),
            // PutServiceServerServerCredentialsPermissions - PUT /services/{serviceId}/servers/{serverId}/serverCredentials/permissions
            &hyper::Method::PUT if path.matched(paths::ID_SERVICES_SERVICEID_SERVERS_SERVERID_SERVERCREDENTIALS_PERMISSIONS) => Ok("PutServiceServerServerCredentialsPermissions"),
            // DeleteServiceModule - DELETE /modules/serviceModules/{serviceModuleUuid}
            &hyper::Method::DELETE if path.matched(paths::ID_MODULES_SERVICEMODULES_SERVICEMODULEUUID) => Ok("DeleteServiceModule"),
            // GetServiceModule - GET /modules/serviceModules/{serviceModuleUuid}
            &hyper::Method::GET if path.matched(paths::ID_MODULES_SERVICEMODULES_SERVICEMODULEUUID) => Ok("GetServiceModule"),
            // GetServiceModulePermissions - GET /modules/serviceModules/{serviceModuleUuid}/permissions
            &hyper::Method::GET if path.matched(paths::ID_MODULES_SERVICEMODULES_SERVICEMODULEUUID_PERMISSIONS) => Ok("GetServiceModulePermissions"),
            // GetServiceModules - GET /modules/serviceModules
            &hyper::Method::GET if path.matched(paths::ID_MODULES_SERVICEMODULES) => Ok("GetServiceModules"),
            // PostServiceModule - POST /modules/serviceModules/{serviceModuleUuid}
            &hyper::Method::POST if path.matched(paths::ID_MODULES_SERVICEMODULES_SERVICEMODULEUUID) => Ok("PostServiceModule"),
            // PostServiceModules - POST /modules/serviceModules
            &hyper::Method::POST if path.matched(paths::ID_MODULES_SERVICEMODULES) => Ok("PostServiceModules"),
            // PutServiceModulePermissions - PUT /modules/serviceModules/{serviceModuleUuid}/permissions
            &hyper::Method::PUT if path.matched(paths::ID_MODULES_SERVICEMODULES_SERVICEMODULEUUID_PERMISSIONS) => Ok("PutServiceModulePermissions"),
            // CreateService - POST /services
            &hyper::Method::POST if path.matched(paths::ID_SERVICES) => Ok("CreateService"),
            // DeleteService - DELETE /services/{serviceId}
            &hyper::Method::DELETE if path.matched(paths::ID_SERVICES_SERVICEID) => Ok("DeleteService"),
            // GetService - GET /services/{serviceId}
            &hyper::Method::GET if path.matched(paths::ID_SERVICES_SERVICEID) => Ok("GetService"),
            // GetServicePermissions - GET /services/{serviceId}/permissions
            &hyper::Method::GET if path.matched(paths::ID_SERVICES_SERVICEID_PERMISSIONS) => Ok("GetServicePermissions"),
            // GetServices - GET /services
            &hyper::Method::GET if path.matched(paths::ID_SERVICES) => Ok("GetServices"),
            // GetServicesPermissions - GET /services/permissions
            &hyper::Method::GET if path.matched(paths::ID_SERVICES_PERMISSIONS) => Ok("GetServicesPermissions"),
            // PostService - POST /services/{serviceId}
            &hyper::Method::POST if path.matched(paths::ID_SERVICES_SERVICEID) => Ok("PostService"),
            // PutServicePermissions - PUT /services/{serviceId}/permissions
            &hyper::Method::PUT if path.matched(paths::ID_SERVICES_SERVICEID_PERMISSIONS) => Ok("PutServicePermissions"),
            // PutServicesPermissions - PUT /services/permissions
            &hyper::Method::PUT if path.matched(paths::ID_SERVICES_PERMISSIONS) => Ok("PutServicesPermissions"),
            // DeleteUser - DELETE /users/{userId}
            &hyper::Method::DELETE if path.matched(paths::ID_USERS_USERID) => Ok("DeleteUser"),
            // GetUser - GET /users/{userId}
            &hyper::Method::GET if path.matched(paths::ID_USERS_USERID) => Ok("GetUser"),
            // GetUsers - GET /users
            &hyper::Method::GET if path.matched(paths::ID_USERS) => Ok("GetUsers"),
            // PostUser - POST /users
            &hyper::Method::POST if path.matched(paths::ID_USERS) => Ok("PostUser"),
            // PostUserLogin - POST /users/login
            &hyper::Method::POST if path.matched(paths::ID_USERS_LOGIN) => Ok("PostUserLogin"),
            // PutUser - PUT /users/{userId}
            &hyper::Method::PUT if path.matched(paths::ID_USERS_USERID) => Ok("PutUser"),
            _ => Err(()),
        }
    }
}
