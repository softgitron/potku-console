//! Main library entry point for potku_console_core implementation.

#![allow(unused_imports)]

use async_trait::async_trait;
use futures::{future, Stream, StreamExt, TryFutureExt, TryStreamExt};
use hyper::server::conn::Http;
use hyper::service::Service;
use log::info;
#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
use openssl::ssl::SslAcceptorBuilder;
use std::future::Future;
use std::marker::PhantomData;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use swagger::{Has, XSpanIdString};
use swagger::auth::MakeAllowAllAuthenticator;
use swagger::EmptyContext;
use tokio::net::TcpListener;

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use potku_console_core::models;

/// Builds an SSL implementation for Simple HTTPS from some hard-coded file names
pub async fn create(addr: &str, https: bool) {
    let addr = addr.parse().expect("Failed to parse bind address");

    let server = Server::new();

    let service = MakeService::new(server);

    let service = MakeAllowAllAuthenticator::new(service, "cosmo");

    let mut service =
        potku_console_core::server::context::MakeAddContext::<_, EmptyContext>::new(
            service
        );

    if https {
        #[cfg(any(target_os = "macos", target_os = "windows", target_os = "ios"))]
        {
            unimplemented!("SSL is not implemented for the examples on MacOS, Windows or iOS");
        }

        #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
        {
            let mut ssl = SslAcceptor::mozilla_intermediate_v5(SslMethod::tls()).expect("Failed to create SSL Acceptor");

            // Server authentication
            ssl.set_private_key_file("examples/server-key.pem", SslFiletype::PEM).expect("Failed to set private key");
            ssl.set_certificate_chain_file("examples/server-chain.pem").expect("Failed to set cerificate chain");
            ssl.check_private_key().expect("Failed to check private key");

            let tls_acceptor = Arc::new(ssl.build());
            let mut tcp_listener = TcpListener::bind(&addr).await.unwrap();
            let mut incoming = tcp_listener.incoming();

            while let (Some(tcp), rest) = incoming.into_future().await {
                if let Ok(tcp) = tcp {
                    let addr = tcp.peer_addr().expect("Unable to get remote address");
                    let service = service.call(addr);
                    let tls_acceptor = Arc::clone(&tls_acceptor);

                    tokio::spawn(async move {
                        let tls = tokio_openssl::accept(&*tls_acceptor, tcp).await.map_err(|_| ())?;

                        let service = service.await.map_err(|_| ())?;

                        Http::new().serve_connection(tls, service).await.map_err(|_| ())
                    });
                }

                incoming = rest;
            }
        }
    } else {
        // Using HTTP
        hyper::server::Server::bind(&addr).serve(service).await.unwrap()
    }
}

#[derive(Copy, Clone)]
pub struct Server<C> {
    marker: PhantomData<C>,
}

impl<C> Server<C> {
    pub fn new() -> Self {
        Server{marker: PhantomData}
    }
}


use potku_console_core::{
    Api,
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
    PutUserResponse,
};
use potku_console_core::server::MakeService;
use std::error::Error;
use swagger::ApiError;

#[async_trait]
impl<C> Api<C> for Server<C> where C: Has<XSpanIdString> + Send + Sync
{
    /// Remove cloud module entirely from the system.
    async fn delete_cloud_module(
        &self,
        cloud_module_uuid: uuid::Uuid,
        context: &C) -> Result<DeleteCloudModuleResponse, ApiError>
    {
        let context = context.clone();
        info!("delete_cloud_module({:?}) - X-Span-ID: {:?}", cloud_module_uuid, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Delete specific cloud account.
    async fn delete_cloud_module_account(
        &self,
        cloud_module_uuid: uuid::Uuid,
        cloud_account_id: String,
        context: &C) -> Result<DeleteCloudModuleAccountResponse, ApiError>
    {
        let context = context.clone();
        info!("delete_cloud_module_account({:?}, \"{}\") - X-Span-ID: {:?}", cloud_module_uuid, cloud_account_id, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Get specific cloud module details.
    async fn get_cloud_module(
        &self,
        cloud_module_uuid: uuid::Uuid,
        context: &C) -> Result<GetCloudModuleResponse, ApiError>
    {
        let context = context.clone();
        info!("get_cloud_module({:?}) - X-Span-ID: {:?}", cloud_module_uuid, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Get specific cloud account details.
    async fn get_cloud_module_account(
        &self,
        cloud_module_uuid: uuid::Uuid,
        cloud_account_id: String,
        context: &C) -> Result<GetCloudModuleAccountResponse, ApiError>
    {
        let context = context.clone();
        info!("get_cloud_module_account({:?}, \"{}\") - X-Span-ID: {:?}", cloud_module_uuid, cloud_account_id, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Get specific cloud account permissions.
    async fn get_cloud_module_account_permissions(
        &self,
        cloud_module_uuid: uuid::Uuid,
        cloud_account_id: String,
        context: &C) -> Result<GetCloudModuleAccountPermissionsResponse, ApiError>
    {
        let context = context.clone();
        info!("get_cloud_module_account_permissions({:?}, \"{}\") - X-Span-ID: {:?}", cloud_module_uuid, cloud_account_id, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Get cloud account related to specific cloud module.
    async fn get_cloud_module_accounts(
        &self,
        cloud_module_uuid: uuid::Uuid,
        context: &C) -> Result<GetCloudModuleAccountsResponse, ApiError>
    {
        let context = context.clone();
        info!("get_cloud_module_accounts({:?}) - X-Span-ID: {:?}", cloud_module_uuid, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Get cloud module endpoint permissions.
    async fn get_cloud_module_permissions(
        &self,
        cloud_module_uuid: uuid::Uuid,
        context: &C) -> Result<GetCloudModulePermissionsResponse, ApiError>
    {
        let context = context.clone();
        info!("get_cloud_module_permissions({:?}) - X-Span-ID: {:?}", cloud_module_uuid, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Get list of available cloud modules.
    async fn get_cloud_modules(
        &self,
        context: &C) -> Result<GetCloudModulesResponse, ApiError>
    {
        let context = context.clone();
        info!("get_cloud_modules() - X-Span-ID: {:?}", context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Enable, disable or change configurations of the cloud module.
    async fn post_cloud_module(
        &self,
        cloud_module_uuid: uuid::Uuid,
        body: models::ServiceModule,
        context: &C) -> Result<PostCloudModuleResponse, ApiError>
    {
        let context = context.clone();
        info!("post_cloud_module({:?}, {:?}) - X-Span-ID: {:?}", cloud_module_uuid, body, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Input new cloud account details.
    async fn post_cloud_module_accounts(
        &self,
        cloud_module_uuid: uuid::Uuid,
        body: models::CloudAccount,
        context: &C) -> Result<PostCloudModuleAccountsResponse, ApiError>
    {
        let context = context.clone();
        info!("post_cloud_module_accounts({:?}, {:?}) - X-Span-ID: {:?}", cloud_module_uuid, body, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Upload new cloud module to the server using zip packet.
    async fn post_cloud_modules(
        &self,
        body: swagger::ByteArray,
        context: &C) -> Result<PostCloudModulesResponse, ApiError>
    {
        let context = context.clone();
        info!("post_cloud_modules({:?}) - X-Span-ID: {:?}", body, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Update existing cloud account details.
    async fn put_cloud_module_account(
        &self,
        cloud_module_uuid: uuid::Uuid,
        cloud_account_id: String,
        body: models::CloudAccount,
        context: &C) -> Result<PutCloudModuleAccountResponse, ApiError>
    {
        let context = context.clone();
        info!("put_cloud_module_account({:?}, \"{}\", {:?}) - X-Span-ID: {:?}", cloud_module_uuid, cloud_account_id, body, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Set new cloud module endpoint permissions.
    async fn put_cloud_module_account_permissions(
        &self,
        cloud_module_uuid: uuid::Uuid,
        cloud_account_id: String,
        body: models::Permissions,
        context: &C) -> Result<PutCloudModuleAccountPermissionsResponse, ApiError>
    {
        let context = context.clone();
        info!("put_cloud_module_account_permissions({:?}, \"{}\", {:?}) - X-Span-ID: {:?}", cloud_module_uuid, cloud_account_id, body, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Set new cloud module endpoint permissions.
    async fn put_cloud_module_permissions(
        &self,
        cloud_module_uuid: uuid::Uuid,
        body: models::Permissions,
        context: &C) -> Result<PutCloudModulePermissionsResponse, ApiError>
    {
        let context = context.clone();
        info!("put_cloud_module_permissions({:?}, {:?}) - X-Span-ID: {:?}", cloud_module_uuid, body, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Get cpu amount profiles.
    async fn get_cpu_cores(
        &self,
        cloud_module_uuid: uuid::Uuid,
        context: &C) -> Result<GetCpuCoresResponse, ApiError>
    {
        let context = context.clone();
        info!("get_cpu_cores({:?}) - X-Span-ID: {:?}", cloud_module_uuid, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Get minimum and maximum disk available.
    async fn get_disk_bounds(
        &self,
        cloud_module_uuid: uuid::Uuid,
        context: &C) -> Result<GetDiskBoundsResponse, ApiError>
    {
        let context = context.clone();
        info!("get_disk_bounds({:?}) - X-Span-ID: {:?}", cloud_module_uuid, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Get available machine types.
    async fn get_machine_types(
        &self,
        cloud_module_uuid: uuid::Uuid,
        context: &C) -> Result<GetMachineTypesResponse, ApiError>
    {
        let context = context.clone();
        info!("get_machine_types({:?}) - X-Span-ID: {:?}", cloud_module_uuid, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Get available operating system names.
    async fn get_operating_systems(
        &self,
        cloud_module_uuid: uuid::Uuid,
        context: &C) -> Result<GetOperatingSystemsResponse, ApiError>
    {
        let context = context.clone();
        info!("get_operating_systems({:?}) - X-Span-ID: {:?}", cloud_module_uuid, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Get available compute orientation names.
    async fn get_orientations(
        &self,
        cloud_module_uuid: uuid::Uuid,
        context: &C) -> Result<GetOrientationsResponse, ApiError>
    {
        let context = context.clone();
        info!("get_orientations({:?}) - X-Span-ID: {:?}", cloud_module_uuid, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Get minimum and maximum ram available.
    async fn get_ram_bounds(
        &self,
        cloud_module_uuid: uuid::Uuid,
        context: &C) -> Result<GetRamBoundsResponse, ApiError>
    {
        let context = context.clone();
        info!("get_ram_bounds({:?}) - X-Span-ID: {:?}", cloud_module_uuid, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Remove group from the system.
    async fn delete_group(
        &self,
        group_id: i32,
        context: &C) -> Result<DeleteGroupResponse, ApiError>
    {
        let context = context.clone();
        info!("delete_group({}) - X-Span-ID: {:?}", group_id, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Get specific permission group details.
    async fn get_group(
        &self,
        group_id: i32,
        context: &C) -> Result<GetGroupResponse, ApiError>
    {
        let context = context.clone();
        info!("get_group({}) - X-Span-ID: {:?}", group_id, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Get all groups.
    async fn get_groups(
        &self,
        context: &C) -> Result<GetGroupsResponse, ApiError>
    {
        let context = context.clone();
        info!("get_groups() - X-Span-ID: {:?}", context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Create a new permission group.
    async fn post_group(
        &self,
        body: models::GroupContent,
        context: &C) -> Result<PostGroupResponse, ApiError>
    {
        let context = context.clone();
        info!("post_group({:?}) - X-Span-ID: {:?}", body, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Alter permission group name or users.
    async fn put_group(
        &self,
        group_id: i32,
        body: models::Group,
        context: &C) -> Result<PutGroupResponse, ApiError>
    {
        let context = context.clone();
        info!("put_group({}, {:?}) - X-Span-ID: {:?}", group_id, body, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Get current potku-console main settings.
    async fn get_console_management(
        &self,
        context: &C) -> Result<GetConsoleManagementResponse, ApiError>
    {
        let context = context.clone();
        info!("get_console_management() - X-Span-ID: {:?}", context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Set current potku-console main settings
    async fn post_console_management(
        &self,
        body: models::ConsoleSettings,
        context: &C) -> Result<PostConsoleManagementResponse, ApiError>
    {
        let context = context.clone();
        info!("post_console_management({:?}) - X-Span-ID: {:?}", body, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Get specific server details.
    async fn get_service_server(
        &self,
        service_id: i32,
        server_id: i32,
        context: &C) -> Result<GetServiceServerResponse, ApiError>
    {
        let context = context.clone();
        info!("get_service_server({}, {}) - X-Span-ID: {:?}", service_id, server_id, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Get specific server's permission configurations.
    async fn get_service_server_permissions(
        &self,
        service_id: i32,
        server_id: i32,
        context: &C) -> Result<GetServiceServerPermissionsResponse, ApiError>
    {
        let context = context.clone();
        info!("get_service_server_permissions({}, {}) - X-Span-ID: {:?}", service_id, server_id, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Get server's credentials.
    async fn get_service_server_server_credentials(
        &self,
        service_id: i32,
        server_id: i32,
        context: &C) -> Result<GetServiceServerServerCredentialsResponse, ApiError>
    {
        let context = context.clone();
        info!("get_service_server_server_credentials({}, {}) - X-Span-ID: {:?}", service_id, server_id, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Get specific server's credentials permission configurations.
    async fn get_service_server_server_credentials_permissions(
        &self,
        service_id: i32,
        server_id: i32,
        context: &C) -> Result<GetServiceServerServerCredentialsPermissionsResponse, ApiError>
    {
        let context = context.clone();
        info!("get_service_server_server_credentials_permissions({}, {}) - X-Span-ID: {:?}", service_id, server_id, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Get all server details from specific service instance.
    async fn get_service_servers(
        &self,
        service_id: i32,
        context: &C) -> Result<GetServiceServersResponse, ApiError>
    {
        let context = context.clone();
        info!("get_service_servers({}) - X-Span-ID: {:?}", service_id, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Validate that cloud provider services can be controlled with the given details.
    async fn post_service_server_validate(
        &self,
        service_id: i32,
        server_id: i32,
        context: &C) -> Result<PostServiceServerValidateResponse, ApiError>
    {
        let context = context.clone();
        info!("post_service_server_validate({}, {}) - X-Span-ID: {:?}", service_id, server_id, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Add new server to the service.
    async fn post_service_servers(
        &self,
        service_id: i32,
        body: models::PostNewServer,
        context: &C) -> Result<PostServiceServersResponse, ApiError>
    {
        let context = context.clone();
        info!("post_service_servers({}, {:?}) - X-Span-ID: {:?}", service_id, body, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Update server details (mainly done by service modules).
    async fn put_service_server(
        &self,
        service_id: i32,
        server_id: i32,
        body: models::Server,
        context: &C) -> Result<PutServiceServerResponse, ApiError>
    {
        let context = context.clone();
        info!("put_service_server({}, {}, {:?}) - X-Span-ID: {:?}", service_id, server_id, body, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Set specific server's permission configurations.
    async fn put_service_server_permissions(
        &self,
        service_id: i32,
        server_id: i32,
        body: models::Permissions,
        context: &C) -> Result<PutServiceServerPermissionsResponse, ApiError>
    {
        let context = context.clone();
        info!("put_service_server_permissions({}, {}, {:?}) - X-Span-ID: {:?}", service_id, server_id, body, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Set server's credentials.
    async fn put_service_server_server_credentials(
        &self,
        service_id: i32,
        server_id: i32,
        body: models::ServerCredentials,
        context: &C) -> Result<PutServiceServerServerCredentialsResponse, ApiError>
    {
        let context = context.clone();
        info!("put_service_server_server_credentials({}, {}, {:?}) - X-Span-ID: {:?}", service_id, server_id, body, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Set specific server's credentials permission configurations.
    async fn put_service_server_server_credentials_permissions(
        &self,
        service_id: i32,
        server_id: i32,
        body: models::Permissions,
        context: &C) -> Result<PutServiceServerServerCredentialsPermissionsResponse, ApiError>
    {
        let context = context.clone();
        info!("put_service_server_server_credentials_permissions({}, {}, {:?}) - X-Span-ID: {:?}", service_id, server_id, body, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Remove service module entirely from the system.
    async fn delete_service_module(
        &self,
        service_module_uuid: uuid::Uuid,
        context: &C) -> Result<DeleteServiceModuleResponse, ApiError>
    {
        let context = context.clone();
        info!("delete_service_module({:?}) - X-Span-ID: {:?}", service_module_uuid, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Get specific service module details.
    async fn get_service_module(
        &self,
        service_module_uuid: uuid::Uuid,
        context: &C) -> Result<GetServiceModuleResponse, ApiError>
    {
        let context = context.clone();
        info!("get_service_module({:?}) - X-Span-ID: {:?}", service_module_uuid, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Get service module endpoint permissions.
    async fn get_service_module_permissions(
        &self,
        service_module_uuid: uuid::Uuid,
        context: &C) -> Result<GetServiceModulePermissionsResponse, ApiError>
    {
        let context = context.clone();
        info!("get_service_module_permissions({:?}) - X-Span-ID: {:?}", service_module_uuid, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Get list of available service modules.
    async fn get_service_modules(
        &self,
        context: &C) -> Result<GetServiceModulesResponse, ApiError>
    {
        let context = context.clone();
        info!("get_service_modules() - X-Span-ID: {:?}", context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Enable, disable or change configurations of the service module.
    async fn post_service_module(
        &self,
        service_module_uuid: uuid::Uuid,
        body: models::ServiceModule,
        context: &C) -> Result<PostServiceModuleResponse, ApiError>
    {
        let context = context.clone();
        info!("post_service_module({:?}, {:?}) - X-Span-ID: {:?}", service_module_uuid, body, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Upload new service module to the server using zip packet.
    async fn post_service_modules(
        &self,
        body: swagger::ByteArray,
        context: &C) -> Result<PostServiceModulesResponse, ApiError>
    {
        let context = context.clone();
        info!("post_service_modules({:?}) - X-Span-ID: {:?}", body, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Set new cloud module endpoint permissions.
    async fn put_service_module_permissions(
        &self,
        service_module_uuid: uuid::Uuid,
        body: models::Permissions,
        context: &C) -> Result<PutServiceModulePermissionsResponse, ApiError>
    {
        let context = context.clone();
        info!("put_service_module_permissions({:?}, {:?}) - X-Span-ID: {:?}", service_module_uuid, body, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Create a new service.
    async fn create_service(
        &self,
        body: models::PostNewService,
        context: &C) -> Result<CreateServiceResponse, ApiError>
    {
        let context = context.clone();
        info!("create_service({:?}) - X-Span-ID: {:?}", body, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Delete service and all of its servers.
    async fn delete_service(
        &self,
        service_id: i32,
        context: &C) -> Result<DeleteServiceResponse, ApiError>
    {
        let context = context.clone();
        info!("delete_service({}) - X-Span-ID: {:?}", service_id, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Get service details from specific service instance.
    async fn get_service(
        &self,
        service_id: i32,
        context: &C) -> Result<GetServiceResponse, ApiError>
    {
        let context = context.clone();
        info!("get_service({}) - X-Span-ID: {:?}", service_id, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Get specific service permission configurations
    async fn get_service_permissions(
        &self,
        service_id: i32,
        context: &C) -> Result<GetServicePermissionsResponse, ApiError>
    {
        let context = context.clone();
        info!("get_service_permissions({}) - X-Span-ID: {:?}", service_id, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Get all service names and IDs from current user.
    async fn get_services(
        &self,
        port: Option<i32>,
        dns: Option<String>,
        context: &C) -> Result<GetServicesResponse, ApiError>
    {
        let context = context.clone();
        info!("get_services({:?}, {:?}) - X-Span-ID: {:?}", port, dns, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Get service endpoint permissions.
    async fn get_services_permissions(
        &self,
        context: &C) -> Result<GetServicesPermissionsResponse, ApiError>
    {
        let context = context.clone();
        info!("get_services_permissions() - X-Span-ID: {:?}", context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Update service details and desired state (start/stop service) related to specific service instance.
    async fn post_service(
        &self,
        service_id: i32,
        body: models::Service,
        context: &C) -> Result<PostServiceResponse, ApiError>
    {
        let context = context.clone();
        info!("post_service({}, {:?}) - X-Span-ID: {:?}", service_id, body, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Set new service specific permissions.
    async fn put_service_permissions(
        &self,
        service_id: i32,
        body: models::Permissions,
        context: &C) -> Result<PutServicePermissionsResponse, ApiError>
    {
        let context = context.clone();
        info!("put_service_permissions({}, {:?}) - X-Span-ID: {:?}", service_id, body, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Set new service endpoint permissions.
    async fn put_services_permissions(
        &self,
        body: models::Permissions,
        context: &C) -> Result<PutServicesPermissionsResponse, ApiError>
    {
        let context = context.clone();
        info!("put_services_permissions({:?}) - X-Span-ID: {:?}", body, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Remove user from the system.
    async fn delete_user(
        &self,
        user_id: i32,
        context: &C) -> Result<DeleteUserResponse, ApiError>
    {
        let context = context.clone();
        info!("delete_user({}) - X-Span-ID: {:?}", user_id, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Get specific user details.
    async fn get_user(
        &self,
        user_id: i32,
        context: &C) -> Result<GetUserResponse, ApiError>
    {
        let context = context.clone();
        info!("get_user({}) - X-Span-ID: {:?}", user_id, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Get all users.
    async fn get_users(
        &self,
        context: &C) -> Result<GetUsersResponse, ApiError>
    {
        let context = context.clone();
        info!("get_users() - X-Span-ID: {:?}", context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Create a new user.
    async fn post_user(
        &self,
        body: models::UserContent,
        context: &C) -> Result<PostUserResponse, ApiError>
    {
        let context = context.clone();
        info!("post_user({:?}) - X-Span-ID: {:?}", body, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Login to user account.
    async fn post_user_login(
        &self,
        body: models::ExistingUserCredentials,
        context: &C) -> Result<PostUserLoginResponse, ApiError>
    {
        let context = context.clone();
        info!("post_user_login({:?}) - X-Span-ID: {:?}", body, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Alter service state.
    async fn put_user(
        &self,
        user_id: i32,
        body: models::User,
        context: &C) -> Result<PutUserResponse, ApiError>
    {
        let context = context.clone();
        info!("put_user({}, {:?}) - X-Span-ID: {:?}", user_id, body, context.get().0.clone());
        Err("Generic failuare".into())
    }

}
