#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum INeurayStartResult {
    Success = 0,
    UnspecifiedFailure = -1,
    ChallengeResponseAuthenticationFailure = -2,
    SpmAuthenticationFailure = -3,
    ProvidedLicenseExpired = -4,
    NoProfessionalGpuFound = -5,
    FlexLmAuthenticationFailure = -6,
    NoNvidiaVcaFound = -7,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum INeurayShutdownResult {
    Success = 0,
    UnspecifiedFailure = -1,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AddPathResult {
    Success = 0,
    InvalidParameters = -1,
    InvalidPath = -2,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BooleanResult {
    Success = 0,
    Failure = -1,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SetOptionResult {
    Success = 0,
    InvalidOptionName = -1,
    WrongType = -2,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BackendSetOptionResult {
    Success = 0,
    UnknownOption = -1,
    UnsupportedValue = -2,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LoadModuleResult {
    Success = 0,
    ModuleNameInvalid = -1,
    FailedToLoadModule = -2,
    NameAlreadyInUse = -3,
    InitializationFailed = -4,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TransactionCommitResult {
    Success = 0,
    UnspecifiedFailure = -1,
    TransactionAlreadyClosed = -2,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CreateMaterialInstanceResult {
    Success = 0,
    NonExistingParameter = -1,
    ParameterTypeMismatch = -2,
    NonDefaultParameterMissing = -3,
    DefinitionNotExported = -4,
    UniformParameterGivenVaryingArgument = -5,
    ArgumentExpressionNotConstantNorCall = -6,
    UniformParameterGivenCallExpression = -8,
}
