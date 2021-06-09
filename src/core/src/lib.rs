#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]

use async_trait::async_trait;
use futures::Stream;
use std::error::Error;
use std::task::{Poll, Context};
use swagger::{ApiError, ContextWrapper};
use serde::{Serialize, Deserialize};

type ServiceError = Box<dyn Error + Send + Sync + 'static>;

pub const BASE_PATH: &'static str = "/v1";
pub const API_VERSION: &'static str = "0.1.0";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum DeleteCloudModuleResponse {
    /// Cloud module deleted.
    CloudModuleDeleted
    ,
    /// You do not have sufficient permissions.
    YouDoNotHaveSufficientPermissions
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum DeleteCloudModuleAccountResponse {
    /// Cloud module account deleted.
    CloudModuleAccountDeleted
    ,
    /// You do not have sufficient permissions.
    YouDoNotHaveSufficientPermissions
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetCloudModuleResponse {
    /// Specific cloud module details.
    SpecificCloudModuleDetails
    (models::CloudModule)
    ,
    /// You do not have sufficient permissions.
    YouDoNotHaveSufficientPermissions
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetCloudModuleAccountResponse {
    /// Cloud module account details.
    CloudModuleAccountDetails
    (models::CloudAccount)
    ,
    /// You do not have sufficient permissions.
    YouDoNotHaveSufficientPermissions
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetCloudModuleAccountPermissionsResponse {
    /// Permissions will be provided.
    PermissionsWillBeProvided
    (models::Permissions)
    ,
    /// You do not have sufficient permissions.
    YouDoNotHaveSufficientPermissions
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetCloudModuleAccountsResponse {
    /// List of all cloud credentials.
    ListOfAllCloudCredentials
    (models::CloudAccounts)
    ,
    /// You do not have sufficient permissions.
    YouDoNotHaveSufficientPermissions
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetCloudModulePermissionsResponse {
    /// Permissions will be provided.
    PermissionsWillBeProvided
    (models::Permissions)
    ,
    /// You do not have sufficient permissions.
    YouDoNotHaveSufficientPermissions
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetCloudModulesResponse {
    /// List of all cloud modules.
    ListOfAllCloudModules
    (models::CloudModules)
    ,
    /// You do not have sufficient permissions.
    YouDoNotHaveSufficientPermissions
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum PostCloudModuleResponse {
    /// Cloud module altered.
    CloudModuleAltered
    ,
    /// You do not have sufficient permissions.
    YouDoNotHaveSufficientPermissions
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum PostCloudModuleAccountsResponse {
    /// Cloud module account created.
    CloudModuleAccountCreated
    ,
    /// You do not have sufficient permissions.
    YouDoNotHaveSufficientPermissions
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum PostCloudModulesResponse {
    /// Specific cloud module details.
    SpecificCloudModuleDetails
    (models::CloudModule)
    ,
    /// You do not have sufficient permissions.
    YouDoNotHaveSufficientPermissions
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum PutCloudModuleAccountResponse {
    /// Cloud module account altered.
    CloudModuleAccountAltered
    ,
    /// You do not have sufficient permissions.
    YouDoNotHaveSufficientPermissions
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum PutCloudModuleAccountPermissionsResponse {
    /// Permissions altered successfully.
    PermissionsAlteredSuccessfully
    ,
    /// You do not have sufficient permissions.
    YouDoNotHaveSufficientPermissions
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum PutCloudModulePermissionsResponse {
    /// Permissions altered successfully.
    PermissionsAlteredSuccessfully
    ,
    /// You do not have sufficient permissions.
    YouDoNotHaveSufficientPermissions
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetCpuCoresResponse {
    /// Available cpu amount profiles.
    AvailableCpuAmountProfiles
    (Vec<f64>)
    ,
    /// Something has gone wrong in the module that was processing request.
    SomethingHasGoneWrongInTheModuleThatWasProcessingRequest
    (models::Error)
    ,
    /// Module does not support this functionality.
    ModuleDoesNotSupportThisFunctionality
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetDiskBoundsResponse {
    /// Available minimum and maximum disk.
    AvailableMinimumAndMaximumDisk
    (models::InlineResponse200)
    ,
    /// Something has gone wrong in the module that was processing request.
    SomethingHasGoneWrongInTheModuleThatWasProcessingRequest
    (models::Error)
    ,
    /// Module does not support this functionality.
    ModuleDoesNotSupportThisFunctionality
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetMachineTypesResponse {
    /// Available machine types.
    AvailableMachineTypes
    (Vec<String>)
    ,
    /// Something has gone wrong in the module that was processing request.
    SomethingHasGoneWrongInTheModuleThatWasProcessingRequest
    (models::Error)
    ,
    /// Module does not support this functionality.
    ModuleDoesNotSupportThisFunctionality
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetOperatingSystemsResponse {
    /// Available operating system types.
    AvailableOperatingSystemTypes
    (Vec<String>)
    ,
    /// Something has gone wrong in the module that was processing request.
    SomethingHasGoneWrongInTheModuleThatWasProcessingRequest
    (models::Error)
    ,
    /// Module does not support this functionality.
    ModuleDoesNotSupportThisFunctionality
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetOrientationsResponse {
    /// Available compute orientation types.
    AvailableComputeOrientationTypes
    (Vec<String>)
    ,
    /// Something has gone wrong in the module that was processing request.
    SomethingHasGoneWrongInTheModuleThatWasProcessingRequest
    (models::Error)
    ,
    /// Module does not support this functionality.
    ModuleDoesNotSupportThisFunctionality
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetRamBoundsResponse {
    /// Available minimum and maximum ram.
    AvailableMinimumAndMaximumRam
    (models::InlineResponse2001)
    ,
    /// Something has gone wrong in the module that was processing request.
    SomethingHasGoneWrongInTheModuleThatWasProcessingRequest
    (models::Error)
    ,
    /// Module does not support this functionality.
    ModuleDoesNotSupportThisFunctionality
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum DeleteGroupResponse {
    /// Group deleted.
    GroupDeleted
    ,
    /// You do not have sufficient permissions.
    YouDoNotHaveSufficientPermissions
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetGroupResponse {
    /// Details of the permission group.
    DetailsOfThePermissionGroup
    (models::Group)
    ,
    /// You do not have sufficient permissions.
    YouDoNotHaveSufficientPermissions
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetGroupsResponse {
    /// List of all permission groups.
    ListOfAllPermissionGroups
    (models::Groups)
    ,
    /// You do not have sufficient permissions.
    YouDoNotHaveSufficientPermissions
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum PostGroupResponse {
    /// Group created successfully.
    GroupCreatedSuccessfully
    ,
    /// You do not have sufficient permissions.
    YouDoNotHaveSufficientPermissions
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum PutGroupResponse {
    /// Group was altered successfully.
    GroupWasAlteredSuccessfully
    ,
    /// You do not have sufficient permissions.
    YouDoNotHaveSufficientPermissions
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetConsoleManagementResponse {
    /// Current Potku console main settings.
    CurrentPotkuConsoleMainSettings
    (models::ConsoleSettings)
    ,
    /// You do not have sufficient permissions.
    YouDoNotHaveSufficientPermissions
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum PostConsoleManagementResponse {
    /// Specific cloud module details.
    SpecificCloudModuleDetails
    (models::CloudModule)
    ,
    /// You do not have sufficient permissions.
    YouDoNotHaveSufficientPermissions
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetServiceServerResponse {
    /// Service's server details
    Service
    (models::Server)
    ,
    /// You do not have sufficient permissions.
    YouDoNotHaveSufficientPermissions
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetServiceServerPermissionsResponse {
    /// Permissions will be provided.
    PermissionsWillBeProvided
    (models::Permissions)
    ,
    /// You do not have sufficient permissions.
    YouDoNotHaveSufficientPermissions
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetServiceServerServerCredentialsResponse {
    /// Server credentials.
    ServerCredentials
    (models::ServerCredentials)
    ,
    /// You do not have sufficient permissions.
    YouDoNotHaveSufficientPermissions
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetServiceServerServerCredentialsPermissionsResponse {
    /// Permissions will be provided.
    PermissionsWillBeProvided
    (models::Permissions)
    ,
    /// You do not have sufficient permissions.
    YouDoNotHaveSufficientPermissions
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetServiceServersResponse {
    /// List of servers under a service.
    ListOfServersUnderAService
    (models::Servers)
    ,
    /// You do not have sufficient permissions.
    YouDoNotHaveSufficientPermissions
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum PostServiceServerValidateResponse {
    /// Validation went successfully.
    ValidationWentSuccessfully
    ,
    /// There were something wrong with the request or configuration.
    ThereWereSomethingWrongWithTheRequestOrConfiguration
    (models::Error)
    ,
    /// You do not have sufficient permissions.
    YouDoNotHaveSufficientPermissions
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum PostServiceServersResponse {
    /// Server details.
    ServerDetails
    (models::Server)
    ,
    /// You do not have sufficient permissions.
    YouDoNotHaveSufficientPermissions
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum PutServiceServerResponse {
    /// Server was updated successfully
    ServerWasUpdatedSuccessfully
    ,
    /// Server can't be altered during runtime
    ServerCan
    (models::Error)
    ,
    /// You do not have sufficient permissions.
    YouDoNotHaveSufficientPermissions
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum PutServiceServerPermissionsResponse {
    /// Permissions altered successfully.
    PermissionsAlteredSuccessfully
    ,
    /// You do not have sufficient permissions.
    YouDoNotHaveSufficientPermissions
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum PutServiceServerServerCredentialsResponse {
    /// Server credentials altered successfully.
    ServerCredentialsAlteredSuccessfully
    ,
    /// You do not have sufficient permissions.
    YouDoNotHaveSufficientPermissions
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum PutServiceServerServerCredentialsPermissionsResponse {
    /// Permissions altered successfully.
    PermissionsAlteredSuccessfully
    ,
    /// You do not have sufficient permissions.
    YouDoNotHaveSufficientPermissions
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum DeleteServiceModuleResponse {
    /// Service module deleted.
    ServiceModuleDeleted
    ,
    /// You do not have sufficient permissions.
    YouDoNotHaveSufficientPermissions
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetServiceModuleResponse {
    /// Specific service module details.
    SpecificServiceModuleDetails
    (models::ServiceModule)
    ,
    /// You do not have sufficient permissions.
    YouDoNotHaveSufficientPermissions
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetServiceModulePermissionsResponse {
    /// Permissions will be provided.
    PermissionsWillBeProvided
    (models::Permissions)
    ,
    /// You do not have sufficient permissions.
    YouDoNotHaveSufficientPermissions
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetServiceModulesResponse {
    /// List of all service modules.
    ListOfAllServiceModules
    (models::ServiceModules)
    ,
    /// You do not have sufficient permissions.
    YouDoNotHaveSufficientPermissions
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum PostServiceModuleResponse {
    /// Service module altered.
    ServiceModuleAltered
    ,
    /// You do not have sufficient permissions.
    YouDoNotHaveSufficientPermissions
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum PostServiceModulesResponse {
    /// Specific service module details.
    SpecificServiceModuleDetails
    (models::ServiceModule)
    ,
    /// You do not have sufficient permissions.
    YouDoNotHaveSufficientPermissions
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum PutServiceModulePermissionsResponse {
    /// Permissions altered successfully.
    PermissionsAlteredSuccessfully
    ,
    /// You do not have sufficient permissions.
    YouDoNotHaveSufficientPermissions
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum CreateServiceResponse {
    /// Service details.
    ServiceDetails
    (models::Service)
    ,
    /// You do not have sufficient permissions.
    YouDoNotHaveSufficientPermissions
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum DeleteServiceResponse {
    /// Delete operation was successful.
    DeleteOperationWasSuccessful
    ,
    /// Service can't be deleted while it is running.
    ServiceCan
    (models::Error)
    ,
    /// You do not have sufficient permissions.
    YouDoNotHaveSufficientPermissions
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetServiceResponse {
    /// Service details.
    ServiceDetails
    (models::Service)
    ,
    /// You do not have sufficient permissions.
    YouDoNotHaveSufficientPermissions
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetServicePermissionsResponse {
    /// Permissions will be provided.
    PermissionsWillBeProvided
    (models::Permissions)
    ,
    /// You do not have sufficient permissions.
    YouDoNotHaveSufficientPermissions
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetServicesResponse {
    /// Valid request, service list will be provided.
    ValidRequest
    (models::Services)
    ,
    /// You do not have sufficient permissions.
    YouDoNotHaveSufficientPermissions
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetServicesPermissionsResponse {
    /// Permissions will be provided.
    PermissionsWillBeProvided
    (models::Permissions)
    ,
    /// You do not have sufficient permissions.
    YouDoNotHaveSufficientPermissions
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum PostServiceResponse {
    /// Service details.
    ServiceDetails
    (models::Service)
    ,
    /// Service ID or service module ID can't be updated.
    ServiceIDOrServiceModuleIDCan
    (models::Error)
    ,
    /// You do not have sufficient permissions.
    YouDoNotHaveSufficientPermissions
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum PutServicePermissionsResponse {
    /// Permissions altered successfully.
    PermissionsAlteredSuccessfully
    ,
    /// You do not have sufficient permissions.
    YouDoNotHaveSufficientPermissions
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum PutServicesPermissionsResponse {
    /// Permissions altered successfully.
    PermissionsAlteredSuccessfully
    ,
    /// You do not have sufficient permissions.
    YouDoNotHaveSufficientPermissions
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum DeleteUserResponse {
    /// User deleted.
    UserDeleted
    ,
    /// You do not have sufficient permissions.
    YouDoNotHaveSufficientPermissions
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetUserResponse {
    /// Details of the user.
    DetailsOfTheUser
    (models::User)
    ,
    /// You do not have sufficient permissions.
    YouDoNotHaveSufficientPermissions
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetUsersResponse {
    /// List of all users.
    ListOfAllUsers
    (models::Users)
    ,
    /// You do not have sufficient permissions.
    YouDoNotHaveSufficientPermissions
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum PostUserResponse {
    /// User created successfully.
    UserCreatedSuccessfully
    ,
    /// You do not have sufficient permissions.
    YouDoNotHaveSufficientPermissions
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum PostUserLoginResponse {
    /// User authenticated successfully.
    UserAuthenticatedSuccessfully
    {
        body: models::InlineResponse2002,
        set_cookie:
        Option<
        String
        >
    }
    ,
    /// Login required two factor authentication / wrong username or password.
    LoginRequiredTwoFactorAuthentication
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum PutUserResponse {
    /// User was altered successfully.
    UserWasAlteredSuccessfully
    ,
    /// You do not have sufficient permissions.
    YouDoNotHaveSufficientPermissions
    (models::Error)
}

/// API
#[async_trait]
pub trait Api<C: Send + Sync> {
    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>> {
        Poll::Ready(Ok(()))
    }

    /// Remove cloud module entirely from the system.
    async fn delete_cloud_module(
        &self,
        cloud_module_uuid: uuid::Uuid,
        context: &C) -> Result<DeleteCloudModuleResponse, ApiError>;

    /// Delete specific cloud account.
    async fn delete_cloud_module_account(
        &self,
        cloud_module_uuid: uuid::Uuid,
        cloud_account_id: String,
        context: &C) -> Result<DeleteCloudModuleAccountResponse, ApiError>;

    /// Get specific cloud module details.
    async fn get_cloud_module(
        &self,
        cloud_module_uuid: uuid::Uuid,
        context: &C) -> Result<GetCloudModuleResponse, ApiError>;

    /// Get specific cloud account details.
    async fn get_cloud_module_account(
        &self,
        cloud_module_uuid: uuid::Uuid,
        cloud_account_id: String,
        context: &C) -> Result<GetCloudModuleAccountResponse, ApiError>;

    /// Get specific cloud account permissions.
    async fn get_cloud_module_account_permissions(
        &self,
        cloud_module_uuid: uuid::Uuid,
        cloud_account_id: String,
        context: &C) -> Result<GetCloudModuleAccountPermissionsResponse, ApiError>;

    /// Get cloud account related to specific cloud module.
    async fn get_cloud_module_accounts(
        &self,
        cloud_module_uuid: uuid::Uuid,
        context: &C) -> Result<GetCloudModuleAccountsResponse, ApiError>;

    /// Get cloud module endpoint permissions.
    async fn get_cloud_module_permissions(
        &self,
        cloud_module_uuid: uuid::Uuid,
        context: &C) -> Result<GetCloudModulePermissionsResponse, ApiError>;

    /// Get list of available cloud modules.
    async fn get_cloud_modules(
        &self,
        context: &C) -> Result<GetCloudModulesResponse, ApiError>;

    /// Enable, disable or change configurations of the cloud module.
    async fn post_cloud_module(
        &self,
        cloud_module_uuid: uuid::Uuid,
        body: models::ServiceModule,
        context: &C) -> Result<PostCloudModuleResponse, ApiError>;

    /// Input new cloud account details.
    async fn post_cloud_module_accounts(
        &self,
        cloud_module_uuid: uuid::Uuid,
        body: models::CloudAccount,
        context: &C) -> Result<PostCloudModuleAccountsResponse, ApiError>;

    /// Upload new cloud module to the server using zip packet.
    async fn post_cloud_modules(
        &self,
        body: swagger::ByteArray,
        context: &C) -> Result<PostCloudModulesResponse, ApiError>;

    /// Update existing cloud account details.
    async fn put_cloud_module_account(
        &self,
        cloud_module_uuid: uuid::Uuid,
        cloud_account_id: String,
        body: models::CloudAccount,
        context: &C) -> Result<PutCloudModuleAccountResponse, ApiError>;

    /// Set new cloud module endpoint permissions.
    async fn put_cloud_module_account_permissions(
        &self,
        cloud_module_uuid: uuid::Uuid,
        cloud_account_id: String,
        body: models::Permissions,
        context: &C) -> Result<PutCloudModuleAccountPermissionsResponse, ApiError>;

    /// Set new cloud module endpoint permissions.
    async fn put_cloud_module_permissions(
        &self,
        cloud_module_uuid: uuid::Uuid,
        body: models::Permissions,
        context: &C) -> Result<PutCloudModulePermissionsResponse, ApiError>;

    /// Get cpu amount profiles.
    async fn get_cpu_cores(
        &self,
        cloud_module_uuid: uuid::Uuid,
        context: &C) -> Result<GetCpuCoresResponse, ApiError>;

    /// Get minimum and maximum disk available.
    async fn get_disk_bounds(
        &self,
        cloud_module_uuid: uuid::Uuid,
        context: &C) -> Result<GetDiskBoundsResponse, ApiError>;

    /// Get available machine types.
    async fn get_machine_types(
        &self,
        cloud_module_uuid: uuid::Uuid,
        context: &C) -> Result<GetMachineTypesResponse, ApiError>;

    /// Get available operating system names.
    async fn get_operating_systems(
        &self,
        cloud_module_uuid: uuid::Uuid,
        context: &C) -> Result<GetOperatingSystemsResponse, ApiError>;

    /// Get available compute orientation names.
    async fn get_orientations(
        &self,
        cloud_module_uuid: uuid::Uuid,
        context: &C) -> Result<GetOrientationsResponse, ApiError>;

    /// Get minimum and maximum ram available.
    async fn get_ram_bounds(
        &self,
        cloud_module_uuid: uuid::Uuid,
        context: &C) -> Result<GetRamBoundsResponse, ApiError>;

    /// Remove group from the system.
    async fn delete_group(
        &self,
        group_id: i32,
        context: &C) -> Result<DeleteGroupResponse, ApiError>;

    /// Get specific permission group details.
    async fn get_group(
        &self,
        group_id: i32,
        context: &C) -> Result<GetGroupResponse, ApiError>;

    /// Get all groups.
    async fn get_groups(
        &self,
        context: &C) -> Result<GetGroupsResponse, ApiError>;

    /// Create a new permission group.
    async fn post_group(
        &self,
        body: models::GroupContent,
        context: &C) -> Result<PostGroupResponse, ApiError>;

    /// Alter permission group name or users.
    async fn put_group(
        &self,
        group_id: i32,
        body: models::Group,
        context: &C) -> Result<PutGroupResponse, ApiError>;

    /// Get current potku-console main settings.
    async fn get_console_management(
        &self,
        context: &C) -> Result<GetConsoleManagementResponse, ApiError>;

    /// Set current potku-console main settings
    async fn post_console_management(
        &self,
        body: models::ConsoleSettings,
        context: &C) -> Result<PostConsoleManagementResponse, ApiError>;

    /// Get specific server details.
    async fn get_service_server(
        &self,
        service_id: i32,
        server_id: i32,
        context: &C) -> Result<GetServiceServerResponse, ApiError>;

    /// Get specific server's permission configurations.
    async fn get_service_server_permissions(
        &self,
        service_id: i32,
        server_id: i32,
        context: &C) -> Result<GetServiceServerPermissionsResponse, ApiError>;

    /// Get server's credentials.
    async fn get_service_server_server_credentials(
        &self,
        service_id: i32,
        server_id: i32,
        context: &C) -> Result<GetServiceServerServerCredentialsResponse, ApiError>;

    /// Get specific server's credentials permission configurations.
    async fn get_service_server_server_credentials_permissions(
        &self,
        service_id: i32,
        server_id: i32,
        context: &C) -> Result<GetServiceServerServerCredentialsPermissionsResponse, ApiError>;

    /// Get all server details from specific service instance.
    async fn get_service_servers(
        &self,
        service_id: i32,
        context: &C) -> Result<GetServiceServersResponse, ApiError>;

    /// Validate that cloud provider services can be controlled with the given details.
    async fn post_service_server_validate(
        &self,
        service_id: i32,
        server_id: i32,
        context: &C) -> Result<PostServiceServerValidateResponse, ApiError>;

    /// Add new server to the service.
    async fn post_service_servers(
        &self,
        service_id: i32,
        body: models::PostNewServer,
        context: &C) -> Result<PostServiceServersResponse, ApiError>;

    /// Update server details (mainly done by service modules).
    async fn put_service_server(
        &self,
        service_id: i32,
        server_id: i32,
        body: models::Server,
        context: &C) -> Result<PutServiceServerResponse, ApiError>;

    /// Set specific server's permission configurations.
    async fn put_service_server_permissions(
        &self,
        service_id: i32,
        server_id: i32,
        body: models::Permissions,
        context: &C) -> Result<PutServiceServerPermissionsResponse, ApiError>;

    /// Set server's credentials.
    async fn put_service_server_server_credentials(
        &self,
        service_id: i32,
        server_id: i32,
        body: models::ServerCredentials,
        context: &C) -> Result<PutServiceServerServerCredentialsResponse, ApiError>;

    /// Set specific server's credentials permission configurations.
    async fn put_service_server_server_credentials_permissions(
        &self,
        service_id: i32,
        server_id: i32,
        body: models::Permissions,
        context: &C) -> Result<PutServiceServerServerCredentialsPermissionsResponse, ApiError>;

    /// Remove service module entirely from the system.
    async fn delete_service_module(
        &self,
        service_module_uuid: uuid::Uuid,
        context: &C) -> Result<DeleteServiceModuleResponse, ApiError>;

    /// Get specific service module details.
    async fn get_service_module(
        &self,
        service_module_uuid: uuid::Uuid,
        context: &C) -> Result<GetServiceModuleResponse, ApiError>;

    /// Get service module endpoint permissions.
    async fn get_service_module_permissions(
        &self,
        service_module_uuid: uuid::Uuid,
        context: &C) -> Result<GetServiceModulePermissionsResponse, ApiError>;

    /// Get list of available service modules.
    async fn get_service_modules(
        &self,
        context: &C) -> Result<GetServiceModulesResponse, ApiError>;

    /// Enable, disable or change configurations of the service module.
    async fn post_service_module(
        &self,
        service_module_uuid: uuid::Uuid,
        body: models::ServiceModule,
        context: &C) -> Result<PostServiceModuleResponse, ApiError>;

    /// Upload new service module to the server using zip packet.
    async fn post_service_modules(
        &self,
        body: swagger::ByteArray,
        context: &C) -> Result<PostServiceModulesResponse, ApiError>;

    /// Set new cloud module endpoint permissions.
    async fn put_service_module_permissions(
        &self,
        service_module_uuid: uuid::Uuid,
        body: models::Permissions,
        context: &C) -> Result<PutServiceModulePermissionsResponse, ApiError>;

    /// Create a new service.
    async fn create_service(
        &self,
        body: models::PostNewService,
        context: &C) -> Result<CreateServiceResponse, ApiError>;

    /// Delete service and all of its servers.
    async fn delete_service(
        &self,
        service_id: i32,
        context: &C) -> Result<DeleteServiceResponse, ApiError>;

    /// Get service details from specific service instance.
    async fn get_service(
        &self,
        service_id: i32,
        context: &C) -> Result<GetServiceResponse, ApiError>;

    /// Get specific service permission configurations
    async fn get_service_permissions(
        &self,
        service_id: i32,
        context: &C) -> Result<GetServicePermissionsResponse, ApiError>;

    /// Get all service names and IDs from current user.
    async fn get_services(
        &self,
        port: Option<i32>,
        dns: Option<String>,
        context: &C) -> Result<GetServicesResponse, ApiError>;

    /// Get service endpoint permissions.
    async fn get_services_permissions(
        &self,
        context: &C) -> Result<GetServicesPermissionsResponse, ApiError>;

    /// Update service details and desired state (start/stop service) related to specific service instance.
    async fn post_service(
        &self,
        service_id: i32,
        body: models::Service,
        context: &C) -> Result<PostServiceResponse, ApiError>;

    /// Set new service specific permissions.
    async fn put_service_permissions(
        &self,
        service_id: i32,
        body: models::Permissions,
        context: &C) -> Result<PutServicePermissionsResponse, ApiError>;

    /// Set new service endpoint permissions.
    async fn put_services_permissions(
        &self,
        body: models::Permissions,
        context: &C) -> Result<PutServicesPermissionsResponse, ApiError>;

    /// Remove user from the system.
    async fn delete_user(
        &self,
        user_id: i32,
        context: &C) -> Result<DeleteUserResponse, ApiError>;

    /// Get specific user details.
    async fn get_user(
        &self,
        user_id: i32,
        context: &C) -> Result<GetUserResponse, ApiError>;

    /// Get all users.
    async fn get_users(
        &self,
        context: &C) -> Result<GetUsersResponse, ApiError>;

    /// Create a new user.
    async fn post_user(
        &self,
        body: models::UserContent,
        context: &C) -> Result<PostUserResponse, ApiError>;

    /// Login to user account.
    async fn post_user_login(
        &self,
        body: models::ExistingUserCredentials,
        context: &C) -> Result<PostUserLoginResponse, ApiError>;

    /// Alter service state.
    async fn put_user(
        &self,
        user_id: i32,
        body: models::User,
        context: &C) -> Result<PutUserResponse, ApiError>;

}

/// API where `Context` isn't passed on every API call
#[async_trait]
pub trait ApiNoContext<C: Send + Sync> {

    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>>;

    fn context(&self) -> &C;

    /// Remove cloud module entirely from the system.
    async fn delete_cloud_module(
        &self,
        cloud_module_uuid: uuid::Uuid,
        ) -> Result<DeleteCloudModuleResponse, ApiError>;

    /// Delete specific cloud account.
    async fn delete_cloud_module_account(
        &self,
        cloud_module_uuid: uuid::Uuid,
        cloud_account_id: String,
        ) -> Result<DeleteCloudModuleAccountResponse, ApiError>;

    /// Get specific cloud module details.
    async fn get_cloud_module(
        &self,
        cloud_module_uuid: uuid::Uuid,
        ) -> Result<GetCloudModuleResponse, ApiError>;

    /// Get specific cloud account details.
    async fn get_cloud_module_account(
        &self,
        cloud_module_uuid: uuid::Uuid,
        cloud_account_id: String,
        ) -> Result<GetCloudModuleAccountResponse, ApiError>;

    /// Get specific cloud account permissions.
    async fn get_cloud_module_account_permissions(
        &self,
        cloud_module_uuid: uuid::Uuid,
        cloud_account_id: String,
        ) -> Result<GetCloudModuleAccountPermissionsResponse, ApiError>;

    /// Get cloud account related to specific cloud module.
    async fn get_cloud_module_accounts(
        &self,
        cloud_module_uuid: uuid::Uuid,
        ) -> Result<GetCloudModuleAccountsResponse, ApiError>;

    /// Get cloud module endpoint permissions.
    async fn get_cloud_module_permissions(
        &self,
        cloud_module_uuid: uuid::Uuid,
        ) -> Result<GetCloudModulePermissionsResponse, ApiError>;

    /// Get list of available cloud modules.
    async fn get_cloud_modules(
        &self,
        ) -> Result<GetCloudModulesResponse, ApiError>;

    /// Enable, disable or change configurations of the cloud module.
    async fn post_cloud_module(
        &self,
        cloud_module_uuid: uuid::Uuid,
        body: models::ServiceModule,
        ) -> Result<PostCloudModuleResponse, ApiError>;

    /// Input new cloud account details.
    async fn post_cloud_module_accounts(
        &self,
        cloud_module_uuid: uuid::Uuid,
        body: models::CloudAccount,
        ) -> Result<PostCloudModuleAccountsResponse, ApiError>;

    /// Upload new cloud module to the server using zip packet.
    async fn post_cloud_modules(
        &self,
        body: swagger::ByteArray,
        ) -> Result<PostCloudModulesResponse, ApiError>;

    /// Update existing cloud account details.
    async fn put_cloud_module_account(
        &self,
        cloud_module_uuid: uuid::Uuid,
        cloud_account_id: String,
        body: models::CloudAccount,
        ) -> Result<PutCloudModuleAccountResponse, ApiError>;

    /// Set new cloud module endpoint permissions.
    async fn put_cloud_module_account_permissions(
        &self,
        cloud_module_uuid: uuid::Uuid,
        cloud_account_id: String,
        body: models::Permissions,
        ) -> Result<PutCloudModuleAccountPermissionsResponse, ApiError>;

    /// Set new cloud module endpoint permissions.
    async fn put_cloud_module_permissions(
        &self,
        cloud_module_uuid: uuid::Uuid,
        body: models::Permissions,
        ) -> Result<PutCloudModulePermissionsResponse, ApiError>;

    /// Get cpu amount profiles.
    async fn get_cpu_cores(
        &self,
        cloud_module_uuid: uuid::Uuid,
        ) -> Result<GetCpuCoresResponse, ApiError>;

    /// Get minimum and maximum disk available.
    async fn get_disk_bounds(
        &self,
        cloud_module_uuid: uuid::Uuid,
        ) -> Result<GetDiskBoundsResponse, ApiError>;

    /// Get available machine types.
    async fn get_machine_types(
        &self,
        cloud_module_uuid: uuid::Uuid,
        ) -> Result<GetMachineTypesResponse, ApiError>;

    /// Get available operating system names.
    async fn get_operating_systems(
        &self,
        cloud_module_uuid: uuid::Uuid,
        ) -> Result<GetOperatingSystemsResponse, ApiError>;

    /// Get available compute orientation names.
    async fn get_orientations(
        &self,
        cloud_module_uuid: uuid::Uuid,
        ) -> Result<GetOrientationsResponse, ApiError>;

    /// Get minimum and maximum ram available.
    async fn get_ram_bounds(
        &self,
        cloud_module_uuid: uuid::Uuid,
        ) -> Result<GetRamBoundsResponse, ApiError>;

    /// Remove group from the system.
    async fn delete_group(
        &self,
        group_id: i32,
        ) -> Result<DeleteGroupResponse, ApiError>;

    /// Get specific permission group details.
    async fn get_group(
        &self,
        group_id: i32,
        ) -> Result<GetGroupResponse, ApiError>;

    /// Get all groups.
    async fn get_groups(
        &self,
        ) -> Result<GetGroupsResponse, ApiError>;

    /// Create a new permission group.
    async fn post_group(
        &self,
        body: models::GroupContent,
        ) -> Result<PostGroupResponse, ApiError>;

    /// Alter permission group name or users.
    async fn put_group(
        &self,
        group_id: i32,
        body: models::Group,
        ) -> Result<PutGroupResponse, ApiError>;

    /// Get current potku-console main settings.
    async fn get_console_management(
        &self,
        ) -> Result<GetConsoleManagementResponse, ApiError>;

    /// Set current potku-console main settings
    async fn post_console_management(
        &self,
        body: models::ConsoleSettings,
        ) -> Result<PostConsoleManagementResponse, ApiError>;

    /// Get specific server details.
    async fn get_service_server(
        &self,
        service_id: i32,
        server_id: i32,
        ) -> Result<GetServiceServerResponse, ApiError>;

    /// Get specific server's permission configurations.
    async fn get_service_server_permissions(
        &self,
        service_id: i32,
        server_id: i32,
        ) -> Result<GetServiceServerPermissionsResponse, ApiError>;

    /// Get server's credentials.
    async fn get_service_server_server_credentials(
        &self,
        service_id: i32,
        server_id: i32,
        ) -> Result<GetServiceServerServerCredentialsResponse, ApiError>;

    /// Get specific server's credentials permission configurations.
    async fn get_service_server_server_credentials_permissions(
        &self,
        service_id: i32,
        server_id: i32,
        ) -> Result<GetServiceServerServerCredentialsPermissionsResponse, ApiError>;

    /// Get all server details from specific service instance.
    async fn get_service_servers(
        &self,
        service_id: i32,
        ) -> Result<GetServiceServersResponse, ApiError>;

    /// Validate that cloud provider services can be controlled with the given details.
    async fn post_service_server_validate(
        &self,
        service_id: i32,
        server_id: i32,
        ) -> Result<PostServiceServerValidateResponse, ApiError>;

    /// Add new server to the service.
    async fn post_service_servers(
        &self,
        service_id: i32,
        body: models::PostNewServer,
        ) -> Result<PostServiceServersResponse, ApiError>;

    /// Update server details (mainly done by service modules).
    async fn put_service_server(
        &self,
        service_id: i32,
        server_id: i32,
        body: models::Server,
        ) -> Result<PutServiceServerResponse, ApiError>;

    /// Set specific server's permission configurations.
    async fn put_service_server_permissions(
        &self,
        service_id: i32,
        server_id: i32,
        body: models::Permissions,
        ) -> Result<PutServiceServerPermissionsResponse, ApiError>;

    /// Set server's credentials.
    async fn put_service_server_server_credentials(
        &self,
        service_id: i32,
        server_id: i32,
        body: models::ServerCredentials,
        ) -> Result<PutServiceServerServerCredentialsResponse, ApiError>;

    /// Set specific server's credentials permission configurations.
    async fn put_service_server_server_credentials_permissions(
        &self,
        service_id: i32,
        server_id: i32,
        body: models::Permissions,
        ) -> Result<PutServiceServerServerCredentialsPermissionsResponse, ApiError>;

    /// Remove service module entirely from the system.
    async fn delete_service_module(
        &self,
        service_module_uuid: uuid::Uuid,
        ) -> Result<DeleteServiceModuleResponse, ApiError>;

    /// Get specific service module details.
    async fn get_service_module(
        &self,
        service_module_uuid: uuid::Uuid,
        ) -> Result<GetServiceModuleResponse, ApiError>;

    /// Get service module endpoint permissions.
    async fn get_service_module_permissions(
        &self,
        service_module_uuid: uuid::Uuid,
        ) -> Result<GetServiceModulePermissionsResponse, ApiError>;

    /// Get list of available service modules.
    async fn get_service_modules(
        &self,
        ) -> Result<GetServiceModulesResponse, ApiError>;

    /// Enable, disable or change configurations of the service module.
    async fn post_service_module(
        &self,
        service_module_uuid: uuid::Uuid,
        body: models::ServiceModule,
        ) -> Result<PostServiceModuleResponse, ApiError>;

    /// Upload new service module to the server using zip packet.
    async fn post_service_modules(
        &self,
        body: swagger::ByteArray,
        ) -> Result<PostServiceModulesResponse, ApiError>;

    /// Set new cloud module endpoint permissions.
    async fn put_service_module_permissions(
        &self,
        service_module_uuid: uuid::Uuid,
        body: models::Permissions,
        ) -> Result<PutServiceModulePermissionsResponse, ApiError>;

    /// Create a new service.
    async fn create_service(
        &self,
        body: models::PostNewService,
        ) -> Result<CreateServiceResponse, ApiError>;

    /// Delete service and all of its servers.
    async fn delete_service(
        &self,
        service_id: i32,
        ) -> Result<DeleteServiceResponse, ApiError>;

    /// Get service details from specific service instance.
    async fn get_service(
        &self,
        service_id: i32,
        ) -> Result<GetServiceResponse, ApiError>;

    /// Get specific service permission configurations
    async fn get_service_permissions(
        &self,
        service_id: i32,
        ) -> Result<GetServicePermissionsResponse, ApiError>;

    /// Get all service names and IDs from current user.
    async fn get_services(
        &self,
        port: Option<i32>,
        dns: Option<String>,
        ) -> Result<GetServicesResponse, ApiError>;

    /// Get service endpoint permissions.
    async fn get_services_permissions(
        &self,
        ) -> Result<GetServicesPermissionsResponse, ApiError>;

    /// Update service details and desired state (start/stop service) related to specific service instance.
    async fn post_service(
        &self,
        service_id: i32,
        body: models::Service,
        ) -> Result<PostServiceResponse, ApiError>;

    /// Set new service specific permissions.
    async fn put_service_permissions(
        &self,
        service_id: i32,
        body: models::Permissions,
        ) -> Result<PutServicePermissionsResponse, ApiError>;

    /// Set new service endpoint permissions.
    async fn put_services_permissions(
        &self,
        body: models::Permissions,
        ) -> Result<PutServicesPermissionsResponse, ApiError>;

    /// Remove user from the system.
    async fn delete_user(
        &self,
        user_id: i32,
        ) -> Result<DeleteUserResponse, ApiError>;

    /// Get specific user details.
    async fn get_user(
        &self,
        user_id: i32,
        ) -> Result<GetUserResponse, ApiError>;

    /// Get all users.
    async fn get_users(
        &self,
        ) -> Result<GetUsersResponse, ApiError>;

    /// Create a new user.
    async fn post_user(
        &self,
        body: models::UserContent,
        ) -> Result<PostUserResponse, ApiError>;

    /// Login to user account.
    async fn post_user_login(
        &self,
        body: models::ExistingUserCredentials,
        ) -> Result<PostUserLoginResponse, ApiError>;

    /// Alter service state.
    async fn put_user(
        &self,
        user_id: i32,
        body: models::User,
        ) -> Result<PutUserResponse, ApiError>;

}

/// Trait to extend an API to make it easy to bind it to a context.
pub trait ContextWrapperExt<C: Send + Sync> where Self: Sized
{
    /// Binds this API to a context.
    fn with_context(self: Self, context: C) -> ContextWrapper<Self, C>;
}

impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ContextWrapperExt<C> for T {
    fn with_context(self: T, context: C) -> ContextWrapper<T, C> {
         ContextWrapper::<T, C>::new(self, context)
    }
}

#[async_trait]
impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ApiNoContext<C> for ContextWrapper<T, C> {
    fn poll_ready(&self, cx: &mut Context) -> Poll<Result<(), ServiceError>> {
        self.api().poll_ready(cx)
    }

    fn context(&self) -> &C {
        ContextWrapper::context(self)
    }

    /// Remove cloud module entirely from the system.
    async fn delete_cloud_module(
        &self,
        cloud_module_uuid: uuid::Uuid,
        ) -> Result<DeleteCloudModuleResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().delete_cloud_module(cloud_module_uuid, &context).await
    }

    /// Delete specific cloud account.
    async fn delete_cloud_module_account(
        &self,
        cloud_module_uuid: uuid::Uuid,
        cloud_account_id: String,
        ) -> Result<DeleteCloudModuleAccountResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().delete_cloud_module_account(cloud_module_uuid, cloud_account_id, &context).await
    }

    /// Get specific cloud module details.
    async fn get_cloud_module(
        &self,
        cloud_module_uuid: uuid::Uuid,
        ) -> Result<GetCloudModuleResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_cloud_module(cloud_module_uuid, &context).await
    }

    /// Get specific cloud account details.
    async fn get_cloud_module_account(
        &self,
        cloud_module_uuid: uuid::Uuid,
        cloud_account_id: String,
        ) -> Result<GetCloudModuleAccountResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_cloud_module_account(cloud_module_uuid, cloud_account_id, &context).await
    }

    /// Get specific cloud account permissions.
    async fn get_cloud_module_account_permissions(
        &self,
        cloud_module_uuid: uuid::Uuid,
        cloud_account_id: String,
        ) -> Result<GetCloudModuleAccountPermissionsResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_cloud_module_account_permissions(cloud_module_uuid, cloud_account_id, &context).await
    }

    /// Get cloud account related to specific cloud module.
    async fn get_cloud_module_accounts(
        &self,
        cloud_module_uuid: uuid::Uuid,
        ) -> Result<GetCloudModuleAccountsResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_cloud_module_accounts(cloud_module_uuid, &context).await
    }

    /// Get cloud module endpoint permissions.
    async fn get_cloud_module_permissions(
        &self,
        cloud_module_uuid: uuid::Uuid,
        ) -> Result<GetCloudModulePermissionsResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_cloud_module_permissions(cloud_module_uuid, &context).await
    }

    /// Get list of available cloud modules.
    async fn get_cloud_modules(
        &self,
        ) -> Result<GetCloudModulesResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_cloud_modules(&context).await
    }

    /// Enable, disable or change configurations of the cloud module.
    async fn post_cloud_module(
        &self,
        cloud_module_uuid: uuid::Uuid,
        body: models::ServiceModule,
        ) -> Result<PostCloudModuleResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().post_cloud_module(cloud_module_uuid, body, &context).await
    }

    /// Input new cloud account details.
    async fn post_cloud_module_accounts(
        &self,
        cloud_module_uuid: uuid::Uuid,
        body: models::CloudAccount,
        ) -> Result<PostCloudModuleAccountsResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().post_cloud_module_accounts(cloud_module_uuid, body, &context).await
    }

    /// Upload new cloud module to the server using zip packet.
    async fn post_cloud_modules(
        &self,
        body: swagger::ByteArray,
        ) -> Result<PostCloudModulesResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().post_cloud_modules(body, &context).await
    }

    /// Update existing cloud account details.
    async fn put_cloud_module_account(
        &self,
        cloud_module_uuid: uuid::Uuid,
        cloud_account_id: String,
        body: models::CloudAccount,
        ) -> Result<PutCloudModuleAccountResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().put_cloud_module_account(cloud_module_uuid, cloud_account_id, body, &context).await
    }

    /// Set new cloud module endpoint permissions.
    async fn put_cloud_module_account_permissions(
        &self,
        cloud_module_uuid: uuid::Uuid,
        cloud_account_id: String,
        body: models::Permissions,
        ) -> Result<PutCloudModuleAccountPermissionsResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().put_cloud_module_account_permissions(cloud_module_uuid, cloud_account_id, body, &context).await
    }

    /// Set new cloud module endpoint permissions.
    async fn put_cloud_module_permissions(
        &self,
        cloud_module_uuid: uuid::Uuid,
        body: models::Permissions,
        ) -> Result<PutCloudModulePermissionsResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().put_cloud_module_permissions(cloud_module_uuid, body, &context).await
    }

    /// Get cpu amount profiles.
    async fn get_cpu_cores(
        &self,
        cloud_module_uuid: uuid::Uuid,
        ) -> Result<GetCpuCoresResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_cpu_cores(cloud_module_uuid, &context).await
    }

    /// Get minimum and maximum disk available.
    async fn get_disk_bounds(
        &self,
        cloud_module_uuid: uuid::Uuid,
        ) -> Result<GetDiskBoundsResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_disk_bounds(cloud_module_uuid, &context).await
    }

    /// Get available machine types.
    async fn get_machine_types(
        &self,
        cloud_module_uuid: uuid::Uuid,
        ) -> Result<GetMachineTypesResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_machine_types(cloud_module_uuid, &context).await
    }

    /// Get available operating system names.
    async fn get_operating_systems(
        &self,
        cloud_module_uuid: uuid::Uuid,
        ) -> Result<GetOperatingSystemsResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_operating_systems(cloud_module_uuid, &context).await
    }

    /// Get available compute orientation names.
    async fn get_orientations(
        &self,
        cloud_module_uuid: uuid::Uuid,
        ) -> Result<GetOrientationsResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_orientations(cloud_module_uuid, &context).await
    }

    /// Get minimum and maximum ram available.
    async fn get_ram_bounds(
        &self,
        cloud_module_uuid: uuid::Uuid,
        ) -> Result<GetRamBoundsResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_ram_bounds(cloud_module_uuid, &context).await
    }

    /// Remove group from the system.
    async fn delete_group(
        &self,
        group_id: i32,
        ) -> Result<DeleteGroupResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().delete_group(group_id, &context).await
    }

    /// Get specific permission group details.
    async fn get_group(
        &self,
        group_id: i32,
        ) -> Result<GetGroupResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_group(group_id, &context).await
    }

    /// Get all groups.
    async fn get_groups(
        &self,
        ) -> Result<GetGroupsResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_groups(&context).await
    }

    /// Create a new permission group.
    async fn post_group(
        &self,
        body: models::GroupContent,
        ) -> Result<PostGroupResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().post_group(body, &context).await
    }

    /// Alter permission group name or users.
    async fn put_group(
        &self,
        group_id: i32,
        body: models::Group,
        ) -> Result<PutGroupResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().put_group(group_id, body, &context).await
    }

    /// Get current potku-console main settings.
    async fn get_console_management(
        &self,
        ) -> Result<GetConsoleManagementResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_console_management(&context).await
    }

    /// Set current potku-console main settings
    async fn post_console_management(
        &self,
        body: models::ConsoleSettings,
        ) -> Result<PostConsoleManagementResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().post_console_management(body, &context).await
    }

    /// Get specific server details.
    async fn get_service_server(
        &self,
        service_id: i32,
        server_id: i32,
        ) -> Result<GetServiceServerResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_service_server(service_id, server_id, &context).await
    }

    /// Get specific server's permission configurations.
    async fn get_service_server_permissions(
        &self,
        service_id: i32,
        server_id: i32,
        ) -> Result<GetServiceServerPermissionsResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_service_server_permissions(service_id, server_id, &context).await
    }

    /// Get server's credentials.
    async fn get_service_server_server_credentials(
        &self,
        service_id: i32,
        server_id: i32,
        ) -> Result<GetServiceServerServerCredentialsResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_service_server_server_credentials(service_id, server_id, &context).await
    }

    /// Get specific server's credentials permission configurations.
    async fn get_service_server_server_credentials_permissions(
        &self,
        service_id: i32,
        server_id: i32,
        ) -> Result<GetServiceServerServerCredentialsPermissionsResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_service_server_server_credentials_permissions(service_id, server_id, &context).await
    }

    /// Get all server details from specific service instance.
    async fn get_service_servers(
        &self,
        service_id: i32,
        ) -> Result<GetServiceServersResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_service_servers(service_id, &context).await
    }

    /// Validate that cloud provider services can be controlled with the given details.
    async fn post_service_server_validate(
        &self,
        service_id: i32,
        server_id: i32,
        ) -> Result<PostServiceServerValidateResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().post_service_server_validate(service_id, server_id, &context).await
    }

    /// Add new server to the service.
    async fn post_service_servers(
        &self,
        service_id: i32,
        body: models::PostNewServer,
        ) -> Result<PostServiceServersResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().post_service_servers(service_id, body, &context).await
    }

    /// Update server details (mainly done by service modules).
    async fn put_service_server(
        &self,
        service_id: i32,
        server_id: i32,
        body: models::Server,
        ) -> Result<PutServiceServerResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().put_service_server(service_id, server_id, body, &context).await
    }

    /// Set specific server's permission configurations.
    async fn put_service_server_permissions(
        &self,
        service_id: i32,
        server_id: i32,
        body: models::Permissions,
        ) -> Result<PutServiceServerPermissionsResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().put_service_server_permissions(service_id, server_id, body, &context).await
    }

    /// Set server's credentials.
    async fn put_service_server_server_credentials(
        &self,
        service_id: i32,
        server_id: i32,
        body: models::ServerCredentials,
        ) -> Result<PutServiceServerServerCredentialsResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().put_service_server_server_credentials(service_id, server_id, body, &context).await
    }

    /// Set specific server's credentials permission configurations.
    async fn put_service_server_server_credentials_permissions(
        &self,
        service_id: i32,
        server_id: i32,
        body: models::Permissions,
        ) -> Result<PutServiceServerServerCredentialsPermissionsResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().put_service_server_server_credentials_permissions(service_id, server_id, body, &context).await
    }

    /// Remove service module entirely from the system.
    async fn delete_service_module(
        &self,
        service_module_uuid: uuid::Uuid,
        ) -> Result<DeleteServiceModuleResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().delete_service_module(service_module_uuid, &context).await
    }

    /// Get specific service module details.
    async fn get_service_module(
        &self,
        service_module_uuid: uuid::Uuid,
        ) -> Result<GetServiceModuleResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_service_module(service_module_uuid, &context).await
    }

    /// Get service module endpoint permissions.
    async fn get_service_module_permissions(
        &self,
        service_module_uuid: uuid::Uuid,
        ) -> Result<GetServiceModulePermissionsResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_service_module_permissions(service_module_uuid, &context).await
    }

    /// Get list of available service modules.
    async fn get_service_modules(
        &self,
        ) -> Result<GetServiceModulesResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_service_modules(&context).await
    }

    /// Enable, disable or change configurations of the service module.
    async fn post_service_module(
        &self,
        service_module_uuid: uuid::Uuid,
        body: models::ServiceModule,
        ) -> Result<PostServiceModuleResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().post_service_module(service_module_uuid, body, &context).await
    }

    /// Upload new service module to the server using zip packet.
    async fn post_service_modules(
        &self,
        body: swagger::ByteArray,
        ) -> Result<PostServiceModulesResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().post_service_modules(body, &context).await
    }

    /// Set new cloud module endpoint permissions.
    async fn put_service_module_permissions(
        &self,
        service_module_uuid: uuid::Uuid,
        body: models::Permissions,
        ) -> Result<PutServiceModulePermissionsResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().put_service_module_permissions(service_module_uuid, body, &context).await
    }

    /// Create a new service.
    async fn create_service(
        &self,
        body: models::PostNewService,
        ) -> Result<CreateServiceResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().create_service(body, &context).await
    }

    /// Delete service and all of its servers.
    async fn delete_service(
        &self,
        service_id: i32,
        ) -> Result<DeleteServiceResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().delete_service(service_id, &context).await
    }

    /// Get service details from specific service instance.
    async fn get_service(
        &self,
        service_id: i32,
        ) -> Result<GetServiceResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_service(service_id, &context).await
    }

    /// Get specific service permission configurations
    async fn get_service_permissions(
        &self,
        service_id: i32,
        ) -> Result<GetServicePermissionsResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_service_permissions(service_id, &context).await
    }

    /// Get all service names and IDs from current user.
    async fn get_services(
        &self,
        port: Option<i32>,
        dns: Option<String>,
        ) -> Result<GetServicesResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_services(port, dns, &context).await
    }

    /// Get service endpoint permissions.
    async fn get_services_permissions(
        &self,
        ) -> Result<GetServicesPermissionsResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_services_permissions(&context).await
    }

    /// Update service details and desired state (start/stop service) related to specific service instance.
    async fn post_service(
        &self,
        service_id: i32,
        body: models::Service,
        ) -> Result<PostServiceResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().post_service(service_id, body, &context).await
    }

    /// Set new service specific permissions.
    async fn put_service_permissions(
        &self,
        service_id: i32,
        body: models::Permissions,
        ) -> Result<PutServicePermissionsResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().put_service_permissions(service_id, body, &context).await
    }

    /// Set new service endpoint permissions.
    async fn put_services_permissions(
        &self,
        body: models::Permissions,
        ) -> Result<PutServicesPermissionsResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().put_services_permissions(body, &context).await
    }

    /// Remove user from the system.
    async fn delete_user(
        &self,
        user_id: i32,
        ) -> Result<DeleteUserResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().delete_user(user_id, &context).await
    }

    /// Get specific user details.
    async fn get_user(
        &self,
        user_id: i32,
        ) -> Result<GetUserResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_user(user_id, &context).await
    }

    /// Get all users.
    async fn get_users(
        &self,
        ) -> Result<GetUsersResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_users(&context).await
    }

    /// Create a new user.
    async fn post_user(
        &self,
        body: models::UserContent,
        ) -> Result<PostUserResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().post_user(body, &context).await
    }

    /// Login to user account.
    async fn post_user_login(
        &self,
        body: models::ExistingUserCredentials,
        ) -> Result<PostUserLoginResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().post_user_login(body, &context).await
    }

    /// Alter service state.
    async fn put_user(
        &self,
        user_id: i32,
        body: models::User,
        ) -> Result<PutUserResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().put_user(user_id, body, &context).await
    }

}


#[cfg(feature = "client")]
pub mod client;

// Re-export Client as a top-level name
#[cfg(feature = "client")]
pub use client::Client;

#[cfg(feature = "server")]
pub mod server;

// Re-export router() as a top-level name
#[cfg(feature = "server")]
pub use self::server::Service;

#[cfg(feature = "server")]
pub mod context;

pub mod models;

#[cfg(any(feature = "client", feature = "server"))]
pub(crate) mod header;
